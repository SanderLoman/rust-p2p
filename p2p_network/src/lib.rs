mod config;
pub mod service;

pub mod discovery;
pub mod listen_addr;
pub mod peer_manager;
pub mod rpc;
pub mod types;

// use config::Config;

// pub use crate::types::{
//     error, Enr, EnrSyncCommitteeBitfield, GossipTopic, NetworkGlobals, PubsubMessage, Subnet,
//     SubnetDiscovery,
// };

use std::collections::HashMap;
use std::sync::RwLock;

pub enum ForkName {
    Base,
    Altair,
    Merge,
    Capella,
}

pub struct ForkContext {
    current_fork: RwLock<ForkName>,
    fork_to_digest: HashMap<ForkName, [u8; 4]>,
    digest_to_fork: HashMap<[u8; 4], ForkName>,
}
