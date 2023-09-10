#![deny(unsafe_code)]

use crate::discv5::discovery::events::discv5_events;
use crate::discv5::discovery::Discovery as CustomDiscovery;
use crate::libp2p::behaviour::CustomBehavior;
use crate::libp2p::swarm::events::swarm_events;
use crate::libp2p::swarm::CustomSwarm;
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
    pub log: Logger,
}

impl P2PNetwork {
    pub async fn new(log: Logger) -> Result<Self> {
        let local_transport_key: Keypair = Keypair::generate_secp256k1();
        let local_swarm_peer_id: PeerId = PeerId::from(local_transport_key.public());

        let swarm = CustomSwarm::new(local_swarm_peer_id, local_transport_key, log.clone()).await.unwrap();

        let discv5 = CustomDiscovery::new(log.clone()).await.unwrap();

        slog::info!(log, "Starting discv5 events INFO");
        slog::warn!(log, "Starting discv5 events WARN");
        slog::error!(log, "Starting discv5 events ERROR");
        slog::crit!(log, "Starting discv5 events CRIT");
        slog::debug!(log, "Starting discv5 events DEBUG");
        slog::trace!(log, "Starting discv5 events TRACE");

        Ok(P2PNetwork { swarm, discv5, log })
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        self.discv5.start().await?;
        slog::info!(self.log, "Discv5 started");

        Ok(())
    }
}
