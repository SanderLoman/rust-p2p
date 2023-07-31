#![deny(unsafe_code)]

use libp2p::{
    ping::{Behaviour, Config as PingConfig},
    swarm::NetworkBehaviour,
    Swarm,
    PeerId,
    swarm::SwarmEvent,
};

// #[derive(NetworkBehaviour)]
pub struct CustomBehaviour {

}

impl CustomBehaviour {
    pub fn new() -> Self {
        CustomBehaviour {}
    }
}