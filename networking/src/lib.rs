pub mod discv5;
pub mod libp2p;
pub mod p2p;

use slog::Drain;
use slog::{o, Level, LevelFilter, Logger};
use slog_async::Async;
use slog_term::FullFormat;
use slog_term::TermDecorator;

pub fn create_logger(verbosity: u64) -> Logger {
    let decorator = TermDecorator::new().build();
    let drain = FullFormat::new(decorator).build().fuse();
    let drain = Async::new(drain).build().fuse();

    let level = match verbosity {
        0 => Level::Info,
        1 => Level::Warning,
        2 => Level::Error,
        3 => Level::Debug,
        _ => Level::Trace,
    };

    let drain = LevelFilter::new(drain, level);

    Logger::root(drain.fuse(), o!())
}
