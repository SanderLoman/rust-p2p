pub mod error;
pub mod globals;
pub mod pubsub;
pub mod subnet;
pub mod sync_state;
pub mod topics;

pub type EnrAttestationBitfield<T> = BitVector<<T as EthSpec>::SubnetBitfieldLength>;
pub type EnrSyncCommitteeBitfield<T> = BitVector<<T as EthSpec>::SyncCommitteeSubnetCount>;

pub type Enr = discv5::enr::Enr<discv5::enr::CombinedKey>;

pub use globals::NetworkGlobals;
pub use pubsub::{PubsubMessage, SnappyTransform};
pub use subnet::{Subnet, SubnetDiscovery};
pub use sync_state::{BackFillState, SyncState};
pub use topics::{
    core_topics_to_subscribe, fork_core_topics, subnet_from_topic_hash, GossipEncoding, GossipKind,
    GossipTopic, LIGHT_CLIENT_GOSSIP_TOPICS,
};

//    error, Enr, EnrSyncCommitteeBitfield, GossipTopic, NetworkGlobals, PubsubMessage, Subnet,
//    SubnetDiscovery,
