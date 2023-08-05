use std::collections::HashMap;
use std::task::{Context, Poll};

use discv5::Enr;
use futures::StreamExt;
use libp2p::core::ConnectedPoint;
use libp2p::swarm::behaviour::{ConnectionClosed, ConnectionEstablished, DialFailure, FromSwarm};
use libp2p::swarm::dial_opts::{DialOpts, PeerCondition};
use libp2p::swarm::dummy::ConnectionHandler;
use libp2p::swarm::{NetworkBehaviour, PollParameters};
use libp2p::{PeerId, Multiaddr};
use slog::{debug, Logger};
use crate::create_logger;

const LOG: Logger = create_logger();

#[derive(Debug, Clone)]
pub enum ConnectionDirection {
    /// The connection was established by a peer dialing us.
    Incoming,
    /// The connection was established by us dialing a peer.
    Outgoing,
}

#[derive(Debug)]
enum NewConnectionState {
    /// A peer has connected to us.
    Connected {
        /// An optional known ENR if the peer was dialed.
        enr: Option<Enr>,
        /// The seen socket address associated with the connection.
        seen_address: Multiaddr,
        /// The direction, incoming/outgoing.
        direction: ConnectionDirection,
    },
    /// The peer is in the process of being disconnected.
    Disconnecting {
        /// Whether the peer should be banned after the disconnect occurs.
        to_ban: bool,
    },
    /// We are dialing this peer.
    Dialing {
        /// An optional known ENR for the peer we are dialing.
        enr: Option<Enr>,
    },
    /// The peer has been disconnected from our local node.
    Disconnected,
    /// The peer has been banned and actions to shift the peer to the banned state should be
    /// undertaken
    Banned,
    /// The peer has been unbanned and the connection state should be updated to reflect this.
    Unbanned,
}

enum ConnectingType {
    /// We are in the process of dialing this peer.
    Dialing,
    /// A peer has dialed us.
    IngoingConnected {
        // The multiaddr the peer connected to us on.
        multiaddr: Multiaddr,
    },
    /// We have successfully dialed a peer.
    OutgoingConnected {
        /// The multiaddr we dialed to reach the peer.
        multiaddr: Multiaddr,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum GoodbyeReason {
    /// This node has shutdown.
    ClientShutdown = 1,

    /// Incompatible networks.
    IrrelevantNetwork = 2,

    /// Error/fault in the RPC.
    Fault = 3,

    /// Teku uses this code for not being able to verify a network.
    UnableToVerifyNetwork = 128,

    /// The node has too many connected peers.
    TooManyPeers = 129,

    /// Scored poorly.
    BadScore = 250,

    /// The peer is banned
    Banned = 251,

    /// The IP address the peer is using is banned.
    BannedIP = 252,

    /// Unknown reason.
    Unknown = 0,
}

#[derive(NetworkBehaviour)]
pub struct CustomBehaviour;

impl NetworkBehaviour for CustomBehaviour {
    type ConnectionHandler = ConnectionHandler;

    type OutEvent = ;

    /* Required trait members */

    fn new_handler(&mut self) -> Self::ConnectionHandler {
        ConnectionHandler
    }

    fn poll(
        &mut self,
        cx: &mut Context<'_>,
        _params: &mut impl PollParameters,
    ) -> Poll<NetworkBehaviourAction<Self::OutEvent, Self::ConnectionHandler>> {
        Poll::Pending
    }

    fn on_swarm_event(&mut self, event: FromSwarm<Self::ConnectionHandler>) {
        match event {
            FromSwarm::ConnectionEstablished(ConnectionEstablished {
                peer_id,
                endpoint,
                other_established,
                connection_id,
                failed_addresses,
            }) => self.on_connection_established(peer_id, endpoint, other_established),
            FromSwarm::ConnectionClosed(ConnectionClosed {
                peer_id,
                remaining_established,
                handler,
                connection_id,
                endpoint,
            }) => self.on_connection_closed(peer_id, remaining_established),
            FromSwarm::DialFailure(DialFailure { peer_id, .. }) => self.on_dial_failure(peer_id),
            FromSwarm::AddressChange(_)
            | FromSwarm::ListenFailure(_)
            | FromSwarm::NewListener(_)
            | FromSwarm::NewListenAddr(_)
            | FromSwarm::ExpiredListenAddr(_)
            | FromSwarm::ListenerError(_)
            | FromSwarm::ListenerClosed(_)
            | FromSwarm::NewExternalAddr(_)
            | FromSwarm::ExpiredExternalAddr(_) => {
                // The rest of the events we ignore since they are handled in their associated
                // `SwarmEvent`
            }
        }
    }
}

impl CustomBehaviour {
    fn on_connection_established(
        &mut self,
        peer_id: PeerId,
        endpoint: &ConnectedPoint,
        other_established: usize,
    ) {
        debug!(LOG, "Connection established"; "peer_id" => %peer_id, "connection" => ?endpoint.to_endpoint());

        // NOTE: We don't register peers that we are disconnecting immediately. The network service
        // does not need to know about these peers.
        match endpoint {
            ConnectedPoint::Listener { send_back_addr, local_addr } => {
                self.inject_connect_ingoing(&peer_id, send_back_addr.clone(), local_addr.clone());
                debug!(LOG, "Connection established"; "peer_id" => %peer_id, "connection" => ?send_back_addr, "local_addr" => ?local_addr);
            }
            ConnectedPoint::Dialer { address, role_override } => {
                self.inject_connect_outgoing(&peer_id, address.clone(), role_override.clone());
                debug!(LOG, "Connection established"; "peer_id" => %peer_id, "connection" => ?address, "role" => ?role_override);
            }
        }

        // increment prometheus metrics
        self.update_connected_peer_metrics();
    }

    fn on_connection_closed(&mut self, peer_id: PeerId, remaining_established: usize) {
        debug!(self.log, "Peer disconnected"; "peer_id" => %peer_id);

        // NOTE: It may be the case that a rejected node, due to too many peers is disconnected
        // here and the peer manager has no knowledge of its connection. We insert it here for
        // reference so that peer manager can track this peer.
        self.inject_disconnect(&peer_id);

        // Update the prometheus metrics
        self.update_connected_peer_metrics();
    }

    /// A dial attempt has failed.
    ///
    /// NOTE: It can be the case that we are dialing a peer and during the dialing process the peer
    /// connects and the dial attempt later fails. To handle this, we only update the peer_db if
    /// the peer is not already connected.
    fn on_dial_failure(&mut self, peer_id: Option<PeerId>) {
        if let Some(peer_id) = peer_id {
            self.inject_disconnect(&peer_id);
        }
    }

    fn inject_connect_ingoing(
        &mut self,
        peer_id: &PeerId,
        enr: Option<Enr>,
        multiaddr: Multiaddr,
    ) -> bool {
        self.inject_peer_connection(peer_id, ConnectingType::IngoingConnected { multiaddr }, enr)
    }

    /// Sets a peer as connected as long as their reputation allows it
    /// Informs if the peer was accepted
    fn inject_connect_outgoing(
        &mut self,
        peer_id: &PeerId,
        multiaddr: Multiaddr,
        enr: Option<Enr>,
    ) -> bool {
        self.inject_peer_connection(
            peer_id,
            ConnectingType::OutgoingConnected { multiaddr },
            enr,
        )
    }

    /// Updates the state of the peer as disconnected.
    ///
    /// This is also called when dialing a peer fails.
    fn inject_disconnect(&mut self, peer_id: &PeerId) {
        self.handle_ban_operation(peer_id, ban_operation, None);
    }

    /// Registers a peer as connected. The `ingoing` parameter determines if the peer is being
    /// dialed or connecting to us.
    ///
    /// This is called by `connect_ingoing` and `connect_outgoing`.
    ///
    /// Informs if the peer was accepted in to the db or not.
    fn inject_peer_connection(
        &mut self,
        peer_id: &PeerId,
        connection: ConnectingType,
        enr: Option<Enr>,
    ) -> bool {
        {
            match connection {
                ConnectingType::Dialing => {
                    
                }
                ConnectingType::IngoingConnected { multiaddr } => {
                }
                ConnectingType::OutgoingConnected { multiaddr } => {
                }
            }
        }

        // start a ping and status timer for the peer
        self.status_peers.insert(*peer_id);

        let connected_peers = self.network_globals.connected_peers() as i64;

        // increment prometheus metrics
        metrics::inc_counter(&metrics::PEER_CONNECT_EVENT_COUNT);
        metrics::set_gauge(&metrics::PEERS_CONNECTED, connected_peers);

        true
    }

    // Gracefully disconnects a peer without banning them.
    fn disconnect_peer(&mut self, peer_id: PeerId, reason: GoodbyeReason) {
        self.events
            .push(PeerManagerEvent::DisconnectPeer(peer_id, reason));
        self.network_globals
            .peers
            .write()
            .notify_disconnecting(&peer_id, false);
    }
    
    fn update_connected_peer_metrics(&self) {
        let mut connected_peer_count = 0;
        let mut inbound_connected_peers = 0;
        let mut outbound_connected_peers = 0;
        let mut clients_per_peer = HashMap::new();

        connected_peer_count += 1;
    }

    fn dialing_peer(&mut self, peer_id: &PeerId, enr: Option<Enr>) {
        self.update_connection_state(peer_id, NewConnectionState::Dialing { enr });
    }

    /// Sets a peer as connected with an ingoing connection.
    // VISIBILITY: Only the peer manager can adjust the connection state.
    fn connect_ingoing(
        &mut self,
        peer_id: &PeerId,
        seen_address: Multiaddr,
        enr: Option<Enr>,
    ) {
        self.update_connection_state(
            peer_id,
            NewConnectionState::Connected {
                enr,
                seen_address,
                direction: ConnectionDirection::Incoming,
            },
        );
    }

    /// Sets a peer as connected with an outgoing connection.
    // VISIBILITY: Only the peer manager can adjust the connection state.
    pub(super) fn connect_outgoing(
        &mut self,
        peer_id: &PeerId,
        seen_address: Multiaddr,
        enr: Option<Enr>,
    ) {
        self.update_connection_state(
            peer_id,
            NewConnectionState::Connected {
                enr,
                seen_address,
                direction: ConnectionDirection::Outgoing,
            },
        );
    }
}