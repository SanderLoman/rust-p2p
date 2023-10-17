#![deny(unsafe_code)]

use dotenv::dotenv;
use eyre::Result;
use std::error::Error;

// use slog::{crit, debug, error, info, trace, warn};

use wagmi::{create_logger, parse_verbosity};

use p2p_network::discovery::enr::generate_enr;

#[tokio::main()]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let verbosity = parse_verbosity();
    let log = create_logger(verbosity);

    generate_enr(log.clone()).await?;

    Ok(())
}
