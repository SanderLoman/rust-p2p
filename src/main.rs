#![deny(unsafe_code)]

use dotenv::dotenv;
use eyre::Result;
use slog::debug;
use std::error::Error;

use wagmi::{create_logger, parse_verbosity};

use proxy::network::discovery::Discovery;

use proxy::{get_lh_tcp_multiaddr, REAL_BEACON_NODE_IP_ADDR, REAL_BEACON_NODE_MULTIADDR};

#[tokio::main()]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let verbosity = parse_verbosity();
    let log = create_logger(verbosity);

    Discovery::new(log.clone()).await?;

    get_lh_tcp_multiaddr().await?;

    let ip_addr = REAL_BEACON_NODE_IP_ADDR.lock().unwrap();
    let multiaddr = REAL_BEACON_NODE_MULTIADDR.lock().unwrap();

    debug!(log, "IP Address: {:?}", *ip_addr);
    debug!(log, "Multiaddr: {:?}", *multiaddr);

    Ok(())
}
