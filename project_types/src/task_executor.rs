use futures::channel::mpsc::Sender;
use futures::prelude::*;
use slog::{crit, debug, o, trace};
use std::sync::Weak;
use tokio::runtime::{Handle, Runtime};

pub use tokio::task::JoinHandle;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShutdownReason {
    Success(&'static str),
    Failure(&'static str),
}

impl ShutdownReason {
    pub fn message(&self) -> &'static str {
        match self {
            ShutdownReason::Success(msg) => msg,
            ShutdownReason::Failure(msg) => msg,
        }
    }
}

#[derive(Clone)]
pub enum HandleProvider {
    Runtime(Weak<Runtime>),
    Handle(Handle),
}

impl From<Handle> for HandleProvider {
    fn from(handle: Handle) -> Self {
        HandleProvider::Handle(handle)
    }
}

impl From<Weak<Runtime>> for HandleProvider {
    fn from(weak_runtime: Weak<Runtime>) -> Self {
        HandleProvider::Runtime(weak_runtime)
    }
}

impl HandleProvider {
    pub fn handle(&self) -> Option<Handle> {
        match self {
            HandleProvider::Runtime(weak_runtime) => weak_runtime
                .upgrade()
                .map(|runtime| runtime.handle().clone()),
            HandleProvider::Handle(handle) => Some(handle.clone()),
        }
    }
}

#[derive(Clone)]
pub struct TaskExecutor {
    handle_provider: HandleProvider,
    exit: exit_future::Exit,
    signal_tx: Sender<ShutdownReason>,
    log: slog::Logger,
}

impl TaskExecutor {
    pub fn new<T: Into<HandleProvider>>(
        handle: T,
        exit: exit_future::Exit,
        log: slog::Logger,
        signal_tx: Sender<ShutdownReason>,
    ) -> Self {
        Self {
            handle_provider: handle.into(),
            exit,
            signal_tx,
            log,
        }
    }

    pub fn clone_with_name(&self, service_name: String) -> Self {
        TaskExecutor {
            handle_provider: self.handle_provider.clone(),
            exit: self.exit.clone(),
            signal_tx: self.signal_tx.clone(),
            log: self.log.new(o!("service" => service_name)),
        }
    }

    pub fn spawn_ignoring_error(
        &self,
        task: impl Future<Output = Result<(), ()>> + Send + 'static,
        name: &'static str,
    ) {
        self.spawn(task.map(|_| ()), name)
    }

    fn spawn_monitor<R: Send>(
        &self,
        task_handle: impl Future<Output = Result<R, tokio::task::JoinError>> + Send + 'static,
        name: &'static str,
    ) {
        let mut shutdown_sender = self.shutdown_sender();
        let log = self.log.clone();

        if let Some(handle) = self.handle() {
            handle.spawn(async move {
                if let Err(join_error) = task_handle.await {
                    if let Ok(panic) = join_error.try_into_panic() {
                        let message = panic.downcast_ref::<&str>().unwrap_or(&"<none>");

                        crit!(
                            log,
                            "Task panic. This is a bug!";
                            "task_name" => name,
                            "message" => message,
                            "advice" => "Please check above for a backtrace and notify \
                                         the developers"
                        );
                        let _ = shutdown_sender
                            .try_send(ShutdownReason::Failure("Panic (fatal error)"));
                    }
                }
            });
        } else {
            debug!(
                self.log,
                "Couldn't spawn monitor task. Runtime shutting down"
            )
        }
    }

    pub fn spawn(&self, task: impl Future<Output = ()> + Send + 'static, name: &'static str) {
        if let Some(handle) = self.handle() {
            handle.spawn(task);
        } else {
            debug!(self.log, "Couldn't spawn task. Runtime shutting down");
        }
    }

    pub fn spawn_without_exit(
        &self,
        task: impl Future<Output = ()> + Send + 'static,
        name: &'static str,
    ) {
        if let Some(handle) = self.handle() {
            handle.spawn(task);
        } else {
            debug!(self.log, "Couldn't spawn task. Runtime shutting down");
        }
    }

    pub fn spawn_blocking<F>(&self, task: F, name: &'static str)
    where
        F: FnOnce() + Send + 'static,
    {
        if let Some(task_handle) = self.spawn_blocking_handle(task, name) {
            self.spawn_monitor(task_handle, name)
        }
    }

    pub fn spawn_handle<R: Send + 'static>(
        &self,
        task: impl Future<Output = R> + Send + 'static,
        name: &'static str,
    ) -> Option<tokio::task::JoinHandle<Option<R>>> {
        let exit = self.exit.clone();
        let log = self.log.clone();

        let future = future::select(Box::pin(task), exit).then(move |either| {
            let result = match either {
                future::Either::Left((value, _)) => {
                    trace!(log, "Async task completed"; "task" => name);
                    Some(value)
                }
                future::Either::Right(_) => {
                    debug!(log, "Async task shutdown, exit received"; "task" => name);
                    None
                }
            };
            futures::future::ready(result)
        });

        if let Some(handle) = self.handle() {
            Some(handle.spawn(future))
        } else {
            debug!(self.log, "Couldn't spawn task. Runtime shutting down");
            None
        }
    }

    pub fn spawn_blocking_handle<F, R>(
        &self,
        task: F,
        name: &'static str,
    ) -> Option<impl Future<Output = Result<R, tokio::task::JoinError>>>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        let log = self.log.clone();

        let join_handle = if let Some(handle) = self.handle() {
            handle.spawn_blocking(task)
        } else {
            debug!(self.log, "Couldn't spawn task. Runtime shutting down");
            return None;
        };

        let future = async move {
            let result = match join_handle.await {
                Ok(result) => {
                    trace!(log, "Blocking task completed"; "task" => name);
                    Ok(result)
                }
                Err(e) => {
                    debug!(log, "Blocking task ended unexpectedly"; "error" => %e);
                    Err(e)
                }
            };
            result
        };

        Some(future)
    }

    pub fn block_on_dangerous<F: Future>(
        &self,
        future: F,
        name: &'static str,
    ) -> Option<F::Output> {
        let log = self.log.clone();
        let handle = self.handle()?;
        let exit = self.exit.clone();

        debug!(
            log,
            "Starting block_on task";
            "name" => name
        );

        handle.block_on(async {
            let output = tokio::select! {
                output = future => {
                    debug!(
                        log,
                        "Completed block_on task";
                        "name" => name
                    );
                    Some(output)
                },
                _ = exit => {
                    debug!(
                        log,
                        "Cancelled block_on task";
                        "name" => name,
                    );
                    None
                }
            };
            output
        })
    }

    /// Returns a `Handle` to the current runtime.
    pub fn handle(&self) -> Option<Handle> {
        self.handle_provider.handle()
    }

    /// Returns a copy of the `exit_future::Exit`.
    pub fn exit(&self) -> exit_future::Exit {
        self.exit.clone()
    }

    /// Get a channel to request shutting down.
    pub fn shutdown_sender(&self) -> Sender<ShutdownReason> {
        self.signal_tx.clone()
    }

    /// Returns a reference to the logger.
    pub fn log(&self) -> &slog::Logger {
        &self.log
    }
}
