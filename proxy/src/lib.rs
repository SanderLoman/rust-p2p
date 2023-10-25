/// Enabled for now.
pub mod network;
pub mod network_manager;
/// Disabled for now, I wanna test the redirect first
// pub mod SSZ;
pub mod redirect;

use git_version::git_version;
use lazy_static::lazy_static;
use libp2p::Multiaddr;
use network_manager::NetworkManager;
use reqwest::header::{HeaderMap, ACCEPT};
use reqwest::Client;
use serde_json::Value;
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Mutex;
use target_info::Target;

lazy_static! {
    // The IP address of the real beacon node
    pub static ref REAL_BEACON_NODE_IP_ADDR: Mutex<Option<SocketAddr>> = Mutex::new(None);
    // The TCP multiaddr of the real beacon node
    pub static ref REAL_BEACON_NODE_MULTIADDR: Mutex<Option<Multiaddr>> = Mutex::new(None);
}

pub const VERSION: &str = git_version!(
    args = [
        "--always",
        "--dirty=+",
        "--abbrev=7",
        // NOTE: using --match instead of --exclude for compatibility with old Git
        "--match=thiswillnevermatchlol"
    ],
    prefix = "/v1.0.0-",
    fallback = "ConTower/v1.0.0" 
);

/// Returns `VERSION`, but with platform information appended to the end.
///
/// ## Example
///
/// `ConTower/v1.5.1-67da032+/x86_64-linux`
pub fn version_with_platform() -> String {
    format!("{}/{}-{}", VERSION, Target::arch(), Target::os())
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

    let addr_str = v["data"]["p2p_addresses"][0]
        .as_str()
        .ok_or("Failed to extract address as a string")?;

    let multiaddr = addr_str
        .parse::<Multiaddr>()
        .map_err(|e| format!("Failed to parse address into Multiaddr: {}", e))?;

    let mut multiaddr_storage = REAL_BEACON_NODE_MULTIADDR.lock().unwrap();
    *multiaddr_storage = Some(multiaddr.clone());

    let mut ip: Option<std::net::IpAddr> = None;
    let mut port: Option<u16> = None;

    for p in multiaddr.iter() {
        match p {
            libp2p::core::multiaddr::Protocol::Ip4(ip4) => ip = Some(ip4.into()),
            libp2p::core::multiaddr::Protocol::Ip6(ip6) => ip = Some(ip6.into()),
            libp2p::core::multiaddr::Protocol::Tcp(p) => port = Some(p),
            _ => {}
        }
    }

    if let (Some(ip), Some(port)) = (ip, port) {
        let socket_addr = SocketAddr::new(ip, port);
        let mut ip_addr_storage = REAL_BEACON_NODE_IP_ADDR.lock().unwrap();
        *ip_addr_storage = Some(socket_addr);
    }

    Ok(())
}
