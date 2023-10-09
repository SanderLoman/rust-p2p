pub mod error;
mod globals;
mod pubsub;
mod subnet;
mod sync_state;
mod topics;

use project_types::BitVector;

pub type EnrAttestationBitfield = BitVector<()>;
pub type EnrSyncCommitteeBitfield = BitVector<()>;

pub type Enr = discv5::enr::Enr<discv5::enr::CombinedKey>;

pub use globals::NetworkGlobals;
pub use pubsub::{PubsubMessage, SnappyTransform};
pub use subnet::{Subnet, SubnetDiscovery};
// pub use sync_state::{BackFillState, SyncState}; // not sure if we need this
pub use topics::{
    core_topics_to_subscribe, fork_core_topics, subnet_from_topic_hash, GossipEncoding, GossipKind,
    GossipTopic, LIGHT_CLIENT_GOSSIP_TOPICS,
};
