use chrono::{DateTime, Local, TimeZone, Utc};
use colored::*;
use ethers::prelude::*;
use eyre::Result;

use serde::Serialize;
use serde_json::Value;

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

use futures::stream::{self, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tokio::runtime::Handle;

use libp2p::{
    core::upgrade,
    dns::DnsConfig,
    identity,
    kad::{record::store::MemoryStore, Kademlia, KademliaConfig, KademliaEvent, QueryResult},
    noise::{AuthenticKeypair, Keypair, NoiseConfig, X25519Spec},
    swarm::{SwarmBuilder, SwarmEvent},
    yamux, Multiaddr, PeerId, Swarm, Transport,
};

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

pub async fn discover_peers() -> Result<(), Box<dyn Error>> {
    let local_key: identity::Keypair = libp2p::identity::Keypair::generate_ed25519();
    let local_peer_id: PeerId = PeerId::from(local_key.public());

    // Create a tokio-based TCP transport
    let tcp_transport: libp2p::tcp::Config = libp2p::tcp::Config::new();
    let dns_transport = DnsConfig::system(tcp_transport).await?;

    println!(
        "{}",
        LogEntry {
            time: Local::now(),
            level: LogLevel::Info,
            message: format!("dns_transport: {:?}", dns_transport),
        }
    );

    let transport = dns_transport
        .upgrade(upgrade::Version::V1)
        .authenticate(NoiseConfig::xx(
            Keypair::<X25519Spec>::new()
                .into_authentic(&local_key)
                .unwrap(),
        )) // XX Handshake pattern, used for encrypted communication.
        .multiplex(libp2p::yamux::YamuxConfig::default())
        .boxed();

    // Create a Kademlia behaviour
    let store: MemoryStore = MemoryStore::new(local_peer_id);
    let mut kademlia_config: KademliaConfig = KademliaConfig::default();
    let kademlia: Kademlia<MemoryStore> =
        Kademlia::with_config(local_peer_id, store, kademlia_config);

    // Create a Swarm that manages peers and events
    let mut swarm = {
        let exec = Box::new(move |fut| {
            Handle::current().spawn(fut);
        });

        SwarmBuilder::with_executor(transport, kademlia, local_peer_id, exec)
    };
    
    // // Start the mDNS service
    // let mut mdns = libp2p::mdns::Config::default();

    // // Start the swarm and listen for events
    // let mut listening = false;
    // let mut listening_addr: Multiaddr = Multiaddr::empty();

    Ok(())
}

pub async fn time_to_reach_node(provider: Arc<Provider<Ws>>) -> Result<()> {
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
                    "BLOCK: {:?} | SECS: {:?} | NANOS: {:?} ",
                    block_number,
                    time_difference.num_seconds(),
                    time_difference.num_nanoseconds().unwrap(),
                ),
            }
        );
    }

    Ok(())
}

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

pub async fn is_node_functioning(provider: &Provider<Ws>, node: &str, timeout: Duration) -> bool {
    let start_time = Instant::now();

    // Send a simple query (e.g., get_block_number) to the node
    let block_number_result = provider.get_block_number().await;

    // Measure the response time
    let elapsed_time = start_time.elapsed();

    // If the response time is less than the specified timeout and the query was successful, return true
    // Otherwise, return false
    elapsed_time < timeout && block_number_result.is_ok()
}

pub async fn update_node_reliability(
    node_reliability_scores: &mut HashMap<String, f64>,
    node: &str,
    reliability: f64,
) {
    // Update the reliability score of the node in the hashmap
    // If the node is not in the hashmap, add it with the given reliability
    node_reliability_scores
        .entry(node.to_string())
        .or_insert(reliability);
}

pub async fn filter_nodes_by_reliability(
    node_reliability_scores: &HashMap<String, f64>,
    reliability_threshold: f64,
) -> Vec<String> {
    // Filter the nodes in the hashmap based on the given reliability threshold
    // Return a Vec<String> of nodes that pass the threshold
    node_reliability_scores
        .iter()
        .filter(|&(_, score)| *score >= reliability_threshold)
        .map(|(node, _)| node.to_string())
        .collect()
}

pub async fn score_node(node: &str) -> f64 {
    // Calculate a score for the node based on its attributes (e.g., uptime, latency, throughput)
    // Return the calculated score
    // For now, I will return a placeholder value. You can implement your own scoring logic.
    1.0
}

async fn prioritize_nodes_by_score(nodes: &Vec<String>) -> Vec<String> {
    let scored_nodes: Vec<(String, f64)> = stream::iter(nodes.iter().map(|node: &String| {
        let node: String = node.clone();
        async move { (node.clone(), score_node(&node).await) }
    }))
    .then(|future| async { future.await })
    .collect::<Vec<_>>()
    .await;

    // Sort the nodes based on their scores
    let mut sorted_nodes: Vec<(String, f64)> = scored_nodes.clone();
    sorted_nodes
        .sort_unstable_by(|a: &(String, f64), b: &(String, f64)| b.1.partial_cmp(&a.1).unwrap());

    // Return the sorted list of nodes
    sorted_nodes.into_iter().map(|(node, _)| node).collect()
}
