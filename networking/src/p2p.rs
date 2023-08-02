#![deny(unsafe_code)]

use crate::create_logger;
use crate::libp2p::transport::transport::setup_transport;
use crate::discv5::discovery::discovery::start_discv5;
use eyre::Result;

pub async fn start_p2p_networking() -> Result<()> {
    let log = create_logger();
    slog::info!(log, "Starting p2p networking");
    let transport = setup_transport().await.unwrap();
    start_discv5().await.unwrap();

    Ok(())
}
