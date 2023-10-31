#![deny(unsafe_code)]

use dotenv::dotenv;
use eyre::Result;
use std::error::Error;

use wagmi::{create_logger, parse_verbosity};

// use proxy::network::discovery::Discovery;


#[tokio::main()]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let verbosity = parse_verbosity();
    let _log = create_logger(verbosity);

    Ok(())
}
