#![deny(unsafe_code)]

use crate::libp2p::behaviour::CustomBehavior;
use futures::StreamExt;
use libp2p::{swarm::SwarmEvent, Swarm};
use slog::Logger;

pub async fn swarm_events(swarm: &mut Swarm<CustomBehavior>, log: Logger) {
    slog::info!(log, "Starting swarm events");
    loop {
        match swarm.select_next_some().await {
            SwarmEvent::Behaviour(event) => {
                // Handle behaviour-specific events here
                slog::info!(log, "Swarm event: Behaviour"; "event" => ?event);
            }
            SwarmEvent::ConnectionEstablished {
                peer_id,
                concurrent_dial_errors,
                endpoint,
                established_in,
                num_established,
            } => {
                slog::info!(log, "Swarm event: ConnectionEstablished"; "peer_id" => ?peer_id, "concurrent_dial_errors" => ?concurrent_dial_errors, "endpoint" => ?endpoint, "established_in" => ?established_in, "num_established" => ?num_established);
            }
            SwarmEvent::ConnectionClosed {
                peer_id,
                cause,
                endpoint,
                num_established,
            } => {
                slog::info!(log, "Swarm event: ConnectionClosed"; "peer_id" => ?peer_id, "cause" => ?cause, "endpoint" => ?endpoint, "num_established" => ?num_established);
            }
            SwarmEvent::Dialing(peer_id) => {
                slog::info!(log, "Swarm event: Dialing"; "peer_id" => ?peer_id);
            }
            SwarmEvent::ExpiredListenAddr {
                listener_id,
                address,
            } => {
                slog::info!(log, "Swarm event: ExpiredListenAddr"; "listener_id" => ?listener_id, "address" => ?address);
            }
            SwarmEvent::IncomingConnection {
                local_addr,
                send_back_addr,
            } => {
                slog::info!(log, "Swarm event: IncomingConnection"; "local_addr" => ?local_addr, "send_back_addr" => ?send_back_addr)
            }
            SwarmEvent::IncomingConnectionError {
                local_addr,
                send_back_addr,
                error,
            } => {
                slog::info!(log, "Swarm event: IncomingConnectionError"; "local_addr" => ?local_addr, "send_back_addr" => ?send_back_addr, "error" => ?error);
            }
            SwarmEvent::ListenerClosed {
                listener_id,
                addresses,
                reason,
            } => {
                slog::info!(log, "Swarm event: ListenerClosed"; "listener_id" => ?listener_id, "addresses" => ?addresses, "reason" => ?reason);
            }
            SwarmEvent::ListenerError { listener_id, error } => {
                slog::info!(log, "Swarm event: ListenerError"; "listener_id" => ?listener_id, "error" => ?error);
            }
            SwarmEvent::NewListenAddr {
                listener_id,
                address,
            } => {
                slog::info!(log, "Swarm event: NewListenAddr"; "listener_id" => ?listener_id, "address" => ?address);
            }
            SwarmEvent::OutgoingConnectionError { peer_id, error } => {
                slog::info!(log, "Swarm event: OutgoingConnectionError"; "peer_id" => ?peer_id, "error" => ?error);
            }
            _ => {}
        }
    }
}
