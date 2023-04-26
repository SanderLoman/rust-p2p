#![deny(unsafe_code)]
use chrono::{DateTime, Local, TimeZone, Utc};
use ethers::prelude::*;
use eyre::Result;

use serde::Serialize;
use serde_json::Value;

use colored::*;
use std::fmt;
use std::path::Path;
use std::sync::Arc;

use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

use libp2p::*;
use std::collections::HashMap;
use std::time::Instant;

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
