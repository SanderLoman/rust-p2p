#![deny(unsafe_code)]

use crate::create_logger;
use crate::libp2p::transport::transport::setup_transport;

use libp2p::{identity, swarm::{SwarmBuilder, SwarmEvent}, PeerId, futures::StreamExt};
use std::error::Error;
use tokio::runtime::Handle;

// #[derive(NetworkBehaviour)]
struct Behaviour {
    // Define your network behaviour here
}

pub async fn setup_swarm() -> Result<(), Box<dyn Error>> {
    let log = create_logger();

    // Get the transport and the local key pair.
    let (transport, local_keys) = setup_transport().await.unwrap();

    // We use the key pair from the transport.rs file otherwise we generate 2 different keys.
    let local_keys = local_keys;
    let local_peer_id = PeerId::from(local_keys.public());

    // Here we just use the transport from the transport.rs file.
    let transport = transport;

    let mut swarm = {
        // Dummy behaviour, this will be changed later.
        let behaviour = libp2p::swarm::dummy::Behaviour;

        let executor = {
            let executor = Handle::current();
            move |fut: _| {
                executor.spawn(fut);
            }
        };

        // Build the Swarm
        SwarmBuilder::with_executor(transport, behaviour, local_peer_id, executor).build()
    };

    // Listen on all interfaces and the port we desire,
    // could listen on port 0 to listen on whatever port the OS assigns us.
    swarm
        .listen_on("/ip4/0.0.0.0/tcp/7777".parse().unwrap())
        .unwrap();

    slog::debug!(log, "{:?}", swarm.network_info());        
    
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::OutgoingConnectionError { peer_id, error } => {
                slog::debug!(log, "peer_id: {:?}; Outgoing Connection Error: {:?}", peer_id, error);
            }
            #[allow(deprecated)]
            SwarmEvent::BannedPeer { peer_id, endpoint } => {
                slog::debug!(log, "Banned Peer: {:?}; endpoint: {:?};", peer_id, endpoint);
            }
            SwarmEvent::ListenerError { listener_id, error } => {
                slog::debug!(log, "Listener Error: {:?}; listener_id: {:?};", error, listener_id);
            }
            SwarmEvent::Dialing(peer_id) => {
                slog::debug!(log, "Dialing: {:?}", peer_id);
            }
            SwarmEvent::Behaviour(event) => {
                slog::debug!(log, "Behaviour Event: {:?}", event);
            }
            SwarmEvent::ConnectionEstablished {
                peer_id,
                endpoint,
                num_established,
                concurrent_dial_errors,
                established_in
            } => {
                slog::debug!(log, "Connection established with peer {:?}; at endpoint {:?}; concurrent_dial_errors: {:?}; established_in: {:?};",
                    peer_id, endpoint, concurrent_dial_errors, established_in
                );
                slog::debug!(log, "There are now {} peers connected to us.", num_established);
            }
            SwarmEvent::ConnectionClosed {
                peer_id,
                endpoint,
                num_established,
                cause
            } => {
                slog::debug!(log, 
                    "Connection with peer {:?} at endpoint {:?} closed, cause: {:?}.",
                    peer_id, endpoint, cause
                );
                slog::debug!(log, "There are now {} peers connected to us.", num_established);
            }
            SwarmEvent::NewListenAddr {address, listener_id} => {
                slog::debug!(log, "Now listening on {:?}; listener_id: {:?};", address, listener_id);
            }
            SwarmEvent::ExpiredListenAddr {address, listener_id} => {
                slog::debug!(log, "No longer listening on {:?}; listener_id: {:?};", address, listener_id);
            }
            SwarmEvent::ListenerClosed { addresses, reason, listener_id } => {
                slog::debug!(log, "Listener closed: {:?}; addresses: {:?}; listener_id: {:?};", reason, addresses, listener_id);
            }
            SwarmEvent::IncomingConnection { local_addr, send_back_addr } => {
                slog::debug!(log, "Incoming connection from {:?}, send_back_addr: {:?}", local_addr, send_back_addr);
            }
            SwarmEvent::IncomingConnectionError { error, local_addr, send_back_addr } => {
                slog::debug!(log, "Incoming connection error: {:?}; local_addr: {:?}, send_back_addr: {:?}", error, local_addr, send_back_addr);
            }
        }
    }
}
