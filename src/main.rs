// Work on contract deployment script.
// Make a simple transaction happen using flashbots.

#![deny(unsafe_code)]

use dotenv::dotenv;
use ethabi::Contract;
use ethers::prelude::*;
use ethers::utils::Ganache;
use ethers_flashbots::{self, FlashbotsMiddleware};
use eyre::Result;
use serde_json::json;
use std::fs;
use url::Url;

mod abi;
mod addresses;
mod arbitrage;

#[tokio::main(flavor = "multi_thread", worker_threads = 20)]
async fn main() -> Result<()> {
    dotenv().ok();

    let test_wallet_private_key: String =
        std::env::var("TESTWALLET_PRIVATE_KEY").expect("TESTWALLET_PRIVATE_KEY must be set");

    let eth_rpc_url: String = std::env::var("ETH_WS_URL").expect("ETH_WS_URL must be set");
    let eth_rpc_url_clone: String = eth_rpc_url.clone();

    let provider_eth = Provider::<Ws>::connect(eth_rpc_url).await?;

    let bundlesigning_wallet = LocalWallet::new(&mut rand::thread_rng());
    println!(
        "Bundle signing wallet: {:?}",
        bundlesigning_wallet.address()
    );

    let ganache = Ganache::new().fork(eth_rpc_url_clone).port(8545u16).spawn();
    let provider_ganache = Provider::<Ws>::connect(ganache.ws_endpoint()).await?;

    let wallet = test_wallet_private_key.parse::<LocalWallet>()?;

    let client = SignerMiddleware::new(
        FlashbotsMiddleware::new(
            provider_eth,
            Url::parse("https://relay-goerli.flashbots.net")?,
            bundlesigning_wallet,
        ),
        wallet,
    );

    abi::abis();
    addresses::addresses();
    arbitrage::arbitrage();

    Ok(())
}
