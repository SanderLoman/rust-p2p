use std::pin::Pin;
use std::time::Duration;

use futures::future::Either;
use libp2p::core::{multiaddr::Multiaddr, muxing::StreamMuxerBox, transport::Boxed};

use libp2p::identity::Keypair;
use libp2p::multiaddr::Protocol;
use libp2p::{core, noise, yamux, PeerId, Transport};
use libp2p_mplex::{MaxBufferBehaviour, MplexConfig};
use libp2p_quic;

type BoxedTransport = Boxed<(PeerId, StreamMuxerBox)>;

use libp2p::{core::upgrade::SelectUpgrade, dns, tcp, Swarm, SwarmBuilder};
use std::io::Result;

use super::task_executor;

pub fn build_swarm(
    local_keypair: Keypair,
    behaviour: impl libp2p::swarm::NetworkBehaviour,
    executor: task_executor::TaskExecutor,
) -> Result<Swarm<impl libp2p::swarm::NetworkBehaviour>> {
    // Set up the multiplexing
    let mut mplex_config = MplexConfig::new();
    mplex_config.set_max_buffer_size(256);
    mplex_config.set_max_buffer_behaviour(MaxBufferBehaviour::Block);

    let mut yamux_config = yamux::Config::default();
    yamux_config.set_window_update_mode(yamux::WindowUpdateMode::on_read());

    let multiplexer_upgrade = SelectUpgrade::new(yamux_config, mplex_config);

    // Set up the security
    let security_upgrade = generate_noise_config(&local_keypair);

    // Set up the TCP transport
    let tcp_config = tcp::Config::default().nodelay(true);
    let transport = tcp::tokio::Transport::new(tcp_config)
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(security_upgrade.clone()) // use cloned security_upgrade
        .multiplex(multiplexer_upgrade)
        .timeout(Duration::from_secs(10));

    // Enable QUIC
    let quic_config = libp2p_quic::Config::new(&local_keypair);
    let transport = transport
        .or_transport(libp2p_quic::tokio::Transport::new(quic_config))
        .map(|either_output, _| match either_output {
            Either::Left((peer_id, muxer)) => (peer_id, StreamMuxerBox::new(muxer)),
            Either::Right((peer_id, muxer)) => (peer_id, StreamMuxerBox::new(muxer)),
        });

    // Enable DNS
    let transport = dns::tokio::Transport::system(transport)?.boxed();

    // Set up the executor for libp2p
    struct Executor(task_executor::TaskExecutor);
    impl libp2p::swarm::Executor for Executor {
        fn exec(&self, f: Pin<Box<dyn futures::Future<Output = ()> + Send>>) {
            self.0.spawn(f, "libp2p");
        }
    }

    // Build the swarm
    let local_peer_id = PeerId::from(local_keypair.public());
    let swarm = SwarmBuilder::with_existing_identity(local_keypair)
        .with_tokio()
        .with_tcp(tcp_config, security_upgrade, multiplexer_upgrade)? // use security_upgrade
        .with_quic()
        .with_dns()?
        .with_behaviour(behaviour)?
        .with_executor(Executor(executor))
        .build();

    Ok(swarm)
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
