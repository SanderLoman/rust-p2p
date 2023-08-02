#![deny(unsafe_code)]

use crate::create_logger;
use crate::libp2p::transport::transport::setup_transport;

use libp2p::{identity, swarm::SwarmBuilder, PeerId};
use std::error::Error;
use tokio::runtime::Handle;

// #[derive(NetworkBehaviour)]
struct Behaviour {
    // Define your network behaviour here
}

pub async fn setup_swarm() -> Result<(), Box<dyn Error>> {
    let log = create_logger();

    // Get the transport and the local key pair.
    let (transport, local_keys) = setup_transport().await.unwrap();

    // We use the key pair from the transport.rs file otherwise we generate 2 different keys.
    let local_keys = local_keys;
    let local_peer_id = PeerId::from(local_keys.public());

    // Here we just use the transport from the transport.rs file.
    let transport = transport;

    let mut swarm = {
        // Dummy behaviour, this will be changed later.
        let behaviour = libp2p::swarm::dummy::Behaviour;

        let executor = {
            let executor = Handle::current();
            move |fut: _| {
                executor.spawn(fut);
            }
        };

        // Build the Swarm
        SwarmBuilder::with_executor(transport, behaviour, local_peer_id, executor).build()
    };

    // Listen on all interfaces and the port we desire,
    // could listen on port 0 to listen on whatever port the OS assigns us.
    swarm
        .listen_on("/ip4/0.0.0.0/tcp/7777".parse().unwrap())
        .unwrap();

    slog::debug!(log, "{:?}", swarm.network_info());

    Ok(())
}
