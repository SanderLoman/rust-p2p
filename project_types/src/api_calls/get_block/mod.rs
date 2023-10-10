#![deny(unsafe_code)]

pub mod eth1_data;
pub mod graffiti;

use eth1_data::Eth1Data;
use ethereum_types::Address;
use graffiti::Graffiti;
use reqwest::{
    header::{HeaderMap, ACCEPT},
    Client,
};
use serde::de::Error;
use ssz::{Decode, DecodeError};
use ssz_types::{FixedVector, VariableList};

use crate::{
    chain_spec::ChainSpec,
    execution_block_hash::ExecutionBlockHash,
    fork_context::{Fork, ForkName},
    Epoch, EthSpec, Hash256, Slot, Uint256,
};

pub type Signature = String;

pub type Transaction<N> = VariableList<u8, N>;
pub type Transactions<T> = VariableList<
    Transaction<<T as EthSpec>::MaxBytesPerTransaction>,
    <T as EthSpec>::MaxTransactionsPerPayload,
>;

pub type Withdrawals<T> = VariableList<Withdrawal, <T as EthSpec>::MaxWithdrawalsPerPayload>;

pub struct BeaconBlockBody<T: EthSpec> {
    pub randao_reveal: String,
    pub eth1_data: Eth1Data,
    pub graffiti: Graffiti,
    pub proposer_slashings: Vec<u8>,
    pub attester_slashings: Vec<u8>,
    pub attestations: Vec<Attestation>,
    pub deposits: Vec<u8>,
    pub voluntary_exits: Vec<u8>,
    pub sync_aggregate: SyncAggregate,
    pub execution_payload: ExecutionPayload<T>,
    pub bls_to_execution_changes: Vec<u8>,
}

impl<T: EthSpec> BeaconBlockBody<T> {
    pub fn from_ssz_bytes() -> Result<Self, ssz::DecodeError> {
        let randao_reveal = String::new();
        let eth1_data = Eth1Data::default();
        let graffiti = Graffiti::default();
        let proposer_slashings = Vec::new();
        let attester_slashings = Vec::new();
        let attestations = Vec::new();
        let deposits = Vec::new();
        let voluntary_exits = Vec::new();
        let sync_aggregate = SyncAggregate {
            sync_committee_bits: String::new(),
            sync_committee_signature: String::new(),
        };

        let execution_payload = ExecutionPayload {
            parent_hash: ExecutionBlockHash::default(),
            fee_recipient: Address::default(),
            state_root: Hash256::default(),
            receipts_root: Hash256::default(),
            logs_bloom: FixedVector::default(),
            prev_randao: Hash256::default(),
            block_number: 0,
            gas_limit: 0,
            gas_used: 0,
            timestamp: 0,
            extra_data: VariableList::default(),
            base_fee_per_gas: Uint256::default(),
            block_hash: ExecutionBlockHash::default(),
            transactions: Transactions::<T>::default(),
            withdrawals: Withdrawals::<T>::default(),
        };

        let bls_to_execution_changes = Vec::new();

        Ok(BeaconBlockBody {
            randao_reveal,
            eth1_data,
            graffiti,
            proposer_slashings,
            attester_slashings,
            attestations,
            deposits,
            voluntary_exits,
            sync_aggregate,
            execution_payload,
            bls_to_execution_changes,
        })
    }
}

pub struct BeaconBlock<T: EthSpec> {
    pub slot: Slot,
    pub proposer_index: u64,
    pub parent_root: Hash256,
    pub state_root: Hash256,
    pub body: BeaconBlockBody<T>,
}

impl<T: EthSpec> BeaconBlock<T> {
    /// Custom SSZ decoder that takes a `ChainSpec` as context.
    pub fn from_ssz_bytes(bytes: &[u8], spec: &ChainSpec) -> Result<Self, ssz::DecodeError> {
        let slot_len = <Slot as Decode>::ssz_fixed_len();
        let slot_bytes = bytes
            .get(0..slot_len)
            .ok_or(DecodeError::InvalidByteLength {
                len: bytes.len(),
                expected: slot_len,
            })?;

        let slot = Slot::from_ssz_bytes(slot_bytes)?;

        Ok(BeaconBlock {
            slot,
            proposer_index: 0,
            parent_root: Hash256::zero(),
            state_root: Hash256::zero(),
            body: BeaconBlockBody::from_ssz_bytes(bytes, spec)?,
        })
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

pub struct ExecutionPayload<T: EthSpec> {
    pub parent_hash: ExecutionBlockHash,
    pub fee_recipient: Address,
    pub state_root: Hash256,
    pub receipts_root: Hash256,

    pub logs_bloom: FixedVector<u8, T::BytesPerLogsBloom>,

    pub prev_randao: Hash256,
    pub block_number: u64,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub timestamp: u64,
    pub extra_data: VariableList<u8, T::MaxExtraDataBytes>,
    pub base_fee_per_gas: Uint256,
    pub block_hash: ExecutionBlockHash,
    pub transactions: Transactions<T>,
    pub withdrawals: Withdrawals<T>,
}

pub struct SignedBeaconBlock<T: EthSpec> {
    pub message: BeaconBlock<T>,
    pub signature: Signature,
}

// Needed for the Withdrawals type
pub struct Withdrawal {
    pub index: u64,
    pub validator_index: u64,
    pub address: Address,
    pub amount: u64,
}
