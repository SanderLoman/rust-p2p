#![deny(unsafe_code)]

use crate::discv5::discovery::events::discv5_events;
use crate::discv5::discovery::Discovery as CustomDiscovery;
/// This file is the main entry point for the p2p networking module.
/// It is responsible for setting up the libp2p swarm and the discv5 discovery protocol.
/// It also sets up the gossipsub protocol and the eth2 rpc protocol.
/// It also sets up the identify protocol which is used for initial interop.
///
/// This file will be used in the main.rs file (the main entry point for the entire application), where other components come together aswell.
// use crate::discv5::discovery::start_discv5;
use crate::libp2p::behaviour::CustomBehavior;
use crate::libp2p::swarm::events::swarm_events;
use crate::libp2p::swarm::swarm_setup;
use eyre::Result;
use libp2p::core::identity::Keypair;
use libp2p::swarm::Swarm;
use libp2p::PeerId;
use slog::Logger;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;

pub struct P2PNetwork {
    pub swarm: Arc<Mutex<Swarm<CustomBehavior>>>,
    pub discv5: CustomDiscovery,
    log: Logger,
}

impl P2PNetwork {
    pub async fn new(log: Logger) -> Result<Self> {
        let log_for_swarm_events = log.clone();
        let log2 = log.clone();
        let local_transport_key: Keypair = Keypair::generate_secp256k1();
        let local_swarm_peer_id: PeerId = PeerId::from(local_transport_key.public());

        let swarm = swarm_setup(local_swarm_peer_id, local_transport_key, log2)
            .await
            .unwrap();

        let discv5 = CustomDiscovery::new().await.unwrap();

        Ok(P2PNetwork { swarm, discv5, log })
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let log = self.log.clone(); // Assuming log is part of your struct
        let swarm_clone = self.swarm.clone();
        let discv5_clone = self.discv5.start();

        // Spawn tasks for swarm and discv5 events
        let swarm_task = task::spawn(async move {
            let mut locked_swarm = swarm_clone.lock().await;
            swarm_events(&mut *locked_swarm, log.clone()).await;
        });

        let discv5_task = task::spawn(async move {
            discv5_events(&mut discv5_clone, log.clone()).await;
        });

        // Wait for both tasks to complete
        tokio::try_join!(swarm_task, discv5_task)?;

        Ok(())
    }
}

pub async fn start_p2p_networking(log: Logger) -> Result<(), Box<dyn Error>> {
    slog::info!(log, "Starting p2p networking");

    let local_transport_key: Keypair = Keypair::generate_secp256k1();
    let local_swarm_peer_id: PeerId = PeerId::from(local_transport_key.public());

    // let swarm = setup_swarm(local_swarm_peer_id, local_transport_key, log);
    // let discv5 = start_discv5();

    // tokio::try_join!(swarm)?;

    Ok(())
}
