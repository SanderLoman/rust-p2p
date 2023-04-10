#![deny(unsafe_code)]

use dotenv::dotenv;
use ethers::core::{rand::thread_rng, types::transaction::eip2718::TypedTransaction};
use ethers::prelude::*;
use ethers_flashbots::*;
use eyre::Result;
use url::Url;

#[tokio::main(flavor = "multi_thread", worker_threads = 20)]
async fn main() -> Result<()> {
    dotenv().ok();

    let test_wallet_private_key: String =
        std::env::var("TESTWALLET_PRIVATE_KEY").expect("TESTWALLET_PRIVATE_KEY must be set");

    let localhost_rpc_url: String =
        std::env::var("LOCALHOST_WS_URL").expect("LOCALHOST_WS_URL must be set");

    let provider = Provider::<Ws>::connect(localhost_rpc_url).await?;
    let block_number = provider.clone();
    let gas_price = provider.clone();

    let bundle_signer = LocalWallet::new(&mut thread_rng());
    // This signs transactions
    let wallet = test_wallet_private_key.parse::<LocalWallet>()?;

    // Add signer and Flashbots middleware
    let client = SignerMiddleware::new(
        FlashbotsMiddleware::new(
            provider,
            Url::parse("https://relay.flashbots.net")?,
            bundle_signer,
        ),
        wallet,
    );

    // get last block number 
    let block_n = block_number.get_block_number().await?;

    // Build a custom bundle that pays 0x0000000000000000000000000000000000000000
    let gas_p = gas_price.get_gas_price().await?;

    let tx = {
        let mut inner: TypedTransaction = TransactionRequest::new()
            .to("0x8C66BA8157808cba80A57a0A29600221973FA29F")
            .value(1)
            .gas(gas_p)
            .into();
        client.fill_transaction(&mut inner, None).await?;
        inner
    };
    let signature = client.signer().sign_transaction(&tx).await?;
    let bundle = BundleRequest::new()
        .push_transaction(tx.rlp_signed(&signature))
        .set_block(block_n + 1)
        .set_simulation_block(block_n)
        .set_simulation_timestamp(0);

    // Simulate it
    let simulated_bundle = client.inner().simulate_bundle(&bundle).await?;
    println!("Simulated bundle: {:?}", simulated_bundle);

    // Send it
    let pending_bundle = client.inner().send_bundle(&bundle).await?;

    // You can also optionally wait to see if the bundle was included
    match pending_bundle.await {
        Ok(bundle_hash) => println!(
            "Bundle with hash {:?} was included in target block",
            bundle_hash
        ),
        Err(PendingBundleError::BundleNotIncluded) => {
            println!("Bundle was not included in target block.")
        }
        Err(e) => println!("An error occured: {}", e),
    }

    Ok(())
}