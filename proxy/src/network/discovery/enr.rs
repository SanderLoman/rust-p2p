#![deny(unsafe_code)]

use discv5::enr::{CombinedKey, Enr, EnrBuilder};
use libp2p::identity::Keypair;
use reqwest::header::{HeaderMap, ACCEPT};
use reqwest::Client;
use serde_json::Value;
use slog::{debug, info, Logger};
use std::error::Error;
use std::net::Ipv4Addr;
use std::str::FromStr;

use crate::generate_discv5_keypair;
use crate::network::types::network_globals::CombinedKeyExt;

/// Generates an Ethereum Node Record (ENR) based on local settings and data fetched from another local node.
///
/// # Returns
///
/// Returns a tuple containing the local ENR, the generated ENR, and the combined key used for the ENR.
pub async fn generate_enr(enr_combined_key: Keypair, log: Logger) -> Enr<CombinedKey> {

    let combined_key = CombinedKey::from_libp2p(enr_combined_key).unwrap();
    
    // Fetch the local ENR and associated data
    let (attnets, eth2, syncnets, quic, ip4) = get_local_enr(log.clone()).await.unwrap();

    let port = 7777;

    // Build the ENR
    let enr: discv5::enr::Enr<CombinedKey> = EnrBuilder::new("v4")
        .ip4(ip4)
        .tcp4(port)
        .udp4(port)
        .add_value("attnets", &attnets)
        .add_value("eth2", &eth2)
        .add_value("syncnets", &syncnets)
        .add_value("quic", &quic)
        .build(&combined_key)
        .unwrap();

    info!(log, "ENR generated"; "enr" => ?enr.to_base64());

    // Decode the generated ENR for verification
    let decoded_generated_enr: Enr<CombinedKey> = Enr::from_str(&enr.to_base64()).unwrap();
    // .map_err(|e| {
    //     error!(log, "Failed to decode generated ENR"; "error" => ?e);
    //     e
    // });

    debug!(log, "ENR generated (decoded)"; "enr" => ?decoded_generated_enr);

    // let lh_enr = Enr::from_str(&lh_enr)?;

    info!(log, "enr"; "enr" => ?enr);

    enr
}

/// Fetches the local ENR and associated data like attnets, eth2, and syncnets fields.
///
/// # Returns
///
/// Returns a tuple containing the local ENR as a string, attnets, eth2, syncnets as byte vectors, and the IP address.
async fn get_local_enr(
    log: Logger,
) -> Result<(Vec<u8>, Vec<u8>, Vec<u8>, Option<Vec<u8>>, Ipv4Addr), Box<dyn Error>> {
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

    debug!(log, "Lighthouse ENR"; "enr" => ?enr);

    // Decode the ENR to extract relevant fields
    let decoded_enr: Enr<CombinedKey> = Enr::from_str(&enr)?;

    debug!(log, "Lighthouse ENR"; "enr" => ?decoded_enr);

    let attnets = decoded_enr.get("attnets").unwrap().to_vec().clone();
    let eth2 = decoded_enr.get("eth2").unwrap().to_vec().clone();
    let syncnets = decoded_enr.get("syncnets").unwrap().to_vec().clone();
    let ip4 = decoded_enr
        .ip4()
        .unwrap_or_else(|| Ipv4Addr::new(83, 128, 37, 242));

    let quic = if let Some(quic_field) = decoded_enr.get("quic") {
        Some(quic_field.to_vec().clone())
    } else {
        None
    };

    Ok((attnets, eth2, syncnets, quic, ip4))
}
