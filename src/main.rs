#![deny(unsafe_code)]

use dotenv::dotenv;
use eyre::Result;
use std::error::Error;

#[tokio::main()]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    networking::p2p::start_p2p_networking().await?;

    // let geth_rpc_endpoint: &str = "/home/sander/.ethereum/goerli/geth.ipc";

    // // Later we will push to this vec when we get the enode urls from the geth nodes
    // let static_nodes_remove: Vec<&str> = vec![];

    // let static_nodes_add: Vec<&str> = vec![];

    // let test_wallet_private_key: String =
    //     std::env::var("TESTWALLET_PRIVATE_KEY").expect("TESTWALLET_PRIVATE_KEY must be set");

    // let localhost_rpc_url: String =
    //     std::env::var("LOCAL_HOST_URL").expect("LOCAL_HOST_URL must be set");

    // let provider: Provider<Ws> = Provider::<Ws>::connect(localhost_rpc_url).await?;
    // let provider_arc: Arc<Provider<Ws>> = Arc::new(provider.clone());

    // let block_number: U64 = provider.get_block_number().await?;
    // let gas_price: U256 = provider.get_gas_price().await?;

    Ok(())
}
