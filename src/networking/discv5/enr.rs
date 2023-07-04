#![deny(unsafe_code)]

use crate::nat::*;
use discv5::{
    enr,
    enr::{CombinedKey, EnrBuilder},
    Enr,
};
use eyre::Result;
use igd::{self, PortMappingProtocol};
use reqwest::header::{HeaderMap, ACCEPT};
use serde_json::Value;
use slog::*;
use std::error::Error;
use std::net::SocketAddrV4;
use std::str::FromStr;

pub async fn get_local_peer_info(
) -> Result<(String, String, String, String, String, String), Box<dyn Error>> {
    let url = "http://127.0.0.1:5052/eth/v1/node/identity";
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());
    let res = client.get(url).headers(headers).send().await?;
    let body = res.text().await?;
    let json: Value = serde_json::from_str(&body)?;
    let peer_id = json["data"]["peer_id"]
        .as_str()
        .ok_or("Peer ID not found")?
        .to_owned();
    let enr = json["data"]["enr"]
        .as_str()
        .ok_or("ENR not found")?
        .to_owned();
    let p2p_address = json["data"]["p2p_addresses"][0]
        .as_str()
        .ok_or("P2P address not found")?
        .to_owned();
    let discovery_address = json["data"]["discovery_addresses"][0]
        .as_str()
        .ok_or("Discovery address not found")?
        .to_owned();
    let attnets = json["data"]["metadata"]["attnets"]
        .as_str()
        .ok_or("attnets not found")?
        .to_owned();
    let syncnets = json["data"]["metadata"]["syncnets"]
        .as_str()
        .ok_or("syncnets not found")?
        .to_owned();
    Ok((
        peer_id,
        enr,
        p2p_address,
        discovery_address,
        attnets,
        syncnets,
    ))
}

pub async fn decode_hex_value(hex_string: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let bytes =
        hex::decode(&hex_string.replace("0x", "")).map_err(|_| "Failed to parse hex string")?;
    Ok(bytes)
}

pub async fn get_eth2_value(enr_string: &str) -> Option<String> {
    if let Some(start) = enr_string.find("\"eth2\", \"") {
        let rest = &enr_string[start + 9..];
        if let Some(end) = rest.find("\")") {
            return Some(rest[..end].to_string());
        }
    }
    None
}

pub async fn generate_enr() -> Result<(Enr, CombinedKey), Box<dyn Error>> {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, slog::o!());

    let (_, enr, _, _, attnets, syncnets) = get_local_peer_info().await?;

    let decoded_enr: enr::Enr<CombinedKey> = Enr::from_str(&enr)?;

    info!(log, "LIGHTHOUSE ENR: {:?}", decoded_enr);
    info!(log, "LIGHTHOUSE ENR: {}", decoded_enr);

    let ip4 = "0.0.0.0".parse::<std::net::Ipv4Addr>().unwrap();
    let port: u16 = 7777;

    // Create an instance of the Nat struct
    info!(log, "Creating NAT instance...");
    let nat = Nat::new().await?;
    info!(log, "NAT instance created");

    // Get the public IP of the gateway
    info!(log, "Sourcing public IP from gateway...");
    let ip = nat.get_public_ip()?;
    info!(log, "Public IP: {:?}", ip);

    // Add port mappings
    // Assuming local_addr is the local address of your node
    let local_addr = SocketAddrV4::new(ip4, port);
    nat.add_port_mapping(
        PortMappingProtocol::TCP,
        port,
        local_addr,
        3600,
        "Ethereum Beacon Node",
    )?;
    nat.add_port_mapping(
        PortMappingProtocol::UDP,
        port,
        local_addr,
        3600,
        "Ethereum Beacon Node",
    )?;

    let syncnets_bytes = decode_hex_value(&syncnets).await?;
    let attnets_bytes = decode_hex_value(&attnets).await?;

    let decoded_enr: enr::Enr<CombinedKey> = Enr::from_str(&enr)?;

    let enr_string = format!("{:?}", decoded_enr);
    let eth2_value = get_eth2_value(&enr_string).await;

    // If eth2_value is None, return early
    let eth2_value = match eth2_value {
        Some(value) => value,
        None => return Err("Failed to get eth2 value from ENR".into()),
    };

    let eth2_bytes = decode_hex_value(&eth2_value).await?;

    let combined_key = CombinedKey::generate_secp256k1();

    let enr = EnrBuilder::new("v4")
        .ip4(ip4)
        .tcp4(port)
        .udp4(port)
        .add_value("syncnets", &syncnets_bytes)
        .add_value("attnets", &attnets_bytes)
        .add_value_rlp("eth2", eth2_bytes.into())
        .build(&combined_key)
        .map_err(|_| "Failed to generate ENR")?;

    info!(log, "SELF ENR: {:?}\n", enr);
    info!(log, "SELF ENR: {}\n", enr);

    Ok((enr, combined_key))
}
