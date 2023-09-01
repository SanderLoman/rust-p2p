#![deny(unsafe_code)]

pub mod gossip;
pub mod identify;

use std::collections::HashMap;
use std::net::IpAddr;
use std::task::{Context, Poll};
use void::Void;

// use crate::discv5::discovery::Discovery;

use crate::create_logger;

use crate::discv5::discovery::Discovery as CustomDiscovery;
use crate::libp2p::behaviour::gossip::Gossipsub as CustomGossipsub;
use crate::libp2p::behaviour::identify::Identity as CustomIdentity;

use discv5::Enr;
use futures::StreamExt;
use libp2p::core::ConnectedPoint;
use libp2p::swarm::behaviour::{ConnectionClosed, ConnectionEstablished, DialFailure, FromSwarm};
use libp2p::swarm::dial_opts::{DialOpts, PeerCondition};
use libp2p::swarm::dummy::{Behaviour, ConnectionHandler};
use libp2p::swarm::{NetworkBehaviour, PollParameters, ToSwarm};
use libp2p::{Multiaddr, PeerId};
use slog::{debug, Logger};

#[derive(NetworkBehaviour)]
pub struct CustomBehavior {
    /// The routing pub-sub mechanism for eth2.
    pub gossipsub: CustomGossipsub,
    /// Discv5 Discovery protocol.
    pub discovery: CustomDiscovery,
    /// Provides IP addresses and peer information.
    pub identify: CustomIdentity,
}
