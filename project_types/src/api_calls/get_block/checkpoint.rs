use crate::{Epoch, Hash256};

pub struct Checkpoint {
    pub epoch: Epoch,
    pub root: Hash256,
}
