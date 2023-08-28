#![deny(unsafe_code)]

pub mod events;

use crate::discv5::discovery::Discovery as CustomDiscovery;
use crate::discv5::discovery::enr::generate_enr;
use crate::libp2p::behaviour::gossip::Gossipsub as CustomGossipsub;
use crate::libp2p::behaviour::identify::Identity as CustomIdentity;
use crate::libp2p::behaviour::CustomBehavior as Behaviour;
use crate::libp2p::transport::transport::setup_transport;
use crate::libp2p::behaviour::CustomBehavior;

use discv5::Discv5ConfigBuilder;
use libp2p::{
    Swarm,
    futures::StreamExt,
    identity::{Keypair, PublicKey},
    swarm::{SwarmBuilder, SwarmEvent},
};
use std::error::Error;
use std::net::Ipv4Addr;
use std::time::Duration;
use tokio::runtime::Handle;

pub async fn setup_swarm(
    swarm_peer_id: libp2p::PeerId,
    transport_key: Keypair,
    log: slog::Logger,
) -> Result<Swarm<CustomBehavior>, Box<dyn Error>> {
    let transport = setup_transport(transport_key.clone()).await.unwrap();
    let log_for_gossip = log.clone();
    let log_for_identity = log.clone();

    let mut swarm = {
        let (lh_enr, enr, key) = generate_enr().await?;

        let listen_port = enr.udp4().unwrap();

        let discv5_listen_config =
            discv5::ListenConfig::from_ip(Ipv4Addr::UNSPECIFIED.into(), listen_port);

        let discv5_config = Discv5ConfigBuilder::new(discv5_listen_config)
            .ban_duration(Some(Duration::from_secs(60)))
            .query_timeout(Duration::from_secs(10))
            .request_retries(1)
            .request_timeout(Duration::from_secs(1))
            .query_parallelism(3)
            .query_peer_timeout(Duration::from_secs(3))
            .ping_interval(Duration::from_secs(300))
            .build();

        let identity_public_key = PublicKey::from(transport_key.public());

        let behaviour = Behaviour {
            gossipsub: CustomGossipsub::new(swarm_peer_id, transport_key, log_for_gossip),
            discovery: CustomDiscovery::new(enr, key, discv5_config).unwrap(),
            identify: CustomIdentity::new(identity_public_key, log_for_identity),
        };

        let executor = {
            let executor = Handle::current();
            move |fut: _| {
                executor.spawn(fut);
            }
        };

        // Build the Swarm
        SwarmBuilder::with_executor(transport, behaviour, swarm_peer_id, executor).build()
    };

    // Listen on all interfaces and the port we desire,
    // could listen on port 0 to listen on whatever port the OS assigns us.
    let listen_addr = format!("/ip4/0.0.0.0/tcp/8888/p2p/{}", swarm_peer_id.to_string());
    slog::debug!(log, "Listening on"; "listen_addr" => ?listen_addr);
    swarm.listen_on(listen_addr.parse().unwrap()).unwrap();

    slog::debug!(log, "Swarm Info"; "network_info" => ?swarm.network_info());

    Ok(swarm)
}
