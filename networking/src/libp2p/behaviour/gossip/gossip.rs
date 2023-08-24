#![deny(unsafe_code)]

use crate::create_logger;
use libp2p::gossipsub::Behaviour;
use libp2p::gossipsub::*;
use libp2p::PeerId;

pub struct Gossipsub {
    /// The underlying libp2p gossipsub behaviour.
    pub gossip_behaviour: Behaviour,
}

impl Gossipsub {
    /// Creates a new gossipsub behaviour.
    pub fn new(local_peer_id: PeerId) -> Self {
        let logger = create_logger();
        let gossip_behaviour = Behaviour::new(MessageAuthenticity::Anonymous, local_peer_id);
        Gossipsub { gossip_behaviour }
    }
}