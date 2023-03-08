// Work on contract deployment script.
// Make a simple transaction happen using flashbots.

#![deny(unsafe_code)]

use dotenv::dotenv;
// use ethers::prelude::{rand::thread_rng, *};
// use ethers_flashbots::*;
use eyre::Result;
// use std::convert::TryFrom;
// use ethers::utils::Ganache;
// use url::Url;

// use hex;
// use std::convert::TryFrom;
// use ethabi::{Contract, Token};
// use std::fs;

mod abi;
mod addresses;
mod arbitrage;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    abi::abis();
    addresses::addresses();
    arbitrage::arbitrage();

    Ok(())
}
