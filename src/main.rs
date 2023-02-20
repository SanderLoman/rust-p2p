#![deny(unsafe_code)]
use dotenv::dotenv;
use ethers::prelude::*;
use ethers_flashbots::*;
use eyre::Result;
use std::convert::TryFrom;
use url::Url;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    println!("Running MEV Bot");

    // Get the environment variables
    let goe_rpc_url: String = std::env::var("GOE_RPC_URL").expect("GOE_RPC_URL must be set");
    let test_wallet_private_key: String =
        std::env::var("TESTWALLET_PRIVATE_KEY").expect("TESTWALLET_PRIVATE_KEY must be set");

    // This signs transactions
    let wallet = test_wallet_private_key.parse::<LocalWallet>()?;
    println!("Wallet: {:?}\n", wallet.address());

    // Connect to the network
    let provider = Provider::<Http>::try_from(goe_rpc_url)?;
    println!("Provider: {:?}\n", provider);

    // This is your searcher identity
    let bundle_signer = test_wallet_private_key.parse::<LocalWallet>()?;
    println!("Bundle signer: {:?}\n", bundle_signer.address());

    // get the balance
    let balance: U256 = provider.get_balance(wallet.address(), None).await?;
    println!("Balance: {}\n", balance);

    // Add signer and Flashbots middleware
    let client = SignerMiddleware::new(
        FlashbotsMiddleware::new(
            provider,
            Url::parse("https://relay-goerli.flashbots.net")?,
            bundle_signer,
        ),
        wallet,
    );
    println!("Client: {:?}\n", client.address());

    let address: Address = "0x7B23298319Ea680e73059AE6eB1fF4162C9bD89e"
        .parse()
        .unwrap();

    // make sure the maxFeePerGas os higher than the baseFee of the block

    // Pay Vitalik using a Flashbots bundle!
    let tx = TransactionRequest::pay(address, 1);
    println!("Transaction: {}\n", serde_json::to_string(&tx)?);

    let pending_tx = client.send_transaction(tx, None).await?;

    // Get the receipt
    let receipt = pending_tx
        .await?
        .ok_or_else(|| eyre::format_err!("tx not included"))?;
    let tx = client.get_transaction(receipt.transaction_hash).await?;

    println!("Sent transaction: {}\n", serde_json::to_string(&tx)?);
    println!("Receipt: {}\n", serde_json::to_string(&receipt)?);

    Ok(())
}