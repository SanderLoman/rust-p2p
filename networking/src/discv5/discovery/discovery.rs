#![deny(unsafe_code)]

use crate::create_logger;
use crate::discv5::enr::enr::generate_enr;
use discv5::*;
use discv5::{
    enr, handler, kbucket, metrics, packet, permit_ban, rpc, service, socket, Discv5, Discv5Config,
    Discv5ConfigBuilder, Discv5Event, Enr, ListenConfig,
};
use std::error::Error;

pub async fn setup_discovery_process() -> Result<(), Box<dyn Error>> {
    let log = create_logger();
    let (local_enr, enr, enr_key) = generate_enr().await?;

    let listen_addr = std::net::Ipv4Addr::new(0, 0, 0, 0);
    let listen_port = enr.udp4().unwrap();
    slog::debug!(log, "Listening on: {}", listen_addr);

    let config = Discv5ConfigBuilder::new(ListenConfig::Ipv4 { ip: listen_addr , port: listen_port })
        .build();

    slog::debug!(log, "config: {:?}", config);

    let mut discv5: Discv5 = Discv5::new(enr, enr_key, config).unwrap();

    discv5.add_enr(local_enr).unwrap();

    discv5.table_entries().iter().for_each(|enr| {
        slog::debug!(log, "{:?}", enr);
    });

    discv5.start().await.unwrap();

    Ok(())
}
