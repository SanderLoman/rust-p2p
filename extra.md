FlashBotsUniswapQuery address: 0x5EF1009b9FCD4fec3094a5564047e190D72Bd511 (for simple arbitrage, maybe not needed)
UniswapRouterV2 address: 0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D
UniswapFactory address: 0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f
PancakeRouterV2 address: 0x10ED43C718714eb63d5aA57B78B54704E256024E
SushiSwap address: 0x1b02dA8Cb0d097eB8D57A175b88c7D8b47997506

Ethereum address: 0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2

```rust
    let sub = provider_eth.watch_pending_transactions().await?;

    sub.for_each(|tx| async move {
        println!("New pending transaction: https://etherscan.io/tx/{:?}", tx);
    })
    .await;

    let ethereum_ca: Address = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse()?;
    let read_ethereum_abi: String = fs::read_to_string("abis/ethereum.json")?;
    let abi = Contract::load(read_ethereum_abi.as_bytes())?;

    let ethereum_contract = ethers::contract::Contract::new(ethereum_ca, abi, provider_eth);
```

I plan to develop a Rust script that can continuously search for random nodes on the Ethereum mainnet and test their response times. The script will maintain a hashmap or similar data structure to log the response times of these nodes. It will also keep track of the 50 best nodes found from different parts of the world (e.g. america, europe, asia, australia) and establish a static connection between geth and these nodes to obtain data as quickly as possible. I wouldn't know if this would be possible for the ethereum concensus client lighthouse aswell, but if it is then we could sort of do the same for lighthouse aswell.

I ideally want to use libp2p for this so we can just look for nodes or peers and test their response time and then try to get a connection once it passes all the checks needed for a good connection. 

As new nodes are discovered, the script will replace slower nodes from the top 50 list. It will also disconnect geth from these slower nodes and connect it to the faster ones.

Some ideas for my project, you could consider implementing the following features:

1. Implement a mechanism to check if a node is responding to queries correctly or if it's malfunctioning.

2. Add a feature that allows the script to automatically identify and exclude nodes that are not reliable.

3. Consider implementing a mechanism that allows the script to prioritize nodes with specific features or attributes, such as high uptime, low latency, or high throughput.

4. You could also add a feature to automatically switch between different Ethereum networks or clients to find the optimal configuration for the best performance.

5. (optional) Finally, you could consider implementing a web interface or dashboard that displays the status of the nodes and the overall performance of the script in real-time.

Do you have any idea how we could build this? So far is have this code:

MAIN FILE:
```rust
// use ethers::core::{rand::thread_rng, types::transaction::eip2718::TypedTransaction};
// use ethers_flashbots::*;
// use url::Url;

#![deny(unsafe_code)]
use chrono::{DateTime, Local};
use colored::*;
use dotenv::dotenv;
use ethers::prelude::*;
use eyre::Result;

use std::fmt;
use std::sync::Arc;

mod liquidations;
mod peers;
mod sandwhich;

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

#[tokio::main()]
async fn main() -> Result<()> {
    dotenv().ok();

    liquidations::liquidations().await?;

    let geth_rpc_endpoint: &str = "/home/sander/.ethereum/goerli/geth.ipc";

    // Later we will push to this vec when we get the enode urls from the geth nodes
    let static_nodes_remove: Vec<&str> = vec![];

    let static_nodes_add: Vec<&str> = vec![];

    // let test_wallet_private_key: String =
    //     std::env::var("TESTWALLET_PRIVATE_KEY").expect("TESTWALLET_PRIVATE_KEY must be set");

    let localhost_rpc_url: String =
        std::env::var("LOCAL_HOST_URL").expect("LOCAL_HOST_URL must be set");

    let provider: Provider<Ws> = Provider::<Ws>::connect(localhost_rpc_url).await?;
    let provider_arc: Arc<Provider<Ws>> = Arc::new(provider.clone());

    let block_number: U64 = provider.get_block_number().await?;
    let gas_price: U256 = provider.get_gas_price().await?;

    println!(
        "{}",
        LogEntry {
            time: Local::now(),
            level: LogLevel::Info,
            message: format!("gas_price {:?}", gas_price),
        }
    );

    peers::time_to_reach_node(provider_arc).await?;

    Ok(())
}
// try get the beacon node blocks and check how long it takes to receive them from another peer and maybe check how long it takes for geth to receive it from the beacon node
```

```rust
PEERS FILE:

#![deny(unsafe_code)]
use chrono::{DateTime, Local, TimeZone, Utc};
use colored::*;
use ethers::prelude::*;
use eyre::Result;

use serde::Serialize;
use serde_json::Value;

use std::collections::HashMap;
use std::fmt;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

use futures::stream::{self, StreamExt};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

use libp2p::*;

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
    sorted_nodes.sort_unstable_by(|a: &(String, f64), b: &(String, f64)| b.1.partial_cmp(&a.1).unwrap());

    // Return the sorted list of nodes
    sorted_nodes.into_iter().map(|(node, _)| node).collect()
}
```