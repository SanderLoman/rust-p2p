#![deny(unsafe_code)]

pub mod enr;

// use clap::{App, Arg};
use discv5::enr::Enr;
use discv5::*;
use discv5::{
    enr as discv5_enr, enr::CombinedKey, handler, kbucket, metrics, packet, permit_ban, rpc,
    service, socket, Discv5, Discv5Config, Discv5ConfigBuilder, Discv5Event, ListenConfig,
};
use futures::stream::FuturesUnordered;
use futures::{Future, FutureExt};
use libp2p::swarm::dummy::ConnectionHandler;
use libp2p::swarm::{DialError, DialFailure, FromSwarm, NetworkBehaviour};
use libp2p::{Multiaddr, PeerId};
use serde_derive::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use slog::{debug, error, info, Logger};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use std::pin::Pin;
use std::task::Poll;
use std::time::Duration;
use std::{collections::HashMap, time::Instant};
use tokio::sync::mpsc;
// use futures::Future;
// use libp2p::swarm::dummy::ConnectionHandler;
// use libp2p::swarm::NetworkBehaviour;
use std::error::Error;
use std::net::Ipv4Addr;

use self::enr::generate_enr;

const PATH: &str = "/home/sander/rust-p2p/json/peers.json";

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

// Define a custom struct to hold the peer information
#[derive(Debug, Serialize, Deserialize)]
pub struct PeerInfo {
    pub enr: discv5::Enr,
    pub multiaddr: Multiaddr,
}

// Modify DiscoveredPeers to use the custom struct
#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoveredPeers {
    pub peers: HashMap<PeerId, PeerInfo>,
}

impl DiscoveredPeers {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let file_contents = fs::read_to_string(PATH)?;
        Ok(from_str(&file_contents)?)
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let path = Path::new(PATH);
        let json = serde_json::to_string_pretty(self)?;

        fs::write(path, json)?;

        Ok(())
    }
}

pub struct Discovery {
    discv5: Discv5,
    seen_peers: DiscoveredPeers,
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

        let seen_peers = {
            let mut file = File::open("json/peers.json").expect("Failed to open json/peers.json");
            let mut contents = String::new();
            file.read_to_string(&mut contents)
                .expect("Failed to read json/peers.json");
            let seen_peers: DiscoveredPeers =
                from_str(&contents).expect("Failed to deserialize peers from json/peers.json");

            seen_peers
        };

        Ok(Self {
            discv5,
            seen_peers,
            event_stream,
            log,
        })
    }

    /// Searches for peers on the network.
    pub async fn discover_peers(&mut self) {
        // Load known peers from file
        let known_peers = DiscoveredPeers::load().expect("Failed to load peers.json");

        for (peer_id, enr) in &known_peers.peers {}

        // Save updated peer list to file
        self.seen_peers.save().expect("Failed to save peers.json");
    }

    /// Add an ENR to the routing table of the discovery mechanism.
    pub fn add_enr(&mut self, enr: Enr<CombinedKey>) {
        if let Err(e) = self.discv5.add_enr(enr) {
            debug!(
                self.log,
                "Could not add peer to the local routing table";
                "error" => %e
            )
        }
    }

    /// Returns an iterator over all enr entries in the DHT.
    pub fn table_entries_enr(&self) -> Vec<Enr<CombinedKey>> {
        self.discv5.table_entries_enr()
    }

    /// Returns the local ENR.
    pub fn local_enr(&self) -> Enr<CombinedKey> {
        self.discv5.local_enr().clone()
    }
}

impl NetworkBehaviour for Discovery {
    type ConnectionHandler = ConnectionHandler;
    type ToSwarm = DiscoveredPeers;

    fn handle_established_inbound_connection(
        &mut self,
        _connection_id: libp2p::swarm::ConnectionId,
        peer: PeerId,
        local_addr: &libp2p::Multiaddr,
        remote_addr: &libp2p::Multiaddr,
    ) -> Result<libp2p::swarm::THandler<Self>, libp2p::swarm::ConnectionDenied> {
        Ok(ConnectionHandler)
    }

    fn handle_established_outbound_connection(
        &mut self,
        _connection_id: libp2p::swarm::ConnectionId,
        peer: PeerId,
        addr: &libp2p::Multiaddr,
        role_override: libp2p::core::Endpoint,
    ) -> Result<libp2p::swarm::THandler<Self>, libp2p::swarm::ConnectionDenied> {
        Ok(ConnectionHandler)
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
        _maybe_peer: Option<PeerId>,
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

    fn on_swarm_event(&mut self, event: FromSwarm<Self::ConnectionHandler>) {
        match event {
            FromSwarm::DialFailure(DialFailure { peer_id, error, .. }) => {
                self.on_dial_failure(peer_id, error)
            }
            FromSwarm::ConnectionEstablished(_)
            | FromSwarm::ConnectionClosed(_)
            | FromSwarm::AddressChange(_)
            | FromSwarm::ListenFailure(_)
            | FromSwarm::NewListener(_)
            | FromSwarm::NewListenAddr(_)
            | FromSwarm::ExpiredListenAddr(_)
            | FromSwarm::ListenerError(_)
            | FromSwarm::ListenerClosed(_)
            | FromSwarm::NewExternalAddrCandidate(_)
            | FromSwarm::ExternalAddrExpired(_)
            | FromSwarm::ExternalAddrConfirmed(_) => {
                // Ignore events not relevant to discovery
            }
        }
    }

    fn poll(
        &mut self,
        cx: &mut std::task::Context<'_>,
        params: &mut impl libp2p::swarm::PollParameters,
    ) -> std::task::Poll<libp2p::swarm::ToSwarm<Self::ToSwarm, libp2p::swarm::THandlerInEvent<Self>>>
    {
        match self.event_stream {
            EventStream::Awaiting(ref mut fut) => {
                // Still awaiting the event stream, poll it
                if let Poll::Ready(event_stream) = fut.poll_unpin(cx) {
                    match event_stream {
                        Ok(stream) => {
                            debug!(self.log, "Discv5 event stream ready");
                            self.event_stream = EventStream::Present(stream);
                        }
                        Err(e) => {
                            slog::crit!(self.log, "Discv5 event stream failed"; "error" => %e);
                            self.event_stream = EventStream::InActive;
                        }
                    }
                }
            }
            EventStream::InActive => {
                // *Conner McGregor Voice* - "You'll do nuttin"
            }
            EventStream::Present(ref mut stream) => {
                while let Poll::Ready(Some(event)) = stream.poll_recv(cx) {
                    match event {
                        // We filter out unwanted discv5 events here and only propagate useful results to
                        // the peer manager.
                        Discv5Event::Discovered(_enr) => {
                            // Peers that get discovered during a query but are not contactable or
                            // don't match a predicate can end up here. For debugging purposes we
                            // log these to see if we are unnecessarily dropping discovered peers
                            /*
                            if enr.eth2() == self.local_enr().eth2() {
                                trace!(self.log, "Peer found in process of query"; "peer_id" => format!("{}", enr.peer_id()), "tcp_socket" => enr.tcp_socket());
                            } else {
                            // this is temporary warning for debugging the DHT
                            warn!(self.log, "Found peer during discovery not on correct fork"; "peer_id" => format!("{}", enr.peer_id()), "tcp_socket" => enr.tcp_socket());
                            }
                            */
                        }
                        Discv5Event::SocketUpdated(socket_addr) => {
                            info!(self.log, "Address updated"; "ip" => %socket_addr.ip(), "udp_port" => %socket_addr.port());
                        }
                        Discv5Event::EnrAdded { replaced, enr } => {
                            // Request the newly added node for its known peers
                            // Assuming `request_peers` is an async function that requests a list of peers from a known node

                            // !!! Make a fix for this we cant use async here
                            //
                            // match self.discv5.request_peers(enr.node_id()) {
                            //     Ok(new_peers) => {
                            //         for new_peer in new_peers {
                            //             // Add each new peer to the local routing table
                            //             self.add_enr(new_peer);
                            //         }
                            //     }
                            //     Err(e) => {
                            //         error!(
                            //             self.log,
                            //             "Error requesting peers from newly added node";
                            //             "error" => %e
                            //         );
                            //     }
                            // }
                        }

                        Discv5Event::TalkRequest(_)
                        | Discv5Event::NodeInserted { .. }
                        | Discv5Event::SessionEstablished { .. } => {} // Ignore all other discv5 server events
                    }
                }
            }
        }
        Poll::Pending
    }
}

// !!! Maybe fix the WrongPeerId later
impl Discovery {
    fn on_dial_failure(&mut self, peer_id: Option<PeerId>, error: &DialError) {
        if let Some(peer_id) = peer_id {
            match error {
                DialError::LocalPeerId { .. }
                | DialError::Denied { .. }
                | DialError::NoAddresses
                | DialError::Transport(_)
                | DialError::WrongPeerId { .. } => {
                    // set peer as disconnected in discovery DHT
                    debug!(self.log, "Dial failure"; "peer_id" => format!("{}", peer_id));
                }
                DialError::DialPeerConditionFalse(_) | DialError::Aborted => {}
            }
        }
    }
}
