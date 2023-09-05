#![deny(unsafe_code)]

/// This file is the main entry point for the p2p networking module.
/// It is responsible for setting up the libp2p swarm and the discv5 discovery protocol.
/// It also sets up the gossipsub protocol and the eth2 rpc protocol.
/// It also sets up the identify protocol which is used for initial interop.
///
/// This file will be used in the main.rs file (the main entry point for the entire application), where other components come together aswell.
// use crate::discv5::discovery::start_discv5;
use crate::libp2p::behaviour::CustomBehavior;
use crate::libp2p::swarm::CustomSwarm;
use eyre::Result;
use libp2p::core::identity::Keypair;
use libp2p::swarm::Swarm;
use libp2p::PeerId;
use slog::Logger;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct P2PNetwork {
    pub swarm: Arc<Mutex<Swarm<CustomBehavior>>>,
}

impl P2PNetwork {
    pub async fn new(log: Logger) -> Result<Self> {
        let log_for_swarm_events = log.clone();
        let log2 = log.clone();
        let local_transport_key: Keypair = Keypair::generate_secp256k1();
        let local_swarm_peer_id: PeerId = PeerId::from(local_transport_key.public());

        let swarm = CustomSwarm::new(local_swarm_peer_id, local_transport_key, log2).await.unwrap();

        Ok(P2PNetwork { swarm })
    }

    pub async fn start() -> Result<()> {
        Ok(())
    }
}

pub async fn start_p2p_networking(log: slog::Logger) -> Result<(), Box<dyn Error>> {
    slog::info!(log, "Starting p2p networking");

    let local_transport_key: Keypair = Keypair::generate_secp256k1();
    let local_swarm_peer_id: PeerId = PeerId::from(local_transport_key.public());

    // let swarm = setup_swarm(local_swarm_peer_id, local_transport_key, log);
    // let discv5 = start_discv5();

    // tokio::try_join!(swarm)?;

    Ok(())
}
