#![deny(unsafe_code)]

use discv5::{enr, Enr, enr::CombinedKey};
use std::error::Error;

async fn get_local_enr() -> Result<Enr, Box<dyn Error>> {

    
    Ok(enr_key)
}

pub fn generate_enr() -> Result<Enr, Box<dyn Error>> {
    let enr_combined_key = enr::CombinedKey::generate_secp256k1();
    let enr_key = enr::EnrBuilder::new("v4").build(&enr_combined_key).unwrap();

    Ok(enr_key)
}