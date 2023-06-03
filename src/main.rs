// use ethers::core::{rand::thread_rng, types::transaction::eip2718::TypedTransaction};
// use ethers_flashbots::*;
// use url::Url;

#![deny(unsafe_code)]
use chrono::{DateTime, Local};
use colored::*;
use dotenv::dotenv;
use ethers::prelude::*;
use eyre::Result;

use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

use libp2p::{
    core::upgrade, dns::DnsConfig, identity, kad::*, noise::*, swarm::*, yamux, Multiaddr, PeerId,
    Swarm, Transport,
};

mod liquidations;
mod peers;
mod peers_retry;
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
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

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

    liquidations::liquidations().await?;

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

    peers_retry::discover_peers().await?;

    // peers::time_to_reach_geth(provider_arc).await?;

    // let enodes = peers::get_enode_addresses().await?;

    // let best_peers =  peers::process_enodes(enodes).await?;

    // let mut peer_addresses = Vec::new();
    // for peers in best_peers.values() {
    //     peer_addresses.extend(peers.keys().cloned());
    // }

    // Connect to the top nodes and obtain data as quickly as possible
    // let mut connected_nodes = HashMap::new();
    // for enode in peer_addresses {
    //     if let Some(peer_id) = PeerId::from_str(&enode) {
    //         let multiaddr = format!("/p2p/{}", peer_id.to_base58());
    //         if let Ok(stream) = libp2p::tcp::Config::new()
    //             .dial(multiaddr.parse::<Multiaddr>().unwrap())
    //             .await
    //         {
    //             let client = provider;
    //             if let Ok(_) = client.get_block_number().await {
    //                 connected_nodes.insert(enode, client);
    //             }
    //         }
    //     }
    // }

    // // Disconnect from slow nodes and connect to fast nodes
    // for (enode, client) in &connected_nodes {
    //     if client.eth_syncing().await.unwrap().is_none() {
    //         connected_nodes.remove(enode);
    //     }
    // }

    // let mut slow_nodes = best_peers;
    // slow_nodes.retain(|region, _| {
    //     let mut region_slow = true;
    //     if let Some(connected_region) = connected_nodes.get(region) {
    //         for (enode, score) in slow_nodes.get(region).unwrap().iter() {
    //             if connected_region.eth_syncing().await.unwrap().is_some()
    //                 && score > &score_peer(Duration::from_secs(0))
    //             {
    //                 slow_nodes.get_mut(region).unwrap().remove(enode);
    //             } else {
    //                 region_slow = false;
    //             }
    //         }
    //         false
    //     } else {
    //         region_slow
    //     }
    // });

    // for (enode, client) in connected_nodes {
    //     println!(
    //         "Connected to {} with score {}",
    //         enode,
    //         score_peer(Duration::from_secs(0))
    //     );
    //     // Do something with the connected client...
    // }

    Ok(())
}
// try get the beacon node blocks and check how long it takes to receive them from another peer and maybe check how long it takes for geth to receive it from the beacon node

// eth_callBundle is for simulating a transaction bundle and seeing if it will be included in the next block mev-geth supports this
