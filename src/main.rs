// Work on contract deployment script.
// Make a simple transaction happen using flashbots.
// watch https://www.youtube.com/watch?v=wn8r674U1B4&t=3171s&ab_channel=RobertMiller aboput simple arbitrage

#![deny(unsafe_code)]

use dotenv::dotenv;
use ethers::prelude::*;
use ethers_flashbots::*;
use eyre::Result;
use std::convert::TryFrom;
use url::Url;
use ethabi::{Contract, Token};
use std::fs;

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

    let goe_rpc_url: String = std::env::var("ETH_RPC_URL").expect("ETH_RPC_URL must be set");
    let test_wallet_private_key: String =
        std::env::var("TESTWALLET_PRIVATE_KEY").expect("TESTWALLET_PRIVATE_KEY must be set");

    let wallet_address = test_wallet_private_key.parse::<LocalWallet>()?;

    let provider = Provider::<Http>::try_from(goe_rpc_url.as_str())?;
    
    Ok(())
}
