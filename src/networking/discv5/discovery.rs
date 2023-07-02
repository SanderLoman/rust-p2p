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
    DefaultProtocolId, Discv5, Discv5Config, Discv5ConfigBuilder, Discv5Error, Discv5Event, Enr,
    IpMode, TokioExecutor,
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

pub async fn setup_discv5() -> Result<Discv5, Box<dyn Error>> {
    let port: u16 = 7777;
    let ip = "0.0.0.0".parse::<std::net::Ipv4Addr>().unwrap();
    let listen_conf = ListenConfig::from_ip(std::net::IpAddr::V4(ip), port);
    
    let discv5_config = Discv5ConfigBuilder::new(listen_conf).build();

    let (enr, enr_key) = generate_enr().await?;

    let mut discv5: Discv5 = Discv5::new(enr.clone(), enr_key, discv5_config)?;
    discv5.start().await.expect("Discv5 failed to start");

    let enr_loc_id = discv5.local_enr().node_id();

    discv5.find_node(enr_loc_id).await.unwrap();

    discv5.event_stream().await.unwrap();
    Ok(discv5)
}

pub async fn discv5_events() {
    let discv5 = setup_discv5().await.unwrap();
    let mut event_stream = discv5.event_stream().await.unwrap();

    println!("Waiting for events...");

    while let Some(event) = event_stream.recv().await {
        match event {
            discv5::Discv5Event::Discovered(enr) => {
                println!("Discovered ENR: {}", enr);
                // handle the Discovered event here
            }
            discv5::Discv5Event::EnrAdded { enr, replaced } => {
                println!("Added ENR: {}, replaced: {:?}", enr, replaced);
                // handle the EnrAdded event here
            }
            discv5::Discv5Event::NodeInserted { node_id, replaced } => {
                println!("Inserted NodeId: {}, replaced: {:?}", node_id, replaced);
                // handle the NodeInserted event here
            }
            discv5::Discv5Event::SocketUpdated(addr) => {
                println!("Updated socket address: {}", addr);
                // handle the SocketUpdated event here
            }
            discv5::Discv5Event::SessionEstablished(enr, socket_addr) => {
                println!(
                    "Established session with ENR: {}, socketAddr: {}",
                    enr, socket_addr
                );
                // handle the SessionEstablished event here
            }
            discv5::Discv5Event::TalkRequest(node_id) => {
                println!("Received talk request from NodeId: {:?}", node_id);
                // handle the TalkRequest event here
            }
        }
    }
}
