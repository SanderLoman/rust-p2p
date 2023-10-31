#![deny(unsafe_code)]

pub mod enr;
pub mod enr_ext;

// use clap::{App, Arg};
use discv5::enr::Enr;
use discv5::*;
use discv5::{
    enr as discv5_enr, enr::CombinedKey, handler, kbucket, metrics, packet, permit_ban, rpc,
    service, socket, Discv5, Discv5Config, Discv5ConfigBuilder, Discv5Event, ListenConfig,
};
use futures::stream::FuturesUnordered;
use futures::{Future, FutureExt};
use libp2p::identity::Keypair;
use libp2p::swarm::dummy::ConnectionHandler;
use libp2p::swarm::{DialError, DialFailure, FromSwarm, NetworkBehaviour};
use libp2p::{Multiaddr, PeerId};
use serde_derive::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use slog::{debug, error, info, Logger};
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::net::Ipv4Addr;
use std::path::Path;
use std::pin::Pin;
use std::task::Poll;
use std::time::Duration;
use std::{collections::HashMap, time::Instant};
use tokio::sync::mpsc;

use self::enr::build_enr;

use super::config::Config as NetworkConfig;
use super::types::network_globals::{CombinedKeyExt, NetworkGlobals};

const PATH: &str = "/home/sander/rust-p2p/json/peers.json";

pub enum EventStream {
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
        let path = Path::new(PATH);
        if !path.exists() {
            // The file does not exist, so create it with an empty DiscoveredPeers object
            return Self::initialize_file(&path);
        }

        // Attempt to read and deserialize the file content
        let file_contents = fs::read_to_string(&path)?;
        match serde_json::from_str::<Self>(&file_contents) {
            Ok(data) => Ok(data),
            Err(_) => {
                // Deserialization failed, so overwrite the file with an empty DiscoveredPeers object
                Self::initialize_file(&path)
            }
        }
    }

    fn initialize_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        let discovered_peers = Self {
            peers: HashMap::new(),
        };
        let json = serde_json::to_string_pretty(&discovered_peers)?;
        fs::write(path, json)?;
        Ok(discovered_peers)
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(PATH, json)?;
        Ok(())
    }
}

pub struct Discovery {
    discv5: Discv5,
    seen_peers: DiscoveredPeers,
    network_globals: NetworkGlobals,
    event_stream: EventStream,
    log: Logger,
}

impl Discovery {
    pub async fn new(
        local_key: Keypair,
        config: &NetworkConfig,
        log: slog::Logger,
    ) -> Result<Self, Box<dyn Error>> {
        Ok(todo!())
    }
}

impl NetworkBehaviour for Discovery {
    type ConnectionHandler = ConnectionHandler;
    type ToSwarm = DiscoveredPeers;

    fn handle_established_inbound_connection(
        &mut self,
        connection_id: libp2p::swarm::ConnectionId,
        peer: PeerId,
        local_addr: &libp2p::Multiaddr,
        remote_addr: &libp2p::Multiaddr,
    ) -> Result<libp2p::swarm::THandler<Self>, libp2p::swarm::ConnectionDenied> {
        Ok(ConnectionHandler)
    }

    fn handle_established_outbound_connection(
        &mut self,
        connection_id: libp2p::swarm::ConnectionId,
        peer: PeerId,
        addr: &libp2p::Multiaddr,
        role_override: libp2p::core::Endpoint,
    ) -> Result<libp2p::swarm::THandler<Self>, libp2p::swarm::ConnectionDenied> {
        Ok(ConnectionHandler)
    }

    fn handle_pending_inbound_connection(
        &mut self,
        connection_id: libp2p::swarm::ConnectionId,
        _local_addr: &libp2p::Multiaddr,
        remote_addr: &libp2p::Multiaddr,
    ) -> Result<(), libp2p::swarm::ConnectionDenied> {
        Ok(())
    }

    fn handle_pending_outbound_connection(
        &mut self,
        connection_id: libp2p::swarm::ConnectionId,
        maybe_peer: Option<PeerId>,
        addresses: &[libp2p::Multiaddr],
        effective_role: libp2p::core::Endpoint,
    ) -> Result<Vec<libp2p::Multiaddr>, libp2p::swarm::ConnectionDenied> {
        Ok(Vec::new())
    }

    fn on_connection_handler_event(
        &mut self,
        peer_id: PeerId,
        connection_id: libp2p::swarm::ConnectionId,
        event: libp2p::swarm::THandlerOutEvent<Self>,
    ) {
    }

    fn on_swarm_event(&mut self, event: FromSwarm<Self::ConnectionHandler>) {
        match event {
            FromSwarm::DialFailure(DialFailure { peer_id, error, .. }) => {
                self.on_dial_failure(peer_id, error)
            }
            FromSwarm::ConnectionEstablished(_) => {}
            FromSwarm::ConnectionClosed(_) => {}
            FromSwarm::AddressChange(_) => {}
            FromSwarm::ListenFailure(_) => {} 
            FromSwarm::NewListener(_) => {} 
            FromSwarm::NewListenAddr(_) => {} 
            FromSwarm::ExpiredListenAddr(_) => {}
            FromSwarm::ListenerError(_) => {}
            FromSwarm::ListenerClosed(_) => {}
            FromSwarm::NewExternalAddrCandidate(_) => {}
            FromSwarm::ExternalAddrExpired(_) => {}
            FromSwarm::ExternalAddrConfirmed(_) => {
                // Ignore events not relevant to discovery
            }
        }
    }

    fn poll(
        &mut self,
        cx: &mut std::task::Context<'_>,
        _: &mut impl libp2p::swarm::PollParameters,
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
                        Discv5Event::Discovered(_enr) => {}
                        Discv5Event::SocketUpdated(socket_addr) => {}
                        Discv5Event::EnrAdded { replaced, enr } => {}
                        Discv5Event::TalkRequest(_) => {}
                        Discv5Event::NodeInserted { node_id, replaced } => {}
                        Discv5Event::SessionEstablished { .. } => {} // Ignore all other discv5 server events
                    }
                }
            }
        }
        Poll::Pending
    }
}

impl Discovery {
    fn on_dial_failure(&mut self, peer_id: Option<PeerId>, error: &DialError) {
        if let Some(peer_id) = peer_id {
            match error {
                DialError::LocalPeerId { .. } => {}
                DialError::Denied { .. } => {}
                DialError::NoAddresses => {}
                DialError::Transport(_) => {}
                DialError::WrongPeerId { .. } => {}
                DialError::DialPeerConditionFalse(_) | DialError::Aborted => {}
            }
        }
    }
}
