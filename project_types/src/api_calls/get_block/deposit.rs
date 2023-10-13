use crate::*;
use serde_derive::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use ssz_types::typenum::U33;
use tree_hash_derive::TreeHash;

pub const DEPOSIT_TREE_DEPTH: usize = 32;

/// A deposit to potentially become a beacon chain validator.
///
/// Spec v0.12.1
#[derive(
    arbitrary::Arbitrary,
    Debug,
    PartialEq,
    Hash,
    Clone,
    Serialize,
    Deserialize,
    Encode,
    Decode,
    TreeHash,
)]
pub struct Deposit {
    pub proof: FixedVector<Hash256, U33>,
    pub data: DepositData,
}

#[derive(
    arbitrary::Arbitrary,
    Debug,
    PartialEq,
    Hash,
    Clone,
    Serialize,
    Deserialize,
    Encode,
    Decode,
    TreeHash,
)]
pub struct DepositData {
    pub pubkey: String,
    pub withdrawal_credentials: Hash256,
    #[serde(with = "serde_utils::quoted_u64")]
    pub amount: u64,
    pub signature: String,
}
