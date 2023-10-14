use super::Signature;
use crate::{BitVector, EthSpec};
use derivative::Derivative;
use serde_derive::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use tree_hash_derive::TreeHash;

#[derive(Debug, Clone, Serialize, Deserialize, Derivative, arbitrary::Arbitrary)]
pub struct SyncAggregate {
    pub sync_committee_bits: String,
    pub sync_committee_signature: Signature,
}
