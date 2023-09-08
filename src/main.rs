#![deny(unsafe_code)]

use clap::{App, Arg};
use dotenv::dotenv;
use eyre::Result;
use slog::Logger;
use std::error::Error;

use networking::p2p::P2PNetwork;
use wagmi::create_logger;

#[tokio::main()]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    // Use clap for command-line argument parsing
    let matches = App::new("wagmi")
        .version("1.0")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    // Get verbosity level
    let verbosity = matches.occurrences_of("v");

    // Initialize the logger
    let log = create_logger(verbosity);

    // networking::p2p::start_p2p_networking(log).await?;
    P2PNetwork::new(log).await?;

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
