// pub mod methods;
// pub mod protocol;
// pub mod transport;

pub mod swarm;

use libp2p::Swarm;
use slog::Logger;

use crate::network::swarm::behaviour::Behaviour;
use crate::{NetworkManager, NetworkRequests};

pub struct Network<N: NetworkRequests>
where {
    // The libp2p Swarm, this will handle incoming and outgoing requests so that we can redirect them. Instead of sending data right back to them 
    swarm: Swarm<Behaviour>,

    // The NetworkManager, this will handle the requests and redirect them to the correct place.
    network_service: NetworkManager<N>,

    // The Logger for the network service.
    log: Logger,
}
