#![deny(unsafe_code)]

use dotenv::dotenv;
use eyre::Result;
use std::error::Error;

use wagmi::{create_logger, parse_verbosity};

// use proxy::network::discovery::Discovery;

use proxy::network::discovery::enr::generate_enr;

use proxy::generate_libp2p_keypair;
use proxy::network::Network;

use proxy::version_with_platform;

#[tokio::main()]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let verbosity = parse_verbosity();
    let log = create_logger(verbosity);

    let test = version_with_platform();
    let local_keypair = generate_libp2p_keypair();

    generate_enr(local_keypair.clone(), log.clone()).await;


    Network::new(local_keypair, log).await?;

    println!("{}", test);

    Ok(())
}
