#![deny(unsafe_code)]

use chrono::{DateTime, Local};
use colored::*;
use dotenv::dotenv;
use ethers::prelude::*;
use eyre::Result;

use std::error::Error;
use std::fmt;
use std::sync::Arc;

mod beacon_node;
mod consensus;
mod evm;
mod mev;
mod networking;
// {
    // pub mod discv5 {
    //     pub mod enr;
    //     pub mod discovery;
    // }
    // pub mod libp2p {
    //     pub mod connections;
    //     pub mod message_handling;
    //     pub mod peer_management;
    //     pub mod status_request;
    // }
    // pub mod peers_retry;
// }

use crate::mev::*;
use beacon_node::*;
use consensus::*;
use evm::*;
use networking::{discv5::*, libp2p::*};

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

    networking::find_peers::discover_peers().await?;
    networking::discv5::discovery::discv5_events().await;
    networking::libp2p::transport::setup_transport().await?;

    Ok(())
}
// try get the beacon node blocks and check how long it takes to receive them from another peer and maybe check how long it takes for geth to receive it from the beacon node

// eth_callBundle is for simulating a transaction bundle and seeing if it will be included in the next block mev-geth supports this
