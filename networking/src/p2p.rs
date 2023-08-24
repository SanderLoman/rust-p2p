#![deny(unsafe_code)]

/// This file is the main entry point for the p2p networking module.
/// It is responsible for setting up the libp2p swarm and the discv5 discovery protocol.
/// It also sets up the gossipsub protocol and the eth2 rpc protocol.
/// It also sets up the identify protocol which is used for initial interop.
///
/// This file will be used in the main.rs file (the main entry point for the entire application), where other components come together aswell.
use crate::create_logger;
use crate::discv5::discovery::discovery::start_discv5;
use crate::libp2p::swarm::swarm::setup_swarm;
use crate::discv5::enr::generate_enr;
use eyre::Result;
use libp2p::core::identity::Keypair;
use slog::Logger;
use std::error::Error;

const LOG: Logger = create_logger();

pub struct P2PNetwork {
    pub swarm: libp2p::swarm::Swarm<libp2p::swarm::dummy::Behaviour>,
}

pub async fn start_p2p_networking() -> Result<(), Box<dyn Error>> {
    slog::info!(LOG, "Starting p2p networking");

    let libp2p_local_keys = Keypair::generate_secp256k1();

    let swarm = setup_swarm(libp2p_local_keys);
    let discv5 = start_discv5();

    tokio::try_join!(swarm, discv5)?;

    Ok(())
}
