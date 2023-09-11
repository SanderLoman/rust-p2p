#![deny(unsafe_code)]

// use crate::discv5::discovery::events::discv5_events;
// use crate::discv5::discovery::Discovery;
// use crate::libp2p::behaviour::CustomBehavior;
// use crate::libp2p::swarm::events::swarm_events;
// use crate::libp2p::swarm::CustomSwarm;
use eyre::Result;
use libp2p::core::identity::Keypair;
use libp2p::swarm::Swarm;
use libp2p::PeerId;
use slog::Logger;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task;

// pub struct P2PNetwork {
//     pub swarm: CustomSwarm,
//     pub discv5: Discovery,
//     pub log: Logger,
// }

// impl P2PNetwork {
//     pub async fn new(log: Logger) -> Result<Self> {
//         Ok(P2PNetwork { swarm, discv5, log })
//     }

//     pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
//         self.discv5.start().await?;
//         slog::info!(self.log, "Discv5 started");

//         Ok(())
//     }
// }
