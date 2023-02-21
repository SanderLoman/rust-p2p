// Work on contract deployment script.
// Make a simple transaction happen using flashbots.
// watch https://www.youtube.com/watch?v=wn8r674U1B4&t=3171s&ab_channel=RobertMiller aboput simple arbitrage

#![deny(unsafe_code)]

use dotenv::dotenv;
use ethers::{
    // contract::abigen,
    // contract::ContractFactory,
    prelude::*,
    // providers::{Http, Provider},
    signers::LocalWallet,
    // types::{Address, U256},
};
// use ethers_flashbots::*;
use eyre::Result;
// use std::convert::TryFrom;
// use url::Url;

mod addresses;
mod arbitrage;
mod deployment;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    println!("Running MEV Bot");
    deployment::deployment();
    arbitrage::arbitrage();
    addresses::addresses();

    // Get the environment variables
    let goe_rpc_url: String = std::env::var("GOE_RPC_URL").expect("GOE_RPC_URL must be set") ;
    let test_wallet_private_key: String =
        std::env::var("TESTWALLET_PRIVATE_KEY").expect("TESTWALLET_PRIVATE_KEY must be set");

    // This signs transactions
    let wallet = test_wallet_private_key.parse::<LocalWallet>()?;
    println!("Wallet: {:?}\n", wallet.address());

    // Connect to the network
    // let provider = Provider::<Http>::try_from(goe_rpc_url)?;

    Ok(())
}
