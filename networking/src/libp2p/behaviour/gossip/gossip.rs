#![deny(unsafe_code)]

use crate::create_logger;
use libp2p::gossipsub::Behaviour;
use libp2p::gossipsub::Message;

pub struct Gossipsub {
    /// The underlying libp2p gossipsub behaviour.
    pub gossip_behaviour: Behaviour,
}