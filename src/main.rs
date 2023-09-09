#![deny(unsafe_code)]

use clap::{App, Arg};
use dotenv::dotenv;
use eyre::Result;
use std::error::Error;

use networking::p2p::P2PNetwork;
use wagmi::create_logger;

#[tokio::main()]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    // Use clap for command-line argument parsing
    let matches = App::new("wagmi")
        .version("69.69.69")
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    // Get verbosity level
    let verbosity = matches.occurrences_of("v");

    // Initialize the logger
    let log = create_logger(verbosity);

    P2PNetwork::new(log).await?;

    Ok(())
}
