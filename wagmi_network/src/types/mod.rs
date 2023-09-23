pub mod error;
pub mod globals;
pub mod pubsub;
pub mod subnet;
pub mod sync_state;
pub mod topics;

pub type EnrAttestationBitfield = BitVector<SubnetBitfieldLength>;
pub type EnrSyncCommitteeBitfield = BitVector<SyncCommitteeSubnetCount>;

