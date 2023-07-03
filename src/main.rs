#![deny(unsafe_code)]

use dotenv::dotenv;
use ethers::prelude::*;
use eyre::Result;

use std::error::Error;
use std::sync::Arc;

mod beacon_node;
mod consensus;
mod evm;
mod mev;
mod networking;

use crate::mev::*;
use beacon_node::*;
use consensus::*;
use evm::*;
use networking::{discv5::*, libp2p::*};

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

    networking::libp2p::swarm::setup_swarm().await?;
    networking::find_peers::discover_peers().await?;
    networking::libp2p::transport::setup_transport().await?;
    networking::discv5::discovery::setup_discv5().await?;

    Ok(())
}
// try get the beacon node blocks and check how long it takes to receive them from another peer and maybe check how long it takes for geth to receive it from the beacon node

// eth_callBundle is for simulating a transaction bundle and seeing if it will be included in the next block mev-geth supports this
