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
use eyre::Result;
use libp2p::PeerId;
use libp2p::core::identity::Keypair;
use std::error::Error;

pub struct P2PNetwork {
    pub swarm: libp2p::swarm::Swarm<libp2p::swarm::dummy::Behaviour>,
}

pub async fn start_p2p_networking() -> Result<(), Box<dyn Error>> {
    let log: slog::Logger = create_logger();
    slog::info!(log, "Starting p2p networking");

    let local_transport_key: Keypair = Keypair::generate_secp256k1();
    let local_swarm_peer_id: PeerId = PeerId::from(local_transport_key.public());
    println!("Local Swarm Peer Id: {:?}", local_swarm_peer_id);

    let swarm = setup_swarm(local_swarm_peer_id, local_transport_key);
    let discv5 = start_discv5();

    tokio::try_join!(swarm, discv5)?;

    Ok(())
}
