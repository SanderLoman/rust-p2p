pub mod discv5;
pub mod libp2p;
pub mod p2p;

use slog::{o, Drain, Logger};
use slog_async::Async;
use slog_term::CompactFormat;
use slog_term::TermDecorator;

pub fn create_logger() -> Logger {
    let decorator = TermDecorator::new().build();
    let drain = CompactFormat::new(decorator).build().fuse();
    let drain = Async::new(drain).build().fuse();

    Logger::root(drain, o!())
}
