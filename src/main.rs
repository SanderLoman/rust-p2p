// use ethers::core::{rand::thread_rng, types::transaction::eip2718::TypedTransaction};
// use ethers_flashbots::*;
// use url::Url;

#![deny(unsafe_code)]
use chrono::{DateTime, Local, TimeZone, Utc};
use colored::*;
use dotenv::dotenv;
use ethers::prelude::*;
use eyre::Result;

use serde::Serialize;
use serde_json::Value;

use std::fmt;
use std::path::Path;
use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

mod liquidations;
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

    for enode_url in static_nodes_remove {
        delete_peer(Path::new(geth_rpc_endpoint), enode_url).await?;
    }

    for enode_url in static_nodes_add {
        add_peer(Path::new(geth_rpc_endpoint), enode_url).await?;
    }

    // let test_wallet_private_key: String =
    //     std::env::var("TESTWALLET_PRIVATE_KEY").expect("TESTWALLET_PRIVATE_KEY must be set");

    let localhost_rpc_url: String =
        std::env::var("LOCAL_HOST_URL").expect("LOCAL_HOST_URL must be set");

    let provider: Provider<Ws> = Provider::<Ws>::connect(localhost_rpc_url).await?;
    let provider_arc = Arc::new(provider.clone());

    time_to_reach_node(provider_arc).await?;

    // let block_number: U64 = provider.get_block_number().await?;
    let gas_price: U256 = provider.get_gas_price().await?;

    println!(
        "{}",
        LogEntry {
            time: Local::now(),
            level: LogLevel::Info,
            message: format!("gas_price {:?}", gas_price),
        }
    );

    Ok(())
}

async fn add_peer(ipc_path: &Path, enode_url: &str) -> Result<()> {
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

    if response.get("error").is_some() {
        println!(
            "{}",
            LogEntry {
                time: Local::now(),
                level: LogLevel::Error,
                message: format!("Failed to add static node: {}", enode_url),
            }
        );
        println!(
            "{}",
            LogEntry {
                time: Local::now(),
                level: LogLevel::Error,
                message: format!("Error: {:?}", response.get("error")),
            }
        );
    } else {
        println!(
            "{}",
            LogEntry {
                time: Local::now(),
                level: LogLevel::Info,
                message: format!("Added static node: {}", enode_url),
            }
        );
        println!(
            "{}",
            LogEntry {
                time: Local::now(),
                level: LogLevel::Info,
                message: format!("Response: {:?}", response),
            }
        );
    }

    Ok(())
}

async fn delete_peer(ipc_path: &Path, enode_url: &str) -> Result<()> {
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

    if response.get("error").is_some() {
        println!(
            "{}",
            LogEntry {
                time: Local::now(),
                level: LogLevel::Error,
                message: format!("Failed to remove static node: {}", enode_url),
            }
        );
        println!(
            "{}",
            LogEntry {
                time: Local::now(),
                level: LogLevel::Error,
                message: format!("Error: {:?}", response.get("error")),
            }
        );
    } else {
        println!(
            "{}",
            LogEntry {
                time: Local::now(),
                level: LogLevel::Info,
                message: format!("Removed static node: {}", enode_url),
            }
        );
        println!(
            "{}",
            LogEntry {
                time: Local::now(),
                level: LogLevel::Info,
                message: format!("Response: {:?}", response),
            }
        );
    }

    Ok(())
}

async fn find_node() {}

async fn time_to_reach_node(provider: Arc<Provider<Ws>>) -> Result<()> {
    let mut stream: SubscriptionStream<Ws, Block<TxHash>> = provider.subscribe_blocks().await?;

    while let Some(block_header) = stream.next().await {
        let block_timestamp: U256 = block_header.timestamp;
        let block_time: chrono::LocalResult<DateTime<Utc>> =
            Utc.timestamp_opt(block_timestamp.as_u64() as i64, 0);

        let now: DateTime<Utc> = Utc::now();
        let time_difference: chrono::Duration = now.signed_duration_since(block_time.unwrap());

        println!(
            "{}",
            LogEntry {
                time: Local::now(),
                level: LogLevel::Info,
                message: format!(
                    "block timestamp: {}, time difference: {:.2?}",
                    block_timestamp, time_difference
                ),
            }
        );
    }

    Ok(())
}

// Amsterdam enode
// enode://35df251d21ec503b24ace1f5b18bb85d06fa0dc113f069ab399c5b4064d3e6d89a947b2ebe373f0fcceba6a3db1441117a74b535f484ade24113c5e0c5cd4795@5.39.169.121:30303

// USA enode
// enode://78253a80431ac5514f3efc0aafe7d5793a0e838a97cbc54acbf2fc5219307bc72b96be2747615847e04aabd5495af96ec6f1e661da9e7f71e86717a51a0fd291@54.146.231.114:30303

// maybe lighthouse receives the block before geth does

// try get the beacon node blocks and check how long it takes to receive them from another peer and maybe check how long it takes for geth to receive it from the beacon node
