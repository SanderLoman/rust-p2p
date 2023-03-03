// Work on contract deployment script.
// Make a simple transaction happen using flashbots.

#![deny(unsafe_code)]

use dotenv::dotenv;
use ethers::prelude::{rand::thread_rng, *};
use ethers_flashbots::*;
use eyre::Result;
// use std::convert::TryFrom;
use url::Url;
use ethers::utils::Ganache;

// use hex;
// use std::convert::TryFrom;
// use ethabi::{Contract, Token};
// use std::fs;

mod abi;
mod addresses;
mod arbitrage;
mod deploy;
mod ganache;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let eth_ws_url: String = std::env::var("ETH_WS_URL").expect("ETH_WS_URL must be set");
    let url: Url = Url::parse(&eth_ws_url)?;
    let provider: Provider<Ws> = Provider::<Ws>::connect(url).await?;

    let test_wallet_private_key: String =
        std::env::var("TESTWALLET_PRIVATE_KEY").expect("TESTWALLET_PRIVATE_KEY must be set");

    let bundle_signer = LocalWallet::new(&mut thread_rng());
    println!("Bundle Signer: {:?}", bundle_signer.address());

    let wallet_address = test_wallet_private_key.parse::<LocalWallet>()?;
    println!("Wallet Address: {:?}", wallet_address.address());

    let client = SignerMiddleware::new(
        FlashbotsMiddleware::new(
            provider,
            Url::parse("https://relay.flashbots.net")?,
            bundle_signer,
        ),
        wallet_address,
    );
    println!("Client: {:?}\n", client);

    Ok(())
}
