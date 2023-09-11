#![deny(unsafe_code)]

pub mod events;

use super::swarm::events::swarm_events;
use crate::discv5::discovery::enr::generate_enr;
use crate::discv5::discovery::Discovery;
use crate::libp2p::behaviour::gossip::Gossipsub;
use crate::libp2p::behaviour::identify::Identity;
use crate::libp2p::behaviour::CustomBehavior as Behaviour;
use crate::libp2p::behaviour::CustomBehavior;
use crate::libp2p::transport::setup_transport;

use discv5::Discv5ConfigBuilder;
use get_if_addrs::get_if_addrs;
use libp2p::{
    futures::StreamExt,
    identity::{Keypair, PublicKey},
    swarm::{NetworkBehaviour, SwarmBuilder, SwarmEvent},
    PeerId, Swarm,
};
use slog::Logger;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::time::Duration;
use std::{error::Error, sync::Arc};
use tokio::{runtime::Handle, sync::Mutex};

pub struct CustomSwarm {
    pub swarm: Arc<Mutex<Swarm<CustomBehavior>>>,
    pub discv5: Discovery,
    pub log: Logger,
}

type BoxedTransport =
    libp2p::core::transport::Boxed<(libp2p::PeerId, libp2p::core::muxing::StreamMuxerBox)>;

impl CustomSwarm {
    pub async fn new(
        swarm_peer_id: libp2p::PeerId,
        transport_key: Keypair,
        discovery: Discovery,
        transport: BoxedTransport,
        log: Logger,
    ) -> Result<Self, Box<dyn Error>> {
        let log_clone = log.clone();
        let transport = setup_transport(transport_key.clone()).await.unwrap();

        let mut swarm = {
            let enr = discovery.get_enr();

            let listen_port = enr.udp4().unwrap();

            let has_ipv4 = get_if_addrs()?.iter().any(|iface| match iface.addr.ip() {
                IpAddr::V4(_) => true,
                IpAddr::V6(_) => false,
            });

            let has_ipv6 = get_if_addrs()?.iter().any(|iface| match iface.addr.ip() {
                IpAddr::V4(_) => false,
                IpAddr::V6(_) => true,
            });

            let discv5_listen_config = if has_ipv4 {
                discv5::ListenConfig::from_ip(IpAddr::V4(Ipv4Addr::UNSPECIFIED), listen_port)
            } else if has_ipv6 {
                discv5::ListenConfig::from_ip(IpAddr::V6(Ipv6Addr::UNSPECIFIED), listen_port)
            } else {
                slog::error!(log, "No valid IP addresses found");
                return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "")));
            };

            let identity_public_key = PublicKey::from(transport_key.public());

            let behaviour = Behaviour {
                gossipsub: Gossipsub::new(swarm_peer_id, transport_key, log.clone()),
                discovery: Discovery::new(log.clone()).await.unwrap(),
                identify: Identity::new(identity_public_key, log.clone()),
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

        let listen_addr = format!("/ip4/0.0.0.0/tcp/8888/p2p/{}", swarm_peer_id.to_string());
        swarm.listen_on(listen_addr.parse().unwrap()).unwrap();

        let swarm = Arc::new(Mutex::new(swarm));

        Ok(CustomSwarm {
            swarm,
            discv5: discovery,
            log,
        })
    }

    pub async fn default() -> Result<Self, Box<dyn Error>> {
        let log = Logger::root(slog::Discard, slog::o!());
        let local_transport_key: Keypair = Keypair::generate_secp256k1();
        let local_swarm_peer_id: PeerId = PeerId::from(local_transport_key.public());

        let discv5 = Discovery::new(log.clone()).await.unwrap();
        let transport = setup_transport(local_transport_key.clone()).await.unwrap();
        let swarm = CustomSwarm::new(
            local_swarm_peer_id,
            local_transport_key,
            discv5,
            transport,
            log.clone(),
        )
        .await
        .unwrap();

        Ok(swarm)
    }
}

// pub async fn swarm_setup(
//     swarm_peer_id: libp2p::PeerId,
//     transport_key: Keypair,
//     log: Logger,
// ) -> Result<Arc<Mutex<Swarm<CustomBehavior>>>, Box<dyn Error>> {
//     let log_clone = log.clone();
//     let transport = setup_transport(transport_key.clone()).await.unwrap();
//     // let log_for_gossip = log.clone();
//     // let log_for_identity = log.clone();
//     // let log_for_discv5 = log.clone();

//     let mut swarm = {
//         let (lh_enr, enr, key) = generate_enr(log_clone).await?;

//         let listen_port = enr.udp4().unwrap();

//         let has_ipv4 = get_if_addrs()?.iter().any(|iface| match iface.addr.ip() {
//             IpAddr::V4(_) => true,
//             IpAddr::V6(_) => false,
//         });

//         let has_ipv6 = get_if_addrs()?.iter().any(|iface| match iface.addr.ip() {
//             IpAddr::V4(_) => false,
//             IpAddr::V6(_) => true,
//         });

//         let discv5_listen_config = if has_ipv4 {
//             discv5::ListenConfig::from_ip(IpAddr::V4(Ipv4Addr::UNSPECIFIED), listen_port)
//         } else if has_ipv6 {
//             discv5::ListenConfig::from_ip(IpAddr::V6(Ipv6Addr::UNSPECIFIED), listen_port)
//         } else {
//             return Err(Box::new(std::io::Error::new(
//                 std::io::ErrorKind::Other,
//                 "No valid IP addresses found",
//             )));
//         };

//         let identity_public_key = PublicKey::from(transport_key.public());

//         let behaviour = Behaviour {
//             gossipsub: CustomGossipsub::new(swarm_peer_id, transport_key, log.clone()),
//             discovery: CustomDiscovery::new(log.clone()).await.unwrap(),
//             identify: CustomIdentity::new(identity_public_key, log.clone()),
//         };

//         let executor = {
//             let executor = Handle::current();
//             move |fut: _| {
//                 executor.spawn(fut);
//             }
//         };

//         // Build the Swarm
//         SwarmBuilder::with_executor(transport, behaviour, swarm_peer_id, executor).build()
//     };

//     let listen_addr = format!("/ip4/0.0.0.0/tcp/8888/p2p/{}", swarm_peer_id.to_string());
//     swarm.listen_on(listen_addr.parse().unwrap()).unwrap();

//     let swarm = Arc::new(Mutex::new(swarm));

//     Ok(swarm)
// }
