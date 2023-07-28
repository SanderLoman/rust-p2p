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

    // Discv5 Unsolicited Packet Rate Limiter
    let filter_rate_limiter = Some(
        discv5::RateLimiterBuilder::new()
            .total_n_every(10, Duration::from_secs(1)) // Allow bursts, average 10 per second
            .ip_n_every(9, Duration::from_secs(1)) // Allow bursts, average 9 per second
            .node_n_every(8, Duration::from_secs(1)) // Allow bursts, average 8 per second
            .build()
            .expect("The total rate limit has been specified"),
    );

    let discv5_listen_config = discv5::ListenConfig::from_ip(Ipv4Addr::UNSPECIFIED.into(), listen_port);

    // discv5 configuration
    let discv5_config = Discv5ConfigBuilder::new(discv5_listen_config)
        .enable_packet_filter()
        .session_cache_capacity(5000)
        .request_timeout(Duration::from_secs(1))
        .query_peer_timeout(Duration::from_secs(2))
        .query_timeout(Duration::from_secs(30))
        .request_retries(1)
        .enr_peer_update_min(10)
        .query_parallelism(5)
        .disable_report_discovered_peers()
        .ip_limit() // limits /24 IP's in buckets.
        .incoming_bucket_limit(8) // half the bucket size
        .filter_rate_limiter(filter_rate_limiter)
        .filter_max_bans_per_ip(Some(5))
        .filter_max_nodes_per_ip(Some(10))
        .table_filter(|enr| enr.ip4().map_or(false, |ip| is_global_ipv4(&ip))) // Filter non-global IPs
        .ban_duration(Some(Duration::from_secs(3600)))
        .ping_interval(Duration::from_secs(300))
        .build();

    slog::debug!(log, "config: {:?}", discv5_config);

    let mut discv5: Discv5 = Discv5::new(enr, enr_key, discv5_config).unwrap();

    discv5.start().await.unwrap();

    Ok(())
}

#[allow(clippy::nonminimal_bool)]
fn is_global_ipv4(addr: &Ipv4Addr) -> bool {
    // check if this address is 192.0.0.9 or 192.0.0.10. These addresses are the only two
    // globally routable addresses in the 192.0.0.0/24 range.
    if u32::from_be_bytes(addr.octets()) == 0xc0000009
        || u32::from_be_bytes(addr.octets()) == 0xc000000a
    {
        return true;
    }
    !addr.is_private()
            && !addr.is_loopback()
            && !addr.is_link_local()
            && !addr.is_broadcast()
            && !addr.is_documentation()
            // shared
            && !(addr.octets()[0] == 100 && (addr.octets()[1] & 0b1100_0000 == 0b0100_0000)) &&!(addr.octets()[0] & 240 == 240 && !addr.is_broadcast())
            // addresses reserved for future protocols (`192.0.0.0/24`)
            // reserved
            && !(addr.octets()[0] == 192 && addr.octets()[1] == 0 && addr.octets()[2] == 0)
            // Make sure the address is not in 0.0.0.0/8
            && addr.octets()[0] != 0
}
