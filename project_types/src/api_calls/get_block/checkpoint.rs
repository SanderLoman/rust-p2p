use serde_derive::{Deserialize, Serialize};

use crate::{Epoch, Hash256};

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Default, PartialEq, Eq)]
pub struct Checkpoint {
    pub epoch: Epoch,
    pub root: Hash256,
}
