#![deny(unsafe_code)]

use clap::{App, Arg};
use slog::{o, Drain, Level, LevelFilter, Logger};

pub fn create_logger(verbosity: u64) -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let level = match verbosity {
        0 => Level::Info,
        1 => Level::Warning,
        2 => Level::Error,
        3 => Level::Critical,
        4 => Level::Debug,
        _ => Level::Trace,
    };

    let drain = LevelFilter::new(drain, level).fuse();
    Logger::root(drain, o!())
}

pub fn parse_verbosity() -> u64 {
    let matches = App::new("wagmi")
        .version("1.0")
        .author("Sander Feitsma")
        .about("Wagmi, brah")
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    matches.occurrences_of("verbosity")
}
