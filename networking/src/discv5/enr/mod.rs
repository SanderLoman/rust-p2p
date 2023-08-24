pub(crate) mod enr;

use crate::create_logger;
use discv5::enr::{CombinedKey, Enr, EnrBuilder};
use reqwest::header::{HeaderMap, ACCEPT};
use reqwest::Client;
use serde_json::Value;
use libp2p::{Multiaddr, PeerId};
use std::error::Error;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;

pub async fn generate_enr(
) -> Result<(Enr<CombinedKey>, Enr<CombinedKey>, CombinedKey), Box<dyn Error>> {
    let log = create_logger();

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
    let decoded_generated_enr = Enr::from_str(&enr.to_base64()).unwrap();

    slog::info!(log, "Generated ENR"; "enr" => %enr);
    slog::info!(log, "Decoded Generated ENR"; "decoded_generated_enr" => ?decoded_generated_enr);

    let local_enr = Enr::from_str(&local_enr)?;

    Ok((local_enr, enr, enr_combined_key))
}

async fn get_local_enr() -> Result<(String, Vec<u8>, Vec<u8>, Vec<u8>, Ipv4Addr), Box<dyn Error>> {
    let log = create_logger();

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
    let decoded_enr = Enr::from_str(&enr)?;

    // Extract the attnets, eth2, and syncnets fields
    let attnets = decoded_enr.get("attnets").unwrap().clone();
    let eth2 = decoded_enr.get("eth2").unwrap().clone();
    let syncnets = decoded_enr.get("syncnets").unwrap().clone();
    let ip4 = decoded_enr
        .ip4()
        .unwrap_or_else(|| Ipv4Addr::new(83, 128, 37, 242));

    slog::info!(log, "Local ENR"; "enr" => %enr);
    slog::info!(log, "Local Decoded ENR"; "decoded_enr" => ?decoded_enr);

    Ok((enr, attnets.to_vec(), eth2.to_vec(), syncnets.to_vec(), ip4))
}

// helper function to decode the ENR
pub fn decode_enr(enr: &str) -> Result<Enr<CombinedKey>, Box<dyn Error>> {
    let decoded_enr = Enr::from_str(&enr)?;
    Ok(decoded_enr)
}

// helper function to extract everything from the ENR
pub fn extract_all(enr: &Enr<CombinedKey>) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>, Ipv4Addr, Ipv6Addr, u16, u16, u16, u16, PeerId), Box<dyn Error>> {
    let attnets = enr.get("attnets").unwrap().clone();
    let eth2 = enr.get("eth2").unwrap().clone();
    let syncnets = enr.get("syncnets").unwrap().clone();
    let ip4 = enr.ip4().unwrap();
    let ip6 = enr.ip6().unwrap();
    let tcp4 = enr.tcp4().unwrap();
    let tcp6 = enr.tcp6().unwrap();
    let udp4 = enr.udp4().unwrap();
    let udp6 = enr.udp6();
    let p2p = enr.id();
    Ok((attnets.to_vec(), eth2.to_vec(), syncnets.to_vec(), ip4, ip6, tcp4, tcp6, udp4, udp6, p2p))
}

// helper function to extract the attnets, eth2, and syncnets fields
pub fn extract_fields(enr: &Enr<CombinedKey>) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>), Box<dyn Error>> {
    let attnets = enr.get("attnets").unwrap().clone();
    let eth2 = enr.get("eth2").unwrap().clone();
    let syncnets = enr.get("syncnets").unwrap().clone();
    Ok((attnets.to_vec(), eth2.to_vec(), syncnets.to_vec()))
}

// helper function to extract the ip4 field
pub fn extract_ip4(enr: &Enr<CombinedKey>) -> Result<Ipv4Addr, Box<dyn Error>> {
    let ip4 = enr.ip4().unwrap();
    Ok(ip4)
}

// helper function to extract the ip6 field
pub fn extract_ip6(enr: &Enr<CombinedKey>) -> Result<Ipv6Addr, Box<dyn Error>> {
    let ip6 = enr.ip6().unwrap();
    Ok(ip6)
}

// helper function to extract the tcp4 field
pub fn extract_tcp4(enr: &Enr<CombinedKey>) -> Result<u16, Box<dyn Error>> {
    let tcp4 = enr.tcp4().unwrap();
    Ok(tcp4)
}

// helper function to extract the tcp6 field
pub fn extract_tcp6(enr: &Enr<CombinedKey>) -> Result<u16, Box<dyn Error>> {
    let tcp6 = enr.tcp6().unwrap();
    Ok(tcp6)
}

// helper function to extract the udp4 field
pub fn extract_udp4(enr: &Enr<CombinedKey>) -> Result<u16, Box<dyn Error>> {
    let udp4 = enr.udp4().unwrap();
    Ok(udp4)
}

// helper function to extract the udp6 field
pub fn extract_udp6(enr: &Enr<CombinedKey>) -> Result<u16, Box<dyn Error>> {
    let udp6 = enr.udp6().unwrap();
    Ok(udp6)
}

// helper function to extract the p2p field
pub fn extract_p2p(enr: &Enr<CombinedKey>) -> Result<PeerId, Box<dyn Error>> {
    let p2p = enr.id().unwrap();
    Ok(p2p)
}
