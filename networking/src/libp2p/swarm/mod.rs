#![deny(unsafe_code)]

pub mod events;

use super::swarm::events::swarm_events;
use crate::discv5::discovery::enr::generate_enr;
use crate::discv5::discovery::Discovery as CustomDiscovery;
use crate::libp2p::behaviour::gossip::Gossipsub as CustomGossipsub;
use crate::libp2p::behaviour::identify::Identity as CustomIdentity;
use crate::libp2p::behaviour::CustomBehavior as Behaviour;
use crate::libp2p::behaviour::CustomBehavior;
use crate::libp2p::transport::setup_transport;

use discv5::Discv5ConfigBuilder;
use libp2p::{
    futures::StreamExt,
    identity::{Keypair, PublicKey},
    swarm::{NetworkBehaviour, SwarmBuilder, SwarmEvent},
    Swarm,
};
use std::net::Ipv4Addr;
use std::time::Duration;
use std::{error::Error, sync::Arc};
use tokio::{runtime::Handle, sync::Mutex};

pub async fn setup_swarm(
    swarm_peer_id: libp2p::PeerId,
    transport_key: Keypair,
    log: slog::Logger,
) -> Result<Arc<Mutex<Swarm<CustomBehavior>>>, Box<dyn Error>> {
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
            discovery: CustomDiscovery::new(enr, key, discv5_config).await.unwrap(),
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

    let swarm = Arc::new(Mutex::new(swarm));

    let log_clone = log.clone();
    let swarm_clone = swarm.clone();
    tokio::spawn(async move {
        slog::info!(log_clone, "Starting swarm events");
        let mut locked_swarm = swarm_clone.lock().await;
        swarm_events(&mut *locked_swarm, log_clone).await;
    });

    slog::error!(log, "test error"; "swarm" => ?swarm.lock().await.network_info());

    Ok(swarm)
}
