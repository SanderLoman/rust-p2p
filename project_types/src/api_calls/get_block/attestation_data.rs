use crate::{Hash256, Slot};

use super::checkpoint::Checkpoint;

pub struct AttestationData {
    pub slot: Slot,
    pub index: u64,
    pub beacon_block_root: Hash256,
    pub source: Checkpoint,
    pub target: Checkpoint,
}
