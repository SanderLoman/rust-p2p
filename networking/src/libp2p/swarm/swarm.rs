#![deny(unsafe_code)]

use crate::create_logger;
use crate::libp2p::transport::transport::setup_transport;

use libp2p::{
    identity,
    mdns::{Mdns, MdnsConfig},
    mplex::MplexConfig,
    noise::{Keypair, NoiseConfig, X25519Spec},
    swarm::SwarmBuilder,
    swarm::SwarmEvent,
    yamux::YamuxConfig,
    Multiaddr, PeerId, Swarm,
};
use futures::executor::block_on;
use std::error::Error;
use tokio::runtime::Handle;
use futures::future::FutureExt;

// #[derive(NetworkBehaviour)]
struct Behaviour {
    // Define your network behaviour here
}

pub async fn run() {
    // Create a random PeerId
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());

    // Create a transport.
    let transport = match setup_transport().await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to setup transport: {:?}", e);
            return;
        }
    };

    // Create a Swarm to manage peers and events.
    let mut swarm = {
        // Create a dummy behaviour.
        let behaviour = libp2p::swarm::dummy::Behaviour;

        // Create a Swarm to manage peers and events.
        let executor = {
            let executor = Handle::current();
            move |fut: _| {
                executor.spawn(fut);
            }
        };
        SwarmBuilder::with_executor(transport, behaviour, peer_id, executor).build()
    };



    // Listen on all interfaces and the port we desire (could listen on port 0 to listen on whatever port the OS assigns us).
    swarm.listen_on("/ip4/0.0.0.0/tcp/7777".parse().unwrap()).unwrap();

    
}
