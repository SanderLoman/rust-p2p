#![deny(unsafe_code)]

use dotenv::dotenv;
use eyre::Result;
use std::error::Error;

use wagmi::{create_logger, parse_verbosity};

use proxy::get_lh_tcp_multiaddr;
use proxy::network::discovery::Discovery;

#[tokio::main()]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let verbosity = parse_verbosity();
    let log = create_logger(verbosity);

    get_lh_tcp_multiaddr(log.clone()).await?;

    Discovery::new(log.clone()).await?;

    Ok(())
}
