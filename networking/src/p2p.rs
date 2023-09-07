#![deny(unsafe_code)]

use crate::discv5::discovery::events::discv5_events;
use crate::discv5::discovery::Discovery as CustomDiscovery;

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
        let local_transport_key: Keypair = Keypair::generate_secp256k1();
        let local_swarm_peer_id: PeerId = PeerId::from(local_transport_key.public());

        let swarm = swarm_setup(local_swarm_peer_id, local_transport_key, log.clone())
            .await
            .unwrap();

        let discv5 = CustomDiscovery::new().await.unwrap();

        Ok(P2PNetwork { swarm, discv5, log })
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        let log = self.log.clone();
        let swarm_clone = self.swarm.clone();

        // Start the discv5 instance and handle any errors
        self.discv5.start().await?;
        slog::info!(self.log, "Discv5 started");

        // Spawn tasks for swarm and discv5 events
        // let swarm_task = task::spawn(async move {
        //     let mut locked_swarm = swarm_clone.lock().await;
        //     swarm_events(&mut *locked_swarm, log.clone()).await;
        // });

        // let discv5_task = task::spawn(async move {
        //     // Use self.discv5 directly here
        //     discv5_events(&mut self.discv5, log.clone()).await;
        // });

        // Wait for both tasks to complete
        // tokio::try_join!(swarm_task, discv5_task)?;

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
