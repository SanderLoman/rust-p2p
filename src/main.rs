// Work on contract deployment script.
// Make a simple transaction happen using flashbots.

#![deny(unsafe_code)]

use dotenv::dotenv;
use ethabi::Contract;
use ethers::prelude::*;
use eyre::Result;
use std::fs;

mod abi;
mod addresses;
mod arbitrage;

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<()> {
    dotenv().ok();

    let eth_rpc_url: String = std::env::var("ETH_WS_URL").expect("ETH_WS_URL must be set");
    
    let provider_eth = Provider::<Ws>::connect(eth_rpc_url).await?;
    let sub  = provider_eth.watch_pending_transactions().await?;
    
    sub.for_each(|tx| async move {
        println!("New pending transaction: https://etherscan.io/tx/{:?}", tx);
    }).await;

    // let ethereum_ca: Address = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse()?;
    // let read_ethereum_abi: String = fs::read_to_string("abis/ethereum.json")?;
    // let abi = Contract::load(read_ethereum_abi.as_bytes())?;

    // let ethereum_contract = ethers::contract::Contract::new(ethereum_ca, abi, provider_eth);


    abi::abis();
    addresses::addresses();
    arbitrage::arbitrage();

    Ok(())
}
