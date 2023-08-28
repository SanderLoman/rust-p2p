#![deny(unsafe_code)]

use crate::create_logger;
use crate::discv5::enr::*;
use discv5::*;
use discv5::{
    enr, enr::CombinedKey, handler, kbucket, metrics, packet, permit_ban, rpc, service, socket,
    Discv5, Discv5Config, Discv5ConfigBuilder, Discv5Event, Enr, ListenConfig,
};
use futures::Future;
use libp2p::swarm::dummy::ConnectionHandler;
use libp2p::swarm::NetworkBehaviour;
use libp2p::PeerId;
use lru::LruCache;
use std::error::Error;
use std::net::Ipv4Addr;
use std::num::NonZeroUsize;
use std::pin::Pin;
use std::time::Duration;
use void::Void;

// https://github.com/sigp/lighthouse/blob/stable/beacon_node/lighthouse_network/src/discovery/mod.rs#L191C27-L191C27
pub struct Discovery {
    cached_enrs: LruCache<PeerId, Enr>,
    discv5: Discv5,
    // event_stream: ,
}

impl Discovery {
    pub fn new(
        enr: Enr,
        enr_key: CombinedKey,
        config: Discv5Config,
    ) -> Result<Self, Box<dyn Error>> {
        let discv5 = Discv5::new(enr, enr_key, config)?;
        let cached_enrs = LruCache::new(NonZeroUsize::new(1000).unwrap());
        Ok(Discovery {
            cached_enrs,
            discv5,
        })
    }
}

impl NetworkBehaviour for Discovery {
    type ConnectionHandler = ConnectionHandler;
    type OutEvent = Void;

    fn new_handler(&mut self) -> Self::ConnectionHandler {
        ConnectionHandler
    }

    fn addresses_of_peer(&mut self, _: &PeerId) -> Vec<libp2p::Multiaddr> {
        Vec::new()
    }

    fn handle_established_inbound_connection(
        &mut self,
        _connection_id: libp2p::swarm::ConnectionId,
        peer: PeerId,
        local_addr: &libp2p::Multiaddr,
        remote_addr: &libp2p::Multiaddr,
    ) -> Result<libp2p::swarm::THandler<Self>, libp2p::swarm::ConnectionDenied> {
        Ok(libp2p::swarm::dummy::ConnectionHandler)
    }

    fn handle_established_outbound_connection(
        &mut self,
        _connection_id: libp2p::swarm::ConnectionId,
        peer: PeerId,
        addr: &libp2p::Multiaddr,
        role_override: libp2p::core::Endpoint,
    ) -> Result<libp2p::swarm::THandler<Self>, libp2p::swarm::ConnectionDenied> {
        Ok(libp2p::swarm::dummy::ConnectionHandler)
    }

    fn handle_pending_inbound_connection(
        &mut self,
        _connection_id: libp2p::swarm::ConnectionId,
        _local_addr: &libp2p::Multiaddr,
        _remote_addr: &libp2p::Multiaddr,
    ) -> Result<(), libp2p::swarm::ConnectionDenied> {
        Ok(())
    }

    fn handle_pending_outbound_connection(
        &mut self,
        _connection_id: libp2p::swarm::ConnectionId,
        maybe_peer: Option<PeerId>,
        _addresses: &[libp2p::Multiaddr],
        _effective_role: libp2p::core::Endpoint,
    ) -> Result<Vec<libp2p::Multiaddr>, libp2p::swarm::ConnectionDenied> {
        Ok(Vec::new())
    }

    fn on_connection_handler_event(
        &mut self,
        _peer_id: PeerId,
        _connection_id: libp2p::swarm::ConnectionId,
        _event: libp2p::swarm::THandlerOutEvent<Self>,
    ) {
    }

    fn on_swarm_event(&mut self, event: libp2p::swarm::FromSwarm<Self::ConnectionHandler>) {}

    fn poll(
        &mut self,
        cx: &mut std::task::Context<'_>,
        params: &mut impl libp2p::swarm::PollParameters,
    ) -> std::task::Poll<libp2p::swarm::ToSwarm<Self::OutEvent, libp2p::swarm::THandlerInEvent<Self>>>
    {
        std::task::Poll::Pending
    }
}

// pub async fn start_discv5() -> Result<Discv5, Box<dyn Error>> {
//     let log = create_logger();
//     let (local_enr, enr, enr_key) = generate_enr().await?;

//     let listen_port = enr.udp4().unwrap();

//     let discv5_listen_config =
//         discv5::ListenConfig::from_ip(Ipv4Addr::UNSPECIFIED.into(), listen_port);
//     slog::debug!(log, "discv5_listen_config"; "config" => ?discv5_listen_config);

//     let discv5_config = Discv5ConfigBuilder::new(discv5_listen_config)
//         .ban_duration(Some(Duration::from_secs(60)))
//         .query_timeout(Duration::from_secs(10))
//         .request_retries(1)
//         .request_timeout(Duration::from_secs(1))
//         .query_parallelism(3)
//         .query_peer_timeout(Duration::from_secs(3))
//         .ping_interval(Duration::from_secs(300))
//         .build();

//     slog::debug!(log, "discv5_config"; "config" => ?discv5_config);

//     let cloned_enr = enr.clone();
//     let mut discv5: Discv5 = Discv5::new(enr, enr_key, discv5_config).unwrap();

//     let cloned_local_enr = local_enr.clone();
//     discv5.start().await.unwrap();

//     let mut discv_events = discv5.event_stream().await.unwrap();

//     loop {
//         match discv_events.recv().await {
//             Some(Discv5Event::SocketUpdated(socket_addr)) => {
//                 slog::debug!(log, "Socket Updated"; "socket_addr" => ?socket_addr);
//             }
//             Some(Discv5Event::Discovered(enr)) => {
//                 slog::debug!(log, "Discovered"; "enr" => ?enr);
//             }
//             Some(Discv5Event::NodeInserted { node_id, replaced }) => {
//                 slog::debug!(log, "Node Inserted"; "node_id" => %node_id, "replaced" => ?replaced);
//             }
//             Some(Discv5Event::EnrAdded { enr, replaced }) => {
//                 slog::debug!(log, "Enr Added"; "enr" => ?enr, "replaced" => ?replaced);
//             }
//             Some(Discv5Event::SessionEstablished(enr, socket_addr)) => {
//                 slog::debug!(log, "Session Established"; "enr" => ?enr, "socket_addr" => ?socket_addr);
//             }
//             Some(Discv5Event::TalkRequest(_)) => {
//                 slog::debug!(log, "Talk Request Received");
//             }
//             None => {
//                 slog::debug!(log, "No events");
//             }
//         }
//     }
// }
