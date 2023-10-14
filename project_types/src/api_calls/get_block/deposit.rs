use crate::*;
use serde_derive::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use ssz_types::typenum::U33;
use tree_hash_derive::TreeHash;

pub const DEPOSIT_TREE_DEPTH: usize = 32;

/// A deposit to potentially become a beacon chain validator.
///
/// Spec v0.12.1
#[derive(Debug, PartialEq, Hash, Clone, Serialize, Deserialize, Encode, Decode, TreeHash)]
pub struct Deposit {
    pub proof: FixedVector<Hash256, U33>,
    pub data: DepositData,
}

#[derive(arbitrary::Arbitrary, Debug, PartialEq, Hash, Clone, Serialize, Deserialize)]
pub struct DepositData {
    pub pubkey: String,
    pub withdrawal_credentials: Hash256,
    #[serde(with = "serde_utils::quoted_u64")]
    pub amount: u64,
    pub signature: String,
}

impl tree_hash::TreeHash for DepositData {
    fn tree_hash_type() -> tree_hash::TreeHashType {
        todo!()
    }

    fn tree_hash_packed_encoding(&self) -> tree_hash::PackedEncoding {
        todo!()
    }

    fn tree_hash_packing_factor() -> usize {
        todo!()
    }

    fn tree_hash_root(&self) -> tree_hash::Hash256 {
        todo!()
    }
}

impl ssz::Decode for DepositData {
    fn is_ssz_fixed_len() -> bool {
        todo!()
    }

    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, ssz::DecodeError> {
        todo!()
    }
}

impl ssz::Encode for DepositData {
    fn is_ssz_fixed_len() -> bool {
        todo!()
    }

    fn ssz_append(&self, buf: &mut Vec<u8>) {
        todo!()
    }

    fn ssz_bytes_len(&self) -> usize {
        todo!()
    }
}
