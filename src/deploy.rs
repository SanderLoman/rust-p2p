#![deny(unsafe_code)]

use dotenv::dotenv;
use ethers::prelude::*;
use eyre::Result;
// use url::Url;
// use hex;

pub fn deploy() -> Result<()> {
    dotenv().ok();
    println!("Deploying contracts.");

    let deployer_priv_key: String =
        std::env::var("TESTWALLET_PRIVATE_KEY").expect("TESTWALLET_PRIVATE_KEY must be set");

    let deployer = deployer_priv_key.parse::<LocalWallet>()?;
    println!("Deployer: {:?}", deployer.address());
    Ok(())
}
