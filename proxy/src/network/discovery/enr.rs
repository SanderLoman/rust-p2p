//! Helper functions and an extension trait for Ethereum 2 ENRs.

pub use discv5::enr::{self, CombinedKey, EnrBuilder};
use discv5::{enr::EnrKey, Enr};
use reqwest::{
    header::{HeaderMap, ACCEPT},
    Client,
};
use serde_json::Value;
use slog::{debug, warn};

use std::{error::Error, str::FromStr};

use crate::network::config::Config as NetworkConfig;

use super::enr_ext::{QUIC6_ENR_KEY, QUIC_ENR_KEY};

/// The ENR field specifying the fork id.
pub const ETH2_ENR_KEY: &str = "eth2";
/// The ENR field specifying the attestation subnet bitfield.
pub const ATTESTATION_BITFIELD_ENR_KEY: &str = "attnets";
/// The ENR field specifying the sync committee subnet bitfield.
pub const SYNC_COMMITTEE_BITFIELD_ENR_KEY: &str = "syncnets";

/// Builds a lighthouse ENR given a `NetworkConfig`.
pub async fn build_enr(
    enr_key: &CombinedKey,
    config: &NetworkConfig,
) -> Result<Enr, Box<dyn Error>> {
    let mut builder: EnrBuilder<CombinedKey> = create_enr_builder_from_config(config);
    let (eth2, attnets, syncnets) = get_values().await.unwrap();

    // set the `eth2` field on our ENR
    builder.add_value(ETH2_ENR_KEY, &eth2);

    // set the "attnets" field on our ENR
    builder.add_value(ATTESTATION_BITFIELD_ENR_KEY, &attnets);

    // set the "syncnets" field on our ENR
    builder.add_value(SYNC_COMMITTEE_BITFIELD_ENR_KEY, &syncnets);

    Ok(builder.build(enr_key).unwrap())
}

pub fn create_enr_builder_from_config<T: EnrKey>(config: &NetworkConfig) -> EnrBuilder<T> {
    let mut builder = EnrBuilder::new("v4");
    let (maybe_ipv4_address, maybe_ipv6_address) = &config.enr_address;

    if let Some(ip) = maybe_ipv4_address {
        builder.ip4(*ip);
    }

    if let Some(ip) = maybe_ipv6_address {
        builder.ip6(*ip);
    }

    if let Some(udp4_port) = config.enr_udp4_port {
        builder.udp4(udp4_port);
    }

    if let Some(udp6_port) = config.enr_udp6_port {
        builder.udp6(udp6_port);
    }

    // If we are listening on ipv4, add the quic ipv4 port.
    if let Some(quic4_port) = config
        .enr_quic4_port
        .or_else(|| config.listen_addrs().v4().map(|v4_addr| v4_addr.quic_port))
    {
        builder.add_value(QUIC_ENR_KEY, &quic4_port);
    }

    // If we are listening on ipv6, add the quic ipv6 port.
    if let Some(quic6_port) = config
        .enr_quic6_port
        .or_else(|| config.listen_addrs().v6().map(|v6_addr| v6_addr.quic_port))
    {
        builder.add_value(QUIC6_ENR_KEY, &quic6_port);
    }

    // If the ENR port is not set, and we are listening over that ip version, use the listening port instead.
    let tcp4_port = config
        .enr_tcp4_port
        .or_else(|| config.listen_addrs().v4().map(|v4_addr| v4_addr.tcp_port));
    if let Some(tcp4_port) = tcp4_port {
        builder.tcp4(tcp4_port);
    }

    let tcp6_port = config
        .enr_tcp6_port
        .or_else(|| config.listen_addrs().v6().map(|v6_addr| v6_addr.tcp_port));
    if let Some(tcp6_port) = tcp6_port {
        builder.tcp6(tcp6_port);
    }

    builder
}

async fn get_values() -> Result<(String, String, String), Box<dyn Error>> {
    // Initialize HTTP client
    let client = Client::new();

    // Set up headers for the request
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());

    // Make the request to the local node
    let res = client
        .get("http://127.0.0.1:5052/eth/v1/node/identity")
        .headers(headers)
        .send()
        .await?;

    // Parse the response body
    let body = res.text().await?;
    let v: Value = serde_json::from_str(&body)?;
    let enr = v["data"]["enr"].as_str().unwrap().to_string();

    let enr: Enr = Enr::from_str(&enr).unwrap();

    let eth2 = std::str::from_utf8(enr.get("eth2").unwrap())?.to_string();
    let attnets = std::str::from_utf8(enr.get("attnets").unwrap())?.to_string();
    let syncnets = std::str::from_utf8(enr.get("syncnets").unwrap())?.to_string();

    Ok((eth2, attnets, syncnets))
}
