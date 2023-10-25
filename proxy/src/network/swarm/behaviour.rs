use libp2p::identify;
use libp2p::swarm::dummy::ConnectionHandler;
use libp2p::swarm::NetworkBehaviour;

use crate::network::discovery::Discovery;

#[derive(NetworkBehaviour)]
pub struct Behaviour {
    pub discovery: Discovery,
    pub identify: identify::Behaviour,
}
