/// !!!
///52062
/// LISTENING ADDRESS: /ip4/0.0.0.0/tcp/9000/p2p/16Uiu2HAm3CHQXGJokLWodbDocko58tCdgotxYcR6BuXyLKcuobUR
///
/// ENR ADDRESS: enr:-K24QGDcHgq97t7pNQ0E4Q-FwiQN3ZT5JDmuMC7hz6A1bIRyO32Sti8NSpclcCTNfPgQvU6L5dgvXRfxLu7L7NeKGUY0h2F0dG5ldHOIAAAAAAAAAACEZXRoMpBiiUHvAwAQIP__________gmlkgnY0iXNlY3AyNTZrMaECc29ruZqHENx-CIWjjqcFRZpVXRmo2h20dbjRHy1fgE6Ic3luY25ldHMAg3RjcIIjKA
///
/// !!!
use chrono::{DateTime, Local, TimeZone, Utc};
use colored::*;
use discv5::{
    enr,
    enr::{CombinedKey, NodeId},
    Discv5, Discv5ConfigBuilder, Enr, TokioExecutor,
};
use ethers::prelude::*;
use eyre::Result;
use futures::stream::{self, StreamExt};
use libp2p::kad::kbucket::{Entry, EntryRefView};
use libp2p::{
    core::upgrade, dns::DnsConfig, identity, kad::*, noise::*, swarm::*, yamux, Multiaddr, PeerId,
    Swarm, Transport,
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

    let response: Value = client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .json()
        .await?;

    let data: &Vec<Value> = response.get("data").unwrap().as_array().unwrap();

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

pub async fn get_local_peer_id() -> Result<String, Box<dyn Error>> {
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
    Ok(peer_id)
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

// probably need to use the discv5 crate for this since its for discovery
pub async fn discover_peers() -> Result<(), Box<dyn Error>> {
    // maybe we dont need the local peer id because we are just discovering peers
    let local_peer_id = get_local_peer_id().await?;
    let enr_key = get_enr_key().await?;

    let enr: discv5::enr::Enr<discv5::enr::CombinedKey> = enr::Enr::from_str(&enr_key)?;

    Ok(())
}

// probably need to use the libp2p crate for this since its for managing peers
pub async fn handle_discovered_peers() -> Result<(), Box<dyn Error>> {
    let local_peer_id = get_local_peer_id().await?;
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
        bind_address: SocketAddr::from(([0, 0, 0, 0], 0)), // Replace with desired bind address
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




 */