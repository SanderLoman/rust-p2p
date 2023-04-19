use discv5::*;
use eyre::Result;
use libp2p::*;

trait MyBehaviour {
    fn new() -> Self;

    fn wagmi() -> Result<Swarm<MyBehaviour>>;


}

impl MyBehaviour {
    fn new() -> Self {
        // Generate a random keypair for our node
        let keypair = Keypair::generate_ed25519();
        let peer_id = PeerId::from(keypair.public());

        // Create the Discv5 configuration
        let config = Discv5Config::default();

        // Create a new instance of MyBehaviour with a Discv5 field
        let my_behaviour = MyBehaviour {
            discv5: Discv5::new(keypair.clone().into_peer_id(), config).unwrap(),
        };

        my_behaviour
    }
}

// Create the Swarm using our custom behavior
pub async fn wagmi() -> Result<Swarm<MyBehaviour>> {
    // Generate a random keypair for our node
    let keypair = Keypair::generate_ed25519();
    let peer_id = PeerId::from(keypair.public());

    // Create the Discv5 configuration
    let config = Discv5Config::default();

    // Create a new instance of MyBehaviour with a Discv5 field
    let my_behaviour = MyBehaviour {
        discv5: Discv5::new(keypair.clone().into_peer_id(), config).unwrap(),
    };

    // Create the Swarm using our custom behavior
    let swarm = Swarm::new(my_behaviour, Transport::TCP)
        .build()
        .await?;

    Ok(swarm)
}

pub async fn wagmi() -> Result<Swarm<MyBehaviour>> {
    // Generate a random keypair for our node
    let keypair = Keypair::generate_ed25519();
    let peer_id = PeerId::from(keypair.public());

    // Create the Discv5 configuration
    let config = Discv5Config::default();

    // Create a new instance of MyBehaviour with a Discv5 field
    let my_behaviour = MyBehaviour {
        discv5: Discv5::new(keypair.clone().into_peer_id(), config).unwrap(),
    };

    // Create the Swarm using our custom behavior
    let swarm = SwarmBuilder::new(my_behaviour, Transport::TCP)
        .build()
        .await?;

    Ok(swarm)
}
