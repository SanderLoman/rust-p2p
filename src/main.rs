#![deny(unsafe_code)]

use dotenv::dotenv;
use eyre::Result;
use std::error::Error;

// use networking::p2p::P2PNetwork;
use wagmi::{create_logger, parse_verbosity};

#[tokio::main()]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let verbosity = parse_verbosity();
    let log = create_logger(verbosity);

    // P2PNetwork::new(log).await?;

    Ok(())
}
