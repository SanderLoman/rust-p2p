use std::{error::Error, net::IpAddr};

use discv5::Enr;
use slog::{debug, Logger};

use hyper::{Body, Request, Response};

use crate::network_manager::NetworkManager;

pub trait NetworkRequests {}

#[derive(Debug)]
pub struct Redirect<N: NetworkRequests> {
    ip: IpAddr,
    port: u16,
    enr: Enr,
    network_manager: NetworkManager<N>,
    log: Logger,
}

impl<N: NetworkRequests + std::fmt::Debug> Redirect<N> {
    pub fn new(
        ip: IpAddr,
        port: u16,
        enr: Enr,
        log: Logger,
    ) -> Self {
        let network_manager = NetworkManager::new(log.clone());
        Self {
            ip,
            port,
            enr,
            network_manager,
            log,
        }
    }

    pub fn redirect(&self, req: Request<N>) -> Response<N> {
        todo!("redirect")
    }
}

pub fn proxy() -> Result<(), Box<dyn Error>> {
    Ok(())
}
