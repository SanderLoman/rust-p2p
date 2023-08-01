#![deny(unsafe_code)]

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

// #[derive(NetworkBehaviour)]
struct Behaviour {
    // Define your network behaviour here
}

pub async fn run() {
    // Create a random PeerId
    let id_keys = identity::Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());

    // Create a transport.
    let transport = libp2p::development_transport(id_keys).await.unwrap();

    // Create a Swarm to manage peers and events.
    let mut swarm = {
        // Create a Kademlia behaviour.
        let behaviour = Behaviour {};

        // Create a Swarm to manage peers and events.
        let executor = {
            let executor = tokio::runtime::Handle::current();
            move |_| executor.enter(|| ())
        }
        SwarmBuilder::with_executor(transport, behaviour, peer_id, executor)
    };

    // Listen on all interfaces and whatever port the OS assigns.
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();

    // Reach out to another node.
    if let Some(to_dial) = std::env::args().nth(1) {
        let addr = to_dial.parse::<Multiaddr>().unwrap();
        swarm.dial_addr(addr).unwrap();
        println!("Dialed {:?}", to_dial);
    }

    // Kick it off.
    let mut listening = false;
    let mut listening_addr = None;
    loop {
        match swarm.next_event().await {
            SwarmEvent::NewListenAddr { address, .. } => {
                if !listening {
                    listening = true;
                    listening_addr = Some(address);
                    println!("Listening on {:?}", address);
                }
            }
            SwarmEvent::Behaviour(_) => {}
            _ => {}
        }
    }
}
