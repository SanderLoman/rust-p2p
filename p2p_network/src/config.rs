use crate::listen_addr::ListenAddr;
use discv5::{Discv5Config, Discv5ConfigBuilder, Enr};
use libp2p::{gossipsub, Multiaddr};
use project_types::fork_context::{ForkContext, ForkName};
use sha2::{Digest, Sha256};
use std::sync::Arc;
use std::{net::Ipv4Addr, time::Duration};

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

    // pub outbound_rate_limiter_config: Option<OutboundRateLimiterConfig>,

    // pub inbound_rate_limiter_config: Option<InboundRateLimiterConfig>,
    pub topics: Vec<String>,
}

impl Config {
    fn default() -> Self {
        // Note: Using the default config here. Use `gossipsub_config` function for getting
        // Lighthouse specific configuration for gossipsub.
        let gs_config = gossipsub::ConfigBuilder::default()
            .build()
            .expect("valid gossipsub configuration");

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
            disc_port: 1111,
            quic_port: 2222,
            tcp_port: 1111,
        });

        let discv5_listen_config =
            discv5::ListenConfig::from_ip(Ipv4Addr::UNSPECIFIED.into(), 8888);

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
            // .ip_limit() // limits /24 IP's in buckets. (Probably want to remove this since we want as many peers as possible)
            // .incoming_bucket_limit(8) // half the bucket size. (Probably want to remove this since we want as many peers as possible)
            .filter_rate_limiter(filter_rate_limiter)
            .filter_max_bans_per_ip(Some(5))
            .filter_max_nodes_per_ip(Some(10))
            .ban_duration(Some(Duration::from_secs(3600)))
            .ping_interval(Duration::from_secs(300))
            .build();

        // NOTE: Some of these get overridden by the corresponding CLI default values.
        Config {
            listen_addresses,
            gs_config,
            discv5_config,
            boot_nodes_enr: vec![],
            boot_nodes_multiaddr: vec![],
            libp2p_nodes: vec![],
            topics: Vec::new(),
            // outbound_rate_limiter_config: None,
            // inbound_rate_limiter_config: None,
        }
    }
}

pub fn gossipsub_config(
    network_load: u8,
    fork_context: Arc<ForkContext>,
    gossipsub_config_params: GossipsubConfigParams,
) -> gossipsub::Config {
    // The function used to generate a gossipsub message id
    // We use the first 8 bytes of SHA256(topic, data) for content addressing
    let fast_gossip_message_id = |message: &gossipsub::RawMessage| {
        let data = [message.topic.as_str().as_bytes(), &message.data].concat();
        gossipsub::FastMessageId::from(&Sha256::digest(&data)[..8])
    };
    fn prefix(
        prefix: [u8; 4],
        message: &gossipsub::Message,
        fork_context: &ForkContext,
    ) -> Vec<u8> {
        let topic_bytes = message.topic.as_str().as_bytes();
        match fork_context.current_fork() {
            ForkName::Altair | ForkName::Merge | ForkName::Capella => {
                println!("prefix: {:?}", prefix);
                let topic_len_bytes = topic_bytes.len().to_le_bytes();
                let mut vec = Vec::with_capacity(
                    prefix.len() + topic_len_bytes.len() + topic_bytes.len() + message.data.len(),
                );
                vec.extend_from_slice(&prefix);
                vec.extend_from_slice(&topic_len_bytes);
                vec.extend_from_slice(topic_bytes);
                vec.extend_from_slice(&message.data);
                vec
            }
            ForkName::Base => {
                let mut vec = Vec::with_capacity(prefix.len() + message.data.len());
                vec.extend_from_slice(&prefix);
                vec.extend_from_slice(&message.data);
                vec
            }
        }
    }

    let message_domain_valid_snappy = gossipsub_config_params.message_domain_valid_snappy;
    let is_merge_enabled = fork_context.fork_exists(ForkName::Merge);
    let gossip_message_id = move |message: &gossipsub::Message| {
        gossipsub::MessageId::from(
            &Sha256::digest(
                prefix(message_domain_valid_snappy, message, &*fork_context).as_slice(),
            )[..20],
        )
    };

    gossipsub::ConfigBuilder::default()
        .max_transmit_size(gossip_max_size(
            is_merge_enabled,
            gossipsub_config_params.gossip_max_size,
        ))
        .max_messages_per_rpc(Some(500)) // Responses to IWANT can be quite large
        .validate_messages()
        .validation_mode(gossipsub::ValidationMode::Anonymous)
        .duplicate_cache_time(Duration::from_secs(60))
        .message_id_fn(gossip_message_id)
        .fast_message_id_fn(fast_gossip_message_id)
        .allow_self_origin(true)
        .build()
        .expect("valid gossipsub configuration")
}
