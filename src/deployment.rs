use ethers::prelude::*;
use dotenv::dotenv;

pub fn deployment() {
    println!("Deploying contract and auto-destructing after completed transaction!\n");
    dotenv().ok();
    let wallet: String = std::env::var("TESTWALLET_PRIVATE_KEY").expect("TESTWALLET_PRIVATE_KEY must be set");
    let wallet = wallet.parse::<LocalWallet>().unwrap();
    println!("Wallet: {:?}\n", wallet.address());
}