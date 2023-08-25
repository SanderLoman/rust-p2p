// use std::collections::HashMap;
// use std::net::IpAddr;
// use std::task::{Context, Poll};
// use void::Void;

// // use crate::discv5::discovery::Discovery;

// use super::eth2rpc;
// use super::gossip;
// use super::identify;
// use crate::create_logger;

// use discv5::Enr;
// use futures::StreamExt;
// use libp2p::core::ConnectedPoint;
// use libp2p::identify::Behaviour as Identify;
// use libp2p::swarm::behaviour::{ConnectionClosed, ConnectionEstablished, DialFailure, FromSwarm};
// use libp2p::swarm::dial_opts::{DialOpts, PeerCondition};
// use libp2p::swarm::dummy::{Behaviour, ConnectionHandler};
// use libp2p::swarm::{NetworkBehaviour, PollParameters, ToSwarm};
// use libp2p::{Multiaddr, PeerId};
// use slog::{debug, Logger};

// // #[derive(NetworkBehaviour)]
// pub struct CustomBehavior {
//     /// The routing pub-sub mechanism for eth2.
//     pub gossipsub: super::gossip::Gossipsub,
//     /// Discv5 Discovery protocol.
//     pub discovery: crate::discv5::discovery::Discovery,
//     /// Keep regular connection to peers and disconnect if absent.
//     // NOTE: The id protocol is used for initial interop. This will be removed by mainnet.
//     /// Provides IP addresses and peer information.
//     pub identify: Identify,
// }
