pub mod router;

use std::{error::Error, net::IpAddr};

use discv5::Enr;
use slog::{debug, Logger};

// use crate::network::network_manager;

#[derive(Debug)]
pub struct Redirect {
    ip: IpAddr,
    port: u16,
    enr: Enr,
    // network_manager: NetworkManager,
    log: Logger,
}

impl Redirect {
    pub async fn new(ip: IpAddr, port: u16, enr: Enr, log: Logger) -> Self {
        // let network_manager = NetworkManager::new(log.clone()).await;
        Self {
            ip,
            port,
            enr,
            // network_manager,
            log,
        }
    }
}

pub fn proxy() -> Result<(), Box<dyn Error>> {
    Ok(())
}
