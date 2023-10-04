pub mod bitvector;
pub mod chain_spec;
pub mod execution_block_hash;
pub mod fork_context;
pub mod int_to_bytes;
pub mod eth_spec;

pub use eth_spec::EthSpec;
pub use ssz_types::{typenum, typenum::Unsigned, BitList, BitVector, FixedVector, VariableList};

pub type Hash256 = ethereum_types::H256;
pub type Epoch = u64;
pub type Uint256 = ethereum_types::U256;
