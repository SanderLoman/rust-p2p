#![deny(unsafe_code)]

use dotenv::dotenv;
use eyre::Result;
use std::error::Error;

use wagmi::{create_logger, parse_verbosity};

// use proxy::network::discovery::Discovery;

use proxy::network::discovery::enr::generate_enr;

use proxy::version_with_platform;

#[tokio::main()]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let verbosity = parse_verbosity();
    let log = create_logger(verbosity);

    // Discovery::new(log.clone()).await?;

    let enr = generate_enr(log.clone()).await?;

    let test = version_with_platform();

    println!("{}", test);

    Ok(())
}
