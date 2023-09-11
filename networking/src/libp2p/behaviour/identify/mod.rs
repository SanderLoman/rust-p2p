#![deny(unsafe_code)]

use libp2p::identity::PublicKey;
use libp2p::swarm::dummy::ConnectionHandler;
use libp2p::swarm::{NetworkBehaviour, PollParameters, SwarmEvent, ToSwarm};
use slog::Logger;
use void::Void;

// pub struct Identity {
//     identify_behaviour: libp2p::identify::Behaviour,
// }

// impl Identity {
//     pub fn new(key: PublicKey, log: Logger) {}
// }

// impl NetworkBehaviour for Identity {
//     type ConnectionHandler = ConnectionHandler;
//     type OutEvent = Void;

//     fn new_handler(&mut self) -> Self::ConnectionHandler {
//         ConnectionHandler
//     }

//     fn addresses_of_peer(&mut self, _: &libp2p::PeerId) -> Vec<libp2p::Multiaddr> {
//         Vec::new()
//     }

//     fn handle_established_inbound_connection(
//         &mut self,
//         connection_id: libp2p::swarm::ConnectionId,
//         peer: libp2p::PeerId,
//         local_addr: &libp2p::Multiaddr,
//         remote_addr: &libp2p::Multiaddr,
//     ) -> Result<libp2p::swarm::THandler<Self>, libp2p::swarm::ConnectionDenied> {
//         Ok(libp2p::swarm::dummy::ConnectionHandler)
//     }

//     fn handle_established_outbound_connection(
//         &mut self,
//         connection_id: libp2p::swarm::ConnectionId,
//         peer: libp2p::PeerId,
//         addr: &libp2p::Multiaddr,
//         role_override: libp2p::core::Endpoint,
//     ) -> Result<libp2p::swarm::THandler<Self>, libp2p::swarm::ConnectionDenied> {
//         Ok(libp2p::swarm::dummy::ConnectionHandler)
//     }

//     fn handle_pending_inbound_connection(
//         &mut self,
//         connection_id: libp2p::swarm::ConnectionId,
//         local_addr: &libp2p::Multiaddr,
//         remote_addr: &libp2p::Multiaddr,
//     ) -> Result<(), libp2p::swarm::ConnectionDenied> {
//         Ok(())
//     }

//     fn handle_pending_outbound_connection(
//         &mut self,
//         connection_id: libp2p::swarm::ConnectionId,
//         maybe_peer: Option<libp2p::PeerId>,
//         addresses: &[libp2p::Multiaddr],
//         effective_role: libp2p::core::Endpoint,
//     ) -> Result<Vec<libp2p::Multiaddr>, libp2p::swarm::ConnectionDenied> {
//         Ok(Vec::new())
//     }

//     fn on_connection_handler_event(
//         &mut self,
//         peer_id: libp2p::PeerId,
//         connection_id: libp2p::swarm::ConnectionId,
//         event: libp2p::swarm::THandlerOutEvent<Self>,
//     ) {
//     }

//     fn on_swarm_event(&mut self, event: libp2p::swarm::FromSwarm<Self::ConnectionHandler>) {}

//     fn poll(
//         &mut self,
//         cx: &mut std::task::Context<'_>,
//         params: &mut impl libp2p::swarm::PollParameters,
//     ) -> std::task::Poll<libp2p::swarm::ToSwarm<Self::OutEvent, libp2p::swarm::THandlerInEvent<Self>>>
//     {
//         std::task::Poll::Pending
//     }
// }
