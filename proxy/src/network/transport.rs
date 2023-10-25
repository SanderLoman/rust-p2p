use std::time::Duration;

use futures::future::Either;
use libp2p::core::{multiaddr::Multiaddr, muxing::StreamMuxerBox, transport::Boxed};

use libp2p::identity::Keypair;
use libp2p::multiaddr::Protocol;
use libp2p::{core, noise, yamux, PeerId, Transport};
use libp2p_quic;

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

    // Creates the TCP transport layer
    let tcp = libp2p::tcp::tokio::Transport::new(libp2p::tcp::Config::default().nodelay(true))
        .upgrade(core::upgrade::Version::V1)
        .authenticate(generate_noise_config(&local_private_key))
        .multiplex(core::upgrade::SelectUpgrade::new(
            yamux_config,
            mplex_config,
        ))
        .timeout(Duration::from_secs(10));

    let transport = {
        // Enables Quic
        // The default quic configuration suits us for now.
        let quic_config = libp2p_quic::Config::new(&local_private_key);
        tcp.or_transport(libp2p_quic::tokio::Transport::new(quic_config))
            .map(|either_output, _| match either_output {
                Either::Left((peer_id, muxer)) => (peer_id, StreamMuxerBox::new(muxer)),
                Either::Right((peer_id, muxer)) => (peer_id, StreamMuxerBox::new(muxer)),
            })
    };

    // Enables DNS over the transport.
    let transport: Boxed<(PeerId, StreamMuxerBox)> =
        libp2p::dns::tokio::Transport::system(transport)?.boxed();

    Ok(transport)
}

/// Generate authenticated XX Noise config from identity keys
fn generate_noise_config(identity_keypair: &Keypair) -> noise::Config {
    noise::Config::new(identity_keypair).expect("signing can fail only once during starting a node")
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
