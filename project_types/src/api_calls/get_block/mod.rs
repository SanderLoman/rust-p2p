#![deny(unsafe_code)]

pub mod attestation;
pub mod attestation_data;
pub mod checkpoint;
pub mod deposit;
pub mod eth1_data;
pub mod execution_payload;
pub mod graffiti;
pub mod proposer_slashing;
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
use ssz::{Decode, DecodeError, Encode};
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
    deposit::Deposit,
    execution_payload::{ExecutionPayload, Withdrawals},
    proposer_slashing::ProposerSlashing,
    sync_aggregate::SyncAggregate,
};

pub type Signature = String;

pub type Transaction<N> = VariableList<u8, N>;
pub type Transactions<T> = VariableList<
    Transaction<<T as EthSpec>::MaxBytesPerTransaction>,
    <T as EthSpec>::MaxTransactionsPerPayload,
>;

pub trait EmptyBlock {
    /// Returns an empty block to be used during genesis.
    fn empty(spec: &ChainSpec) -> Self;
}

pub struct SignedBeaconBlock<T: EthSpec> {
    pub message: BeaconBlock<T>,
    pub signature: Signature,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Derivative, Encode, Decode, TreeHash, arbitrary::Arbitrary,
)]
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
    #[ssz(skip_serializing, skip_deserializing)]
    #[tree_hash(skip_hashing)]
    #[serde(skip)]
    #[arbitrary(default)]
    pub _phantom: PhantomData<ExecutionPayload<T>>,
}

pub struct BeaconBlock<T: EthSpec> {
    pub slot: Slot,
    pub proposer_index: u64,
    pub parent_root: Hash256,
    pub state_root: Hash256,
    pub body: BeaconBlockBody<T>,
}

impl<T: EthSpec> Encode for BeaconBlock<T> {
    fn as_ssz_bytes(&self) -> Vec<u8> {}

    fn is_ssz_fixed_len() -> bool {}

    fn ssz_append(&self, buf: &mut Vec<u8>) {}

    fn ssz_bytes_len(&self) -> usize {}

    fn ssz_fixed_len() -> usize {}
}

impl<T: EthSpec> Decode for BeaconBlock<T> {
    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, DecodeError> {}

    fn is_ssz_fixed_len() -> bool {}

    fn ssz_fixed_len() -> usize {}
}

impl<T: EthSpec> Encode for BeaconBlockBody<T> {
    fn as_ssz_bytes(&self) -> Vec<u8> {}

    fn is_ssz_fixed_len() -> bool {}

    fn ssz_append(&self, buf: &mut Vec<u8>) {}

    fn ssz_bytes_len(&self) -> usize {}

    fn ssz_fixed_len() -> usize {}
}

impl<T: EthSpec> Decode for BeaconBlockBody<T> {
    fn from_ssz_bytes(bytes: &[u8]) -> Result<Self, DecodeError> {}

    fn is_ssz_fixed_len() -> bool {}

    fn ssz_fixed_len() -> usize {}
}

// impl<T: EthSpec> BeaconBlockBody<T> {
//     pub fn from_ssz_bytes() -> Result<Self, ssz::DecodeError> {
//         let randao_reveal = String::new();
//         let eth1_data = Eth1Data::default();
//         let graffiti = Graffiti::default();
//         let proposer_slashings = Vec::new();
//         let attester_slashings = Vec::new();
//         let attestations = Vec::new();
//         let deposits = Vec::new();
//         let voluntary_exits = Vec::new();
//         let sync_aggregate = SyncAggregate {
//             sync_committee_bits: String::new(),
//             sync_committee_signature: String::new(),
//         };

//         let execution_payload = ExecutionPayload {
//             parent_hash: ExecutionBlockHash::default(),
//             fee_recipient: Address::default(),
//             state_root: Hash256::default(),
//             receipts_root: Hash256::default(),
//             logs_bloom: FixedVector::default(),
//             prev_randao: Hash256::default(),
//             block_number: 0,
//             gas_limit: 0,
//             gas_used: 0,
//             timestamp: 0,
//             extra_data: VariableList::default(),
//             base_fee_per_gas: Uint256::default(),
//             block_hash: ExecutionBlockHash::default(),
//             transactions: Transactions::<T>::default(),
//             withdrawals: Withdrawals::<T>::default(),
//         };

//         let bls_to_execution_changes = Vec::new();

//         Ok(BeaconBlockBody {
//             randao_reveal,
//             eth1_data,
//             graffiti,
//             proposer_slashings,
//             attester_slashings,
//             attestations,
//             deposits,
//             voluntary_exits,
//             sync_aggregate,
//             execution_payload,
//             bls_to_execution_changes,
//         })
//     }
// }

// impl<T: EthSpec> BeaconBlock<T> {
//     pub fn from_ssz_bytes(bytes: &[u8], spec: &ChainSpec) -> Result<Self, ssz::DecodeError> {
//         let slot_len = <Slot as Decode>::ssz_fixed_len();
//         let slot_bytes = bytes
//             .get(0..slot_len)
//             .ok_or(DecodeError::InvalidByteLength {
//                 len: bytes.len(),
//                 expected: slot_len,
//             })?;

//         let slot = Slot::from_ssz_bytes(slot_bytes)?;

//         Ok(BeaconBlock {
//             slot,
//             proposer_index: 0,
//             parent_root: Hash256::zero(),
//             state_root: Hash256::zero(),
//             body: BeaconBlockBody::from_ssz_bytes(),
//         })
//     }

//     pub fn any_from_ssz_bytes(bytes: &[u8]) -> Result<Self, ssz::DecodeError> {
//         BeaconBlock::from_ssz_bytes(bytes, &ChainSpec::mainnet())
//     }
// }

// impl<T: EthSpec> BeaconBlock<T> {
//     /// Return a Capella block where the block has maximum size.
//     pub fn full(spec: &ChainSpec) -> Self {
//         // let base_block: BeaconBlockBase<_, Payload> = BeaconBlockBase::full(spec);
//         let bls_to_execution_changes = vec![
//             SignedBlsToExecutionChange {
//                 message: BlsToExecutionChange {
//                     validator_index: 0,
//                     from_bls_pubkey: PublicKeyBytes::empty(),
//                     to_execution_address: Address::zero(),
//                 },
//                 signature: Signature::empty()
//             };
//             T::max_bls_to_execution_changes()
//         ]
//         .into();
//         let sync_aggregate = SyncAggregate {
//             sync_committee_signature: AggregateSignature::empty(),
//             sync_committee_bits: BitVector::default(),
//         };
//         BeaconBlock {
//             slot: spec.genesis_slot,
//             proposer_index: 0,
//             parent_root: Hash256::zero(),
//             state_root: Hash256::zero(),
//             body: BeaconBlockBody {
//                 proposer_slashings: base_block.body.proposer_slashings,
//                 attester_slashings: base_block.body.attester_slashings,
//                 attestations: base_block.body.attestations,
//                 deposits: base_block.body.deposits,
//                 voluntary_exits: base_block.body.voluntary_exits,
//                 bls_to_execution_changes,
//                 sync_aggregate,
//                 randao_reveal: Signature::default(),
//                 eth1_data: Eth1Data {
//                     deposit_root: Hash256::zero(),
//                     block_hash: Hash256::zero(),
//                     deposit_count: 0,
//                 },
//                 graffiti: Graffiti::default(),
//                 execution_payload: ExecutionPayload::,
//             },
//         }
//     }
// }

// impl<T: EthSpec> BeaconBlock<T> {
//     pub fn from_ssz_bytes(bytes: &[u8], spec: &ChainSpec) -> Result<Self, ssz::DecodeError> {
//         let slot_len = <Slot as Decode>::ssz_fixed_len();
//         let slot_bytes = bytes
//             .get(0..slot_len)
//             .ok_or(DecodeError::InvalidByteLength {
//                 len: bytes.len(),
//                 expected: slot_len,
//             })?;

//         let slot = Slot::from_ssz_bytes(slot_bytes)?;

//         Ok(BeaconBlock {
//             slot,
//             proposer_index: 0,
//             parent_root: Hash256::zero(),
//             state_root: Hash256::zero(),
//             body: BeaconBlockBody::from_ssz_bytes(),
//         })
//     }

//     pub fn any_from_ssz_bytes(bytes: &[u8]) -> Result<Self, ssz::DecodeError> {
//         BeaconBlock::from_ssz_bytes(bytes, &ChainSpec::mainnet())
//     }
// }

// impl<T: EthSpec> BeaconBlock<T> {
//     /// Return a Capella block where the block has maximum size.
//     pub fn full(spec: &ChainSpec) -> Self {
//         // let base_block: BeaconBlockBase<_, Payload> = BeaconBlockBase::full(spec);
//         let bls_to_execution_changes = vec![
//             SignedBlsToExecutionChange {
//                 message: BlsToExecutionChange {
//                     validator_index: 0,
//                     from_bls_pubkey: PublicKeyBytes::empty(),
//                     to_execution_address: Address::zero(),
//                 },
//                 signature: Signature::empty()
//             };
//             T::max_bls_to_execution_changes()
//         ]
//         .into();
//         let sync_aggregate = SyncAggregate {
//             sync_committee_signature: AggregateSignature::empty(),
//             sync_committee_bits: BitVector::default(),
//         };
//         BeaconBlock {
//             slot: spec.genesis_slot,
//             proposer_index: 0,
//             parent_root: Hash256::zero(),
//             state_root: Hash256::zero(),
//             body: BeaconBlockBody {
//                 proposer_slashings: base_block.body.proposer_slashings,
//                 attester_slashings: base_block.body.attester_slashings,
//                 attestations: base_block.body.attestations,
//                 deposits: base_block.body.deposits,
//                 voluntary_exits: base_block.body.voluntary_exits,
//                 bls_to_execution_changes,
//                 sync_aggregate,
//                 randao_reveal: Signature::default(),
//                 eth1_data: Eth1Data {
//                     deposit_root: Hash256::zero(),
//                     block_hash: Hash256::zero(),
//                     deposit_count: 0,
//                 },
//                 graffiti: Graffiti::default(),
//                 execution_payload: ExecutionPayload::,
//             },
//         }
//     }
// }

// impl<T: EthSpec> BeaconBlock<T> {
//     pub fn from_ssz_bytes(bytes: &[u8], spec: &ChainSpec) -> Result<Self, ssz::DecodeError> {
//         let slot_len = <Slot as Decode>::ssz_fixed_len();
//         let slot_bytes = bytes
//             .get(0..slot_len)
//             .ok_or(DecodeError::InvalidByteLength {
//                 len: bytes.len(),
//                 expected: slot_len,
//             })?;

//         let slot = Slot::from_ssz_bytes(slot_bytes)?;

//         Ok(BeaconBlock {
//             slot,
//             proposer_index: 0,
//             parent_root: Hash256::zero(),
//             state_root: Hash256::zero(),
//             body: BeaconBlockBody::from_ssz_bytes(),
//         })
//     }

//     pub fn any_from_ssz_bytes(bytes: &[u8]) -> Result<Self, ssz::DecodeError> {
//         BeaconBlock::from_ssz_bytes(bytes, &ChainSpec::mainnet())
//     }
// }

// impl<T: EthSpec> BeaconBlock<T> {
//     /// Return a Capella block where the block has maximum size.
//     pub fn full(spec: &ChainSpec) -> Self {
//         // let base_block: BeaconBlockBase<_, Payload> = BeaconBlockBase::full(spec);
//         let bls_to_execution_changes = vec![
//             SignedBlsToExecutionChange {
//                 message: BlsToExecutionChange {
//                     validator_index: 0,
//                     from_bls_pubkey: PublicKeyBytes::empty(),
//                     to_execution_address: Address::zero(),
//                 },
//                 signature: Signature::empty()
//             };
//             T::max_bls_to_execution_changes()
//         ]
//         .into();
//         let sync_aggregate = SyncAggregate {
//             sync_committee_signature: AggregateSignature::empty(),
//             sync_committee_bits: BitVector::default(),
//         };
//         BeaconBlock {
//             slot: spec.genesis_slot,
//             proposer_index: 0,
//             parent_root: Hash256::zero(),
//             state_root: Hash256::zero(),
//             body: BeaconBlockBody {
//                 proposer_slashings: base_block.body.proposer_slashings,
//                 attester_slashings: base_block.body.attester_slashings,
//                 attestations: base_block.body.attestations,
//                 deposits: base_block.body.deposits,
//                 voluntary_exits: base_block.body.voluntary_exits,
//                 bls_to_execution_changes,
//                 sync_aggregate,
//                 randao_reveal: Signature::default(),
//                 eth1_data: Eth1Data {
//                     deposit_root: Hash256::zero(),
//                     block_hash: Hash256::zero(),
//                     deposit_count: 0,
//                 },
//                 graffiti: Graffiti::default(),
//                 execution_payload: ExecutionPayload::,
//             },
//         }
//     }
// }

// impl<T: EthSpec> BeaconBlock<T> {
//     pub fn from_ssz_bytes(bytes: &[u8], spec: &ChainSpec) -> Result<Self, ssz::DecodeError> {
//         let slot_len = <Slot as Decode>::ssz_fixed_len();
//         let slot_bytes = bytes
//             .get(0..slot_len)
//             .ok_or(DecodeError::InvalidByteLength {
//                 len: bytes.len(),
//                 expected: slot_len,
//             })?;

//         let slot = Slot::from_ssz_bytes(slot_bytes)?;

//         Ok(BeaconBlock {
//             slot,
//             proposer_index: 0,
//             parent_root: Hash256::zero(),
//             state_root: Hash256::zero(),
//             body: BeaconBlockBody::from_ssz_bytes(),
//         })
//     }

//     pub fn any_from_ssz_bytes(bytes: &[u8]) -> Result<Self, ssz::DecodeError> {
//         BeaconBlock::from_ssz_bytes(bytes, &ChainSpec::mainnet())
//     }
// }

// impl<T: EthSpec> BeaconBlock<T> {
//     /// Return a Capella block where the block has maximum size.
//     pub fn full(spec: &ChainSpec) -> Self {
//         // let base_block: BeaconBlockBase<_, Payload> = BeaconBlockBase::full(spec);
//         let bls_to_execution_changes = vec![
//             SignedBlsToExecutionChange {
//                 message: BlsToExecutionChange {
//                     validator_index: 0,
//                     from_bls_pubkey: PublicKeyBytes::empty(),
//                     to_execution_address: Address::zero(),
//                 },
//                 signature: Signature::empty()
//             };
//             T::max_bls_to_execution_changes()
//         ]
//         .into();
//         let sync_aggregate = SyncAggregate {
//             sync_committee_signature: AggregateSignature::empty(),
//             sync_committee_bits: BitVector::default(),
//         };
//         BeaconBlock {
//             slot: spec.genesis_slot,
//             proposer_index: 0,
//             parent_root: Hash256::zero(),
//             state_root: Hash256::zero(),
//             body: BeaconBlockBody {
//                 proposer_slashings: base_block.body.proposer_slashings,
//                 attester_slashings: base_block.body.attester_slashings,
//                 attestations: base_block.body.attestations,
//                 deposits: base_block.body.deposits,
//                 voluntary_exits: base_block.body.voluntary_exits,
//                 bls_to_execution_changes,
//                 sync_aggregate,
//                 randao_reveal: Signature::default(),
//                 eth1_data: Eth1Data {
//                     deposit_root: Hash256::zero(),
//                     block_hash: Hash256::zero(),
//                     deposit_count: 0,
//                 },
//                 graffiti: Graffiti::default(),
//                 execution_payload: ExecutionPayload::,
//             },
//         }
//     }
// }

// impl<T: EthSpec> BeaconBlock<T> {
//     pub fn from_ssz_bytes(bytes: &[u8], spec: &ChainSpec) -> Result<Self, ssz::DecodeError> {
//         let slot_len = <Slot as Decode>::ssz_fixed_len();
//         let slot_bytes = bytes
//             .get(0..slot_len)
//             .ok_or(DecodeError::InvalidByteLength {
//                 len: bytes.len(),
//                 expected: slot_len,
//             })?;

//         let slot = Slot::from_ssz_bytes(slot_bytes)?;

//         Ok(BeaconBlock {
//             slot,
//             proposer_index: 0,
//             parent_root: Hash256::zero(),
//             state_root: Hash256::zero(),
//             body: BeaconBlockBody::from_ssz_bytes(),
//         })
//     }

//     pub fn any_from_ssz_bytes(bytes: &[u8]) -> Result<Self, ssz::DecodeError> {
//         BeaconBlock::from_ssz_bytes(bytes, &ChainSpec::mainnet())
//     }
// }

// impl<T: EthSpec> BeaconBlock<T> {
//     /// Return a Capella block where the block has maximum size.
//     pub fn full(spec: &ChainSpec) -> Self {
//         // let base_block: BeaconBlockBase<_, Payload> = BeaconBlockBase::full(spec);
//         let bls_to_execution_changes = vec![
//             SignedBlsToExecutionChange {
//                 message: BlsToExecutionChange {
//                     validator_index: 0,
//                     from_bls_pubkey: PublicKeyBytes::empty(),
//                     to_execution_address: Address::zero(),
//                 },
//                 signature: Signature::empty()
//             };
//             T::max_bls_to_execution_changes()
//         ]
//         .into();
//         let sync_aggregate = SyncAggregate {
//             sync_committee_signature: AggregateSignature::empty(),
//             sync_committee_bits: BitVector::default(),
//         };
//         BeaconBlock {
//             slot: spec.genesis_slot,
//             proposer_index: 0,
//             parent_root: Hash256::zero(),
//             state_root: Hash256::zero(),
//             body: BeaconBlockBody {
//                 proposer_slashings: base_block.body.proposer_slashings,
//                 attester_slashings: base_block.body.attester_slashings,
//                 attestations: base_block.body.attestations,
//                 deposits: base_block.body.deposits,
//                 voluntary_exits: base_block.body.voluntary_exits,
//                 bls_to_execution_changes,
//                 sync_aggregate,
//                 randao_reveal: Signature::default(),
//                 eth1_data: Eth1Data {
//                     deposit_root: Hash256::zero(),
//                     block_hash: Hash256::zero(),
//                     deposit_count: 0,
//                 },
//                 graffiti: Graffiti::default(),
//                 execution_payload: ExecutionPayload::,
//             },
//         }
//     }
// }
