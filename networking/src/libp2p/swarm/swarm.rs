#![deny(unsafe_code)]

use libp2p::{
    swarm::SwarmBuilder,
    PeerId,
    Multiaddr,
    Swarm,
    mdns::{Mdns, MdnsConfig},
    noise::{NoiseConfig, X25519Spec, Keypair},
    yamux::YamuxConfig,
    identity,
    mplex::MplexConfig,
    swarm::SwarmEvent,
};

// #[derive(NetworkBehaviour)]
struct Behaviour {
    // Define your network behaviour here
}

fn main() {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    let transport = libp2p::development_transport(local_key.clone()).unwrap();

    let behaviour = Behaviour {
        // Initialize your network behaviour here
    };

    let swarm = SwarmBuilder::new(transport, behaviour, local_peer_id)
        .executor(Box::new(|fut| {
            async_std::task::spawn(fut);
        }))
        .build();

    async_std::task::block_on(async {
        let mut swarm = swarm;

        // Listen on all interfaces and whatever port the OS assigns
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();

        loop {
            match swarm.select_next_some().await {
                SwarmEvent::Behaviour(event) => {
                    // Handle behaviour events here
                }
                _ => {}
            }
        }
    });
}
