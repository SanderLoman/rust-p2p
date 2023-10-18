use std::time::Duration;

use futures::future::Either;
use libp2p::{
    core::{muxing::StreamMuxerBox, transport::Boxed},
    identity::Keypair,
    multiaddr::Protocol,
    noise, yamux, Multiaddr, PeerId,
};
use slog::debug;

use crate::network::methods::{MetaData, MetaDataV1, MetaDataV2};

pub type Attnets = String;
pub type Syncnets = String;

type BoxedTransport = Boxed<(PeerId, StreamMuxerBox)>;

/// The implementation supports TCP/IP, QUIC (experimental) over UDP, noise as the encryption layer, and
/// mplex/yamux as the multiplexing layer (when using TCP).
pub fn build_transport(local_private_key: Keypair) -> std::io::Result<BoxedTransport> {
    // mplex config
    let mut mplex_config = libp2p_mplex::MplexConfig::new();
    mplex_config.set_max_buffer_size(256);
    mplex_config.set_max_buffer_behaviour(libp2p_mplex::MaxBufferBehaviour::Block);

    // yamux config
    let mut yamux_config = yamux::Config::default();
    yamux_config.set_window_update_mode(yamux::WindowUpdateMode::on_read());

    /// Generate authenticated XX Noise config from identity keys
    fn generate_noise_config(identity_keypair: &Keypair) -> noise::Config {
        noise::Config::new(identity_keypair)
            .expect("signing can fail only once during starting a node")
    }

    // Creates the TCP transport layer
    let tcp = libp2p::tcp::tokio::Transport::new(libp2p::tcp::Config::default().nodelay(true))
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(generate_noise_config(&local_private_key))
        .multiplex(libp2p::core::upgrade::SelectUpgrade::new(
            yamux_config,
            mplex_config,
        ))
        .timeout(Duration::from_secs(10));

    let (transport, bandwidth) = {
        // Enables Quic
        // The default quic configuration suits us for now.
        let quic_config = libp2p_quic::Config::new(&local_private_key);
        tcp.or_transport(libp2p_quic::tokio::Transport::new(quic_config))
            .map(|either_output, _| match either_output {
                Either::Left((peer_id, muxer)) => (peer_id, StreamMuxerBox::new(muxer)),
                Either::Right((peer_id, muxer)) => (peer_id, StreamMuxerBox::new(muxer)),
            })
            .with_bandwidth_logging()
    };

    // // Enables DNS over the transport.
    let transport = libp2p::dns::TokioDnsConfig::system(transport)?.boxed();

    Ok((transport, bandwidth))
}

/// For a multiaddr that ends with a peer id, this strips this suffix. Rust-libp2p
/// only supports dialing to an address without providing the peer id.
pub fn strip_peer_id(addr: &mut Multiaddr) {
    let last = addr.pop();
    match last {
        Some(Protocol::P2p(_)) => {}
        Some(other) => addr.push(other),
        _ => {}
    }
}

/// Load metadata from persisted file. Return default metadata if loading fails.
pub fn load_or_build_metadata(log: &slog::Logger) -> MetaData {
    // We load a V2 metadata version by default (regardless of current fork)
    // since a V2 metadata can be converted to V1. The RPC encoder is responsible
    // for sending the correct metadata version based on the negotiated protocol version.
    let mut meta_data = MetaDataV2 {
        seq_number: 0,
        attnets: String,
        syncnets: String,
    };
    // Read metadata from persisted file if available

    let mut metadata_ssz = Vec::new();

    match MetaDataV2::from_ssz_bytes(&metadata_ssz) {
        Ok(persisted_metadata) => {
            meta_data.seq_number = persisted_metadata.seq_number;
            // Increment seq number if persisted attnet is not default
            if persisted_metadata.attnets != meta_data.attnets
                || persisted_metadata.syncnets != meta_data.syncnets
            {
                meta_data.seq_number += 1;
            }
            debug!(log, "Loaded metadata from disk");
        }
        Err(_) => {
            match MetaDataV1::from_ssz_bytes(&metadata_ssz) {
                Ok(persisted_metadata) => {
                    let persisted_metadata = MetaData::V1(persisted_metadata);
                    // Increment seq number as the persisted metadata version is updated
                    meta_data.seq_number = *persisted_metadata.seq_number() + 1;
                    debug!(log, "Loaded metadata from disk");
                }
                Err(e) => {
                    debug!(
                        log,
                        "Metadata from file could not be decoded";
                        "error" => ?e,
                    );
                }
            }
        }
    };

    // Wrap the MetaData
    let meta_data = MetaData::V2(meta_data);

    debug!(log, "Metadata sequence number"; "seq_num" => meta_data.seq_number());
    meta_data
}
