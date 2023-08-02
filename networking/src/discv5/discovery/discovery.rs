#![deny(unsafe_code)]

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

pub async fn start_discv5() -> Result<Discv5, Box<dyn Error>> {
    let log = create_logger();
    let (local_enr, enr, enr_key) = generate_enr().await?;

    let listen_addr = std::net::Ipv4Addr::new(0, 0, 0, 0);
    let listen_port = enr.udp4().unwrap();
    slog::debug!(log, "Listening on: {}", listen_addr);

    let discv5_listen_config =
        discv5::ListenConfig::from_ip(Ipv4Addr::UNSPECIFIED.into(), listen_port);
    slog::debug!(log, "discv5_listen_config: {:?}", discv5_listen_config);

    let discv5_config = Discv5ConfigBuilder::new(discv5_listen_config)
        .ban_duration(Some(Duration::from_secs(60)))
        .query_timeout(Duration::from_secs(10))
        .request_timeout(Duration::from_secs(10))
        .query_parallelism(3)
        .query_peer_timeout(Duration::from_secs(3))
        .ping_interval(Duration::from_secs(300))
        .build();

    slog::debug!(log, "config: {:?}", discv5_config);

    let cloned_enr = enr.clone();
    let mut discv5: Discv5 = Discv5::new(enr, enr_key, discv5_config).unwrap();

    let cloned_local_enr = local_enr.clone();
    discv5.start().await.unwrap();
    discv5.send_ping(cloned_enr).await.unwrap();
    discv5.kbuckets();


    let mut discv_events = discv5.event_stream().await.unwrap();

    loop {
        match discv_events.recv().await {
            Some(Discv5Event::SocketUpdated(socket_addr)) => {
                slog::debug!(log, "Socket Updated: {:?}", socket_addr);
            }
            Some(Discv5Event::Discovered(enr)) => {
                slog::debug!(log, "Discovered: {:?}", enr);
            }
            Some(Discv5Event::NodeInserted { node_id, replaced }) => {
                slog::debug!(
                    log,
                    "node_id: {:?}; replaced: {:?}",
                    node_id,
                    replaced
                );
            }
            Some(Discv5Event::EnrAdded { enr, replaced }) => {
                slog::debug!(log, "Enr Added: {:?}; replaced: {:?}", enr, replaced);
            }
            Some(Discv5Event::SessionEstablished(enr, socket_addr)) => {
                slog::debug!(
                    log,
                    "Session Established: {:?}; socket_addr: {:?}",
                    enr,
                    socket_addr
                );
            }
            Some(Discv5Event::TalkRequest(_)) => {
                slog::debug!(log, "Talk Request Received");
            }
            None => {
                slog::debug!(log, "No events");
            }
        }
    }
}
