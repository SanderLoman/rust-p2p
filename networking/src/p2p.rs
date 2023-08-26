#![deny(unsafe_code)]

/// This file is the main entry point for the p2p networking module.
/// It is responsible for setting up the libp2p swarm and the discv5 discovery protocol.
/// It also sets up the gossipsub protocol and the eth2 rpc protocol.
/// It also sets up the identify protocol which is used for initial interop.
///
/// This file will be used in the main.rs file (the main entry point for the entire application), where other components come together aswell.
// use crate::discv5::discovery::start_discv5;
use crate::libp2p::behaviour::CustomBehavior;
use crate::libp2p::swarm::swarm::setup_swarm;
use eyre::Result;
use libp2p::core::identity::Keypair;
use libp2p::PeerId;
use std::error::Error;

pub struct P2PNetwork {
    // pub swarm: libp2p::swarm::Swarm<CustomBehavior>,
}

impl P2PNetwork {
    pub fn new() -> Result<Self> {
        let local_transport_key: Keypair = Keypair::generate_secp256k1();
        let local_swarm_peer_id: PeerId = PeerId::from(local_transport_key.public());

        Ok(P2PNetwork {})
    }
}

pub async fn start_p2p_networking(log: slog::Logger) -> Result<(), Box<dyn Error>> {
    slog::info!(log, "Starting p2p networking");

    let local_transport_key: Keypair = Keypair::generate_secp256k1();
    let local_swarm_peer_id: PeerId = PeerId::from(local_transport_key.public());

    let swarm = setup_swarm(local_swarm_peer_id, local_transport_key, log);
    // let discv5 = start_discv5();

    tokio::try_join!(swarm)?;

    Ok(())
}
