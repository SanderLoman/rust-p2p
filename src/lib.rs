use slog::{o, Drain, Level, Logger};
use slog_async::Async;
use slog_term::{FullFormat, PlainSyncDecorator};

pub fn create_logger(verbosity: u64) -> Logger {
    let decorator = PlainSyncDecorator::new(std::io::stdout());
    let drain = FullFormat::new(decorator).build().fuse();
    let drain = Async::new(drain).build().fuse();

    let level = match verbosity {
        0 => Level::Error,
        1 => Level::Warning,
        2 => Level::Info,
        3 => Level::Debug,
        _ => Level::Trace,
    };

    Logger::root(drain.filter_level(level).fuse(), o!())
}
