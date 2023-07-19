#![deny(unsafe_code)]
#![allow(deprecated)]

use eyre::Result;
use libp2p::{
    identity::Keypair,
    noise::*, PeerId,
    Transport,
};
use std::error::Error;
use std::time::Duration;

pub async fn setup_transport() -> Result<libp2p::core::transport::Boxed<(PeerId, libp2p::core::muxing::StreamMuxerBox)>, Box<dyn Error>> {
    let libp2p_local_key = Keypair::generate_secp256k1();

    let tcp = libp2p::tcp::tokio::Transport::new(libp2p::tcp::Config::default().nodelay(true));
    let transport1 = libp2p::dns::TokioDnsConfig::system(tcp)?;
    let transport2 = libp2p::dns::TokioDnsConfig::system(libp2p::tcp::tokio::Transport::new(
        libp2p::tcp::Config::default().nodelay(true),
    ))?;

    let transport = transport1.or_transport(libp2p::websocket::WsConfig::new(transport2));

    // mplex config
    let mut mplex_config = libp2p::mplex::MplexConfig::new();
    mplex_config.set_max_buffer_size(256);
    mplex_config.set_max_buffer_behaviour(libp2p::mplex::MaxBufferBehaviour::Block);

    // yamux config
    let mut yamux_config = libp2p::yamux::YamuxConfig::default();
    yamux_config.set_window_update_mode(libp2p::yamux::WindowUpdateMode::on_read());

    fn generate_noise_config(
        identity_keypair: &Keypair,
    ) -> libp2p::noise::NoiseAuthenticated<XX, X25519Spec, ()> {
        let static_dh_keys = libp2p::noise::Keypair::<X25519Spec>::new()
            .into_authentic(identity_keypair)
            .expect("signing can fail only once during starting a node");
        libp2p::noise::NoiseConfig::xx(static_dh_keys).into_authenticated()
    }

    let upgraded_transport = transport
        .upgrade(libp2p::core::upgrade::Version::V1)
        .authenticate(generate_noise_config(&libp2p_local_key))
        .multiplex(libp2p::core::upgrade::SelectUpgrade::new(
            yamux_config,
            mplex_config,
        ))
        .timeout(Duration::from_secs(10))
        .boxed();

    Ok(upgraded_transport)
}