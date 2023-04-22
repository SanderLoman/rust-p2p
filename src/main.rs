// Work on a script to find the fastest node on the network and then try to connect to those with geth.

// use discv5 to find nodes

#![deny(unsafe_code)]

use chrono::{DateTime, Local};
use colored::*;
use dotenv::dotenv;
// use ethers::core::{rand::thread_rng, types::transaction::eip2718::TypedTransaction};
use ethers::prelude::*;
// use ethers_flashbots::*;
use eyre::Result;
use std::fmt;
// use url::Url;

use std::error::Error;
use std::path::Path;

use serde::Serialize;
use serde_json::Value;

use reqwest::Client;

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

        let level_str = match self.level {
            LogLevel::Info => "INFO".green(),
            LogLevel::Warning => "WARN".yellow(),
            LogLevel::Error => "ERRO".red(),
            LogLevel::Critical => "CRIT".magenta(),
        };

        write!(f, "{} [{}] {}", level_str, time_str, msg_str)
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 20)]
async fn main() -> Result<()> {
    dotenv().ok();

    liquidations::liquidations().await?;

    let geth_rpc_endpoint: &str = "ws://localhost:8551";

    // Replace with your desired static nodes' enode URLs
    let static_nodes: Vec<&str> = vec![
        "enode://node1@example.com:30303",
        "enode://node2@example.com:30303",
    ];

    let client: Client = Client::new();

    for enode_url in static_nodes {
        add_peer(Path::new(geth_rpc_endpoint), enode_url);
    }

    println!("Static nodes added successfully.");

    // let test_wallet_private_key: String =
    //     std::env::var("TESTWALLET_PRIVATE_KEY").expect("TESTWALLET_PRIVATE_KEY must be set");

    let localhost_rpc_url: String =
        std::env::var("LOCAL_HOST_URL").expect("LOCAL_HOST_URL must be set");

    let provider: Provider<Ws> = Provider::<Ws>::connect(localhost_rpc_url).await?;
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

// use tokio::io::BufReader;
use tokio_uds::UnixStream;

async fn add_peer(ipc_path: &Path, enode_url: &str) -> Result<(), Box<dyn Error>> {
    #[derive(Serialize)]
    struct JsonRpcRequest<'a> {
        jsonrpc: &'a str,
        id: i32,
        method: &'a str,
        params: Vec<&'a str>,
    }

    let request = JsonRpcRequest {
        jsonrpc: "2.0",
        id: 1,
        method: "admin_addPeer",
        params: vec![enode_url],
    };

    let request_data = serde_json::to_string(&request)?;

    let mut stream = UnixStream::connect(ipc_path);

    // Send the request
    stream.write_all(request_data.as_bytes()).await?;

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
    } else {
        println!(
            "{}",
            LogEntry {
                time: Local::now(),
                level: LogLevel::Info,
                message: format!("Added static node: {}", enode_url),
            }
        );
    }

    Ok(())
}
