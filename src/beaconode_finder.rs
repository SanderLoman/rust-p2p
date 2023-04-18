use libp2p::*;

pub struct BeaconodeFinder {
    swarm: Swarm<TBehaviour>,
}