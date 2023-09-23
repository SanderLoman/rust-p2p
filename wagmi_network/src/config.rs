use discv5::{Discv5Config, Enr};
use libp2p::{gossipsub, Multiaddr};

use crate::listen_addr::ListenAddress;

pub fn gossip_max_size(is_merge_enabled: bool, gossip_max_size: usize) -> usize {
    if is_merge_enabled {
        gossip_max_size
    } else {
        gossip_max_size / 10
    }
}

pub struct GossipsubConfigParams {
    pub message_domain_valid_snappy: [u8; 4],
    pub gossip_max_size: usize,
}

pub struct Config {
    listen_addresses: ListenAddress,

    pub gs_config: gossipsub::Config,

    pub discv5_config: Discv5Config,

    pub boot_nodes_enr: Vec<Enr>,

    pub boot_nodes_multiaddr: Vec<Multiaddr>,

    pub libp2p_nodes: Vec<Multiaddr>,

    pub outbound_rate_limiter_config: Option<OutboundRateLimiterConfig>,

    pub inbound_rate_limiter_config: Option<InboundRateLimiterConfig>,
}

impl Config {
    fn default() -> Self {
        let mut listen_addresses = ListenAddress::default();
        Config {
            
        }
    }
}
