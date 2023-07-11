#![deny(unsafe_code)]

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
    autonat::*, core::upgrade, dns::DnsConfig, floodsub::*, identity, identity::Keypair, kad::*,
    multiaddr, noise::*, ping, swarm::behaviour::*, swarm::NetworkBehaviour, swarm::*, yamux,
    Multiaddr, PeerId, Swarm, Transport,
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
use slog::*;

use super::behaviour::*;
use super::listen_addr::*;
use super::transport::*;

pub async fn setup_swarm() -> Result<(), Box<dyn Error>> {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, slog::o!());
    let upgraded_transport = setup_transport().await.unwrap();

    let behaviour = dummy::Behaviour;

    let libp2p_local_key = Keypair::generate_secp256k1();
    let libp2p_local_peer_id = PeerId::from(libp2p_local_key.public());

    let executor = move |fut: Pin<Box<dyn Future<Output = ()> + Send + 'static>>| {
        tokio::spawn(fut);
    };

    let mut swarm: Swarm<dummy::Behaviour> = SwarmBuilder::with_executor(
        upgraded_transport,
        behaviour,
        libp2p_local_peer_id,
        executor,
    )
    .build();

    let listen_addr = setup_listen_addr().await.unwrap();
    info!(log, "Listening on {:?}", listen_addr);

    Ok(())
}
