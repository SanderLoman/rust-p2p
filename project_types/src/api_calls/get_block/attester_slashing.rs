use crate::{EthSpec, Hash256, Slot};

use derivative::Derivative;
use serde_derive::{Deserialize, Serialize};
use ssz_types::VariableList;

use super::{checkpoint::Checkpoint, Signature};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "T: EthSpec")]
pub struct AttesterSlashing<T: EthSpec> {
    pub attestation_1: IndexedAttestation<T>,
    pub attestation_2: IndexedAttestation<T>,
}

impl<T: EthSpec + tree_hash::TreeHash> tree_hash::TreeHash for AttesterSlashing<T> {
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

impl<T: EthSpec + ssz::Decode> ssz::Decode for AttesterSlashing<T> {
    fn is_ssz_fixed_len() -> bool {
        todo!()
    }

    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, ssz::DecodeError> {
        todo!()
    }
}

impl<T: EthSpec + ssz::Encode> ssz::Encode for AttesterSlashing<T> {
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

#[derive(Derivative, Debug, Clone, Serialize, Deserialize)]
pub struct IndexedAttestation<T: EthSpec> {
    #[serde(with = "quoted_variable_list_u64")]
    pub attesting_indices: VariableList<u64, T::MaxValidatorsPerCommittee>,
    pub data: AttestationData,
    pub signature: Signature,
}

impl<T: EthSpec + tree_hash::TreeHash> tree_hash::TreeHash for IndexedAttestation<T> {
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

impl<T: EthSpec + ssz::Decode> ssz::Decode for IndexedAttestation<T> {
    fn is_ssz_fixed_len() -> bool {
        todo!()
    }

    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, ssz::DecodeError> {
        todo!()
    }

    fn ssz_fixed_len() -> usize {
        todo!()
    }
}

impl<T: EthSpec + ssz::Encode> ssz::Encode for IndexedAttestation<T> {
    fn is_ssz_fixed_len() -> bool {
        todo!()
    }

    fn ssz_append(&self, buf: &mut Vec<u8>) {
        todo!()
    }

    fn ssz_bytes_len(&self) -> usize {
        todo!()
    }

    fn as_ssz_bytes(&self) -> Vec<u8> {
        todo!()
    }

    fn ssz_fixed_len() -> usize {
        todo!()
    }
}

mod quoted_variable_list_u64 {
    use super::*;
    use crate::Unsigned;
    use serde::ser::SerializeSeq;
    use serde::{Deserializer, Serializer};
    use serde_utils::quoted_u64_vec::{QuotedIntVecVisitor, QuotedIntWrapper};

    pub fn serialize<S, T>(value: &VariableList<u64, T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Unsigned,
    {
        let mut seq = serializer.serialize_seq(Some(value.len()))?;
        for &int in value.iter() {
            seq.serialize_element(&QuotedIntWrapper { int })?;
        }
        seq.end()
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<VariableList<u64, T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Unsigned,
    {
        deserializer
            .deserialize_any(QuotedIntVecVisitor)
            .and_then(|vec| {
                VariableList::new(vec)
                    .map_err(|e| serde::de::Error::custom(format!("invalid length: {:?}", e)))
            })
    }
}

#[derive(Derivative, Debug, Clone, Serialize, Deserialize)]
pub struct AttestationData {
    pub slot: Slot,
    #[serde(with = "serde_utils::quoted_u64")]
    pub index: u64,

    // LMD GHOST vote
    pub beacon_block_root: Hash256,

    // FFG Vote
    pub source: Checkpoint,
    pub target: Checkpoint,
}

impl ssz::Encode for AttestationData {
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

impl ssz::Decode for AttestationData {
    fn is_ssz_fixed_len() -> bool {
        todo!()
    }

    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, ssz::DecodeError> {
        todo!()
    }
}

impl tree_hash::TreeHash for AttestationData {
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

pub trait SlotData {
    fn get_slot(&self) -> Slot;
}

impl SlotData for AttestationData {
    fn get_slot(&self) -> Slot {
        self.slot
    }
}
