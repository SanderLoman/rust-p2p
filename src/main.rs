#![deny(unsafe_code)]

use dotenv::dotenv;
use eyre::Result;
use std::error::Error;

use slog::{warn, error, crit, debug, trace, info};

use wagmi::{create_logger, parse_verbosity};

#[tokio::main()]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let verbosity = parse_verbosity();
    let log = create_logger(verbosity);


    info!(log, "Starting wagmi");
    warn!(log, "This is a warning");
    error!(log, "This is an error");
    crit!(log, "This is a critical error");
    debug!(log, "This is a debug message");
    trace!(log, "This is a trace message");

    Ok(())
}
