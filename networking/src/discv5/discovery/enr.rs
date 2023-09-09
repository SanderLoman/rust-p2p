#![deny(unsafe_code)]

use discv5::enr::{CombinedKey, Enr, EnrBuilder};
use reqwest::header::{HeaderMap, ACCEPT};
use reqwest::Client;
use serde_json::Value;
use std::error::Error;
use std::net::Ipv4Addr;
use std::str::FromStr;

pub async fn generate_enr(
) -> Result<(Enr<CombinedKey>, Enr<CombinedKey>, CombinedKey), Box<dyn Error>> {
    let enr_combined_key: CombinedKey = CombinedKey::generate_secp256k1();
    let (local_enr, attnets, eth2, syncnets, ip4) = get_local_enr().await?;

    let port = 7777;

    let enr: discv5::enr::Enr<CombinedKey> = EnrBuilder::new("v4")
        .ip4(ip4)
        .tcp4(port)
        .udp4(port)
        .add_value("attnets", &attnets)
        .add_value("eth2", &eth2)
        .add_value("syncnets", &syncnets)
        .build(&enr_combined_key)?;

    // Decode the ENR
    let decoded_generated_enr: Enr<CombinedKey> = Enr::from_str(&enr.to_base64()).unwrap();

    let local_enr = Enr::from_str(&local_enr)?;

    Ok((local_enr, enr, enr_combined_key))
}

async fn get_local_enr() -> Result<(String, Vec<u8>, Vec<u8>, Vec<u8>, Ipv4Addr), Box<dyn Error>> {
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());

    let res = client
        .get("http://127.0.0.1:5052/eth/v1/node/identity")
        .headers(headers)
        .send()
        .await?;

    let body = res.text().await?;
    let v: Value = serde_json::from_str(&body)?;
    let enr = v["data"]["enr"].as_str().unwrap().to_string();

    // Decode the ENR
    let decoded_enr: Enr<CombinedKey> = Enr::from_str(&enr)?;

    // Extract the attnets, eth2, and syncnets fields
    let attnets = decoded_enr.get("attnets").unwrap().clone();
    let eth2 = decoded_enr.get("eth2").unwrap().clone();
    let syncnets = decoded_enr.get("syncnets").unwrap().clone();
    let ip4 = decoded_enr
        .ip4()
        .unwrap_or_else(|| Ipv4Addr::new(83, 128, 37, 242));

    Ok((enr, attnets.to_vec(), eth2.to_vec(), syncnets.to_vec(), ip4))
}
