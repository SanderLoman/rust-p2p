#![deny(unsafe_code)]

use std::error::Error;

#[tokio::main()]
async fn main() -> Result<(), Box<dyn Error>> {
    if std::env::var("RUST_BACKTRACE").is_err() {
        std::env::set_var("RUST_BACKTRACE", "1");
    }

    Ok(())
}
