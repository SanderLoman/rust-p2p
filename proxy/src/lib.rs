pub mod network_manager;
/// Disabled for now, I wanna test the redirect first
// pub mod network;
/// Disabled for now, I wanna test the redirect first
// pub mod SSZ;
pub mod redirect;

use lazy_static::lazy_static;
use libp2p::Multiaddr;
use reqwest::header::{HeaderMap, ACCEPT};
use reqwest::Client;
use serde_json::Value;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Mutex;

lazy_static! {
    // The IP address of the real beacon node
    static ref REAL_BEACON_NODE_IP_ADDR: Mutex<Option<SocketAddr>> = Mutex::new(None);
    // The TCP multiaddr of the real beacon node
    static ref REAL_BEACON_NODE_MULTIADDR: Mutex<Option<Multiaddr>> = Mutex::new(None);
}

pub async fn get_lh_tcp_multiaddr() -> Result<(), Box<dyn Error>> {
    let url = "http://127.0.0.1:5052/eth/v1/node/identity";

    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse().unwrap());

    let res = client
        .get(url)
        .headers(headers)
        .send()
        .await?
        .text()
        .await?;

    let v: Value = serde_json::from_str(&res)?;

    // Assuming p2p_addresses is an array, extract the first address
    let addr_str = v["data"]["p2p_addresses"]
        .as_str()
        .ok_or("Failed to extract address as a string")?;

    // Now parse the address string into a Multiaddr
    let multiaddr = addr_str
        .parse::<Multiaddr>()
        .map_err(|e| format!("Failed to parse address into Multiaddr: {}", e))?;

    // Store the multiaddr in the static variable
    let mut multiaddr_storage = REAL_BEACON_NODE_MULTIADDR.lock().unwrap();
    *multiaddr_storage = Some(multiaddr.clone());

    // Assuming the IP address can be extracted from the multiaddr
    let ip_addr = multiaddr
        .iter()
        .find_map(|p: libp2p::multiaddr::Protocol<'_>| match p {
            libp2p::core::multiaddr::Protocol::Ip4(ip) => Some(SocketAddr::new(ip.into(), 0)), // Assuming port 0, adjust as needed
            libp2p::core::multiaddr::Protocol::Ip6(ip) => Some(SocketAddr::new(ip.into(), 0)), // Assuming port 0, adjust as needed
            _ => None,
        });
    // None = null = false
    if let Some(ip_addr) = ip_addr {
        let mut ip_addr_storage = REAL_BEACON_NODE_IP_ADDR.lock().unwrap();
        *ip_addr_storage = Some(ip_addr);
    }

    Ok(())
}
