// use crate::rpc::protocol::ForkName;
// use libp2p::gossipsub::{IdentTopic as Topic, TopicHash};
// use serde_derive::{Deserialize, Serialize};
// use strum::AsRefStr;
// // use types::{ForkName, SubnetId, SyncSubnetId};

// use crate::types::Subnet;

// /// The gossipsub topic names.
// // These constants form a topic name of the form /TOPIC_PREFIX/TOPIC/ENCODING_POSTFIX
// // For example /eth2/beacon_block/ssz
// pub const TOPIC_PREFIX: &str = "eth2";
// pub const SSZ_SNAPPY_ENCODING_POSTFIX: &str = "ssz_snappy";
// pub const BEACON_BLOCK_TOPIC: &str = "beacon_block";
// pub const BEACON_AGGREGATE_AND_PROOF_TOPIC: &str = "beacon_aggregate_and_proof";
// pub const BEACON_ATTESTATION_PREFIX: &str = "beacon_attestation_";
// pub const VOLUNTARY_EXIT_TOPIC: &str = "voluntary_exit";
// pub const PROPOSER_SLASHING_TOPIC: &str = "proposer_slashing";
// pub const ATTESTER_SLASHING_TOPIC: &str = "attester_slashing";
// pub const SIGNED_CONTRIBUTION_AND_PROOF_TOPIC: &str = "sync_committee_contribution_and_proof";
// pub const SYNC_COMMITTEE_PREFIX_TOPIC: &str = "sync_committee_";
// pub const BLS_TO_EXECUTION_CHANGE_TOPIC: &str = "bls_to_execution_change";
// pub const LIGHT_CLIENT_FINALITY_UPDATE: &str = "light_client_finality_update";
// pub const LIGHT_CLIENT_OPTIMISTIC_UPDATE: &str = "light_client_optimistic_update";

// pub const BASE_CORE_TOPICS: [GossipKind; 5] = [
//     GossipKind::BeaconBlock,
//     GossipKind::BeaconAggregateAndProof,
//     GossipKind::VoluntaryExit,
//     GossipKind::ProposerSlashing,
//     GossipKind::AttesterSlashing,
// ];

// pub const ALTAIR_CORE_TOPICS: [GossipKind; 1] = [GossipKind::SignedContributionAndProof];

// pub const CAPELLA_CORE_TOPICS: [GossipKind; 1] = [GossipKind::BlsToExecutionChange];

// pub const LIGHT_CLIENT_GOSSIP_TOPICS: [GossipKind; 2] = [
//     GossipKind::LightClientFinalityUpdate,
//     GossipKind::LightClientOptimisticUpdate,
// ];

// /// Returns the core topics associated with each fork that are new to the previous fork
// pub fn fork_core_topics(fork_name: &ForkName) -> Vec<GossipKind> {
//     match fork_name {
//         ForkName::Base => BASE_CORE_TOPICS.to_vec(),
//         ForkName::Altair => ALTAIR_CORE_TOPICS.to_vec(),
//         ForkName::Merge => vec![],
//         ForkName::Capella => CAPELLA_CORE_TOPICS.to_vec(),
//     }
// }

// /// Returns all the topics that we need to subscribe to for a given fork
// /// including topics from older forks and new topics for the current fork.
// pub fn core_topics_to_subscribe(mut current_fork: ForkName) -> Vec<GossipKind> {
//     let mut topics = fork_core_topics(&current_fork);
//     while let Some(previous_fork) = current_fork.previous_fork() {
//         let previous_fork_topics = fork_core_topics(&previous_fork);
//         topics.extend(previous_fork_topics);
//         current_fork = previous_fork;
//     }
//     topics
// }

// /// A gossipsub topic which encapsulates the type of messages that should be sent and received over
// /// the pubsub protocol and the way the messages should be encoded.
// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
// pub struct GossipTopic {
//     /// The encoding of the topic.
//     encoding: GossipEncoding,
//     /// The fork digest of the topic,
//     pub fork_digest: [u8; 4],
//     /// The kind of topic.
//     kind: GossipKind,
// }

// /// Enum that brings these topics into the rust type system.
// // NOTE: There is intentionally no unknown type here. We only allow known gossipsub topics.
// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, AsRefStr)]
// #[strum(serialize_all = "snake_case")]
// pub enum GossipKind {
//     /// Topic for publishing beacon blocks.
//     BeaconBlock,
//     /// Topic for publishing aggregate attestations and proofs.
//     BeaconAggregateAndProof,
//     /// Topic for publishing raw attestations on a particular subnet.
//     #[strum(serialize = "beacon_attestation")]
//     Attestation(SubnetId),
//     /// Topic for publishing voluntary exits.
//     VoluntaryExit,
//     /// Topic for publishing block proposer slashings.
//     ProposerSlashing,
//     /// Topic for publishing attester slashings.
//     AttesterSlashing,
//     /// Topic for publishing partially aggregated sync committee signatures.
//     SignedContributionAndProof,
//     /// Topic for publishing unaggregated sync committee signatures on a particular subnet.
//     #[strum(serialize = "sync_committee")]
//     SyncCommitteeMessage(SyncSubnetId),
//     /// Topic for validator messages which change their withdrawal address.
//     BlsToExecutionChange,
//     /// Topic for publishing finality updates for light clients.
//     LightClientFinalityUpdate,
//     /// Topic for publishing optimistic updates for light clients.
//     LightClientOptimisticUpdate,
// }

// impl std::fmt::Display for GossipKind {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             GossipKind::Attestation(subnet_id) => write!(f, "beacon_attestation_{}", **subnet_id),
//             GossipKind::SyncCommitteeMessage(subnet_id) => {
//                 write!(f, "sync_committee_{}", **subnet_id)
//             }
//             x => f.write_str(x.as_ref()),
//         }
//     }
// }

// /// The known encoding types for gossipsub messages.
// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
// pub enum GossipEncoding {
//     /// Messages are encoded with SSZSnappy.
//     #[default]
//     SSZSnappy,
// }

// impl GossipTopic {
//     pub fn new(kind: GossipKind, encoding: GossipEncoding, fork_digest: [u8; 4]) -> Self {
//         GossipTopic {
//             encoding,
//             fork_digest,
//             kind,
//         }
//     }

//     /// Returns the encoding type for the gossipsub topic.
//     pub fn encoding(&self) -> &GossipEncoding {
//         &self.encoding
//     }

//     /// Returns a mutable reference to the fork digest of the gossipsub topic.
//     pub fn digest(&mut self) -> &mut [u8; 4] {
//         &mut self.fork_digest
//     }

//     /// Returns the kind of message expected on the gossipsub topic.
//     pub fn kind(&self) -> &GossipKind {
//         &self.kind
//     }

//     pub fn decode(topic: &str) -> Result<Self, String> {
//         let topic_parts: Vec<&str> = topic.split('/').collect();
//         if topic_parts.len() == 5 && topic_parts[1] == TOPIC_PREFIX {
//             let digest_bytes = hex::decode(topic_parts[2])
//                 .map_err(|e| format!("Could not decode fork_digest hex: {}", e))?;

//             if digest_bytes.len() != 4 {
//                 return Err(format!(
//                     "Invalid gossipsub fork digest size: {}",
//                     digest_bytes.len()
//                 ));
//             }

//             let mut fork_digest = [0; 4];
//             fork_digest.copy_from_slice(&digest_bytes);

//             let encoding = match topic_parts[4] {
//                 SSZ_SNAPPY_ENCODING_POSTFIX => GossipEncoding::SSZSnappy,
//                 _ => return Err(format!("Unknown encoding: {}", topic)),
//             };
//             let kind = match topic_parts[3] {
//                 BEACON_BLOCK_TOPIC => GossipKind::BeaconBlock,
//                 BEACON_AGGREGATE_AND_PROOF_TOPIC => GossipKind::BeaconAggregateAndProof,
//                 SIGNED_CONTRIBUTION_AND_PROOF_TOPIC => GossipKind::SignedContributionAndProof,
//                 VOLUNTARY_EXIT_TOPIC => GossipKind::VoluntaryExit,
//                 PROPOSER_SLASHING_TOPIC => GossipKind::ProposerSlashing,
//                 ATTESTER_SLASHING_TOPIC => GossipKind::AttesterSlashing,
//                 BLS_TO_EXECUTION_CHANGE_TOPIC => GossipKind::BlsToExecutionChange,
//                 LIGHT_CLIENT_FINALITY_UPDATE => GossipKind::LightClientFinalityUpdate,
//                 LIGHT_CLIENT_OPTIMISTIC_UPDATE => GossipKind::LightClientOptimisticUpdate,
//                 topic => match committee_topic_index(topic) {
//                     Some(subnet) => match subnet {
//                         Subnet::Attestation(s) => GossipKind::Attestation(s),
//                         Subnet::SyncCommittee(s) => GossipKind::SyncCommitteeMessage(s),
//                     },
//                     None => return Err(format!("Unknown topic: {}", topic)),
//                 },
//             };

//             return Ok(GossipTopic {
//                 encoding,
//                 fork_digest,
//                 kind,
//             });
//         }

//         Err(format!("Unknown topic: {}", topic))
//     }

//     pub fn subnet_id(&self) -> Option<Subnet> {
//         match self.kind() {
//             GossipKind::Attestation(subnet_id) => Some(Subnet::Attestation(*subnet_id)),
//             GossipKind::SyncCommitteeMessage(subnet_id) => Some(Subnet::SyncCommittee(*subnet_id)),
//             _ => None,
//         }
//     }
// }

// impl From<GossipTopic> for Topic {
//     fn from(topic: GossipTopic) -> Topic {
//         Topic::new(topic)
//     }
// }

// impl From<GossipTopic> for String {
//     fn from(topic: GossipTopic) -> String {
//         // Use the `Display` implementation below.
//         topic.to_string()
//     }
// }

// impl std::fmt::Display for GossipTopic {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let encoding = match self.encoding {
//             GossipEncoding::SSZSnappy => SSZ_SNAPPY_ENCODING_POSTFIX,
//         };

//         let kind = match self.kind {
//             GossipKind::BeaconBlock => BEACON_BLOCK_TOPIC.into(),
//             GossipKind::BeaconAggregateAndProof => BEACON_AGGREGATE_AND_PROOF_TOPIC.into(),
//             GossipKind::VoluntaryExit => VOLUNTARY_EXIT_TOPIC.into(),
//             GossipKind::ProposerSlashing => PROPOSER_SLASHING_TOPIC.into(),
//             GossipKind::AttesterSlashing => ATTESTER_SLASHING_TOPIC.into(),
//             GossipKind::Attestation(index) => format!("{}{}", BEACON_ATTESTATION_PREFIX, *index,),
//             GossipKind::SignedContributionAndProof => SIGNED_CONTRIBUTION_AND_PROOF_TOPIC.into(),
//             GossipKind::SyncCommitteeMessage(index) => {
//                 format!("{}{}", SYNC_COMMITTEE_PREFIX_TOPIC, *index)
//             }
//             GossipKind::BlsToExecutionChange => BLS_TO_EXECUTION_CHANGE_TOPIC.into(),
//             GossipKind::LightClientFinalityUpdate => LIGHT_CLIENT_FINALITY_UPDATE.into(),
//             GossipKind::LightClientOptimisticUpdate => LIGHT_CLIENT_OPTIMISTIC_UPDATE.into(),
//         };
//         write!(
//             f,
//             "/{}/{}/{}/{}",
//             TOPIC_PREFIX,
//             hex::encode(self.fork_digest),
//             kind,
//             encoding
//         )
//     }
// }

// impl From<Subnet> for GossipKind {
//     fn from(subnet_id: Subnet) -> Self {
//         match subnet_id {
//             Subnet::Attestation(s) => GossipKind::Attestation(s),
//             Subnet::SyncCommittee(s) => GossipKind::SyncCommitteeMessage(s),
//         }
//     }
// }

// // helper functions

// /// Get subnet id from an attestation subnet topic hash.
// pub fn subnet_from_topic_hash(topic_hash: &TopicHash) -> Option<Subnet> {
//     GossipTopic::decode(topic_hash.as_str()).ok()?.subnet_id()
// }

// // Determines if a string is an attestation or sync committee topic.
// fn committee_topic_index(topic: &str) -> Option<Subnet> {
//     if topic.starts_with(BEACON_ATTESTATION_PREFIX) {
//         return Some(Subnet::Attestation(SubnetId::new(
//             topic
//                 .trim_start_matches(BEACON_ATTESTATION_PREFIX)
//                 .parse::<u64>()
//                 .ok()?,
//         )));
//     } else if topic.starts_with(SYNC_COMMITTEE_PREFIX_TOPIC) {
//         return Some(Subnet::SyncCommittee(SyncSubnetId::new(
//             topic
//                 .trim_start_matches(SYNC_COMMITTEE_PREFIX_TOPIC)
//                 .parse::<u64>()
//                 .ok()?,
//         )));
//     }
//     None
// }
