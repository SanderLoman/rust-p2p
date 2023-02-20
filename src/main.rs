// Work on contract deployment script.
// Make a simple transaction happen using flashbots.
// watch https://www.youtube.com/watch?v=wn8r674U1B4&t=3171s&ab_channel=RobertMiller aboput simple arbitrage


#![deny(unsafe_code)]

use dotenv::dotenv;
// use ethers::contract::abigen;
use ethers::prelude::*;
use ethers_flashbots::*;
use eyre::Result;
use std::convert::TryFrom;
use url::Url;

mod arbitrage;
mod addresses;

// Generate the ERC-20 contract bindings using `ethers-contract`
// abigen!(
//     "contracts/IERC20.sol",
//     "contracts/ERC20.sol",
// );

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    println!("Running MEV Bot");
    arbitrage::arbitrage();
    addresses::addresses();

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

    // The address of the ERC-20 token contract
    // let token_address: Address = "0x8aa561B38c7f5aB263cf35CF388c87dCAA1A03D9"
    //     .parse()
    //     .unwrap();

    // The amount of tokens to buy and sell
    // let amount_to_buy: U256 = U256::from(1_000_000);

    // Get the balance of the ERC-20 token
    // let token_contract = ERC20::new(token_address, client.provider());
    // let balance: U256 = token_contract.balance_of(client.address()).await?;
    println!("Token balance: {}\n", balance);

    // Buy the tokens using Flashbots
    // let buy_tx = token_contract
    //     .transfer(client.address(), amount_to_buy)
    //     .gas(500_000)
    //     .unwrap()
    //     .send()
    //     .await?;
    // println!(
    //     "Bought {} tokens with tx hash: {:?}\n",
    //     amount_to_buy, buy_tx.hash
    // );

    // // Approve the contract to spend the tokens
    // let approve_tx = token_contract
    //     .approve(client.address(), amount_to_buy)
    //     .gas(500_000)
    //     .unwrap()
    //     .send()
    //     .await?;
    // println!(
    //     "Approved contract to spend {} tokens with tx hash: {:?}\n",
    //     amount_to_buy, approve_tx.hash
    // );

    // // Sell the tokens using Flashbots
    // let sell_tx = token_contract
    //     .transfer(client.address(), amount_to_buy)
    //     .gas(500_000)
    //     .unwrap()
    //     .send()
    //     .await?;
    // println!(
    //     "Sold {} tokens with tx hash: {:?}\n",
    //     amount_to_buy, sell_tx.hash
    // );

    Ok(())
}