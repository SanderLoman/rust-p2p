pub mod discovery;
pub mod metrics_for_task_executor;
pub mod network_manager;
pub mod swarm;
pub mod task_executor;
pub mod types;

use std::error::Error;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

use discv5::rpc::{Request, Response};
use futures::channel::mpsc;
use futures::StreamExt;
use libp2p::gossipsub::{MessageId, TopicHash};
use libp2p::identity::Keypair;
use libp2p::swarm::{ConnectionId, SwarmEvent};
use libp2p::tcp::Config;
use libp2p::{identify, Multiaddr, Swarm};
use libp2p::{PeerId, SwarmBuilder};
use slog::{crit, debug, trace, warn, Logger};

use crate::network::swarm::behaviour::Behaviour;
use crate::network::swarm::build_swarm::build_swarm;
use crate::version_with_platform;

use self::discovery::{DiscoveredPeers, Discovery};
use self::swarm::behaviour::BehaviourEvent;
use self::types::network_globals::{Enr, NetworkGlobals};

pub type PeerRequestId = (ConnectionId, SubstreamId);

/// Identifier of inbound and outbound substreams from the handler's perspective.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct SubstreamId(usize);

pub trait ReqId: Send + 'static + std::fmt::Debug + Copy + Clone {}
impl<T> ReqId for T where T: Send + 'static + std::fmt::Debug + Copy + Clone {}

#[derive(Debug, Clone, PartialEq)]
pub enum PubsubMessage {
    // !!!
    // These are placeholder types for now.
    // !!!
    /// Gossipsub message providing notification of a new block.
    BeaconBlock(Arc<()>),
    /// Gossipsub message providing notification of a Aggregate attestation and associated proof.
    AggregateAndProofAttestation(Arc<()>),
    /// Gossipsub message providing notification of a raw un-aggregated attestation with its shard id.
    Attestation(Arc<()>),
    /// Gossipsub message providing notification of a voluntary exit.
    VoluntaryExit(Arc<()>),
    /// Gossipsub message providing notification of a new proposer slashing.
    ProposerSlashing(Arc<()>),
    /// Gossipsub message providing notification of a new attester slashing.
    AttesterSlashing(Arc<()>),
    /// Gossipsub message providing notification of partially aggregated sync committee signatures.
    SignedContributionAndProof(Arc<()>),
    /// Gossipsub message providing notification of unaggregated sync committee signatures with its subnet id.
    SyncCommitteeMessage(Arc<()>),
    /// Gossipsub message for BLS to execution change messages.
    BlsToExecutionChange(Arc<()>),
    /// Gossipsub message providing notification of a light client finality update.
    LightClientFinalityUpdate(Arc<()>),
    /// Gossipsub message providing notification of a light client optimistic update.
    LightClientOptimisticUpdate(Arc<()>),
}

/// The types of events than can be obtained from polling the behaviour.
#[derive(Debug)]
pub enum NetworkEvent<AppReqId: ReqId> {
    /// We have successfully dialed and connected to a peer.
    PeerConnectedOutgoing(PeerId),
    /// A peer has successfully dialed and connected to us.
    PeerConnectedIncoming(PeerId),
    /// A peer has disconnected.
    PeerDisconnected(PeerId),
    /// An RPC Request that was sent failed.
    RPCFailed {
        /// The id of the failed request.
        id: AppReqId,
        /// The peer to which this request was sent.
        peer_id: PeerId,
    },
    RequestReceived {
        /// The peer that sent the request.
        peer_id: PeerId,
        /// Identifier of the request. All responses to this request must use this id.
        id: PeerRequestId,
        /// Request the peer sent.
        request: Request,
    },
    ResponseReceived {
        /// Peer that sent the response.
        peer_id: PeerId,
        /// Id of the request to which the peer is responding.
        id: AppReqId,
        /// Response the peer sent.
        response: Response,
    },
    PubsubMessage {
        /// The gossipsub message id. Used when propagating blocks after validation.
        id: MessageId,
        /// The peer from which we received this message, not the peer that published it.
        source: PeerId,
        /// The topic that this message was sent on.
        topic: TopicHash,
        /// The message itself.
        message: PubsubMessage,
    },
    /// Inform the network to send a Status to this peer.
    StatusPeer(PeerId),
    NewListenAddr(Multiaddr),
    ZeroListeners,
}

pub struct Network {
    // The libp2p Swarm, this will handle incoming and outgoing requests so that we can redirect them. Instead of sending data right back to them
    swarm: Swarm<Behaviour>,

    /// A collections of variables accessible outside the network service.
    network_globals: Arc<NetworkGlobals>,

    // The Logger for the network service.
    log: Logger,
}

impl Network {
    pub async fn new(local_keypair: Keypair, log: Logger) -> Result<Network, Box<dyn Error>> {
        let network_globals = {
            // Create an ENR or load from disk if appropriate
            let enr = crate::network::discovery::enr::generate_enr(log.clone()).await;
            // Create the network globals
            let globals = NetworkGlobals::new(enr, &log);
            Arc::new(globals)
        };

        let identify = {
            let local_public_key = local_keypair.public().clone().into();
            let identify_config = identify::Config::new("eth2/1.0.0".into(), local_public_key)
                .with_agent_version(version_with_platform())
                .with_cache_size(0);
            identify::Behaviour::new(identify_config)
        };

        // Build and start the discovery sub-behaviour
        let discovery = Discovery::new(local_keypair.clone(), log.clone())
            .await
            .unwrap();

        let behaviour = {
            Behaviour {
                discovery,
                identify,
            }
        };

        let local_peer_id = network_globals.local_peer_id();

        // Create a runtime for executing asynchronous tasks.
        let runtime = Arc::new(tokio::runtime::Runtime::new().unwrap());

        // Create a channel for shutdown signaling.
        let (signal_tx, _signal_rx) = mpsc::channel(1); // Adjust the channel size as needed

        // Set up shutdown signaling.
        let (_, exit) = exit_future::signal();

        // Create a TaskExecutor instance.
        let handle = Arc::downgrade(&runtime);
        let executor = task_executor::TaskExecutor::new(handle, exit, log.clone(), signal_tx);

        let swarm = build_swarm(local_keypair, behaviour, local_peer_id, executor).unwrap();

        let mut network = Network {
            swarm,
            network_globals,
            log,
        };

        Ok(network)
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    /// Discv5 Discovery protocol.
    pub fn discovery_mut(&mut self) -> &mut Discovery {
        &mut self.swarm.behaviour_mut().discovery
    }

    /// Provides IP addresses and peer information.
    pub fn identify_mut(&mut self) -> &mut identify::Behaviour {
        &mut self.swarm.behaviour_mut().identify
    }

    /// Discv5 Discovery protocol.
    pub fn discovery(&self) -> &Discovery {
        &self.swarm.behaviour().discovery
    }
    /// Provides IP addresses and peer information.
    pub fn identify(&self) -> &identify::Behaviour {
        &self.swarm.behaviour().identify
    }

    /// Returns the local ENR of the node.
    pub fn local_enr(&self) -> Enr {
        self.network_globals.local_enr()
    }

    pub fn poll_network(&mut self, cx: &mut Context) -> Poll<NetworkEvent> {
        while let Poll::Ready(Some(swarm_event)) = self.swarm.poll_next_unpin(cx) {
            let maybe_event = match swarm_event {
                SwarmEvent::Behaviour(behaviour_event) => match behaviour_event {
                    BehaviourEvent::Discovery(_) => None,
                    BehaviourEvent::Identify(_) => None,
                },
                SwarmEvent::ConnectionEstablished { .. } => None,
                SwarmEvent::ConnectionClosed { .. } => None,
                SwarmEvent::IncomingConnection {
                    local_addr,
                    send_back_addr,
                    connection_id: _,
                } => {
                    trace!(self.log, "Incoming connection"; "our_addr" => %local_addr, "from" => %send_back_addr);
                    None
                }
                SwarmEvent::IncomingConnectionError {
                    local_addr,
                    send_back_addr,
                    error,
                    connection_id: _,
                } => {
                    let error_repr = match error {
                        libp2p::swarm::ListenError::Aborted => {
                            "Incoming connection aborted".to_string()
                        }
                        libp2p::swarm::ListenError::WrongPeerId { obtained, endpoint } => {
                            format!("Wrong peer id, obtained {obtained}, endpoint {endpoint:?}")
                        }
                        libp2p::swarm::ListenError::LocalPeerId { endpoint } => {
                            format!("Dialing local peer id {endpoint:?}")
                        }
                        libp2p::swarm::ListenError::Denied { cause } => {
                            format!("Connection was denied with cause: {cause:?}")
                        }
                        libp2p::swarm::ListenError::Transport(t) => match t {
                            libp2p::TransportError::MultiaddrNotSupported(m) => {
                                format!("Transport error: Multiaddr not supported: {m}")
                            }
                            libp2p::TransportError::Other(e) => {
                                format!("Transport error: other: {e}")
                            }
                        },
                    };
                    debug!(self.log, "Failed incoming connection"; "our_addr" => %local_addr, "from" => %send_back_addr, "error" => error_repr);
                    None
                }
                SwarmEvent::OutgoingConnectionError {
                    peer_id: _,
                    error: _,
                    connection_id: _,
                } => {
                    // The Behaviour event is more general than the swarm event here. It includes
                    // connection failures. So we use that log for now, in the peer manager
                    // behaviour implementation.
                    None
                }
                SwarmEvent::NewListenAddr { address, .. } => {
                    Some(NetworkEvent::NewListenAddr(address))
                }
                SwarmEvent::ExpiredListenAddr { address, .. } => {
                    debug!(self.log, "Listen address expired"; "address" => %address);
                    None
                }
                SwarmEvent::ListenerClosed {
                    addresses, reason, ..
                } => {
                    crit!(self.log, "Listener closed"; "addresses" => ?addresses, "reason" => ?reason);
                    if Swarm::listeners(&self.swarm).count() == 0 {
                        Some(NetworkEvent::ZeroListeners)
                    } else {
                        None
                    }
                }
                SwarmEvent::ListenerError { error, .. } => {
                    // this is non fatal, but we still check
                    warn!(self.log, "Listener error"; "error" => ?error);
                    if Swarm::listeners(&self.swarm).count() == 0 {
                        Some(NetworkEvent::ZeroListeners)
                    } else {
                        None
                    }
                }
                SwarmEvent::Dialing { .. } => None,
            };

            if let Some(ev) = maybe_event {
                return Poll::Ready(ev);
            }
        }

        Poll::Pending
    }

    pub async fn next_event(&mut self) -> NetworkEvent {
        futures::future::poll_fn(|cx| self.poll_network(cx)).await
    }
}
