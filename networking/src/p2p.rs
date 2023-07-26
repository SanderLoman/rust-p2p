#![deny(unsafe_code)]

use eyre::Result;
use crate::discv5::enr::enr::*;

pub async fn start_p2p_networking() -> Result<()>  {
    println!("Starting p2p networking...");
    generate_enr().await.unwrap();

    Ok(())
}
