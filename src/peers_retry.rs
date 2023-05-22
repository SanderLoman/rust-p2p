/// !!!
/// 52062
/// LISTENING ADDRESS: /ip4/0.0.0.0/tcp/9000/p2p/16Uiu2HAm3CHQXGJokLWodbDocko58tCdgotxYcR6BuXyLKcuobUR
///
/// ENR ADDRESS LAPTOP: enr:-K24QGDcHgq97t7pNQ0E4Q-FwiQN3ZT5JDmuMC7hz6A1bIRyO32Sti8NSpclcCTNfPgQvU6L5dgvXRfxLu7L7NeKGUY0h2F0dG5ldHOIAAAAAAAAAACEZXRoMpBiiUHvAwAQIP__________gmlkgnY0iXNlY3AyNTZrMaECc29ruZqHENx-CIWjjqcFRZpVXRmo2h20dbjRHy1fgE6Ic3luY25ldHMAg3RjcIIjKA
///
/// !!!
use chrono::{DateTime, Local, TimeZone, Utc};
use colored::*;
use discv5::{
    enr,
    enr::{CombinedKey, NodeId, k256, EnrBuilder},
    Discv5, Discv5ConfigBuilder, Enr, TokioExecutor,
};
use ethers::prelude::*;
use eyre::Result;
use futures::stream::{self, StreamExt};
use libp2p::kad::kbucket::{Entry, EntryRefView};
use libp2p::{
    core::upgrade, dns::DnsConfig, identity, kad::*, noise::*, ping, swarm::*, yamux, Multiaddr,
    PeerId, Swarm, Transport, multiaddr
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
use rand::thread_rng;

#[derive(Debug)]
struct LogEntry {
    time: DateTime<Local>,
    level: LogLevel,
    message: String,
}

#[derive(NetworkBehaviour, Default)]
struct Behavior {
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
pub async fn bootstrapped_peers() -> Result<Vec<String>, Box<dyn Error>> {
    let url: &str = "http://127.0.0.1:5052/eth/v1/node/peers";
    let client: Client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());

    let res: Value = client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .json()
        .await?;

    let data: &Vec<Value> = res.get("data").unwrap().as_array().unwrap();

    let mut connected_peers = Vec::new();

    for peer in data {
        if let Some(Value::String(address)) = peer.get("last_seen_p2p_address") {
            if let Some(Value::String(peer_id)) = peer.get("peer_id") {
                if let Some(Value::String(state)) = peer.get("state") {
                    if state == "connected" {
                        connected_peers.push(format!("{}/p2p/{}", address, peer_id));
                    }
                }
            }
        }
    }

    Ok(connected_peers)
}

pub async fn get_local_peer_info() -> Result<(String, String, String, String), Box<dyn Error>> {
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
    Ok((peer_id, enr, p2p_address, discovery_address))
}

pub async fn get_enr_key() -> Result<String, Box<dyn Error>> {
    let url = "http://127.0.0.1:5052/eth/v1/node/identity";
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    let res = client.get(url).headers(headers).send().await?;
    let body = res.text().await?;
    let json: Value = serde_json::from_str(&body)?;
    let enr_key = json["data"]["enr"]
        .as_str()
        .ok_or("ENR not found")?
        .to_owned();
    Ok(enr_key)
}

pub fn decode_enr(enr_key: &str) -> Result<Enr, Box<dyn Error>> {
    let enr = Enr::from_str(enr_key)?;
    println!("ENR: {:?}", enr);
    println!("Node ID: {:?}", enr.node_id());
    println!("IP: {:?}", enr.ip4());
    println!("TCP Port: {:?}", enr.tcp4());
    println!("UDP Port: {:?}", enr.udp4());
    // and so on for other fields...
    Ok(enr)
}

pub async fn gen_enr() -> Result<String, Box<dyn Error>> {
    let (peer_id, _, p2p_address, _) = get_local_peer_info().await?;

    // generate a random secp256k1 key
    let mut rng = thread_rng();
    let key = k256::ecdsa::SigningKey::random(&mut rng);

    let ip = p2p_address.split("/").nth(2).unwrap();
    let port = p2p_address.split("/").nth(4).unwrap();

    let ip = ip.parse::<std::net::Ipv4Addr>().unwrap();
    let port = port.parse::<u16>().unwrap();

    let enr = EnrBuilder::new("v4").ip4(ip).tcp4(port).udp4(port).build(&key)?;  // Added udp(port)
    let enr_key = enr.to_base64();
    println!("Generated ENR: {:?}", enr_key);

    Ok(enr_key)
}

// probably need to use the discv5 crate for this since its for discovery
pub async fn discover_peers() -> Result<Vec<String>, Box<dyn Error>> {
    // found_peers is a vector of peer addresses that we have found, we will push more to this vector as we discover more peers
    let mut found_peers: Vec<String> = Vec::new();
    let bootstrapped_peers = bootstrapped_peers().await?;
    bootstrapped_peers.iter().for_each(|peer| {
        let peer = peer.clone();
        found_peers.push(peer);
    });
    // println!("Found {found_peers:?}");

    let local_peer_id = get_local_peer_info().await?;
    let enr_key = get_enr_key().await?;

    let enr: discv5::enr::Enr<discv5::enr::CombinedKey> = enr::Enr::from_str(&enr_key)?;
    let test = decode_enr(enr.clone().to_base64().as_str())?;
    println!("ENR: {:?}\n\n\n", test);


    gen_enr().await?;

    decode_enr(gen_enr().await?.as_str())?;

    Ok(found_peers)
}

// probably need to use the libp2p crate for this since its for managing peers
pub async fn handle_discovered_peers() -> Result<(), Box<dyn Error>> {
    let discovered_peers = discover_peers().await?;
    let local_peer_id = get_local_peer_info().await?;
    let enr_key = get_enr_key().await?;
    Ok(())
}

/*
use async_std::task;
use discv5::{enr::{CombinedKey, Enr, EnrBuilder}, enr_ext::create_enr, enr_key::secp256k1, Discv5Config, Discv5Service};
use std::error::Error;
use std::net::SocketAddr;

fn main() -> Result<(), Box<dyn Error>> {
    // Generate a random secp256k1 key pair
    let key = secp256k1::SecretKey::random();
    let keypair = CombinedKey::from_secret(key);

    // Generate the local ENR (Ethereum Node Record)
    let local_enr: Enr<CombinedKey> = EnrBuilder::new("v4").build(&keypair)?;

    // Specify the configuration for the Discv5 service
    let config = Discv5Config {
        bind_address: SocketAddr::from(([0, 0, 0, 0], 0)),
        enr: Some(local_enr.clone()),
        ..Default::default()
    };

    // Create a Discv5 service
    let mut discv5 = Discv5Service::new(keypair, local_enr, config)?;

    // Start the Discv5 service in a separate task
    let discv5_task = task::spawn(async move {
        while let Some(event) = discv5.next().await {
            match event {
                discv5::Discv5Event::Discovered(enr) => {
                    println!("Discovered new peer: {:?}", enr);
                    // Handle the discovered peer (e.g., add it to a peer list)
                }
                discv5::Discv5Event::SocketError(err) => {
                    println!("Discv5 socket error: {:?}", err);
                    // Handle socket errors
                }
                _ => {}
            }
        }
    });

    // Bootstrap the Discv5 service by connecting to known bootstrap nodes
    let bootstrap_nodes = vec![
        "enr://enr.example.com?key=value", // Replace with actual bootstrap nodes
        "enr://enr.example.com?key=value",
    ];
    discv5.bootstrap(&bootstrap_nodes)?;

    // Perform any other application-specific tasks
    // ...

    // Block the main thread to keep the Discv5 service running
    task::block_on(discv5_task);

    Ok(())
}


The discovery domain: discv5
Discovery Version 5 (discv5) (Protocol version v5.1) is used for peer discovery.

discv5 is a standalone protocol, running on UDP on a dedicated port, meant for peer discovery only. discv5 supports self-certified, flexible peer records (ENRs) and topic-based advertisement, both of which are (or will be) requirements in this context.

Integration into libp2p stacks
discv5 SHOULD be integrated into the client’s libp2p stack by implementing an adaptor to make it conform to the service discovery and peer routing abstractions and interfaces (go-libp2p links provided).

Inputs to operations include peer IDs (when locating a specific peer) or capabilities (when searching for peers with a specific capability), and the outputs will be multiaddrs converted from the ENR records returned by the discv5 backend.

This integration enables the libp2p stack to subsequently form connections and streams with discovered peers.

ENR structure
The Ethereum Node Record (ENR) for an Ethereum consensus client MUST contain the following entries (exclusive of the sequence number and signature, which MUST be present in an ENR):

The compressed secp256k1 publickey, 33 bytes (secp256k1 field).
The ENR MAY contain the following entries:

An IPv4 address (ip field) and/or IPv6 address (ip6 field).
A TCP port (tcp field) representing the local libp2p listening port.
A UDP port (udp field) representing the local discv5 listening port.
Specifications of these parameters can be found in the ENR Specification.

Attestation subnet bitfield
The ENR attnets entry signifies the attestation subnet bitfield with the following form to more easily discover peers participating in particular attestation gossip subnets.

Key	Value
attnets	SSZ Bitvector[ATTESTATION_SUBNET_COUNT]
If a node's MetaData.attnets has any non-zero bit, the ENR MUST include the attnets entry with the same value as MetaData.attnets.

If a node's MetaData.attnets is composed of all zeros, the ENR MAY optionally include the attnets entry or leave it out entirely.

eth2 field
ENRs MUST carry a generic eth2 key with an 16-byte value of the node's current fork digest, next fork version, and next fork epoch to ensure connections are made with peers on the intended Ethereum network.

Key	Value
eth2	SSZ ENRForkID
Specifically, the value of the eth2 key MUST be the following SSZ encoded object (ENRForkID)

(
    fork_digest: ForkDigest
    next_fork_version: Version
    next_fork_epoch: Epoch
)
where the fields of ENRForkID are defined as

fork_digest is compute_fork_digest(current_fork_version, genesis_validators_root) where
current_fork_version is the fork version at the node's current epoch defined by the wall-clock time (not necessarily the epoch to which the node is sync)
genesis_validators_root is the static Root found in state.genesis_validators_root
next_fork_version is the fork version corresponding to the next planned hard fork at a future epoch. If no future fork is planned, set next_fork_version = current_fork_version to signal this fact
next_fork_epoch is the epoch at which the next fork is planned and the current_fork_version will be updated. If no future fork is planned, set next_fork_epoch = FAR_FUTURE_EPOCH to signal this fact
Note: fork_digest is composed of values that are not known until the genesis block/state are available. Due to this, clients SHOULD NOT form ENRs and begin peer discovery until genesis values are known. One notable exception to this rule is the distribution of bootnode ENRs prior to genesis. In this case, bootnode ENRs SHOULD be initially distributed with eth2 field set as ENRForkID(fork_digest=compute_fork_digest(GENESIS_FORK_VERSION, b'\x00'*32), next_fork_version=GENESIS_FORK_VERSION, next_fork_epoch=FAR_FUTURE_EPOCH). After genesis values are known, the bootnodes SHOULD update ENRs to participate in normal discovery operations.

Clients SHOULD connect to peers with fork_digest, next_fork_version, and next_fork_epoch that match local values.

Clients MAY connect to peers with the same fork_digest but a different next_fork_version/next_fork_epoch. Unless ENRForkID is manually updated to matching prior to the earlier next_fork_epoch of the two clients, these connecting clients will be unable to successfully interact starting at the earlier next_fork_epoch.

pub async fn discover_peers() -> Result<Vec<String>, Box<dyn Error>> {
    // found_peers is a vector of peer addresses that we have found, we will push more to this vector as we discover more peers
    let mut found_peers: Vec<String> = Vec::new();
    let bootstrapped_peers = bootstrapped_peers().await?;
    bootstrapped_peers.iter().for_each(|peer| {
        let peer = peer.clone();
        found_peers.push(peer);
    });
    println!("Found {found_peers:?}");

    let local_peer_id = get_local_peer_id().await?;
    let enr_key = get_enr_key().await?;

    // let enr: discv5::enr::Enr<discv5::enr::CombinedKey> = enr::Enr::from_str(&enr_key)?;
    // println!("ENR: {:?}", enr);

    Ok(found_peers)
}

implement the discovery mechanism BUT i need you to do it a little different. So the idea is to have this Rust script find all peers on the ethereum beacon chain by first of all bootstrapping some peers (which it gets the peers from another function that i made, returning the bootstrapped peers)

 */
