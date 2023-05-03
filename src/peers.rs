use chrono::{DateTime, Local, TimeZone, Utc};
use colored::*;
use ethers::prelude::*;
use eyre::Result;
use futures::future::ok;
use futures::stream::{self, StreamExt};
use libp2p::{
    core::upgrade,
    dns::DnsConfig,
    identity,
    kad::{record::store::MemoryStore, Kademlia, KademliaConfig, KademliaEvent, QueryResult},
    mdns::{Behaviour, Event},
    noise::{AuthenticKeypair, Keypair, NoiseConfig, X25519Spec},
    swarm::{NetworkBehaviour, SwarmBuilder, SwarmEvent},
    yamux, Multiaddr, PeerId, Swarm, Transport,
};
use reqwest::header::{HeaderMap, ACCEPT};
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{Duration, Instant};
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

/// A structure representing an Ethereum node
#[derive(Debug)]
struct Node {
    url: String,
    enodes: Vec<String>,
    response_time: Option<Duration>,
}

impl Node {
    /// Creates a new Node with the given URL
    fn new(url: String) -> Self {
        Node {
            url,
            enodes: Vec::new(),
            response_time: None,
        }
    }
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

const SCORE_THRESHOLD: f64 = 0.0;
const MAX_TOP_NODES: usize = 50;
const RESPONSE_TIME_THRESHOLD: Duration = Duration::from_secs(5);

pub async fn discover_peers() -> Result<Vec<String>, Box<dyn Error>> {
    let local_key = libp2p::identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    // Create a Tokio-based TCP transport
    let transport = libp2p::development_transport(local_key).await?;

    // Create a Kademlia behaviour
    let store = MemoryStore::new(local_peer_id.clone());
    let kademlia_config = KademliaConfig::default();
    let kademlia = Kademlia::with_config(local_peer_id.clone(), store, kademlia_config);

    // Create a Swarm that manages peers and events
    let mut swarm = {
        let exec = Box::new(move |fut| {
            tokio::spawn(fut);
        });
        SwarmBuilder::with_executor(transport, kademlia, local_peer_id, exec).build()
    };

    // Bootstrap Kademlia with peers from get_connected_peers()
    let connected_peers = get_connected_peers().await?;
    let mut discovered_peers = HashSet::new();
    let mut pending_peers = HashSet::new();

    println!(
        "{}",
        LogEntry {
            time: Local::now(),
            level: LogLevel::Info,
            message: format!("Initial connected peers: {:?}", connected_peers),
        }
    );

    for peer_id_str in &connected_peers {
        if let Ok(peer_id) = PeerId::from_str(peer_id_str) {
            pending_peers.insert(peer_id);
        }
    }

    // Add initial connected peers to the swarm
    for addr_str in &connected_peers {
        let addr = addr_str.parse::<Multiaddr>()?;
        if let Some(peer_id) = addr.iter().find_map(|proto| match proto {
            libp2p::core::multiaddr::Protocol::P2p(hash) => PeerId::from_multihash(hash).ok(),
            _ => None,
        }) {
            swarm.dial(addr.clone()).unwrap_or_else(|_| {
                println!(
                    "{}",
                    LogEntry {
                        time: Local::now(),
                        level: LogLevel::Error,
                        message: format!("Failed to dial peer: {:?}", peer_id),
                    }
                )
            });

            println!(
                "{}",
                LogEntry {
                    time: Local::now(),
                    level: LogLevel::Info,
                    message: format!("Dialed peer: {:?}", peer_id),
                }
            );
        }
    }

    while !pending_peers.is_empty() {
        // Take a peer from the pending_peers set
        let peer_id = pending_peers.iter().next().unwrap().clone();
        pending_peers.remove(&peer_id);

        // Add the peer to the discovered_peers set
        discovered_peers.insert(peer_id.clone());

        println!(
            "{}",
            LogEntry {
                time: Local::now(),
                level: LogLevel::Info,
                message: format!("Processing peer: {:?}", peer_id),
            }
        );

        // Perform an iterative search for the closest peers
        swarm.behaviour_mut().get_closest_peers(peer_id);

        // Process the swarm events
        loop {
            let event = swarm.next().await;
            match event {
                Some(SwarmEvent::Behaviour(KademliaEvent::OutboundQueryProgressed {
                    id: _,
                    result,
                    ..
                })) => {
                    println!(
                        "{}",
                        LogEntry {
                            time: Local::now(),
                            level: LogLevel::Info,
                            message: format!("Outbound query progressed: {:?}", result),
                        }
                    );
                    match result {
                        QueryResult::GetClosestPeers(Ok(ok)) => {
                            let mut new_peers = false;
                            for peer_id in ok.peers.into_iter() {
                                // If the peer is not already discovered, add it to pending_peers
                                if !discovered_peers.contains(&peer_id) {
                                    pending_peers.insert(peer_id);
                                    new_peers = true;
                                }
                            }

                            // If new peers were added to pending_peers, break the loop to process them
                            if new_peers {
                                println!(
                                    "{}",
                                    LogEntry {
                                        time: Local::now(),
                                        level: LogLevel::Info,
                                        message: format!("New peers found: {:?}", pending_peers),
                                    }
                                );
                                break;
                            }
                        }
                        _ => println!(
                            "{}",
                            LogEntry {
                                time: Local::now(),
                                level: LogLevel::Warning,
                                message: format!("Unhandled query result: {:?}", result),
                            }
                        ),
                    }
                }
                _ => println!(
                    "{}",
                    LogEntry {
                        time: Local::now(),
                        level: LogLevel::Warning,
                        message: format!("Unhandled event: {:?}", event),
                    }
                ),
            }
        }
    }

    // Convert the discovered_peers set to a vector of strings
    let discovered_peers: Vec<String> = discovered_peers
        .into_iter()
        .map(|peer_id| peer_id.to_base58())
        .collect();

    // Print discovered peers
    for peer in &discovered_peers {
        println!(
            "{}",
            LogEntry {
                time: Local::now(),
                level: LogLevel::Info,
                message: format!("Discovered peer: {:?}", peer),
            }
        );
    }

    Ok(discovered_peers)
}

// curl -X 'GET' 'http://127.0.0.1:5052/eth/v1/node/peers' -H 'accept: application/json'
pub async fn get_connected_peers() -> Result<Vec<String>, Box<dyn Error>> {
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
            if let Some(Value::String(state)) = peer.get("state") {
                if state == "connected" {
                    connected_peers.push(address.clone());
                    // println!(
                    //     "{}",
                    //     LogEntry {
                    //         time: Local::now(),
                    //         level: LogLevel::Info,
                    //         message: format!("{:?}", address),
                    //     }
                    // );
                }
            }
        }
    }

    Ok(connected_peers)
}

// curl -X 'GET' 'http://127.0.0.1:5052/eth/v2/beacon/blocks/head' -H 'accept: application/json'
pub async fn get_consensus_block() -> Result<Vec<String>, Box<dyn Error>> {
    let url: &str = "http://127.0.0.1:5052/eth/v2/beacon/blocks/head";
    let client: reqwest::Client = reqwest::Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());

    let response: Value = client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .json()
        .await?;

    let data: &Value = response.get("data").unwrap();
    let message: &Value = data.get("message").unwrap();
    let body: &Value = message.get("body").unwrap();
    let execution_payload: &Value = body.get("execution_payload").unwrap();
    let transactions: &Value = execution_payload.get("transactions").unwrap();

    let mut transaction_strings: Vec<String> = Vec::new();

    for transaction in transactions.as_array().unwrap() {
        let transaction_string: String = transaction.to_string();
        let transaction_string = transaction_string.trim_matches('"').to_string();
        transaction_strings.push(transaction_string);
    }

    // enable if you want to flood the terminal
    // println!("{:?}", transaction_strings);

    Ok(transaction_strings)
}

// sending a block request to consensus client to see if they have a new block
pub async fn try_send_block_request() {}

// maybe not needed, incase the block request method is already working.
pub async fn get_response_time() {}

// needed for developement purposes
pub async fn time_to_reach_geth(provider: Arc<Provider<Ws>>) -> Result<()> {
    let mut stream: SubscriptionStream<Ws, Block<TxHash>> = provider.subscribe_blocks().await?;

    while let Some(block_header) = stream.next().await {
        let block_timestamp: U256 = block_header.timestamp;
        let block_time: chrono::LocalResult<DateTime<Utc>> =
            Utc.timestamp_opt(block_timestamp.as_u64() as i64, 0);

        let now: DateTime<Utc> = Utc::now();
        let time_difference = now.signed_duration_since(block_time.unwrap());
        let block_number = provider.get_block_number().await?;

        println!(
            "{}",
            LogEntry {
                time: Local::now(),
                level: LogLevel::Info,
                message: format!(
                    "BLOCK: {:?} | SECS: {:.10?}.{:?}",
                    block_number,
                    time_difference.num_seconds(),
                    time_difference.num_nanoseconds().unwrap(),
                ),
            }
        );
    }

    Ok(())
}

// for geth to add a static peer
pub async fn add_peer(ipc_path: &Path, enode_url: &str) -> Result<()> {
    #[derive(Serialize)]
    struct JsonRpcRequest<'a> {
        jsonrpc: &'a str,
        id: i32,
        method: &'a str,
        params: Vec<&'a str>,
    }

    let request: JsonRpcRequest = JsonRpcRequest {
        jsonrpc: "2.0",
        id: 1,
        method: "admin_addPeer",
        params: vec![enode_url],
    };

    let request_data: String = serde_json::to_string(&request)?;
    let mut stream: UnixStream = UnixStream::connect(ipc_path).await?;

    // Send the request
    stream.write_all(request_data.as_bytes()).await?;
    stream.shutdown().await?;

    let mut response_data: String = String::new();
    let mut buf_reader: BufReader<UnixStream> = BufReader::new(stream);
    buf_reader.read_to_string(&mut response_data).await?;

    let response: Value = serde_json::from_str(&response_data)?;

    Ok(())
}

// for geth to remove a static peer
pub async fn delete_peer(ipc_path: &Path, enode_url: &str) -> Result<()> {
    #[derive(Serialize)]
    struct JsonRpcRequest<'a> {
        jsonrpc: &'a str,
        id: i32,
        method: &'a str,
        params: Vec<&'a str>,
    }

    let request: JsonRpcRequest = JsonRpcRequest {
        jsonrpc: "2.0",
        id: 1,
        method: "admin_removePeer",
        params: vec![enode_url],
    };

    let request_data: String = serde_json::to_string(&request)?;
    let mut stream: UnixStream = UnixStream::connect(ipc_path).await?;

    // Send the request
    stream.write_all(request_data.as_bytes()).await?;
    stream.shutdown().await?;

    let mut response_data: String = String::new();
    let mut buf_reader: BufReader<UnixStream> = BufReader::new(stream);
    buf_reader.read_to_string(&mut response_data).await?;

    let response: Value = serde_json::from_str(&response_data)?;

    Ok(())
}

// maybe we dont even need these, but we will see later.
pub async fn update_node_reliability() {}
pub async fn filter_nodes_by_reliability() {}
pub async fn score_node() {}
pub async fn prioritize_nodes_by_score() {}
pub async fn get_best_peers() {}
pub async fn process_peers() {}
pub async fn get_region() {}
