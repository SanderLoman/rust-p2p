use discv5::Enr;
use discv5::{Discv5Config, Discv5ConfigBuilder};
// use libp2p::gossipsub;
use libp2p::Multiaddr;
use serde_derive::{Deserialize, Serialize};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::time::Duration;

use super::listen_addr::{ListenAddr, ListenAddress};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
/// Network configuration for lighthouse.
pub struct Config {
    /// IP addresses to listen on.
    listen_addresses: ListenAddress,

    /// The address to broadcast to peers about which address we are listening on. None indicates
    /// that no discovery address has been set in the CLI args.
    pub enr_address: (Option<Ipv4Addr>, Option<Ipv6Addr>),

    /// The udp ipv4 port to broadcast to peers in order to reach back for discovery.
    pub enr_udp4_port: Option<u16>,

    /// The quic ipv4 port to broadcast to peers in order to reach back for libp2p services.
    pub enr_quic4_port: Option<u16>,

    /// The tcp ipv4 port to broadcast to peers in order to reach back for libp2p services.
    pub enr_tcp4_port: Option<u16>,

    /// The udp ipv6 port to broadcast to peers in order to reach back for discovery.
    pub enr_udp6_port: Option<u16>,

    /// The tcp ipv6 port to broadcast to peers in order to reach back for libp2p services.
    pub enr_tcp6_port: Option<u16>,

    /// The quic ipv6 port to broadcast to peers in order to reach back for libp2p services.
    pub enr_quic6_port: Option<u16>,

    /// Discv5 configuration parameters.
    #[serde(skip)]
    pub discv5_config: Discv5Config,

    /// List of nodes to initially connect to.
    pub boot_nodes_enr: Vec<Enr>,

    /// List of nodes to initially connect to, on Multiaddr format.
    pub boot_nodes_multiaddr: Vec<Multiaddr>,

    /// List of libp2p nodes to initially connect to.
    pub libp2p_nodes: Vec<Multiaddr>,

    /// Client version
    pub client_version: String,
    // /// List of extra topics to initially subscribe to as strings.
    // pub topics: Vec<GossipKind>,
}

impl Config {
    /// Sets the listening address to use an ipv4 address. The discv5 ip_mode and table filter are
    /// adjusted accordingly to ensure addresses that are present in the enr are globally
    /// reachable.
    pub fn set_ipv4_listening_address(
        &mut self,
        addr: Ipv4Addr,
        tcp_port: u16,
        disc_port: u16,
        quic_port: u16,
    ) {
        self.listen_addresses = ListenAddress::V4(ListenAddr {
            addr,
            disc_port,
            quic_port,
            tcp_port,
        });
        self.discv5_config.listen_config = discv5::ListenConfig::from_ip(addr.into(), disc_port);
        self.discv5_config.table_filter = |enr| enr.ip4().as_ref().map_or(false, is_global_ipv4)
    }

    /// Sets the listening address to use an ipv6 address. The discv5 ip_mode and table filter is
    /// adjusted accordingly to ensure addresses that are present in the enr are globally
    /// reachable.
    pub fn set_ipv6_listening_address(
        &mut self,
        addr: Ipv6Addr,
        tcp_port: u16,
        disc_port: u16,
        quic_port: u16,
    ) {
        self.listen_addresses = ListenAddress::V6(ListenAddr {
            addr,
            disc_port,
            quic_port,
            tcp_port,
        });

        self.discv5_config.listen_config = discv5::ListenConfig::from_ip(addr.into(), disc_port);
        self.discv5_config.table_filter = |enr| enr.ip6().as_ref().map_or(false, is_global_ipv6)
    }

    /// Sets the listening address to use both an ipv4 and ipv6 address. The discv5 ip_mode and
    /// table filter is adjusted accordingly to ensure addresses that are present in the enr are
    /// globally reachable.
    #[allow(clippy::too_many_arguments)]
    pub fn set_ipv4_ipv6_listening_addresses(
        &mut self,
        v4_addr: Ipv4Addr,
        tcp4_port: u16,
        disc4_port: u16,
        quic4_port: u16,
        v6_addr: Ipv6Addr,
        tcp6_port: u16,
        disc6_port: u16,
        quic6_port: u16,
    ) {
        self.listen_addresses = ListenAddress::DualStack(
            ListenAddr {
                addr: v4_addr,
                disc_port: disc4_port,
                quic_port: quic4_port,
                tcp_port: tcp4_port,
            },
            ListenAddr {
                addr: v6_addr,
                disc_port: disc6_port,
                quic_port: quic6_port,
                tcp_port: tcp6_port,
            },
        );
        self.discv5_config.listen_config = discv5::ListenConfig::default()
            .with_ipv4(v4_addr, disc4_port)
            .with_ipv6(v6_addr, disc6_port);

        self.discv5_config.table_filter = |enr| match (&enr.ip4(), &enr.ip6()) {
            (None, None) => false,
            (None, Some(ip6)) => is_global_ipv6(ip6),
            (Some(ip4), None) => is_global_ipv4(ip4),
            (Some(ip4), Some(ip6)) => is_global_ipv4(ip4) && is_global_ipv6(ip6),
        };
    }

    pub fn set_listening_addr(&mut self, listen_addr: ListenAddress) {
        match listen_addr {
            ListenAddress::V4(ListenAddr {
                addr,
                disc_port,
                quic_port,
                tcp_port,
            }) => self.set_ipv4_listening_address(addr, tcp_port, disc_port, quic_port),
            ListenAddress::V6(ListenAddr {
                addr,
                disc_port,
                quic_port,
                tcp_port,
            }) => self.set_ipv6_listening_address(addr, tcp_port, disc_port, quic_port),
            ListenAddress::DualStack(
                ListenAddr {
                    addr: ip4addr,
                    disc_port: disc4_port,
                    quic_port: quic4_port,
                    tcp_port: tcp4_port,
                },
                ListenAddr {
                    addr: ip6addr,
                    disc_port: disc6_port,
                    quic_port: quic6_port,
                    tcp_port: tcp6_port,
                },
            ) => self.set_ipv4_ipv6_listening_addresses(
                ip4addr, tcp4_port, disc4_port, quic4_port, ip6addr, tcp6_port, disc6_port,
                quic6_port,
            ),
        }
    }

    pub fn listen_addrs(&self) -> &ListenAddress {
        &self.listen_addresses
    }
}

impl Default for Config {
    /// Generate a default network configuration.
    fn default() -> Self {
        // Note: Using the default config here. Use `gossipsub_config` function for getting
        // Lighthouse specific configuration for gossipsub.
        // let gs_config = gossipsub::ConfigBuilder::default()
        //     .build()
        //     .expect("valid gossipsub configuration");

        // Discv5 Unsolicited Packet Rate Limiter
        let filter_rate_limiter = Some(
            discv5::RateLimiterBuilder::new()
                .total_n_every(10, Duration::from_secs(1)) // Allow bursts, average 10 per second
                .ip_n_every(9, Duration::from_secs(1)) // Allow bursts, average 9 per second
                .node_n_every(8, Duration::from_secs(1)) // Allow bursts, average 8 per second
                .build()
                .expect("The total rate limit has been specified"),
        );
        let listen_addresses = ListenAddress::V4(ListenAddr {
            addr: Ipv4Addr::UNSPECIFIED,
            disc_port: 9000,
            quic_port: 9001,
            tcp_port: 9000,
        });

        let discv5_listen_config =
            discv5::ListenConfig::from_ip(Ipv4Addr::UNSPECIFIED.into(), 9000);

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

        // NOTE: Some of these get overridden by the corresponding CLI default values.
        Config {
            listen_addresses,
            enr_address: (None, None),
            enr_udp4_port: None,
            enr_quic4_port: None,
            enr_tcp4_port: None,
            enr_udp6_port: None,
            enr_quic6_port: None,
            enr_tcp6_port: None,
            discv5_config,
            boot_nodes_enr: vec![],
            boot_nodes_multiaddr: vec![],
            libp2p_nodes: vec![],
            client_version: crate::version_with_platform(),
            // topics: Vec::new(),
        }
    }
}

/// Helper function to determine if the IpAddr is a global address or not. The `is_global()`
/// function is not yet stable on IpAddr.
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

/// NOTE: Docs taken from https://doc.rust-lang.org/stable/std/net/struct.Ipv6Addr.html#method.is_global
///
/// Returns true if the address appears to be globally reachable as specified by the IANA IPv6
/// Special-Purpose Address Registry. Whether or not an address is practically reachable will
/// depend on your network configuration.
///
/// Most IPv6 addresses are globally reachable; unless they are specifically defined as not
/// globally reachable.
///
/// Non-exhaustive list of notable addresses that are not globally reachable:
///
/// - The unspecified address (is_unspecified)
/// - The loopback address (is_loopback)
/// - IPv4-mapped addresses
/// - Addresses reserved for benchmarking
/// - Addresses reserved for documentation (is_documentation)
/// - Unique local addresses (is_unique_local)
/// - Unicast addresses with link-local scope (is_unicast_link_local)
// TODO: replace with [`Ipv6Addr::is_global`] once
//       [Ip](https://github.com/rust-lang/rust/issues/27709) is stable.
pub const fn is_global_ipv6(addr: &Ipv6Addr) -> bool {
    const fn is_documentation(addr: &Ipv6Addr) -> bool {
        (addr.segments()[0] == 0x2001) && (addr.segments()[1] == 0xdb8)
    }
    const fn is_unique_local(addr: &Ipv6Addr) -> bool {
        (addr.segments()[0] & 0xfe00) == 0xfc00
    }
    const fn is_unicast_link_local(addr: &Ipv6Addr) -> bool {
        (addr.segments()[0] & 0xffc0) == 0xfe80
    }
    !(addr.is_unspecified()
            || addr.is_loopback()
            // IPv4-mapped Address (`::ffff:0:0/96`)
            || matches!(addr.segments(), [0, 0, 0, 0, 0, 0xffff, _, _])
            // IPv4-IPv6 Translat. (`64:ff9b:1::/48`)
            || matches!(addr.segments(), [0x64, 0xff9b, 1, _, _, _, _, _])
            // Discard-Only Address Block (`100::/64`)
            || matches!(addr.segments(), [0x100, 0, 0, 0, _, _, _, _])
            // IETF Protocol Assignments (`2001::/23`)
            || (matches!(addr.segments(), [0x2001, b, _, _, _, _, _, _] if b < 0x200)
                && !(
                    // Port Control Protocol Anycast (`2001:1::1`)
                    u128::from_be_bytes(addr.octets()) == 0x2001_0001_0000_0000_0000_0000_0000_0001
                    // Traversal Using Relays around NAT Anycast (`2001:1::2`)
                    || u128::from_be_bytes(addr.octets()) == 0x2001_0001_0000_0000_0000_0000_0000_0002
                    // AMT (`2001:3::/32`)
                    || matches!(addr.segments(), [0x2001, 3, _, _, _, _, _, _])
                    // AS112-v6 (`2001:4:112::/48`)
                    || matches!(addr.segments(), [0x2001, 4, 0x112, _, _, _, _, _])
                    // ORCHIDv2 (`2001:20::/28`)
                    || matches!(addr.segments(), [0x2001, b, _, _, _, _, _, _] if b >= 0x20 && b <= 0x2F)
                ))
            || is_documentation(addr)
            || is_unique_local(addr)
            || is_unicast_link_local(addr))
}
