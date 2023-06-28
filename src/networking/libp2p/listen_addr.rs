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
use libp2p::gossipsub::{
    FastMessageId, GossipsubConfig, GossipsubConfigBuilder, GossipsubMessage, MessageId,
    RawGossipsubMessage, ValidationMode,
};
use libp2p::kad::kbucket::{Entry, EntryRefView};
use libp2p::{
    autonat::*, core::upgrade, dns::DnsConfig, floodsub::*, identity, identity::Keypair, kad::*,
    multiaddr, noise::*, ping, swarm::behaviour::*, swarm::*, yamux, Multiaddr, PeerId, Swarm,
    Transport,
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
use std::path::PathBuf;
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

pub async fn setup_listen_addr() -> Result<Multiaddr, Box<dyn Error>> {
    let listen_addr: Multiaddr = "/ip4/83.128.33.146/tcp/7777"
        .parse()
        .expect("Failed to parse multiaddr");
    Ok(listen_addr)
}
