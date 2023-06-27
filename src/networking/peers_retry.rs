#![deny(unsafe_code)]

use async_std::task;
use base64::prelude::*;
use chrono::{DateTime, Local, TimeZone, Utc};
use colored::*;
use discv5::{
    enr,
    enr::{ed25519_dalek, k256, CombinedKey, CombinedPublicKey, EnrBuilder, NodeId},
    socket::ListenConfig,
    Discv5, Discv5Config, Discv5ConfigBuilder, Discv5Error, Discv5Event, Enr, TokioExecutor,
};
use ethers::prelude::*;
use eyre::Result;
use futures::stream::{self, StreamExt};
use futures::Future;
use generic_array::GenericArray;
use hex::*;
use libp2p::core::{identity::PublicKey, multiaddr::Protocol};
use libp2p::kad::kbucket::{Entry, EntryRefView};
use libp2p::{
    autonat::*, core::upgrade, dns::DnsConfig, floodsub::*, identity, identity::Keypair, kad::*,
    multiaddr, noise::*, ping, swarm::behaviour::*, swarm::*, yamux, Multiaddr, PeerId, Swarm,
    Transport,
};
use pnet::packet::ip;
use rand::thread_rng;
use reqwest::header::{HeaderMap, ACCEPT};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use ssz::*;
use ssz_derive::{Decode, Encode};
use ssz_types::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4};
use std::path::Path;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::macros::support::Pin;
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

#[derive(NetworkBehaviour, Default)]
pub struct CustomBehavior {
    keep_alive: keep_alive::Behaviour,
    ping: ping::Behaviour,
}

#[derive(Debug)]
#[allow(unused)]
enum LogLevel {
    Info,
    Warning,
    Error,
    Critical,
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

// curl -X 'GET' 'http://127.0.0.1:5052/eth/v1/node/peers' -H 'accept: application/json'
pub async fn bootstrapped_peers() -> Result<Vec<(String, String, String, String)>, Box<dyn Error>> {
    let url: &str = "http://127.0.0.1:5052/eth/v1/node/peers";
    let client: Client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    let res = client.get(url).headers(headers).send().await?;
    let body = res.text().await?;
    let json: Value = serde_json::from_str(&body)?;

    let peers: Vec<Value> = json["data"].as_array().ok_or("Data not found")?.clone();

    let mut results: Vec<(String, String, String, String)> = Vec::new();

    for peer in peers {
        let state = peer["state"].as_str().ok_or("State not found")?.to_owned();
        if state == "connected" {
            let peer_id = peer["peer_id"]
                .as_str()
                .ok_or("Peer ID not found")?
                .to_owned();
            let enr = peer["enr"].as_str().ok_or("ENR not found")?.to_owned();
            let last_seen_p2p_address = peer["last_seen_p2p_address"]
                .as_str()
                .ok_or("Last seen P2P address not found")?
                .to_owned();

            results.push((peer_id, enr, last_seen_p2p_address, state));
        }
    }

    Ok(results)
}

pub async fn get_local_peer_info(
) -> Result<(String, String, String, String, String, String), Box<dyn Error>> {
    let url = "http://127.0.0.1:5052/eth/v1/node/identity";
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    let res = client.get(url).headers(headers).send().await?;
    let body = res.text().await?;
    let json: Value = serde_json::from_str(&body)?;
    let peer_id = json["data"]["peer_id"]
        .as_str()
        .ok_or("Peer ID not found")?
        .to_owned();
    let enr = json["data"]["enr"]
        .as_str()
        .ok_or("ENR not found")?
        .to_owned();
    let p2p_address = json["data"]["p2p_addresses"][0]
        .as_str()
        .ok_or("P2P address not found")?
        .to_owned();
    let discovery_address = json["data"]["discovery_addresses"][0]
        .as_str()
        .ok_or("Discovery address not found")?
        .to_owned();
    let attnets = json["data"]["metadata"]["attnets"]
        .as_str()
        .ok_or("attnets not found")?
        .to_owned();
    let syncnets = json["data"]["metadata"]["syncnets"]
        .as_str()
        .ok_or("syncnets not found")?
        .to_owned();
    Ok((
        peer_id,
        enr,
        p2p_address,
        discovery_address,
        attnets,
        syncnets,
    ))
}

pub async fn decode_hex_value(hex_string: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let bytes =
        hex::decode(&hex_string.replace("0x", "")).map_err(|_| "Failed to parse hex string")?;
    Ok(bytes)
}

pub async fn get_eth2_value(enr_string: &str) -> Option<String> {
    if let Some(start) = enr_string.find("\"eth2\", \"") {
        let rest = &enr_string[start + 9..];
        if let Some(end) = rest.find("\")") {
            return Some(rest[..end].to_string());
        }
    }
    None
}

pub async fn generate_enr(
    ip4: std::net::Ipv4Addr,
    port: u16,
    syncnets_bytes: Vec<u8>,
    attnets_bytes: Vec<u8>,
    eth2_bytes: Vec<u8>,
) -> Result<(Enr, CombinedKey), Box<dyn Error>> {
    let combined_key = CombinedKey::generate_secp256k1();
    let enr = EnrBuilder::new("v4")
        .ip4(ip4)
        .tcp4(port)
        .udp4(port)
        .add_value("syncnets", &syncnets_bytes)
        .add_value("attnets", &attnets_bytes)
        .add_value_rlp("eth2", eth2_bytes.into())
        .build(&combined_key)
        .map_err(|_| "Failed to generate ENR")?;
    Ok((enr, combined_key))
}

pub async fn discover_peers() -> Result<Vec<Vec<(String, String, String, String)>>, Box<dyn Error>>
{
    let mut found_peers: Vec<Vec<(String, String, String, String)>> = Vec::new();
    let bootstrapped_peers = bootstrapped_peers().await?;
    found_peers.push(bootstrapped_peers);

    // for peer in &found_peers {
    //     for (peer_id, enr, p2p_address, state) in peer {
    //         println!("Peer ID: {:?}", peer_id);
    //         println!("ENR: {:?}", enr);
    //         println!("P2P Address: {:?}", p2p_address);
    //         println!("State: {:?}", state);
    //     }
    //     println!("Number of peers bootstrapped: {:?}\n\n\n", peer.len());
    // }

    let (
        peer_id_local,
        enr_local,
        p2p_address_local,
        discovery_address_local,
        attnets_local,
        syncnets_local,
    ) = get_local_peer_info().await?;

    let decoded_enr: enr::Enr<CombinedKey> = Enr::from_str(&enr_local)?;

    println!("LIGHTHOUSE ENR: {:?}\n", decoded_enr);
    println!("LIGHTHOUSE ENR: {}\n", decoded_enr);

    let attnets_bytes = decode_hex_value(&attnets_local).await?;
    let syncnets_bytes = decode_hex_value(&syncnets_local).await?;

    let enr_string = format!("{:?}", decoded_enr);
    let eth2_value = get_eth2_value(&enr_string).await;

    // If eth2_value is None, return early
    let eth2_value = match eth2_value {
        Some(value) => value,
        None => return Ok(found_peers),
    };

    let eth2_bytes = decode_hex_value(&eth2_value).await?;

    let port: u16 = 7777;
    let ip = "0.0.0.0".parse::<std::net::Ipv4Addr>().unwrap();
    // !!!
    //
    // NEED TO FIX IP ISSUE IN ENR (0.0.0.0:7777), needs to be a public ip
    //
    // !!!
    let (enr, enr_key) = generate_enr(ip, port, syncnets_bytes, attnets_bytes, eth2_bytes).await?;

    let listen_conf = ListenConfig::from_ip(std::net::IpAddr::V4(ip), port);
    let discv5_config = Discv5ConfigBuilder::new(listen_conf).build();

    println!("SELF GENERATED ENR {:?}\n", enr);
    println!("SELF GENERATED ENR {}", enr);

    let libp2p_local_key = Keypair::generate_secp256k1();
    let libp2p_local_peer_id = PeerId::from(libp2p_local_key.public());

    let tcp = libp2p::tcp::tokio::Transport::new(libp2p::tcp::Config::default().nodelay(true));
    let transport1 = libp2p::dns::TokioDnsConfig::system(tcp)?;
    let transport2 = libp2p::dns::TokioDnsConfig::system(libp2p::tcp::tokio::Transport::new(
        libp2p::tcp::Config::default().nodelay(true),
    ))?;

    let transport = transport1.or_transport(libp2p::websocket::WsConfig::new(transport2));

    // mplex config
    let mut mplex_config = libp2p::mplex::MplexConfig::new();
    mplex_config.set_max_buffer_size(256);
    mplex_config.set_max_buffer_behaviour(libp2p::mplex::MaxBufferBehaviour::Block);

    // yamux config
    let mut yamux_config = libp2p::yamux::YamuxConfig::default();
    yamux_config.set_window_update_mode(libp2p::yamux::WindowUpdateMode::on_read());

    fn generate_noise_config(
        identity_keypair: &Keypair,
    ) -> libp2p::noise::NoiseAuthenticated<XX, X25519Spec, ()> {
        let static_dh_keys = libp2p::noise::Keypair::<X25519Spec>::new()
            .into_authentic(identity_keypair)
            .expect("signing can fail only once during starting a node");
        libp2p::noise::NoiseConfig::xx(static_dh_keys).into_authenticated()
    }

    let upgraded_transport = transport
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(generate_noise_config(&libp2p_local_key))
        .multiplex(libp2p::core::upgrade::SelectUpgrade::new(
            yamux_config,
            mplex_config,
        ))
        .timeout(Duration::from_secs(10))
        .boxed();

    let mut discv5: Discv5 = Discv5::new(enr.clone(), enr_key, discv5_config)?;

    discv5.start().await.expect("Discv5 failed to start");

    // !!!
    //
    // We have to make our custom behaviour here
    //
    // !!!
    let behaviour = dummy::Behaviour;

    let executor = move |fut: Pin<Box<dyn Future<Output = ()> + Send + 'static>>| {
        tokio::spawn(fut);
    };

    task::block_on(async {
        let listen_addr: Multiaddr = "/ip4/0.0.0.0/tcp/7777"
            .parse()
            .expect("Failed to parse multiaddr");
        let mut swarm = SwarmBuilder::with_executor(
            upgraded_transport,
            behaviour,
            libp2p_local_peer_id,
            executor,
        )
        .build();
        swarm.listen_on(listen_addr).unwrap();

        // loop {
        //     match swarm.select_next_some().await {
        //         SwarmEvent::Behaviour(_) => {
        //             println!("Behaviour event");
        //         }
        //         SwarmEvent::ConnectionEstablished {
        //             peer_id,
        //             endpoint,
        //             num_established,
        //             concurrent_dial_errors,
        //             established_in,
        //         } => {
        //             println!(
        //                 "Connection established to {:?} at {:?} (total: {:?}), concurrent dial errors: {:?}, established in {:?}",
        //                 peer_id, endpoint, num_established, concurrent_dial_errors, established_in
        //             );
        //         }
        //         SwarmEvent::ConnectionClosed {
        //             peer_id,
        //             endpoint,
        //             num_established,
        //             cause,
        //         } => {
        //             println!(
        //                 "Connection closed to {:?} at {:?} (total: {:?}), cause: {:?}",
        //                 peer_id, endpoint, num_established, cause
        //             );
        //         }
        //         SwarmEvent::IncomingConnection {
        //             local_addr,
        //             send_back_addr,
        //         } => {
        //             println!(
        //                 "Incoming connection, addr: {:?} send_back_addr: {:?}",
        //                 local_addr, send_back_addr
        //             );
        //         }
        //         SwarmEvent::IncomingConnectionError {
        //             local_addr,
        //             send_back_addr,
        //             error,
        //         } => {
        //             println!(
        //                 "Incoming connection error, addr: {:?} send_back_addr: {:?} error: {:?}",
        //                 local_addr, send_back_addr, error
        //             );
        //         }
        //         SwarmEvent::OutgoingConnectionError { peer_id, error } => {
        //             println!(
        //                 "Outgoing connection error, peer_id: {:?} error: {:?}",
        //                 peer_id, error
        //             );
        //         }
        //         SwarmEvent::BannedPeer { peer_id, endpoint } => {
        //             println!("Banned peer {:?} at {:?}", peer_id, endpoint);
        //         }
        //         SwarmEvent::NewListenAddr { address, .. } => {
        //             println!("Listening on {:?}", address);
        //         }
        //         SwarmEvent::ExpiredListenAddr {
        //             listener_id,
        //             address,
        //         } => {
        //             println!("Expired listen addr {:?} {:?}", listener_id, address);
        //         }
        //         SwarmEvent::ListenerClosed {
        //             listener_id,
        //             addresses,
        //             reason,
        //         } => {
        //             println!(
        //                 "Listener closed {:?} {:?} {:?}",
        //                 listener_id, addresses, reason
        //             );
        //         }
        //         SwarmEvent::ListenerError { listener_id, error } => {
        //             println!("Listener error {:?} {:?}", listener_id, error);
        //         }
        //         SwarmEvent::Dialing(peer_id) => {
        //             println!("Dialing {:?}", peer_id);
        //         }
        //     }
        // }
    });

    Ok(found_peers)
}

pub async fn handle_discovered_peers() -> Result<(), Box<dyn Error>> {
    let discovered_peers: Vec<Vec<(String, String, String, String)>> = discover_peers().await?;
    let (peer_id, enr, p2p_address, discovery_addresss, attnets, syncnets) =
        get_local_peer_info().await?;
    Ok(())
}
