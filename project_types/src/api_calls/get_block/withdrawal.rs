use derivative::Derivative;
use ethereum_types::Address;
use serde_derive::{Deserialize, Serialize};
use ssz_derive::Encode;
use tree_hash_derive::TreeHash;

// Needed for the Withdrawals type
#[derive(Debug, Clone, Serialize, Encode, Deserialize, TreeHash, Derivative)]
pub struct Withdrawal {
    pub index: u64,
    pub validator_index: u64,
    pub address: Address,
    pub amount: u64,
}
