use chrono::{DateTime, Local, TimeZone, Utc};
use colored::*;
use discv5::{
    enr,
    enr::{CombinedKey, NodeId},
    Discv5, Discv5ConfigBuilder, TokioExecutor,
};
use ethers::prelude::*;
use eyre::Result;
use futures::stream::{self, StreamExt};
use libp2p::kad::kbucket::{Entry, EntryRefView};
use libp2p::{
    core::upgrade,
    dns::DnsConfig,
    identity,
    kad::{record::store::MemoryStore, Kademlia, KademliaConfig, KademliaEvent, QueryResult},
    mdns::*,
    noise::{AuthenticKeypair, Keypair, NoiseConfig, X25519Spec},
    swarm::{NetworkBehaviour, PollParameters, SwarmBuilder, SwarmEvent},
    yamux, Multiaddr, PeerId, Swarm, Transport,
};
use reqwest::header::{HeaderMap, ACCEPT};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::Path;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tokio::runtime::Handle;
use tokio::sync::mpsc;
use tokio::time::timeout;

#[derive(Debug)]
struct LogEntry {
    time: DateTime<Local>,
    level: LogLevel,
    message: String,
}

struct MyBehaviour {
    kademlia: Kademlia<MemoryStore>,
    mdns: libp2p::mdns::async_io::Behaviour,
    pex: mpsc::UnboundedSender<PexMessage>,
}

#[derive(Debug)]
#[allow(unused)]
enum LogLevel {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Serialize, Deserialize, Debug)]
enum PexMessage {
    Request,
    Response(HashSet<Multiaddr>),
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let time_str: String = format!("{}", self.time.format("%m-%d|%H:%M:%S%.3f"));
        let msg_str: &str = self.message.as_str();

        let level_str: ColoredString = match self.level {
            LogLevel::Info => "INFO".green(),
            LogLevel::Warning => "WARN".yellow(),
            LogLevel::Error => "ERRO".red(),
            LogLevel::Critical => "CRIT".magenta(),
        };

        write!(f, "{} [{}] {}", level_str, time_str, msg_str)
    }
}

impl NetworkBehaviour for MyBehaviour {
    type ConnectionHandler = <Kademlia<MemoryStore> as NetworkBehaviour>::ConnectionHandler;
    type OutEvent = <Kademlia<MemoryStore> as NetworkBehaviour>::OutEvent;

    fn handle_established_inbound_connection(
        &mut self,
        connection_id: libp2p::swarm::ConnectionId,
        peer: PeerId,
        local_addr: &Multiaddr,
        remote_addr: &Multiaddr,
    ) -> std::result::Result<Self::ConnectionHandler, libp2p::swarm::ConnectionDenied> {
        NetworkBehaviour::handle_established_inbound_connection(
            &mut self.kademlia,
            connection_id,
            peer,
            local_addr,
            remote_addr,
        )
    }

    fn handle_established_outbound_connection(
        &mut self,
        connection_id: libp2p::swarm::ConnectionId,
        peer: PeerId,
        addr: &Multiaddr,
        role_override: libp2p::core::Endpoint,
    ) -> std::result::Result<Self::ConnectionHandler, libp2p::swarm::ConnectionDenied> {
        NetworkBehaviour::handle_established_outbound_connection(
            &mut self.kademlia,
            connection_id,
            peer,
            addr,
            role_override,
        )
    }

    fn handle_pending_inbound_connection(
        &mut self,
        connection_id: libp2p::swarm::ConnectionId,
        local_addr: &Multiaddr,
        remote_addr: &Multiaddr,
    ) -> std::result::Result<(), libp2p::swarm::ConnectionDenied> {
        NetworkBehaviour::handle_pending_inbound_connection(
            &mut self.kademlia,
            connection_id,
            local_addr,
            remote_addr,
        )
    }

    fn handle_pending_outbound_connection(
        &mut self,
        connection_id: libp2p::swarm::ConnectionId,
        maybe_peer: Option<PeerId>,
        addresses: &[Multiaddr],
        effective_role: libp2p::core::Endpoint,
    ) -> std::result::Result<Vec<Multiaddr>, libp2p::swarm::ConnectionDenied> {
        NetworkBehaviour::handle_pending_outbound_connection(
            &mut self.kademlia,
            connection_id,
            maybe_peer,
            addresses,
            effective_role,
        )
    }

    fn on_swarm_event(&mut self, event: libp2p::swarm::FromSwarm<Self::ConnectionHandler>) {
        NetworkBehaviour::on_swarm_event(&mut self.kademlia, event);
    }

    fn on_connection_handler_event(
        &mut self,
        peer_id: PeerId,
        connection_id: libp2p::swarm::ConnectionId,
        event: libp2p::swarm::THandlerOutEvent<Self>,
    ) {
        self.kademlia
            .on_connection_handler_event(peer_id, connection_id, event)
    }

    fn poll(
        &mut self,
        cx: &mut Context<'_>,
        params: &mut impl PollParameters,
    ) -> Poll<libp2p::swarm::ToSwarm<Self::OutEvent, libp2p::swarm::THandlerInEvent<Self>>> {
        self.kademlia.poll(cx, params)
    }
}

pub async fn discover_peers() -> Result<(), Box<dyn Error>> {
    let local_key = libp2p::identity::Keypair::generate_ed25519();
    println!("Local peer id: {:?}", local_key.public());
    let local_peer_id = PeerId::from(local_key.public());

    // Create a Tokio-based TCP transport
    let transport = libp2p::development_transport(local_key).await?;

    // Create a Kademlia behaviour
    let store = MemoryStore::new(local_peer_id.clone());
    let kademlia_config = KademliaConfig::default();
    let kademlia = Kademlia::with_config(local_peer_id.clone(), store, kademlia_config);

    let (pex_tx, mut pex_rx) = mpsc::unbounded_channel::<PexMessage>();

    let mdns_config: Config = libp2p::mdns::Config::default();
    let mdns_behaviour: std::result::Result<Behaviour<_>, std::io::Error> =
        libp2p::mdns::Behaviour::new(mdns_config, local_peer_id);
    let mdns: Behaviour<async_io::AsyncIo> = mdns_behaviour.unwrap();

    let handle = Handle::current();

    // let mut swarm = {
    //     let behaviour = MyBehaviour {
    //         kademlia,
    //         mdns,
    //         pex: pex_tx,
    //     };

    //     SwarmBuilder::with_executor(transport, behaviour, local_peer_id)
    //         .executor(Box::new(TokioExecutor { handle: handle.clone() }))
    //         .build()
    // };

    let mut swarm = {
        let behaviour = MyBehaviour {
            kademlia,
            mdns,
            pex: pex_tx,
        };

        let exec = Box::new(move |fut| {
            tokio::spawn(fut);
        });

        SwarmBuilder::with_executor(transport, behaviour, local_peer_id, exec).build()
    };

    // Handle incoming PEX messages.
    tokio::spawn(async move {
        loop {
            if let Some(message) = pex_rx.recv().await {
                match message {
                    PexMessage::Request => {
                        // Get the list of connected peers.
                        let connected_peers: Vec<PeerId> =
                            swarm.connected_peers().cloned().collect();

                        // Send a PEX response to each connected peer.
                        for peer_id in connected_peers {
                            let _ = pex_tx.send(PexMessage::Response(
                                swarm.addresses_of_peer(&peer_id).unwrap(),
                            ));
                        }
                    }
                    PexMessage::Response(peers) => {
                        // Update the list of known peers.
                        for peer in peers {
                            // Add the new peer to Kademlia.
                        }
                    }
                }
            }
        }
    });

    // Periodically send PEX requests to connected peers.
    tokio::spawn(async move {
        loop {
            // Wait for some time between PEX requests.
            tokio::time::sleep(Duration::from_secs(60)).await;

            // Get the list of connected peers.
            let connected_peers: Vec<PeerId> = swarm.connected_peers().cloned().collect();

            // Send a PEX request to each connected peer.
            for peer_id in connected_peers {
                let _ = pex_tx.send(PexMessage::Request);
            }
        }
    });

    Ok(())
}
