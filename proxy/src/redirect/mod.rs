use libp2p::Multiaddr;
use slog::{debug, Logger};
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tokio::sync::Mutex;

use crate::get_lh_tcp_multiaddr;

use crate::{REAL_BEACON_NODE_IP_ADDR, REAL_BEACON_NODE_MULTIADDR};

pub async fn log_statics(log: Logger) {
    get_lh_tcp_multiaddr().await.unwrap();

    let ip_addr_storage = REAL_BEACON_NODE_IP_ADDR.lock().unwrap();
    debug!(log, "REAL_BEACON_NODE_IP_ADDR: {:?}", *ip_addr_storage);

    let multiaddr_storage = REAL_BEACON_NODE_MULTIADDR.lock().unwrap();
    debug!(log, "REAL_BEACON_NODE_MULTIADDR: {:?}", *multiaddr_storage);
}
