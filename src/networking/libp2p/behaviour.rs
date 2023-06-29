#![deny(unsafe_code)]

use crate::networking::discv5::enr::*;
use async_std::task;
use base64::prelude::*;
use chrono::{DateTime, Local, TimeZone, Utc};
use colored::*;
use discv5::{
    enr,
    enr::{ed25519_dalek, k256, CombinedKey, CombinedPublicKey, EnrBuilder, NodeId},
    socket::ListenConfig,
    Discv5, Discv5Config, Discv5ConfigBuilder, Discv5Error, Discv5Event, Enr, TokioExecutor,
};
use ethers::prelude::*;
use eyre::Result;
use futures::stream::{self, StreamExt};
use futures::Future;
use generic_array::GenericArray;
use hex::*;
use libp2p::core::{identity::PublicKey, multiaddr::Protocol};
use libp2p::kad::kbucket::{Entry, EntryRefView};
use libp2p::{
    autonat::*,
    core::upgrade,
    dns::DnsConfig,
    floodsub::*,
    identity,
    identity::Keypair,
    kad::*,
    multiaddr,
    noise::*,
    ping,
    swarm::behaviour::*,
    swarm::*,
    swarm::{dummy::ConnectionHandler, NetworkBehaviour},
    yamux, Multiaddr, PeerId, Swarm, Transport,
};
use pnet::packet::ip;
use rand::thread_rng;
use reqwest::header::{HeaderMap, ACCEPT};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use ssz::*;
use ssz_derive::{Decode, Encode};
use ssz_types::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4};
use std::path::Path;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::macros::support::Pin;
use tokio::net::UnixStream;
use tokio::runtime::Handle;
use tokio::sync::mpsc;
use tokio::time::timeout;

struct Behaviour;

impl NetworkBehaviour for Behaviour {
    type ConnectionHandler = ConnectionHandler;
    type OutEvent = ();

    fn addresses_of_peer(&mut self, _: &PeerId) -> Vec<Multiaddr> {
        Vec::new()
    }

    fn handle_established_inbound_connection(
        &mut self,
        _connection_id: ConnectionId,
        peer: PeerId,
        local_addr: &Multiaddr,
        remote_addr: &Multiaddr,
    ) -> std::result::Result<THandler<Self>, ConnectionDenied> {
        Ok(ConnectionHandler)
    }

    fn handle_established_outbound_connection(
        &mut self,
        _connection_id: ConnectionId,
        peer: PeerId,
        addr: &Multiaddr,
        role_override: libp2p::core::Endpoint,
    ) -> std::result::Result<THandler<Self>, ConnectionDenied> {
        Ok(ConnectionHandler)
    }

    fn handle_pending_inbound_connection(
        &mut self,
        _connection_id: ConnectionId,
        _local_addr: &Multiaddr,
        _remote_addr: &Multiaddr,
    ) -> std::result::Result<(), ConnectionDenied> {
        Ok(())
    }

    fn handle_pending_outbound_connection(
        &mut self,
        _connection_id: ConnectionId,
        maybe_peer: Option<PeerId>,
        _addresses: &[Multiaddr],
        _effective_role: libp2p::core::Endpoint,
    ) -> std::result::Result<Vec<Multiaddr>, ConnectionDenied> {
        Ok(Vec::new())
    }

    fn new_handler(&mut self) -> Self::ConnectionHandler {
        ConnectionHandler
    }

    fn on_connection_handler_event(
        &mut self,
        _peer_id: PeerId,
        _connection_id: ConnectionId,
        _event: THandlerOutEvent<Self>,
    ) {
    }

    fn on_swarm_event(&mut self, event: FromSwarm<Self::ConnectionHandler>) {
        match event {
            FromSwarm::ConnectionEstablished(_) => {}
            FromSwarm::ConnectionClosed(_) => {}
            FromSwarm::DialFailure(_) => {}
            FromSwarm::AddressChange(_) => {}
            FromSwarm::ListenFailure(_) => {}
            FromSwarm::NewListener(_) => {}
            FromSwarm::NewListenAddr(_) => {}
            FromSwarm::ExpiredListenAddr(_) => {}
            FromSwarm::ListenerError(_) => {}
            FromSwarm::ListenerClosed(_) => {}
            FromSwarm::NewExternalAddr(_) => {}
            FromSwarm::ExpiredExternalAddr(_) => {}
        }
    }

    fn poll(
        &mut self,
        cx: &mut Context<'_>,
        params: &mut impl PollParameters,
    ) -> Poll<ToSwarm<Self::OutEvent, THandlerInEvent<Self>>> {
        Poll::Pending
    }
}

pub async fn custom_behaviour() {}
