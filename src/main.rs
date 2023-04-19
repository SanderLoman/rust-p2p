// Work on a script to find the fastest node on the network and then try to connect to those with geth.

// use discv5 to find nodes

#![deny(unsafe_code)]

use chrono::{DateTime, Local};
use colored::*;
use dotenv::dotenv;
// use ethers::core::{rand::thread_rng, types::transaction::eip2718::TypedTransaction};
use ethers::prelude::*;
// use ethers_flashbots::*;
use eyre::Result;
use std::fmt;
// use url::Url;

mod sandwhich;
mod wagmi;

#[derive(Debug)]
struct LogEntry {
    time: DateTime<Local>,
    level: LogLevel,
    message: String,
}

#[derive(Debug)]
#[allow(unused)]
enum LogLevel {
    Info,
    Warning,
    Error,
    Critical,
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let time_str = format!("{}", self.time.format("%m-%d|%H:%M:%S%.3f"));
        let msg_str = self.message.as_str();

        let level_str = match self.level {
            LogLevel::Info => "INFO".green(),
            LogLevel::Warning => "WARN".yellow(),
            LogLevel::Error => "ERRO".red(),
            LogLevel::Critical => "CRIT".magenta(),
        };

        write!(f, "{} [{}] {}", level_str, time_str, msg_str)
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 20)]
async fn main() -> Result<()> {
    dotenv().ok();

    wagmi::wagmi();

    // let test_wallet_private_key: String =
    //     std::env::var("TESTWALLET_PRIVATE_KEY").expect("TESTWALLET_PRIVATE_KEY must be set");

    let localhost_rpc_url: String =
        std::env::var("LOCAL_HOST_URL").expect("LOCAL_HOST_URL must be set");

    let provider: Provider<Ws> = Provider::<Ws>::connect(localhost_rpc_url).await?;
    // let block_number: U64 = provider.get_block_number().await?;
    let gas_price: U256 = provider.get_gas_price().await?;

    println!(
        "{}",
        LogEntry {
            time: Local::now(),
            level: LogLevel::Info,
            message: format!("gas_price {:?}", gas_price),
        }
    );

    // let bundle_signer: LocalWallet = LocalWallet::new(&mut thread_rng());
    // // This signs transactions
    // let wallet: LocalWallet = test_wallet_private_key.parse()?;
    // let wallet_clone = wallet.clone();

    // // Add signer and Flashbots middleware
    // let client = SignerMiddleware::new(
    //     FlashbotsMiddleware::new(
    //         provider,
    //         Url::parse("https://relay-goerli.flashbots.net")?,
    //         bundle_signer,
    //     ),
    //     wallet,
    // );

    // let tx = {
    //     let mut inner: TypedTransaction = TransactionRequest::new()
    //         .from(wallet_clone.address())
    //         .to("0x8C66BA8157808cba80A57a0A29600221973FA29F")
    //         .value(1)
    //         .chain_id(5)
    //         .into();
    //     client.fill_transaction(&mut inner, None).await?;
    //     inner
    // };

    // println!(
    //     "{}",
    //     LogEntry {
    //         time: Local::now(),
    //         level: LogLevel::Info,
    //         message: format!("Transaction: {:?}", tx),
    //     }
    // );

    // let signature = client.signer().sign_transaction(&tx).await?;

    // let bundle = BundleRequest::new()
    //     .push_transaction(tx.rlp_signed(&signature))
    //     .set_block(block_number + 1)
    //     .set_simulation_block(block_number)
    //     .set_simulation_timestamp(0);

    // let simulated_bundle = client.inner().simulate_bundle(&bundle).await?;

    // println!(
    //     "{}",
    //     LogEntry {
    //         time: Local::now(),
    //         level: LogLevel::Info,
    //         message: format!("Simulated bundle: {:?}", simulated_bundle),
    //     }
    // );

    // println!(
    //     "{}",
    //     LogEntry {
    //         time: Local::now(),
    //         level: LogLevel::Info,
    //         message: format!("BN {:?}", block_number),
    //     }
    // );

    // let pending_bundle = client.inner().send_bundle(&bundle).await?;

    // println!(
    //     "{}",
    //     LogEntry {
    //         time: Local::now(),
    //         level: LogLevel::Info,
    //         message: format!("Pending bundle: {:?} BN {:?}", pending_bundle.bundle_hash, pending_bundle.block),
    //     }
    // );

    // match pending_bundle.await {
    //     Ok(bundle_hash) => println!(
    //         "{}",
    //         LogEntry {
    //             time: Local::now(),
    //             level: LogLevel::Info,
    //             message: format!("Bundle was included in block: {}", bundle_hash),
    //         }
    //     ),
    //     Err(PendingBundleError::BundleNotIncluded) => {
    //         println!(
    //             "{}",
    //             LogEntry {
    //                 time: Local::now(),
    //                 level: LogLevel::Error,
    //                 message: "Bundle was not included in any block".to_string(),
    //             }
    //         )
    //     }
    //     Err(e) => {
    //         println!(
    //             "{}",
    //             LogEntry {
    //                 time: Local::now(),
    //                 level: LogLevel::Error,
    //                 message: format!("{:?}", e),
    //             }
    //         )
    //     }
    // }

    Ok(())
}
