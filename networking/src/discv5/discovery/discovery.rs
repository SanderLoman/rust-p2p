#![deny(unsafe_code)]
#![allow(unused_imports)]

use crate::create_logger;
use crate::discv5::enr::enr::generate_enr;
use discv5::*;
use discv5::{
    enr, handler, kbucket, metrics, packet, permit_ban, rpc, service, socket, Discv5, Discv5Config,
    Discv5ConfigBuilder, Discv5Event, Enr, ListenConfig,
};
use std::error::Error;
use std::net::Ipv4Addr;
use std::time::Duration;

pub async fn setup_discv5() -> Result<(), Box<dyn Error>> {
    let log = create_logger();
    let (local_enr, enr, enr_key) = generate_enr().await?;

    let listen_addr = std::net::Ipv4Addr::new(0, 0, 0, 0);
    let listen_port = enr.udp4().unwrap();
    slog::debug!(log, "Listening on: {}", listen_addr);

    let discv5_listen_config =
        discv5::ListenConfig::from_ip(Ipv4Addr::UNSPECIFIED.into(), listen_port);

    let discv5_config = Discv5ConfigBuilder::new(discv5_listen_config).build();

    slog::debug!(log, "config: {:?}", discv5_config);

    let mut discv5: Discv5 = Discv5::new(enr, enr_key, discv5_config).unwrap();

    discv5.start().await.unwrap();

    Ok(())
}
