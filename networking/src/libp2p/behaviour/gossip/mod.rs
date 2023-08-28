#![deny(unsafe_code)]

use libp2p::gossipsub::Behaviour;
use libp2p::gossipsub::*;
use libp2p::identity::Keypair;
use libp2p::swarm::{dummy::ConnectionHandler, NetworkBehaviour};
use libp2p::PeerId;
use slog::Logger;
use std::time::Duration;
use void::Void;

pub struct Gossipsub {
    /// The underlying libp2p gossipsub behaviour.
    pub gossip_behaviour: Behaviour,
}

impl Gossipsub {
    /// Creates a new gossipsub behaviour.
    pub fn new(local_peer_id: PeerId, keypair: Keypair, log: Logger) -> Self {
        let gossip_config = ConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10))
            .build()
            .unwrap();
        let gossip_behaviour =
            libp2p::gossipsub::Behaviour::new(MessageAuthenticity::Signed(keypair), gossip_config)
                .unwrap();

        Gossipsub { gossip_behaviour }
    }
}

impl NetworkBehaviour for Gossipsub {
    type ConnectionHandler = ConnectionHandler;
    type OutEvent = Void;

    fn new_handler(&mut self) -> Self::ConnectionHandler {
        ConnectionHandler
    }

    fn addresses_of_peer(&mut self, _: &PeerId) -> Vec<libp2p::Multiaddr> {
        Vec::new()
    }

    fn handle_established_inbound_connection(
        &mut self,
        _connection_id: libp2p::swarm::ConnectionId,
        peer: PeerId,
        local_addr: &libp2p::Multiaddr,
        remote_addr: &libp2p::Multiaddr,
    ) -> Result<libp2p::swarm::THandler<Self>, libp2p::swarm::ConnectionDenied> {
        Ok(libp2p::swarm::dummy::ConnectionHandler)
    }

    fn handle_established_outbound_connection(
        &mut self,
        _connection_id: libp2p::swarm::ConnectionId,
        peer: PeerId,
        addr: &libp2p::Multiaddr,
        role_override: libp2p::core::Endpoint,
    ) -> Result<libp2p::swarm::THandler<Self>, libp2p::swarm::ConnectionDenied> {
        Ok(libp2p::swarm::dummy::ConnectionHandler)
    }

    fn handle_pending_inbound_connection(
        &mut self,
        _connection_id: libp2p::swarm::ConnectionId,
        _local_addr: &libp2p::Multiaddr,
        _remote_addr: &libp2p::Multiaddr,
    ) -> Result<(), libp2p::swarm::ConnectionDenied> {
        Ok(())
    }

    fn handle_pending_outbound_connection(
        &mut self,
        _connection_id: libp2p::swarm::ConnectionId,
        maybe_peer: Option<PeerId>,
        _addresses: &[libp2p::Multiaddr],
        _effective_role: libp2p::core::Endpoint,
    ) -> Result<Vec<libp2p::Multiaddr>, libp2p::swarm::ConnectionDenied> {
        Ok(Vec::new())
    }

    fn on_connection_handler_event(
        &mut self,
        _peer_id: PeerId,
        _connection_id: libp2p::swarm::ConnectionId,
        _event: libp2p::swarm::THandlerOutEvent<Self>,
    ) {
    }

    fn on_swarm_event(&mut self, event: libp2p::swarm::FromSwarm<Self::ConnectionHandler>) {}

    fn poll(
        &mut self,
        cx: &mut std::task::Context<'_>,
        params: &mut impl libp2p::swarm::PollParameters,
    ) -> std::task::Poll<libp2p::swarm::ToSwarm<Self::OutEvent, libp2p::swarm::THandlerInEvent<Self>>>
    {
        std::task::Poll::Pending
    }
}
