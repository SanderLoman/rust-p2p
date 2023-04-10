#![deny(unsafe_code)]

use chrono::{DateTime, Local};
use colored::*;
use dotenv::dotenv;
use ethers::core::{rand::thread_rng, types::transaction::eip2718::TypedTransaction};
use ethers::prelude::*;
use ethers_flashbots::*;
use eyre::Result;
use std::fmt;
use url::Url;

#[derive(Debug)]
struct LogEntry {
    time: DateTime<Local>,
    level: LogLevel,
    message: String,
}

#[derive(Debug)]
enum LogLevel {
    Info,
    Warning,
    Error,
    Critical,
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let time_str = format!("{}", self.time.format("%m-%d | %H:%M:%S%.3f"));
        let msg_str = self.message.as_str();

        let level_str = match self.level {
            LogLevel::Info => "[INFO]".green().bold(),
            LogLevel::Warning => "[WARN]".yellow().bold(),
            LogLevel::Error => "[ERRO]".red().bold(),
            LogLevel::Critical => "[CRIT]".magenta().bold(),
        };

        write!(f, "{} [{}] {}", level_str, time_str, msg_str)
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 20)]
async fn main() -> Result<()> {
    dotenv().ok();

    let now: DateTime<Local> = Local::now();

    let test_wallet_private_key: String =
        std::env::var("TESTWALLET_PRIVATE_KEY").expect("TESTWALLET_PRIVATE_KEY must be set");

    let localhost_rpc_url: String =
        std::env::var("LOCALHOST_WS_URL").expect("LOCALHOST_WS_URL must be set");

    let provider: Provider<Ws> = Provider::<Ws>::connect(localhost_rpc_url).await?;
    let block_number: U64 = provider.get_block_number().await?;
    let gas_price: U256 = provider.get_gas_price().await?;

    let info_entry = LogEntry {
        time: now,
        level: LogLevel::Info,
        message: format!("BN: {:?}", block_number),
    };

    let warning_entry = LogEntry {
        time: now,
        level: LogLevel::Warning,
        message: format!("BN: {:?}", gas_price),
    };

    let error_entry = LogEntry {
        time: now,
        level: LogLevel::Error,
        message: format!("BN: {:?}", block_number),
    };

    let critical_entry = LogEntry {
        time: now,
        level: LogLevel::Critical,
        message: format!("BN: {:?}", gas_price),
    };

    println!("{}", info_entry);
    println!("{}", warning_entry);
    println!("{}", error_entry);
    println!("{}", critical_entry);

    let bundle_signer: LocalWallet = LocalWallet::new(&mut thread_rng());
    // This signs transactions
    let wallet: LocalWallet = test_wallet_private_key.parse()?;

    // Add signer and Flashbots middleware
    let client = SignerMiddleware::new(
        FlashbotsMiddleware::new(
            provider,
            Url::parse("https://relay.flashbots.net")?,
            bundle_signer,
        ),
        wallet,
    );

    // let tx = {
    //     let mut inner: TypedTransaction = TransactionRequest::new()
    //         .to("0x8C66BA8157808cba80A57a0A29600221973FA29F")
    //         .value(1)
    //         .gas(gas_p)
    //         .into();
    //     client.fill_transaction(&mut inner, None).await?;
    //     inner
    // };

    // let signature = client.signer().sign_transaction(&tx).await?;
    // let bundle = BundleRequest::new()
    //     .push_transaction(tx.rlp_signed(&signature))
    //     .set_block(block_n + 1)
    //     .set_simulation_block(block_n)
    //     .set_simulation_timestamp(0);

    // // Simulate it
    // let simulated_bundle = client.inner().simulate_bundle(&bundle).await?;
    // println!("Simulated bundle: {:?}", simulated_bundle);

    // // Send it
    // let pending_bundle = client.inner().send_bundle(&bundle).await?;

    // // You can also optionally wait to see if the bundle was included
    // match pending_bundle.await {
    //     Ok(bundle_hash) => println!(
    //         "Bundle with hash {:?} was included in target block",
    //         bundle_hash
    //     ),
    //     Err(PendingBundleError::BundleNotIncluded) => {
    //         println!("Bundle was not included in target block.")
    //     }
    //     Err(e) => println!("An error occured: {}", e),
    // }

    Ok(())
}
