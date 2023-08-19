use std::collections::HashMap;
use std::net::IpAddr;
use std::task::{Context, Poll};
use void::Void;

use crate::create_logger;
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
    
}
