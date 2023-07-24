#![deny(unsafe_code)]

use discv5::{
    enr::{CombinedKey, EnrBuilder},
    Enr,
};
use reqwest::header::{HeaderMap, ACCEPT};
use reqwest::Client;
use std::error::Error;

async fn get_local_enr() -> Result<Enr, Box<dyn Error>> {
    // start a beacon client for this to work
    // curl -X 'GET' 'http://127.0.0.1:5052/eth/v1/node/identity' -H 'accept: application/json'
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());

    let res = client.get("http://127.0.0.1:5052/eth/v1/node/identity")
        .headers(headers)
        .send()
        .await?;
    
    Ok()
}

pub async fn generate_enr() -> Result<Enr, Box<dyn Error>> {
    let enr_combined_key: CombinedKey = CombinedKey::generate_secp256k1();
    let enr_key: discv5::enr::Enr<CombinedKey> = EnrBuilder::new("v4")
        .add_value("", "")
        .add_value("", "")
        .add_value("", "")
        .build(&enr_combined_key)?;

    Ok(())
}
