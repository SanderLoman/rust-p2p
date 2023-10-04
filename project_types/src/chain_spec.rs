// use crate::application_domain::{ApplicationDomain, APPLICATION_DOMAIN_BUILDER};
use crate::execution_block_hash::ExecutionBlockHash;
use crate::fork_context::{ForkData, Slot};
use crate::{Epoch, Hash256, Uint256};
use ethereum_types::Address;
use serde::{Deserializer, Serialize, Serializer};
use serde_derive::Deserialize;
use serde_utils::quoted_u64::MaybeQuoted;
use std::time::Duration;
use tree_hash::TreeHash;

/// This value is an application index of 0 with the bitmask applied (so it's equivalent to the bit mask).
/// Little endian hex: 0x00000001, Binary: 1000000000000000000000000
pub const APPLICATION_DOMAIN_BUILDER: u32 = 16777216;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ApplicationDomain {
    Builder,
}

impl ApplicationDomain {
    pub fn get_domain_constant(&self) -> u32 {
        match self {
            ApplicationDomain::Builder => APPLICATION_DOMAIN_BUILDER,
        }
    }
}

/// Each of the BLS signature domains.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Domain {
    BlsToExecutionChange,
    BeaconProposer,
    BeaconAttester,
    Randao,
    Deposit,
    VoluntaryExit,
    SelectionProof,
    AggregateAndProof,
    SyncCommittee,
    ContributionAndProof,
    SyncCommitteeSelectionProof,
    ApplicationMask(ApplicationDomain),
}

/// Lighthouse's internal configuration struct.
///
/// Contains a mixture of "preset" and "config" values w.r.t to the EF definitions.
#[derive(arbitrary::Arbitrary, PartialEq, Debug, Clone)]
pub struct ChainSpec {
    /*
     * Config name
     */
    pub config_name: Option<String>,

    /*
     * Constants
     */
    pub genesis_slot: Slot,
    pub far_future_epoch: Epoch,
    pub base_rewards_per_epoch: u64,
    pub deposit_contract_tree_depth: u64,

    /*
     * Misc
     */
    pub max_committees_per_slot: usize,
    pub target_committee_size: usize,
    pub min_per_epoch_churn_limit: u64,
    pub churn_limit_quotient: u64,
    pub shuffle_round_count: u8,
    pub min_genesis_active_validator_count: u64,
    pub min_genesis_time: u64,
    pub hysteresis_quotient: u64,
    pub hysteresis_downward_multiplier: u64,
    pub hysteresis_upward_multiplier: u64,

    /*
     * Capella hard fork params
     */
    pub capella_fork_version: [u8; 4],
    /// The Capella fork epoch is optional, with `None` representing "Capella never happens".
    pub capella_fork_epoch: Option<Epoch>,
    pub max_validators_per_withdrawals_sweep: u64,

    /*
     * Networking
     */
    pub boot_nodes: Vec<String>,
    pub network_id: u8,
    pub attestation_propagation_slot_range: u64,
    pub maximum_gossip_clock_disparity_millis: u64,
    pub target_aggregators_per_committee: u64,
    pub attestation_subnet_count: u64,
    pub subnets_per_node: u8,
    pub epochs_per_subnet_subscription: u64,
    pub gossip_max_size: u64,
    pub min_epochs_for_block_requests: u64,
    pub max_chunk_size: u64,
    pub ttfb_timeout: u64,
    pub resp_timeout: u64,
    pub message_domain_invalid_snappy: [u8; 4],
    pub message_domain_valid_snappy: [u8; 4],
    pub attestation_subnet_extra_bits: u8,
    pub attestation_subnet_prefix_bits: u8,
}

impl ChainSpec {
    // /// Construct a `ChainSpec` from a standard config.
    // pub fn from_config(config: &Config) -> Option<Self> {
    //     let spec = default_spec();
    //     config.apply_to_chain_spec(&spec)
    // }

    /// Returns an `EnrForkId` for the given `slot`.
    // pub fn enr_fork_id(&self, slot: Slot, genesis_validators_root: Hash256) -> EnrForkId {
    //     EnrForkId {
    //         fork_digest: self.fork_digest(slot, genesis_validators_root),
    //         next_fork_version: self.next_fork_version(slot),
    //         next_fork_epoch: self
    //             .next_fork_epoch(slot)
    //             .map(|(_, e)| e)
    //             .unwrap_or(self.far_future_epoch),
    //     }
    // }

    /// Returns the `ForkDigest` for the given slot.
    ///
    /// If `self.altair_fork_epoch == None`, then this function returns the genesis fork digest
    /// otherwise, returns the fork digest based on the slot.
    // pub fn fork_digest(&self, slot: Slot, genesis_validators_root: Hash256) -> [u8; 4] {
    //     let fork_name = self.fork_name_at_slot(slot);
    //     Self::compute_fork_digest(
    //         self.fork_version_for_name(fork_name),
    //         genesis_validators_root,
    //     )
    // }

    // /// Returns the `next_fork_version`.
    // ///
    // /// `next_fork_version = current_fork_version` if no future fork is planned,
    // pub fn next_fork_version(&self, slot: Slot) -> [u8; 4] {
    //     match self.next_fork_epoch(slot) {
    //         Some((fork, _)) => self.fork_version_for_name(fork),
    //         None => self.fork_version_for_name(self.fork_name_at_slot(slot)),
    // }

    // /// Returns the epoch of the next scheduled fork along with its corresponding `ForkName`.
    // ///
    // /// If no future forks are scheduled, this function returns `None`.
    // pub fn next_fork_epoch<T: EthSpec>(&self, slot: Slot) -> Option<(ForkName, Epoch)> {
    //     let current_fork_name = self.fork_name_at_slot::<T>(slot);
    //     let next_fork_name = current_fork_name.next_fork()?;
    //     let fork_epoch = self.fork_epoch(next_fork_name)?;
    //     Some((next_fork_name, fork_epoch))
    // }

    // /// Returns the name of the fork which is active at `slot`.
    // pub fn fork_name_at_slot<E: EthSpec>(&self, slot: Slot) -> ForkName {
    //     self.fork_name_at_epoch(slot.epoch(E::slots_per_epoch()))
    // }

    // /// Returns the name of the fork which is active at `epoch`.
    // pub fn fork_name_at_epoch(&self, epoch: Epoch) -> ForkName {
    //     match self.capella_fork_epoch {
    //         Some(fork_epoch) if epoch >= fork_epoch => ForkName::Capella,
    //         _ => match self.bellatrix_fork_epoch {
    //             Some(fork_epoch) if epoch >= fork_epoch => ForkName::Merge,
    //             _ => match self.altair_fork_epoch {
    //                 Some(fork_epoch) if epoch >= fork_epoch => ForkName::Altair,
    //                 _ => ForkName::Base,
    //             },
    //         },
    //     }
    // }

    // /// Returns the fork version for a named fork.
    // pub fn fork_version_for_name(&self, fork_name: ForkName) -> [u8; 4] {
    //     self.capella_fork_version
    // }

    // /// For a given fork name, return the epoch at which it activates.
    // pub fn fork_epoch(&self, fork_name: ForkName) -> Option<Epoch> {
    //     self.capella_fork_epoch
    // }

    // /// Returns a full `Fork` struct for a given `ForkName` or `None` if the fork does not yet have
    // /// an activation epoch.
    // pub fn fork_for_name(&self, fork_name: ForkName) -> Option<Fork> {
    //     let previous_fork_name = fork_name.previous_fork().unwrap_or(ForkName::Base);
    //     let epoch = self.fork_epoch(fork_name)?;

    //     Some(Fork {
    //         previous_version: self.fork_version_for_name(previous_fork_name),
    //         current_version: self.fork_version_for_name(fork_name),
    //         epoch,
    //     })
    // }

    /// Get the domain number, unmodified by the fork.
    ///
    /// Spec v0.12.1
    // pub fn get_domain_constant(&self, domain: Domain) -> u32 {
    //     match domain {
    //         Domain::BeaconProposer => self.domain_beacon_proposer,
    //         Domain::BeaconAttester => self.domain_beacon_attester,
    //         Domain::Randao => self.domain_randao,
    //         Domain::Deposit => self.domain_deposit,
    //         Domain::VoluntaryExit => self.domain_voluntary_exit,
    //         Domain::SelectionProof => self.domain_selection_proof,
    //         Domain::AggregateAndProof => self.domain_aggregate_and_proof,
    //         Domain::SyncCommittee => self.domain_sync_committee,
    //         Domain::ContributionAndProof => self.domain_contribution_and_proof,
    //         Domain::SyncCommitteeSelectionProof => self.domain_sync_committee_selection_proof,
    //         Domain::ApplicationMask(application_domain) => application_domain.get_domain_constant(),
    //         Domain::BlsToExecutionChange => self.domain_bls_to_execution_change,
    //     }
    // }

    /// Get the domain that represents the fork meta and signature domain.
    ///
    /// Spec v0.12.1
    // pub fn get_domain(
    //     &self,
    //     epoch: Epoch,
    //     domain: Domain,
    //     fork: &Fork,
    //     genesis_validators_root: Hash256,
    // ) -> Hash256 {
    //     let fork_version = fork.get_fork_version(epoch);
    //     self.compute_domain(domain, fork_version, genesis_validators_root)
    // }

    /// Get the domain for a deposit signature.
    ///
    /// Deposits are valid across forks, thus the deposit domain is computed
    /// with the genesis fork version.
    ///
    /// Spec v0.12.1
    // pub fn get_deposit_domain(&self) -> Hash256 {
    //     self.compute_domain(Domain::Deposit, self.genesis_fork_version, Hash256::zero())
    // }

    // This should be updated to include the current fork and the genesis validators root, but discussion is ongoing:
    //
    // https://github.com/ethereum/builder-specs/issues/14
    // pub fn get_builder_domain(&self) -> Hash256 {
    //     self.compute_domain(
    //         Domain::ApplicationMask(ApplicationDomain::Builder),
    //         self.genesis_fork_version,
    //         Hash256::zero(),
    //     )
    // }

    /// Return the 32-byte fork data root for the `current_version` and `genesis_validators_root`.
    ///
    /// This is used primarily in signature domains to avoid collisions across forks/chains.
    ///
    /// Spec v0.12.1
    pub fn compute_fork_data_root(
        current_version: [u8; 4],
        genesis_validators_root: Hash256,
    ) -> Hash256 {
        ForkData {
            current_version,
            genesis_validators_root,
        }
        .tree_hash_root()
    }

    /// Return the 4-byte fork digest for the `current_version` and `genesis_validators_root`.
    ///
    /// This is a digest primarily used for domain separation on the p2p layer.
    /// 4-bytes suffices for practical separation of forks/chains.
    pub fn compute_fork_digest(
        current_version: [u8; 4],
        genesis_validators_root: Hash256,
    ) -> [u8; 4] {
        let mut result = [0; 4];
        let root = Self::compute_fork_data_root(current_version, genesis_validators_root);
        result.copy_from_slice(
            root.as_bytes()
                .get(0..4)
                .expect("root hash is at least 4 bytes"),
        );
        result
    }

    // /// Compute a domain by applying the given `fork_version`.
    // pub fn compute_domain(
    //     &self,
    //     domain: Domain,
    //     fork_version: [u8; 4],
    //     genesis_validators_root: Hash256,
    // ) -> Hash256 {
    //     let domain_constant = self.get_domain_constant(domain);

    //     let mut domain = [0; 32];
    //     domain[0..4].copy_from_slice(&int_to_bytes4(domain_constant));
    //     domain[4..].copy_from_slice(
    //         Self::compute_fork_data_root(fork_version, genesis_validators_root)
    //             .as_bytes()
    //             .get(..28)
    //             .expect("fork has is 32 bytes so first 28 bytes should exist"),
    //     );

    //     Hash256::from(domain)
    // }

    pub fn maximum_gossip_clock_disparity(&self) -> Duration {
        Duration::from_millis(self.maximum_gossip_clock_disparity_millis)
    }

    pub fn ttfb_timeout(&self) -> Duration {
        Duration::from_secs(self.ttfb_timeout)
    }

    pub fn resp_timeout(&self) -> Duration {
        Duration::from_secs(self.resp_timeout)
    }

    /// Returns a `ChainSpec` compatible with the Ethereum Foundation specification.
    pub fn mainnet() -> Self {
        Self {
            /*
             * Config name
             */
            config_name: Some("mainnet".to_string()),

            /*
             * Constants
             */
            genesis_slot: 0u64,
            far_future_epoch: u64::MAX,
            base_rewards_per_epoch: 4,
            deposit_contract_tree_depth: 32,

            /*
             * Misc
             */
            max_committees_per_slot: 64,
            target_committee_size: 128,
            min_per_epoch_churn_limit: 4,
            churn_limit_quotient: 65_536,
            shuffle_round_count: 90,
            min_genesis_active_validator_count: 16_384,
            min_genesis_time: 1606824000, // Dec 1, 2020
            hysteresis_quotient: 4,
            hysteresis_downward_multiplier: 1,
            hysteresis_upward_multiplier: 5,

            /*
             * Capella hard fork params
             */
            capella_fork_version: [0x03, 00, 00, 00],
            capella_fork_epoch: Some(194048u64),
            max_validators_per_withdrawals_sweep: 16384,

            /*
             * Network specific
             */
            boot_nodes: vec![],
            network_id: 1, // mainnet network id
            attestation_propagation_slot_range: 32,
            attestation_subnet_count: 64,
            subnets_per_node: 2,
            maximum_gossip_clock_disparity_millis: 500,
            target_aggregators_per_committee: 16,
            epochs_per_subnet_subscription: 256,
            gossip_max_size: default_gossip_max_size(),
            min_epochs_for_block_requests: default_min_epochs_for_block_requests(),
            max_chunk_size: default_max_chunk_size(),
            ttfb_timeout: default_ttfb_timeout(),
            resp_timeout: default_resp_timeout(),
            message_domain_invalid_snappy: default_message_domain_invalid_snappy(),
            message_domain_valid_snappy: default_message_domain_valid_snappy(),
            attestation_subnet_extra_bits: default_attestation_subnet_extra_bits(),
            attestation_subnet_prefix_bits: default_attestation_subnet_prefix_bits(),
        }
    }
}

impl Default for ChainSpec {
    fn default() -> Self {
        Self::mainnet()
    }
}

/// Exact implementation of the *config* object from the Ethereum spec (YAML/JSON).
///
/// Fields relevant to hard forks after Altair should be optional so that we can continue
/// to parse Altair configs. This default approach turns out to be much simpler than trying to
/// make `Config` a superstruct because of the hassle of deserializing an untagged enum.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct Config {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_name: Option<String>,

    #[serde(default)]
    pub preset_base: String,

    #[serde(default = "default_terminal_total_difficulty")]
    #[serde(with = "serde_utils::quoted_u256")]
    pub terminal_total_difficulty: Uint256,
    // #[serde(default = "default_terminal_block_hash")]
    pub terminal_block_hash: ExecutionBlockHash,
    // #[serde(default = "default_terminal_block_hash_activation_epoch")]
    pub terminal_block_hash_activation_epoch: Epoch,
    #[serde(default = "default_safe_slots_to_import_optimistically")]
    #[serde(with = "serde_utils::quoted_u64")]
    pub safe_slots_to_import_optimistically: u64,

    #[serde(with = "serde_utils::quoted_u64")]
    min_genesis_active_validator_count: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    min_genesis_time: u64,
    #[serde(with = "serde_utils::bytes_4_hex")]
    genesis_fork_version: [u8; 4],
    #[serde(with = "serde_utils::quoted_u64")]
    genesis_delay: u64,

    #[serde(with = "serde_utils::bytes_4_hex")]
    altair_fork_version: [u8; 4],
    #[serde(serialize_with = "serialize_fork_epoch")]
    #[serde(deserialize_with = "deserialize_fork_epoch")]
    pub altair_fork_epoch: Option<MaybeQuoted<Epoch>>,

    #[serde(default = "default_bellatrix_fork_version")]
    #[serde(with = "serde_utils::bytes_4_hex")]
    bellatrix_fork_version: [u8; 4],
    #[serde(default)]
    #[serde(serialize_with = "serialize_fork_epoch")]
    #[serde(deserialize_with = "deserialize_fork_epoch")]
    pub bellatrix_fork_epoch: Option<MaybeQuoted<Epoch>>,

    #[serde(default = "default_capella_fork_version")]
    #[serde(with = "serde_utils::bytes_4_hex")]
    capella_fork_version: [u8; 4],
    #[serde(default)]
    #[serde(serialize_with = "serialize_fork_epoch")]
    #[serde(deserialize_with = "deserialize_fork_epoch")]
    pub capella_fork_epoch: Option<MaybeQuoted<Epoch>>,

    #[serde(with = "serde_utils::quoted_u64")]
    seconds_per_slot: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    seconds_per_eth1_block: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    min_validator_withdrawability_delay: Epoch,
    #[serde(with = "serde_utils::quoted_u64")]
    shard_committee_period: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    eth1_follow_distance: u64,
    #[serde(default = "default_subnets_per_node")]
    #[serde(with = "serde_utils::quoted_u8")]
    subnets_per_node: u8,

    #[serde(with = "serde_utils::quoted_u64")]
    inactivity_score_bias: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    inactivity_score_recovery_rate: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    min_per_epoch_churn_limit: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    churn_limit_quotient: u64,

    #[serde(skip_serializing_if = "Option::is_none")]
    proposer_score_boost: Option<MaybeQuoted<u64>>,

    #[serde(with = "serde_utils::quoted_u64")]
    deposit_chain_id: u64,
    #[serde(with = "serde_utils::quoted_u64")]
    deposit_network_id: u64,
    deposit_contract_address: Address,

    #[serde(default = "default_gossip_max_size")]
    #[serde(with = "serde_utils::quoted_u64")]
    gossip_max_size: u64,
    #[serde(default = "default_min_epochs_for_block_requests")]
    #[serde(with = "serde_utils::quoted_u64")]
    min_epochs_for_block_requests: u64,
    #[serde(default = "default_max_chunk_size")]
    #[serde(with = "serde_utils::quoted_u64")]
    max_chunk_size: u64,
    #[serde(default = "default_ttfb_timeout")]
    #[serde(with = "serde_utils::quoted_u64")]
    ttfb_timeout: u64,
    #[serde(default = "default_resp_timeout")]
    #[serde(with = "serde_utils::quoted_u64")]
    resp_timeout: u64,
    #[serde(default = "default_message_domain_invalid_snappy")]
    #[serde(with = "serde_utils::bytes_4_hex")]
    message_domain_invalid_snappy: [u8; 4],
    #[serde(default = "default_message_domain_valid_snappy")]
    #[serde(with = "serde_utils::bytes_4_hex")]
    message_domain_valid_snappy: [u8; 4],
    #[serde(default = "default_attestation_subnet_extra_bits")]
    #[serde(with = "serde_utils::quoted_u8")]
    attestation_subnet_extra_bits: u8,
    #[serde(default = "default_attestation_subnet_prefix_bits")]
    #[serde(with = "serde_utils::quoted_u8")]
    attestation_subnet_prefix_bits: u8,
}

fn default_bellatrix_fork_version() -> [u8; 4] {
    // This value shouldn't be used.
    [0xff, 0xff, 0xff, 0xff]
}

fn default_capella_fork_version() -> [u8; 4] {
    // TODO: determine if the bellatrix example should be copied like this
    [0xff, 0xff, 0xff, 0xff]
}

/// Placeholder value: 2^256-2^10 (115792089237316195423570985008687907853269984665640564039457584007913129638912).
///
/// Taken from https://github.com/ethereum/consensus-specs/blob/d5e4828aecafaf1c57ef67a5f23c4ae7b08c5137/configs/mainnet.yaml#L15-L16
const fn default_terminal_total_difficulty() -> Uint256 {
    ethereum_types::U256([
        18446744073709550592,
        18446744073709551615,
        18446744073709551615,
        18446744073709551615,
    ])
}

// fn default_terminal_block_hash() -> ExecutionBlockHash {
//     ExecutionBlockHash::zero()
// }

// fn default_terminal_block_hash_activation_epoch() -> Epoch {
//     Epoch::new(u64::MAX)
// }

fn default_safe_slots_to_import_optimistically() -> u64 {
    128u64
}

fn default_subnets_per_node() -> u8 {
    2u8
}

const fn default_gossip_max_size() -> u64 {
    10485760
}

const fn default_min_epochs_for_block_requests() -> u64 {
    33024
}

const fn default_max_chunk_size() -> u64 {
    10485760
}

const fn default_ttfb_timeout() -> u64 {
    5
}

const fn default_resp_timeout() -> u64 {
    10
}

const fn default_message_domain_invalid_snappy() -> [u8; 4] {
    [0, 0, 0, 0]
}

const fn default_message_domain_valid_snappy() -> [u8; 4] {
    [1, 0, 0, 0]
}

const fn default_attestation_subnet_extra_bits() -> u8 {
    0
}

const fn default_attestation_subnet_prefix_bits() -> u8 {
    6
}

// impl Default for Config {
//     fn default() -> Self {
//         let chain_spec = MainnetEthSpec::default_spec();
//         Config::from_chain_spec::<MainnetEthSpec>(&chain_spec)
//     }
// }

/// Util function to serialize a `None` fork epoch value
/// as `Epoch::max_value()`.
fn serialize_fork_epoch<S>(val: &Option<MaybeQuoted<Epoch>>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match val {
        None => MaybeQuoted {
            value: Epoch::max_value(),
        }
        .serialize(s),
        Some(epoch) => epoch.serialize(s),
    }
}

/// Util function to deserialize a u64::max() fork epoch as `None`.
fn deserialize_fork_epoch<'de, D>(deserializer: D) -> Result<Option<MaybeQuoted<Epoch>>, D::Error>
where
    D: Deserializer<'de>,
{
    let decoded: Option<MaybeQuoted<Epoch>> = serde::de::Deserialize::deserialize(deserializer)?;
    if let Some(fork_epoch) = decoded {
        if fork_epoch.value != Epoch::max_value() {
            return Ok(Some(fork_epoch));
        }
    }
    Ok(None)
}

impl Config {
    // pub fn eth_spec_id(&self) -> Option<EthSpecId> {
    //     match self.preset_base.as_str() {
    //         "minimal" => Some(EthSpecId::Minimal),
    //         "mainnet" => Some(EthSpecId::Mainnet),
    //         "gnosis" => Some(EthSpecId::Gnosis),
    //         _ => None,
    //     }
    // }

    // pub fn from_chain_spec<T: EthSpec>(spec: &ChainSpec) -> Self {
    //     Self {
    //         config_name: spec.config_name.clone(),
    //         preset_base: T::spec_name().to_string(),

    //         terminal_total_difficulty: spec.terminal_total_difficulty,
    //         terminal_block_hash: spec.terminal_block_hash,
    //         terminal_block_hash_activation_epoch: spec.terminal_block_hash_activation_epoch,
    //         safe_slots_to_import_optimistically: spec.safe_slots_to_import_optimistically,

    //         min_genesis_active_validator_count: spec.min_genesis_active_validator_count,
    //         min_genesis_time: spec.min_genesis_time,
    //         genesis_fork_version: spec.genesis_fork_version,
    //         genesis_delay: spec.genesis_delay,

    //         altair_fork_version: spec.altair_fork_version,
    //         altair_fork_epoch: spec
    //             .altair_fork_epoch
    //             .map(|epoch| MaybeQuoted { value: epoch }),
    //         bellatrix_fork_version: spec.bellatrix_fork_version,
    //         bellatrix_fork_epoch: spec
    //             .bellatrix_fork_epoch
    //             .map(|epoch| MaybeQuoted { value: epoch }),
    //         capella_fork_version: spec.capella_fork_version,
    //         capella_fork_epoch: spec
    //             .capella_fork_epoch
    //             .map(|epoch| MaybeQuoted { value: epoch }),

    //         seconds_per_slot: spec.seconds_per_slot,
    //         seconds_per_eth1_block: spec.seconds_per_eth1_block,
    //         min_validator_withdrawability_delay: spec.min_validator_withdrawability_delay,
    //         shard_committee_period: spec.shard_committee_period,
    //         eth1_follow_distance: spec.eth1_follow_distance,
    //         subnets_per_node: spec.subnets_per_node,

    //         inactivity_score_bias: spec.inactivity_score_bias,
    //         inactivity_score_recovery_rate: spec.inactivity_score_recovery_rate,
    //         ejection_balance: spec.ejection_balance,
    //         churn_limit_quotient: spec.churn_limit_quotient,
    //         min_per_epoch_churn_limit: spec.min_per_epoch_churn_limit,

    //         proposer_score_boost: spec.proposer_score_boost.map(|value| MaybeQuoted { value }),

    //         deposit_chain_id: spec.deposit_chain_id,
    //         deposit_network_id: spec.deposit_network_id,
    //         deposit_contract_address: spec.deposit_contract_address,

    //         gossip_max_size: spec.gossip_max_size,
    //         min_epochs_for_block_requests: spec.min_epochs_for_block_requests,
    //         max_chunk_size: spec.max_chunk_size,
    //         ttfb_timeout: spec.ttfb_timeout,
    //         resp_timeout: spec.resp_timeout,
    //         message_domain_invalid_snappy: spec.message_domain_invalid_snappy,
    //         message_domain_valid_snappy: spec.message_domain_valid_snappy,
    //         attestation_subnet_extra_bits: spec.attestation_subnet_extra_bits,
    //         attestation_subnet_prefix_bits: spec.attestation_subnet_prefix_bits,
    //     }
    // }

    // pub fn from_file(filename: &Path) -> Result<Self, String> {
    //     let f = File::open(filename)
    //         .map_err(|e| format!("Error opening spec at {}: {:?}", filename.display(), e))?;
    //     serde_yaml::from_reader(f)
    //         .map_err(|e| format!("Error parsing spec at {}: {:?}", filename.display(), e))
    // }

    // pub fn apply_to_chain_spec<T: EthSpec>(&self, chain_spec: &ChainSpec) -> Option<ChainSpec> {
    //     // Pattern match here to avoid missing any fields.
    //     let &Config {
    //         ref config_name,
    //         ref preset_base,
    //         terminal_total_difficulty,
    //         terminal_block_hash,
    //         terminal_block_hash_activation_epoch,
    //         safe_slots_to_import_optimistically,
    //         min_genesis_active_validator_count,
    //         min_genesis_time,
    //         genesis_fork_version,
    //         genesis_delay,
    //         altair_fork_version,
    //         altair_fork_epoch,
    //         bellatrix_fork_epoch,
    //         bellatrix_fork_version,
    //         capella_fork_epoch,
    //         capella_fork_version,
    //         seconds_per_slot,
    //         seconds_per_eth1_block,
    //         min_validator_withdrawability_delay,
    //         shard_committee_period,
    //         eth1_follow_distance,
    //         subnets_per_node,
    //         inactivity_score_bias,
    //         inactivity_score_recovery_rate,
    //         ejection_balance,
    //         min_per_epoch_churn_limit,
    //         churn_limit_quotient,
    //         proposer_score_boost,
    //         deposit_chain_id,
    //         deposit_network_id,
    //         deposit_contract_address,
    //         gossip_max_size,
    //         min_epochs_for_block_requests,
    //         max_chunk_size,
    //         ttfb_timeout,
    //         resp_timeout,
    //         message_domain_invalid_snappy,
    //         message_domain_valid_snappy,
    //         attestation_subnet_extra_bits,
    //         attestation_subnet_prefix_bits,
    //     } = self;

    //     if preset_base != T::spec_name().to_string().as_str() {
    //         return None;
    //     }

    //     Some(ChainSpec {
    //         config_name: config_name.clone(),
    //         min_genesis_active_validator_count,
    //         min_genesis_time,
    //         genesis_fork_version,
    //         genesis_delay,
    //         altair_fork_version,
    //         altair_fork_epoch: altair_fork_epoch.map(|q| q.value),
    //         bellatrix_fork_epoch: bellatrix_fork_epoch.map(|q| q.value),
    //         bellatrix_fork_version,
    //         capella_fork_epoch: capella_fork_epoch.map(|q| q.value),
    //         capella_fork_version,
    //         seconds_per_slot,
    //         seconds_per_eth1_block,
    //         min_validator_withdrawability_delay,
    //         shard_committee_period,
    //         eth1_follow_distance,
    //         subnets_per_node,
    //         inactivity_score_bias,
    //         inactivity_score_recovery_rate,
    //         ejection_balance,
    //         min_per_epoch_churn_limit,
    //         churn_limit_quotient,
    //         proposer_score_boost: proposer_score_boost.map(|q| q.value),
    //         deposit_chain_id,
    //         deposit_network_id,
    //         deposit_contract_address,
    //         terminal_total_difficulty,
    //         terminal_block_hash,
    //         terminal_block_hash_activation_epoch,
    //         safe_slots_to_import_optimistically,
    //         gossip_max_size,
    //         min_epochs_for_block_requests,
    //         max_chunk_size,
    //         ttfb_timeout,
    //         resp_timeout,
    //         message_domain_invalid_snappy,
    //         message_domain_valid_snappy,
    //         attestation_subnet_extra_bits,
    //         attestation_subnet_prefix_bits,
    //         ..chain_spec.clone()
    //     })
    // }
}

/// A simple wrapper to permit the in-line use of `?`.
fn option_wrapper<F, T>(f: F) -> Option<T>
where
    F: Fn() -> Option<T>,
{
    f()
}
