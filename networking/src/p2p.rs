#![deny(unsafe_code)]

use crate::discv5::discovery::events::discv5_events;
use crate::discv5::discovery::Discovery;
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
    pub swarm: CustomSwarm,
    pub discv5: Discovery,
    pub log: Logger,
}

impl P2PNetwork {
    pub async fn new(log: Logger) -> Result<Self> {
        let local_transport_key: Keypair = Keypair::generate_secp256k1();
        let local_swarm_peer_id: PeerId = PeerId::from(local_transport_key.public());

        let discv5 = Discovery::new(log.clone()).await.unwrap();
        let swarm = CustomSwarm::default().await.unwrap();

        slog::info!(log, "Starting Discv5");
        slog::warn!(log, "Starting Discv5");
        slog::error!(log, "Starting Discv5");
        slog::crit!(log, "Starting Discv5");
        slog::debug!(log, "Starting Discv5");
        slog::trace!(log, "Starting Discv5");


        Ok(P2PNetwork { swarm, discv5, log })
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        self.discv5.start().await?;
        slog::info!(self.log, "Discv5 started");

        Ok(())
    }
}
