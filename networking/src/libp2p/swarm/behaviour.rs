#![deny(unsafe_code)]

use libp2p::{
    ping::{Behaviour, Config as PingConfig},
    swarm::NetworkBehaviour,
    swarm::SwarmEvent,
    PeerId, Swarm,
};

// Write our own custom behaviour later on when we have a better understanding
// of what we want to do when we discover peers and connect to them.
pub trait CustomBehaviour {
    fn new(&self);
}

impl<T: NetworkBehaviour> CustomBehaviour for T {
    fn new(&self) {}
}
