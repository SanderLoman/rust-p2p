use crate::chain_spec::ChainSpec;
use crate::{Epoch, Hash256};
use parking_lot::RwLock;
use serde_derive::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use std::collections::HashMap;
use tree_hash_derive::TreeHash;

#[derive(
    arbitrary::Arbitrary,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Default,
    Serialize,
    Deserialize,
    Encode,
    Decode,
    TreeHash,
)]
pub struct Fork {
    #[serde(with = "serde_utils::bytes_4_hex")]
    pub previous_version: [u8; 4],
    #[serde(with = "serde_utils::bytes_4_hex")]
    pub current_version: [u8; 4],
    pub epoch: Epoch,
}

/// Specifies a fork of the `BeaconChain`, to prevent replay attacks.
///
/// Spec v0.12.1
#[derive(
    arbitrary::Arbitrary,
    Debug,
    Clone,
    PartialEq,
    Default,
    Serialize,
    Deserialize,
    Encode,
    Decode,
    TreeHash,
)]
pub struct ForkData {
    #[serde(with = "serde_utils::bytes_4_hex")]
    pub current_version: [u8; 4],
    pub genesis_validators_root: Hash256,
}

impl Fork {
    /// Return the fork version of the given ``epoch``.
    pub fn get_fork_version(&self, epoch: Epoch) -> [u8; 4] {
        if epoch < self.epoch {
            return self.previous_version;
        }
        self.current_version
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Hash)]
pub enum ForkName {
    Base,
    Altair,
    Merge,
    Capella,
}

// impl ForkName {
//     pub fn list_all() -> Vec<ForkName> {
//         vec![
//             ForkName::Base,
//             ForkName::Altair,
//             ForkName::Merge,
//             ForkName::Capella,
//         ]
//     }

//     // pub fn latest() -> ForkName {
//     //     // This unwrap is safe as long as we have 1+ forks. It is tested below.
//     //     *ForkName::list_all().last().unwrap()
//     // }

//     /// Set the activation slots in the given `ChainSpec` so that the fork named by `self`
//     /// is the only fork in effect from genesis.
//     // pub fn make_genesis_spec(&self, mut spec: ChainSpec) -> ChainSpec {
//     //     // Assumes GENESIS_EPOCH = 0, which is safe because it's a constant.
//     //     match self {
//     //         ForkName::Base => {
//     //             spec.altair_fork_epoch = None;
//     //             spec.bellatrix_fork_epoch = None;
//     //             spec.capella_fork_epoch = None;
//     //             spec
//     //         }
//     //         ForkName::Altair => {
//     //             spec.altair_fork_epoch = Some(0u64);
//     //             spec.bellatrix_fork_epoch = None;
//     //             spec.capella_fork_epoch = None;
//     //             spec
//     //         }
//     //         ForkName::Merge => {
//     //             spec.altair_fork_epoch = Some(0u64);
//     //             spec.bellatrix_fork_epoch = Some(0u64);
//     //             spec.capella_fork_epoch = None;
//     //             spec
//     //         }
//     //         ForkName::Capella => {
//     //             spec.altair_fork_epoch = Some(0u64);
//     //             spec.bellatrix_fork_epoch = Some(0u64);
//     //             spec.capella_fork_epoch = Some(0u64);
//     //             spec
//     //         }
//     //     }
//     // }

//     /// Return the name of the fork immediately prior to the current one.
//     ///
//     /// If `self` is `ForkName::Base` then `Base` is returned.
//     pub fn previous_fork(self) -> Option<ForkName> {
//         match self {
//             ForkName::Base => None,
//             ForkName::Altair => Some(ForkName::Base),
//             ForkName::Merge => Some(ForkName::Altair),
//             ForkName::Capella => Some(ForkName::Merge),
//         }
//     }

//     /// Return the name of the fork immediately after the current one.
//     ///
//     /// If `self` is the last known fork and has no successor, `None` is returned.
//     pub fn next_fork(self) -> Option<ForkName> {
//         match self {
//             ForkName::Base => Some(ForkName::Altair),
//             ForkName::Altair => Some(ForkName::Merge),
//             ForkName::Merge => Some(ForkName::Capella),
//             ForkName::Capella => None,
//         }
//     }
// }

/// Provides fork specific info like the current fork name and the fork digests corresponding to every valid fork.
// #[derive(Debug)]
pub struct ForkContext {
    current_fork: RwLock<ForkName>,
    fork_to_digest: HashMap<ForkName, [u8; 4]>,
    digest_to_fork: HashMap<[u8; 4], ForkName>,
}

impl ForkContext {
    /// Creates a new `ForkContext` object by enumerating all enabled forks and computing their
    /// fork digest.
    ///
    /// A fork is disabled in the `ChainSpec` if the activation slot corresponding to that fork is `None`.
    pub fn new(genesis_validators_root: Hash256, spec: &ChainSpec) -> Self {
        let fork_to_digest = vec![(
            ForkName::Capella,
            ChainSpec::compute_fork_digest(spec.capella_fork_version, genesis_validators_root),
        )];

        let fork_to_digest: HashMap<ForkName, [u8; 4]> = fork_to_digest.into_iter().collect();

        let digest_to_fork = fork_to_digest
            .clone()
            .into_iter()
            .map(|(k, v)| (v, k))
            .collect();

        Self {
            current_fork: RwLock::new(ForkName::Capella),
            fork_to_digest,
            digest_to_fork,
        }
    }

    /// Returns `true` if the provided `fork_name` exists in the `ForkContext` object.
    pub fn fork_exists(&self, fork_name: ForkName) -> bool {
        self.fork_to_digest.contains_key(&fork_name)
    }

    /// Returns the `current_fork`.
    pub fn current_fork(&self) -> ForkName {
        self.current_fork.read().clone()
    }

    /// Updates the `current_fork` field to a new fork.
    pub fn update_current_fork(&self, new_fork: ForkName) {
        *self.current_fork.write() = new_fork;
    }

    /// Returns the context bytes/fork_digest corresponding to the genesis fork version.
    pub fn genesis_context_bytes(&self) -> [u8; 4] {
        *self
            .fork_to_digest
            .get(&ForkName::Base)
            .expect("ForkContext must contain genesis context bytes")
    }

    /// Returns the fork type given the context bytes/fork_digest.
    /// Returns `None` if context bytes doesn't correspond to any valid `ForkName`.
    pub fn from_context_bytes(&self, context: [u8; 4]) -> Option<&ForkName> {
        self.digest_to_fork.get(&context)
    }

    /// Returns the context bytes/fork_digest corresponding to a fork name.
    /// Returns `None` if the `ForkName` has not been initialized.
    pub fn to_context_bytes(&self, fork_name: ForkName) -> Option<[u8; 4]> {
        self.fork_to_digest.get(&fork_name).cloned()
    }

    /// Returns all `fork_digest`s that are currently in the `ForkContext` object.
    pub fn all_fork_digests(&self) -> Vec<[u8; 4]> {
        self.digest_to_fork.keys().cloned().collect()
    }
}
