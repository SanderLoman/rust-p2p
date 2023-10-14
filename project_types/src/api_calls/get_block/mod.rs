#![deny(unsafe_code)]

pub mod attestation;
pub mod attestation_data;
pub mod attester_slashing;
pub mod checkpoint;
pub mod deposit;
pub mod eth1_data;
pub mod execution_payload;
pub mod graffiti;
pub mod proposer_slashing;
pub mod signed_bls_to_execution;
pub mod signed_voluntary_exit;
pub mod sync_aggregate;
pub mod withdrawal;

use std::marker::PhantomData;

use derivative::Derivative;
use eth1_data::Eth1Data;
use ethereum_types::Address;
use graffiti::Graffiti;
use reqwest::{
    header::{HeaderMap, ACCEPT},
    Client,
};
use serde::de::Error;
use serde_derive::{Deserialize, Serialize};
use ssz::DecodeError;
use ssz_derive::{Decode, Encode};
use ssz_types::{BitVector, FixedVector, VariableList};
use tree_hash_derive::TreeHash;

use crate::{
    chain_spec::ChainSpec,
    execution_block_hash::ExecutionBlockHash,
    fork_context::{Fork, ForkName},
    Epoch, EthSpec, Hash256, Slot, Uint256,
};

use self::{
    attestation::Attestation,
    attester_slashing::AttesterSlashing,
    deposit::Deposit,
    execution_payload::{ExecutionPayload, Withdrawals},
    proposer_slashing::ProposerSlashing,
    signed_voluntary_exit::SignedVoluntaryExit,
    sync_aggregate::SyncAggregate,
};

use crate::api_calls::get_block::signed_bls_to_execution::SignedBlsToExecutionChange;

pub type Signature = String;

pub type Transaction<N> = VariableList<u8, N>;
pub type Transactions<T> = VariableList<
    Transaction<<T as EthSpec>::MaxBytesPerTransaction>,
    <T as EthSpec>::MaxTransactionsPerPayload,
>;

pub trait EmptyBlock {
    fn empty(spec: &ChainSpec) -> Self;
}

pub struct SignedBeaconBlock<T: EthSpec> {
    pub message: BeaconBlock<T>,
    pub signature: Signature,
}

#[derive(Debug, Clone, Serialize, Deserialize, Derivative)]
pub struct BeaconBlockBody<T: EthSpec> {
    pub randao_reveal: Signature,
    pub eth1_data: Eth1Data,
    pub graffiti: Graffiti,
    pub proposer_slashings: VariableList<ProposerSlashing, T::MaxProposerSlashings>,
    pub attester_slashings: VariableList<AttesterSlashing<T>, T::MaxAttesterSlashings>,
    pub attestations: VariableList<Attestation, T::MaxAttestations>,
    pub deposits: VariableList<Deposit, T::MaxDeposits>,
    pub voluntary_exits: VariableList<SignedVoluntaryExit, T::MaxVoluntaryExits>,
    pub sync_aggregate: SyncAggregate,
    #[serde(flatten)]
    pub execution_payload: ExecutionPayload<T>,
    pub bls_to_execution_changes:
        VariableList<SignedBlsToExecutionChange, T::MaxBlsToExecutionChanges>,

    #[serde(skip)]
    pub _phantom: PhantomData<ExecutionPayload<T>>,
}

impl<T: EthSpec> BeaconBlockBody<T> {
    pub async fn get_block_body(&self) -> &ExecutionPayload<T> {
        let url = "https://127.0.0.1:5052/eth/v2/beacon/blocks/head";
        let mut headers = HeaderMap::new();

        headers.insert(ACCEPT, "application/json".parse().unwrap());

        let client = Client::builder().default_headers(headers).build().unwrap();

        let response = client.get(url).send().await.unwrap();

        let body = response.text().await.unwrap();

        let body: ExecutionPayload<T> = serde_json::from_str(&body).unwrap();

        &body
    }
}

impl<T: EthSpec + tree_hash::TreeHash> tree_hash::TreeHash for BeaconBlockBody<T> {
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

impl<T: EthSpec + ssz::Decode> ssz::Decode for BeaconBlockBody<T> {
    fn is_ssz_fixed_len() -> bool {
        todo!()
    }

    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, DecodeError> {
        todo!()
    }

    fn ssz_fixed_len() -> usize {
        todo!()
    }
}

impl<T: EthSpec + ssz::Encode> ssz::Encode for BeaconBlockBody<T> {
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

#[derive(Debug, Clone, Serialize, Deserialize, Derivative)]
pub struct BeaconBlock<T: EthSpec> {
    pub slot: Slot,
    pub proposer_index: u64,
    pub parent_root: Hash256,
    pub state_root: Hash256,
    pub body: BeaconBlockBody<T>,
}

impl<T: EthSpec + tree_hash::TreeHash> tree_hash::TreeHash for BeaconBlock<T> {
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

impl<T: EthSpec> ssz::Encode for BeaconBlock<T> {
    fn as_ssz_bytes(&self) -> Vec<u8> {
        todo!()
    }

    fn is_ssz_fixed_len() -> bool {
        todo!()
    }

    fn ssz_append(&self, buf: &mut Vec<u8>) {
        todo!()
    }

    fn ssz_bytes_len(&self) -> usize {
        todo!()
    }

    fn ssz_fixed_len() -> usize {
        todo!()
    }
}

impl<T: EthSpec> ssz::Decode for BeaconBlock<T> {
    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, DecodeError> {
        todo!()
    }

    fn is_ssz_fixed_len() -> bool {
        todo!()
    }

    fn ssz_fixed_len() -> usize {
        todo!()
    }
}
