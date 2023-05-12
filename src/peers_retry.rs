use base64;
use chrono::{DateTime, Local, TimeZone, Utc};
use colored::*;
use discv5::{
    enr,
    enr::{CombinedKey, NodeId},
    Discv5, Discv5ConfigBuilder, TokioExecutor,
};
use ethers::prelude::*;
use eyre::Result;
use futures::stream::{self, StreamExt};
use libp2p::kad::kbucket::{Entry, EntryRefView};
use libp2p::{
    core::upgrade, dns::DnsConfig, identity, kad::*, noise::*, swarm::*, yamux, Multiaddr, PeerId,
    Swarm, Transport,
};
use reqwest::header::{HeaderMap, ACCEPT};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::net::IpAddr;
use std::net::SocketAddr;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::path::Path;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::UnixStream;
use tokio::runtime::Handle;
use tokio::sync::mpsc;
use tokio::time::timeout;

#[derive(Debug)]
struct LogEntry {
    time: DateTime<Local>,
    level: LogLevel,
    message: String,
}

#[derive(Debug)]
#[allow(unused)]
enum LogLevel {
    Info,
    Warning,
    Error,
    Critical,
}

impl fmt::Display for LogEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let time_str: String = format!("{}", self.time.format("%m-%d|%H:%M:%S%.3f"));
        let msg_str: &str = self.message.as_str();

        let level_str: ColoredString = match self.level {
            LogLevel::Info => "INFO".green(),
            LogLevel::Warning => "WARN".yellow(),
            LogLevel::Error => "ERRO".red(),
            LogLevel::Critical => "CRIT".magenta(),
        };

        write!(f, "{} [{}] {}", level_str, time_str, msg_str)
    }
}

pub async fn discover_peers() -> Result<(), Box<dyn Error>> {
    // get the local peer_id from the beacon node

    // setup 
    Ok(())
}
