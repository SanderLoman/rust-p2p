#![deny(unsafe_code)]

use crate::discv5::discovery::Discovery as CustomDiscovery;
use crate::discv5::discovery::enr::generate_enr;
use crate::libp2p::behaviour::gossip::Gossipsub as CustomGossipsub;
use crate::libp2p::behaviour::identify::Identity as CustomIdentity;
use crate::libp2p::behaviour::CustomBehavior as Behaviour;
use crate::libp2p::transport::transport::setup_transport;

use discv5::Discv5ConfigBuilder;
use libp2p::{
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
) -> Result<(), Box<dyn Error>> {
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

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::OutgoingConnectionError { peer_id, error } => {
                slog::debug!(log, "Outgoing Connection Error"; "peer_id" => ?peer_id, "error" => ?error);
            }
            #[allow(deprecated)]
            SwarmEvent::BannedPeer { peer_id, endpoint } => {
                slog::debug!(log, "Banned Peer"; "peer_id" => %peer_id, "endpoint" => ?endpoint);
            }
            SwarmEvent::ListenerError { listener_id, error } => {
                slog::debug!(log, "Listener Error"; "listener_id" => ?listener_id, "error" => ?error);
            }
            SwarmEvent::Dialing(peer_id) => {
                slog::debug!(log, "Dialing"; "peer_id" => %peer_id);
            }
            SwarmEvent::Behaviour(event) => {
                slog::debug!(log, "Behaviour Event"; "event" => ?event);
            }
            SwarmEvent::ConnectionEstablished {
                peer_id,
                endpoint,
                num_established,
                concurrent_dial_errors,
                established_in,
            } => {
                slog::debug!(log, "Connection established"; "peer_id" => %peer_id, "endpoint" => ?endpoint, "concurrent_dial_errors" => ?concurrent_dial_errors, "established_in" => ?established_in);
                slog::debug!(log, "Peers connected"; "num_established" => ?num_established);
            }
            SwarmEvent::ConnectionClosed {
                peer_id,
                endpoint,
                num_established,
                cause,
            } => {
                slog::debug!(log, "Connection closed"; "peer_id" => %peer_id, "endpoint" => ?endpoint, "cause" => ?cause);
                slog::debug!(log, "Peers connected"; "num_established" => num_established);
            }
            SwarmEvent::NewListenAddr {
                address,
                listener_id,
            } => {
                slog::debug!(log, "New Listen Address"; "address" => ?address, "listener_id" => ?listener_id);
            }
            SwarmEvent::ExpiredListenAddr {
                address,
                listener_id,
            } => {
                slog::debug!(log, "Expired Listen Address"; "address" => ?address, "listener_id" => ?listener_id);
            }
            SwarmEvent::ListenerClosed {
                addresses,
                reason,
                listener_id,
            } => {
                slog::debug!(log, "Listener closed"; "reason" => ?reason, "addresses" => ?addresses, "listener_id" => ?listener_id);
            }
            SwarmEvent::IncomingConnection {
                local_addr,
                send_back_addr,
            } => {
                slog::debug!(log, "Incoming connection"; "local_addr" => ?local_addr, "send_back_addr" => ?send_back_addr);
            }
            SwarmEvent::IncomingConnectionError {
                error,
                local_addr,
                send_back_addr,
            } => {
                slog::debug!(log, "Incoming connection error"; "error" => ?error, "local_addr" => ?local_addr, "send_back_addr" => ?send_back_addr);
            }
        }
    }
}
