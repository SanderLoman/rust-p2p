#![deny(unsafe_code)]

use libp2p::gossipsub::Behaviour;
use libp2p::gossipsub::*;
use libp2p::identity::Keypair;
use libp2p::PeerId;
use slog::Logger;
use std::time::Duration;

pub struct Gossipsub {
    /// The underlying libp2p gossipsub behaviour.
    pub gossip_behaviour: Behaviour,
}

impl Gossipsub {
    /// Creates a new gossipsub behaviour.
    pub fn new(local_peer_id: PeerId, keypair: Keypair, log: Logger) -> Self {
        let gossip_config = ConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10))
            .build()
            .unwrap();
        let gossip_behaviour =
            libp2p::gossipsub::Behaviour::new(MessageAuthenticity::Signed(keypair), gossip_config)
                .unwrap();

        Gossipsub { gossip_behaviour }
    }
}
