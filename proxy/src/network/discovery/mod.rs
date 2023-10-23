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
use futures::Future;
use libp2p::swarm::dummy::ConnectionHandler;
use libp2p::swarm::NetworkBehaviour;
use libp2p::PeerId;
use serde_derive::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use slog::{debug, error, Logger};
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
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

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscoveredPeers {
    pub peers: HashMap<PeerId, Enr<CombinedKey>>,
}

impl DiscoveredPeers {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        let file_contents = fs::read_to_string(PATH)?;
        Ok(from_str(&file_contents)?)
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let path = Path::new(PATH);
        let json = serde_json::to_string_pretty(self)?;

        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(path)?;

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

    pub async fn discover_peers(&mut self) {
        // Load known peers from file
        let known_peers = DiscoveredPeers::load().expect("Failed to load peers.json");

        for (_, enr) in &known_peers.peers {
            // Assume request_peers is an async function to request a list of peers from a known peer
            match self.discv5.request_peers(enr.clone()).await {
                Ok(new_peers) => {
                    for new_peer in new_peers {
                        // Add new peer to your local routing table
                        self.add_enr(new_peer.clone());

                        // Update the json file with the new peer
                        self.seen_peers
                            .peers
                            .insert(new_peer.peer_id().to_base58(), new_peer.clone());
                    }
                }
                Err(e) => {
                    error!(
                        self.log,
                        "Error requesting peers from known peer";
                        "error" => %e
                    );
                }
            }
        }

        // Save updated peer list to file
        self.seen_peers.save().expect("Failed to save peers.json");
    }

    /// Add an ENR to the routing table of the discovery mechanism.
    pub fn add_enr(&mut self, enr: Enr<CombinedKey>) {
        // add the enr to seen caches

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

    fn on_swarm_event(&mut self, event: libp2p::swarm::FromSwarm<Self::ConnectionHandler>) {}

    fn poll(
        &mut self,
        cx: &mut std::task::Context<'_>,
        params: &mut impl libp2p::swarm::PollParameters,
    ) -> std::task::Poll<libp2p::swarm::ToSwarm<Self::ToSwarm, libp2p::swarm::THandlerInEvent<Self>>>
    {
        std::task::Poll::Pending
    }
}
