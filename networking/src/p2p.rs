#![deny(unsafe_code)]

use crate::create_logger;
use crate::discv5::discovery::discovery::setup_discv5;
use eyre::Result;

pub async fn start_p2p_networking() -> Result<()> {
    let log = create_logger();
    slog::info!(log, "Starting p2p networking");
    setup_discv5().await.unwrap();

    Ok(())
}
