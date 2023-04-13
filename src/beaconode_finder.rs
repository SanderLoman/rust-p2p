use chrono::{DateTime, Local};
use colored::*;
use eyre::Result;
use futures::prelude::*;
use libp2p::{
    core::Multiaddr,
    gossipsub::{Gossipsub, GossipsubConfig, GossipsubEvent, MessageAuthenticity, Topic},
    identity,
    tcp::TokioTcpConfig,
    PeerId, Swarm, Transport,
};
use reqwest::Client as AsyncClient;
use serde::Serialize;
use serde_json::Value;
use std::cmp::Ordering;
use std::fmt;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;

use std::time::Instant;

#[derive(Debug)]
struct LogEntry {
    time: DateTime<Local>,
    level: LogLevel,
    message: String,
}

#[derive(Debug, Serialize, Clone)]
struct NodeResponseTime {
    peer_id: PeerId,
    multiaddr: Multiaddr,
    elapsed: std::time::Duration,
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
        let time_str = format!("{}", self.time.format("%m-%d|%H:%M:%S%.3f"));
        let msg_str = self.message.as_str();

        let level_str = match self.level {
            LogLevel::Info => "INFO".green(),
            LogLevel::Warning => "WARN".yellow(),
            LogLevel::Error => "ERRO".red(),
            LogLevel::Critical => "CRIT".magenta(),
        };

        write!(f, "{} [{}] {}", level_str, time_str, msg_str)
    }
}


async fn discover_nodes() -> Result<Vec<(PeerId, Multiaddr)>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    let transport = TokioTcpConfig::new()
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(libp2p::noise::NoiseConfig::xx(local_key))
        .multiplex(libp2p::yamux::YamuxConfig::default())
        .boxed();

    let topic = Topic::new("eth2-gossip");
    let gossipsub_config = GossipsubConfig::default();
    let mut gossipsub = Gossipsub::new(MessageAuthenticity::Signed(local_key), gossipsub_config);

    gossipsub.subscribe(topic.clone()).unwrap();

    let mut swarm = {
        let mut swarm = Swarm::new(transport, gossipsub, local_peer_id);
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;
        swarm
    };

    let mut discovered_nodes = vec![];

    swarm
        .for_each_concurrent(None, |event| async {
            match event {
                libp2p::swarm::SwarmEvent::NewListenAddr(addr) => {
                    println!("Listening on {:?}", addr);
                }
                libp2p::swarm::SwarmEvent::ConnectionEstablished {
                    peer_id, endpoint, ..
                } => {
                    println!("Connected to {:?}", peer_id);
                    discovered_nodes.push((peer_id, endpoint.get_remote_address().clone()));
                }
                libp2p::swarm::SwarmEvent::Behaviour(GossipsubEvent::Message { .. }) => {}
                _ => {}
            }
        })
        .await;

    Ok(discovered_nodes)
}

async fn test_node_response_time(nodes: Vec<(PeerId, Multiaddr)>) -> Result<()> {
    let mut response_times = vec![];

    for (peer_id, multiaddr) in nodes {
        let api_url = format!("http://{}/eth/v1/beacon/states/head/validators", multiaddr);
        let client = reqwest::Client::new();

        let start = Instant::now();
        let response = client.get(&api_url).send().await;
        let elapsed = start.elapsed();

        match response {
            Ok(_) => {
                response_times.push(NodeResponseTime {
                    peer_id: peer_id.clone(),
                    multiaddr: multiaddr.clone(),
                    elapsed,
                });
                println!(
                    "{}",
                    LogEntry {
                        time: Local::now(),
                        level: LogLevel::Info,
                        message: format!(
                            "Node with peer ID {} and multiaddr {} responded in {:?}",
                            peer_id, multiaddr, elapsed
                        ),
                    }
                );
            }
            Err(err) => {
                println!(
                    "{}",
                    LogEntry {
                        time: Local::now(),
                        level: LogLevel::Warning,
                        message: format!(
                            "Node with peer ID {} and multiaddr {} failed to respond: {}",
                            peer_id, multiaddr, err
                        ),
                    }
                );
            }
        }
    }

    // Sort response_times by elapsed time in ascending order
    response_times.sort_by(|a, b| match a.elapsed.cmp(&b.elapsed) {
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
        Ordering::Equal => a.peer_id.cmp(&b.peer_id),
    });

    let top_fastest_nodes = response_times
        .iter()
        .take(50)
        .cloned()
        .collect::<Vec<NodeResponseTime>>();

    // Create the responses directory and write the top 50 response times to a JSON file
    let responses_folder = Path::new("responses");
    create_dir_all(&responses_folder)?;
    let output_file = responses_folder.join("top_fastest_nodes.json");
    let json_data = serde_json::to_string_pretty(&top_fastest_nodes)?;
    let mut file = File::create(output_file)?;
    file.write_all(json_data.as_bytes())?;

    println!(
        "{}",
        LogEntry {
            time: Local::now(),
            level: LogLevel::Info,
            message: "Top 50 fastest response times written to responses/top_fastest_nodes.json"
                .to_string(),
        }
    );

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Call the discover_nodes function to get a list of discovered nodes.
    let nodes = discover_nodes().await?;

    // Call the test_node_response_time function with the discovered nodes.
    test_node_response_time(nodes).await?;

    Ok(())
}
