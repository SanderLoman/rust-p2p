#![deny(unsafe_code)]

use crate::create_logger;
use crate::libp2p::transport::transport::setup_transport;
use crate::discv5::discovery::discovery::start_discv5;
use crate::libp2p::swarm::swarm::setup_swarm;
use std::error::Error;
use eyre::Result;

pub async fn start_p2p_networking() -> Result<(), Box<dyn Error>> {
    let log = create_logger();
    slog::info!(log, "Starting p2p networking");
    setup_transport().await.unwrap();
    let swarm_future = setup_swarm();
    let discv5_future = start_discv5();

    tokio::try_join!(swarm_future, discv5_future)?;


    Ok(())
}
