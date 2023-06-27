#![allow(unused)]

use chrono::{DateTime, TimeZone, Utc};
use ethers::prelude::*;
use eyre::Result;
use futures::stream::StreamExt;
use libp2p::Multiaddr;
use reqwest::header::{HeaderMap, ACCEPT};
use reqwest::Client;
use serde::Serialize;
use serde_json::Value;
use std::error::Error;
use std::net::IpAddr;
use std::path::Path;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;

pub async fn discover_peers() -> Result<Vec<Multiaddr>, Box<dyn Error>> {
    Ok(vec![])
}

// Add this function inside your module
async fn get_beacon_chain_block_number(ip: IpAddr, port: u16) -> Result<u64, Box<dyn Error>> {
    let url = format!("http://{}:{}/eth/v2/beacon/blocks/head", ip, port);
    let client = reqwest::Client::new();

    let response: Value = client.get(&url).send().await?.json().await?;

    let block_number = response["data"]["message"]["body"]["block_number"]
        .as_u64()
        .ok_or_else(|| {
            format!(
                "Failed to get block number from response: {:?}",
                response["data"]["message"]["body"]["block_number"]
            )
        })?;

    Ok(block_number)
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

pub async fn bootstrapped_peers_enr() -> Result<Vec<String>, Box<dyn Error>> {
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

    let mut connected_peers_enr = Vec::new();

    for peer in data {
        if let Some(Value::String(enr)) = peer.get("enr") {
            connected_peers_enr.push(enr.clone());
        }
    }

    Ok(connected_peers_enr)
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
