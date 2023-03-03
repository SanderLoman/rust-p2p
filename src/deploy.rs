#![deny(unsafe_code)]

use dotenv::dotenv;
// use ethers::prelude::*;
use eyre::Result;
// use url::Url;
// use hex;

pub fn deploy() -> Result<()> {
    dotenv().ok();
    println!("Deploying contracts.");

    Ok(())
}
