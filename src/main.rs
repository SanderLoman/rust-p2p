#![deny(unsafe_code)]

use dotenv::dotenv;
use eyre::Result;
use std::error::Error;

use wagmi::{create_logger, parse_verbosity};

#[tokio::main()]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let verbosity = parse_verbosity();
    let _log = create_logger(verbosity);

    Ok(())
}

// tests/integration_test.rs

#[tokio::test]
async fn test_run() {
    use wagmi::run; // replace `your_crate` with the actual name of your crate

    // Call the `run` function and assert on its result.
    // Here we're just checking that it doesn't return an error.
    assert!(run().await.is_ok());
}
