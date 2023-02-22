// Work on contract deployment script.
// Make a simple transaction happen using flashbots.
// watch https://www.youtube.com/watch?v=wn8r674U1B4&t=3171s&ab_channel=RobertMiller aboput simple arbitrage

#![deny(unsafe_code)]

use dotenv::dotenv;
use ethers::prelude::{rand::thread_rng, *};
use ethers_flashbots::*;
use eyre::Result;
// use std::convert::TryFrom;
use url::Url;
// use ethabi::{Contract, Token};
// use std::fs;
use ethers::utils::Ganache;
// use serde_json::*;

mod addresses;
mod arbitrage;
mod deploy;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    println!("Running MEV Bot");
    deploy::deploy();
    arbitrage::arbitrage();
    addresses::addresses();

    let eth_ws_url: String = std::env::var("ETH_WS_URL").expect("ETH_WS_URL must be set");
    let url = Url::parse(&eth_ws_url)?;
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

    Ganache::new().spawn();


    Ok(())
}
