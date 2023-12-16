#![deny(unsafe_code)]

use dotenv::dotenv;
use eyre::Result;
use std::error::Error;

use contower::{create_logger, parse_verbosity};

#[tokio::main()]
async fn main() -> Result<(), Box<dyn Error>> {
    // Enable backtraces unless a RUST_BACKTRACE value has already been explicitly provided.
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    dotenv().ok();
    let verbosity = parse_verbosity();
    let _log = create_logger(verbosity);

    Ok(())
}
