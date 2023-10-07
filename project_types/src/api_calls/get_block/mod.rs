#![deny(unsafe_code)]

pub mod eth1_data;
pub mod graffiti;

use eth1_data::Eth1Data;
use graffiti::Graffiti;
use reqwest::{
    header::{HeaderMap, ACCEPT},
    Client,
};
use serde::de::Error;
use ssz::{Decode, DecodeError};
use ssz_types::VariableList;

pub type Signature = String;

use crate::{
    chain_spec::ChainSpec,
    fork_context::{Fork, ForkName},
    Epoch, EthSpec, Hash256, Slot,
};

pub struct BeaconBlockBody {
    pub randao_reveal: String,
    pub eth1_data: Eth1Data,
    pub graffiti: Graffiti,
    pub proposer_slashings: Vec<u8>,
    pub attester_slashings: Vec<u8>,
    pub attestations: Vec<Attestation>,
    pub deposits: Vec<u8>,
    pub voluntary_exits: Vec<u8>,
    pub sync_aggregate: SyncAggregate,
    pub execution_payload: ExecutionPayload,
}

pub struct BeaconBlock {
    pub slot: Slot,
    pub proposer_index: u64,
    pub parent_root: Hash256,
    pub state_root: Hash256,
    pub body: BeaconBlockBody,
}

/// Empty block trait for each block variant to implement.
pub trait EmptyBlock {
    /// Returns an empty block to be used during genesis.
    fn empty(spec: &ChainSpec) -> Self;
}

impl BeaconBlock {
    /// Returns an empty block to be used during genesis.
    pub fn empty(spec: &ChainSpec) -> Self {
        BeaconBlock {
            slot: Slot::new(0),
            proposer_index: 0,
            parent_root: Hash256::zero(),
            state_root: Hash256::zero(),
            body: BeaconBlockBody::,
        }
    }

    /// Custom SSZ decoder that takes a `ChainSpec` as context.
    pub fn from_ssz_bytes(bytes: &[u8], spec: &ChainSpec) -> Result<Self, ssz::DecodeError> {
        Ok(())
    }

    /// Try decoding each beacon block variant in sequence.
    ///
    /// This is *not* recommended unless you really have no idea what variant the block should be.
    /// Usually it's better to prefer `from_ssz_bytes` which will decode the correct variant based
    /// on the fork slot.
    pub fn any_from_ssz_bytes(bytes: &[u8]) -> Result<Self, ssz::DecodeError> {
        BeaconBlock::from_ssz_bytes(bytes, &ChainSpec::mainnet())
    }

    // /// Convenience accessor for the `body` as a `BeaconBlockBodyRef`.
    // pub fn body(&self) -> BeaconBlockBodyRef<'_, T, Payload> {
    //     self.to_ref().body()
    // }

    // /// Convenience accessor for the `body` as a `BeaconBlockBodyRefMut`.
    // pub fn body_mut(&mut self) -> BeaconBlockBodyRefMut<'_, T, Payload> {
    //     self.to_mut().body_mut()
    // }

    // /// Returns the epoch corresponding to `self.slot()`.
    // pub fn epoch(&self) -> Epoch {
    //     self.slot().epoch(T::slots_per_epoch())
    // }

    // /// Returns the `tree_hash_root` of the block.
    // pub fn canonical_root(&self) -> Hash256 {
    //     self.tree_hash_root()
    // }

    // /// Returns a full `BeaconBlockHeader` of this block.
    // ///
    // /// Note: This method is used instead of an `Into` impl to avoid a `Clone` of an entire block
    // /// when you want to have the block _and_ the header.
    // ///
    // /// Note: performs a full tree-hash of `self.body`.
    // pub fn block_header(&self) -> BeaconBlockHeader {
    //     self.to_ref().block_header()
    // }

    // /// Returns a "temporary" header, where the `state_root` is `Hash256::zero()`.
    // pub fn temporary_block_header(&self) -> BeaconBlockHeader {
    //     self.to_ref().temporary_block_header()
    // }

    // /// Return the tree hash root of the block's body.
    // pub fn body_root(&self) -> Hash256 {
    //     self.to_ref().body_root()
    // }
}

impl EmptyBlock for BeaconBlock {
    /// Returns an empty Capella block to be used during genesis.
    fn empty(spec: &ChainSpec) -> Self {
        BeaconBlock {
            slot: spec.genesis_slot,
            proposer_index: 0,
            parent_root: Hash256::zero(),
            state_root: Hash256::zero(),
            body: BeaconBlockBody {
                randao_reveal: Signature::empty(),
                eth1_data: Eth1Data {
                    deposit_root: Hash256::zero(),
                    block_hash: Hash256::zero(),
                    deposit_count: 0,
                },
                graffiti: Graffiti::default(),
                proposer_slashings: VariableList::empty(),
                attester_slashings: VariableList::empty(),
                attestations: VariableList::empty(),
                deposits: VariableList::empty(),
                voluntary_exits: VariableList::empty(),
                sync_aggregate: SyncAggregate::empty(),
                execution_payload: Payload::Capella::default(),
                bls_to_execution_changes: VariableList::empty(),
            },
        }
    }
}

pub struct Checkpoint {
    pub epoch: Epoch,
    pub root: Hash256,
}

pub struct AttestationData {
    pub slot: Slot,
    pub index: u64,
    pub beacon_block_root: Hash256,
    pub source: Checkpoint,
    pub target: Checkpoint,
}

pub struct Attestation {
    pub aggregation_bits: String,
    pub data: AttestationData,
    pub signature: Signature,
}

pub struct SyncAggregate {
    pub sync_committee_bits: String,
    pub sync_committee_signature: Signature,
}

pub struct ExecutionPayload {
    pub parent_hash: Hash256,
    pub fee_recipient: String,
}

pub struct SignedBeaconBlock {
    pub message: BeaconBlock,
    pub signature: Signature,
}
