#![deny(unsafe_code)]

pub mod enr;

// use clap::{App, Arg};
use discv5::*;
use discv5::{
    enr as discv5_enr, enr::CombinedKey, handler, kbucket, metrics, packet, permit_ban, rpc,
    service, socket, Discv5, Discv5Config, Discv5ConfigBuilder, Discv5Event, Enr, ListenConfig,
};
use futures::stream::FuturesUnordered;
use futures::Future;
use libp2p::PeerId;
use slog::{debug, error, Logger};
use std::pin::Pin;
use std::time::Duration;
use std::{collections::HashMap, time::Instant};
use tokio::sync::mpsc;
// use futures::Future;
// use libp2p::swarm::dummy::ConnectionHandler;
// use libp2p::swarm::NetworkBehaviour;
use std::error::Error;
use std::net::Ipv4Addr;

use self::enr::generate_enr;
// use std::num::NonZeroUsize;
// use std::pin::Pin;
// use void::Void;

enum EventStream {
    /// Awaiting an event stream to be generated. This is required due to the poll nature of
    /// `Discovery`
    Awaiting(
        Pin<
            Box<
                dyn Future<Output = Result<mpsc::Receiver<Discv5Event>, discv5::Discv5Error>>
                    + Send,
            >,
        >,
    ),
    /// The future has completed.
    Present(mpsc::Receiver<Discv5Event>),
    // The future has failed or discv5 has been disabled. There are no events from discv5.
    InActive,
}

#[derive(Debug)]
pub struct DiscoveredPeers {
    pub peers: HashMap<PeerId, Option<Instant>>,
}

pub struct Discovery {
    discv5: Discv5,
    event_stream: EventStream,
    log: Logger,
}

impl Discovery {
    pub async fn new(log: slog::Logger) -> Result<Self, Box<dyn Error>> {
        let (local_enr, enr, enr_key) = generate_enr(log.clone()).await?;

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

        let mut discv5: Discv5 = Discv5::new(enr.clone(), enr_key, discv5_config)?;

        let event_stream = {
            discv5.start().await.unwrap();
            debug!(log, "Discv5 started"; "enr" => ?enr);
            EventStream::Awaiting(Box::pin(discv5.event_stream()))
        };

        // !!! TODO: !!!
        //
        // Instead of using the `boot_nodes_multiaddr` from the config, we should use have another file with all the found ENRs, and use that instead.
        //
        // // get futures for requesting the Enrs associated to these multiaddr and wait for their
        // // completion
        // let mut fut_coll = config
        //     .boot_nodes_multiaddr
        //     .iter()
        //     .map(|addr| addr.to_string())
        //     // request the ENR for this multiaddr and keep the original for logging
        //     .map(|addr| {
        //         futures::future::join(
        //             discv5.request_enr(addr.clone()),
        //             futures::future::ready(addr),
        //         )
        //     })
        //     .collect::<FuturesUnordered<_>>();

        // while let Some((result, original_addr)) = fut_coll.next().await {
        //     match result {
        //         Ok(enr) => {
        //             debug!(
        //                 log,
        //                 "Adding node to routing table";
        //                 "node_id" => %enr.node_id(),
        //                 "peer_id" => %enr.peer_id(),
        //                 "ip" => ?enr.ip4(),
        //                 "udp" => ?enr.udp4(),
        //                 "tcp" => ?enr.tcp4(),
        //                 "quic" => ?enr.quic4()
        //             );
        //             let _ = discv5.add_enr(enr).map_err(|e| {
        //                 error!(
        //                     log,
        //                     "Could not add peer to the local routing table";
        //                     "addr" => original_addr.to_string(),
        //                     "error" => e.to_string(),
        //                 )
        //             });
        //         }
        //         Err(e) => {
        //             error!(log, "Error getting mapping to ENR"; "multiaddr" => original_addr.to_string(), "error" => e.to_string())
        //         }
        //     }
        // }

        Ok(Self {
            discv5,
            event_stream,
            log,
        })
    }

    pub fn get_enr(&self) -> Enr {
        self.discv5.local_enr().clone()
    }
}

// impl NetworkBehaviour for Discovery {
//     type ConnectionHandler = ConnectionHandler;
//     type OutEvent = Void;

//     fn new_handler(&mut self) -> Self::ConnectionHandler {
//         ConnectionHandler
//     }

//     fn addresses_of_peer(&mut self, _: &PeerId) -> Vec<libp2p::Multiaddr> {
//         Vec::new()
//     }

//     fn handle_established_inbound_connection(
//         &mut self,
//         connection_id: libp2p::swarm::ConnectionId,
//         peer: PeerId,
//         local_addr: &libp2p::Multiaddr,
//         remote_addr: &libp2p::Multiaddr,
//     ) -> Result<libp2p::swarm::THandler<Self>, libp2p::swarm::ConnectionDenied> {
//         Ok(ConnectionHandler)
//     }

//     fn handle_established_outbound_connection(
//         &mut self,
//         connection_id: libp2p::swarm::ConnectionId,
//         peer: PeerId,
//         addr: &libp2p::Multiaddr,
//         role_override: libp2p::core::Endpoint,
//     ) -> Result<libp2p::swarm::THandler<Self>, libp2p::swarm::ConnectionDenied> {
//         Ok(ConnectionHandler)
//     }

//     fn handle_pending_inbound_connection(
//         &mut self,
//         connection_id: libp2p::swarm::ConnectionId,
//         local_addr: &libp2p::Multiaddr,
//         remote_addr: &libp2p::Multiaddr,
//     ) -> Result<(), libp2p::swarm::ConnectionDenied> {
//         Ok(())
//     }

//     fn handle_pending_outbound_connection(
//         &mut self,
//         connection_id: libp2p::swarm::ConnectionId,
//         maybe_peer: Option<PeerId>,
//         addresses: &[libp2p::Multiaddr],
//         effective_role: libp2p::core::Endpoint,
//     ) -> Result<Vec<libp2p::Multiaddr>, libp2p::swarm::ConnectionDenied> {
//         Ok(Vec::new())
//     }

//     fn on_connection_handler_event(
//         &mut self,
//         peer_id: PeerId,
//         connection_id: libp2p::swarm::ConnectionId,
//         event: libp2p::swarm::THandlerOutEvent<Self>,
//     ) {
//     }

//     fn on_swarm_event(&mut self, event: libp2p::swarm::FromSwarm<Self::ConnectionHandler>) {}

//     fn poll(
//         &mut self,
//         cx: &mut std::task::Context<'_>,
//         params: &mut impl libp2p::swarm::PollParameters,
//     ) -> std::task::Poll<libp2p::swarm::ToSwarm<Self::OutEvent, libp2p::swarm::THandlerInEvent<Self>>>
//     {
//         std::task::Poll::Pending
//     }
// }
