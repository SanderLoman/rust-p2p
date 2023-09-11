#![deny(unsafe_code)]

pub mod enr;
pub mod events;

use super::discovery::enr::*;
use super::discovery::events::discv5_events;

use clap::{App, Arg};
use discv5::*;
use discv5::{
    enr as discv5_enr, enr::CombinedKey, handler, kbucket, metrics, packet, permit_ban, rpc,
    service, socket, Discv5, Discv5Config, Discv5ConfigBuilder, Discv5Event, Enr, ListenConfig,
};
use eyre::Result;
use futures::Future;
use libp2p::identity::{secp256k1, KeyType, Keypair};
use libp2p::multiaddr::Protocol;
use libp2p::swarm::dummy::ConnectionHandler;
use libp2p::swarm::{DialError, DialFailure, FromSwarm, NetworkBehaviour, PollParameters, ToSwarm};
use libp2p::{Multiaddr, PeerId};
use lru::LruCache;
use slog::{Logger, *};
use std::error::Error;
use std::net::{IpAddr, Ipv4Addr};
use std::num::NonZeroUsize;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::sync::mpsc;
use void::Void;

pub struct Discovery {
    /// A collection of seen live ENRs for quick lookup and to map peer-id's to ENRs.
    cached_enrs: LruCache<PeerId, Enr>,

    /// The directory where the ENR is stored.
    enr: Enr,

    /// The handle for the underlying discv5 Server.
    ///
    /// This is behind a Reference counter to allow for futures to be spawned and polled with a
    /// static lifetime.
    discv5: Discv5,

    /// The discv5 event stream.
    event_stream: EventStream,

    /// Logger for the discovery behaviour.
    log: slog::Logger,
}

enum EventStream {
    /// Awaiting an event stream to be generated. This is required due to the poll nature of
    /// `Discovery`
    Awaiting(
        Pin<
            Box<
                dyn Future<Output = Result<mpsc::Receiver<Discv5Event>, discv5::Discv5Error>>
                    + Send,
            >,
        >,
    ),
    /// The future has completed.
    Present(mpsc::Receiver<Discv5Event>),
    // The future has failed or discv5 has been disabled. There are no events from discv5.
    InActive,
}

impl Discovery {
    pub async fn new(log: slog::Logger) -> Result<Self, Box<dyn Error>> {
        let log_clone = log.clone();
        let (local_enr, enr, enr_key) = generate_enr(log_clone).await?;

        let listen_port = enr.udp4().unwrap();

        let discv5_listen_config =
            discv5::ListenConfig::from_ip(Ipv4Addr::UNSPECIFIED.into(), listen_port);

        let discv5_config = Discv5ConfigBuilder::new(discv5_listen_config)
            .ban_duration(Some(Duration::from_secs(60)))
            .query_timeout(Duration::from_secs(10))
            .request_retries(1)
            .request_timeout(Duration::from_secs(1))
            .query_parallelism(3)
            .query_peer_timeout(Duration::from_secs(3))
            .ping_interval(Duration::from_secs(300))
            .build();

        let discv5: Discv5 = Discv5::new(enr.clone(), enr_key, discv5_config)?;
        let cached_enrs = LruCache::new(NonZeroUsize::new(1000).unwrap());

        Ok(Discovery {
            cached_enrs,
            enr,
            discv5,
            event_stream: EventStream::InActive,
            log,
        })
    }

    pub fn get_enr(&self) -> Enr {
        self.discv5.local_enr()
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn Error>> {
        self.discv5.start().await.unwrap();

        Ok(())
    }
}

impl NetworkBehaviour for Discovery {
    type ConnectionHandler = ConnectionHandler;
    type OutEvent = Void;

    fn new_handler(&mut self) -> Self::ConnectionHandler {
        ConnectionHandler
    }

    fn addresses_of_peer(&mut self, _: &PeerId) -> Vec<libp2p::Multiaddr> {
        Vec::new()
    }

    fn handle_established_inbound_connection(
        &mut self,
        connection_id: libp2p::swarm::ConnectionId,
        peer: PeerId,
        local_addr: &libp2p::Multiaddr,
        remote_addr: &libp2p::Multiaddr,
    ) -> Result<libp2p::swarm::THandler<Self>, libp2p::swarm::ConnectionDenied> {
        Ok(ConnectionHandler)
    }

    fn handle_established_outbound_connection(
        &mut self,
        connection_id: libp2p::swarm::ConnectionId,
        peer: PeerId,
        addr: &libp2p::Multiaddr,
        role_override: libp2p::core::Endpoint,
    ) -> Result<libp2p::swarm::THandler<Self>, libp2p::swarm::ConnectionDenied> {
        Ok(ConnectionHandler)
    }

    fn handle_pending_inbound_connection(
        &mut self,
        connection_id: libp2p::swarm::ConnectionId,
        local_addr: &libp2p::Multiaddr,
        remote_addr: &libp2p::Multiaddr,
    ) -> Result<(), libp2p::swarm::ConnectionDenied> {
        Ok(())
    }

    fn handle_pending_outbound_connection(
        &mut self,
        connection_id: libp2p::swarm::ConnectionId,
        maybe_peer: Option<PeerId>,
        addresses: &[libp2p::Multiaddr],
        effective_role: libp2p::core::Endpoint,
    ) -> Result<Vec<libp2p::Multiaddr>, libp2p::swarm::ConnectionDenied> {
        Ok(Vec::new())
    }

    fn on_connection_handler_event(
        &mut self,
        peer_id: PeerId,
        connection_id: libp2p::swarm::ConnectionId,
        event: libp2p::swarm::THandlerOutEvent<Self>,
    ) {
    }

    fn poll(
        &mut self,
        cx: &mut Context,
        params: &mut impl PollParameters,
    ) -> std::task::Poll<libp2p::swarm::ToSwarm<Self::OutEvent, libp2p::swarm::THandlerInEvent<Self>>>
    {
        match self.event_stream {
            EventStream::Awaiting(ref mut fut) => match fut.as_mut().poll(cx) {
                Poll::Ready(Ok(event_stream)) => {
                    info!(self.log, "Discv5 event stream started");
                    self.event_stream = EventStream::Present(event_stream);
                }
                Poll::Ready(Err(e)) => {
                    warn!(self.log, "Discv5 event stream failed"; "error" => format!("{:?}", e));
                    self.event_stream = EventStream::InActive;
                }
                Poll::Pending => return Poll::Pending,
            },
            EventStream::InActive => {} // ignore checking the event stream
            EventStream::Present(ref mut stream) => {
                while let Poll::Ready(Some(event)) = stream.poll_recv(cx) {
                    match event {
                        Discv5Event::Discovered(enr) => {
                            slog::debug!(self.log, "Discovered peer"; "peer_id" => enr.to_string());
                        }
                        Discv5Event::SocketUpdated(socket_addr) => {
                            slog::debug!(self.log, "Socket updated"; "socket_addr" => format!("{:?}", socket_addr));
                        }
                        Discv5Event::EnrAdded { enr, replaced } => {
                            slog::debug!(self.log, "ENR added"; "peer_id" => enr.to_string());
                            println!("Replaced: {:?}", replaced);
                        }
                        Discv5Event::TalkRequest(TalkRequest) => {}
                        Discv5Event::NodeInserted { node_id, replaced } => {}
                        Discv5Event::SessionEstablished(enr, socketaddr) => {}
                    }
                }
            }
        }
        Poll::Pending
    }

    fn on_swarm_event(&mut self, event: libp2p::swarm::FromSwarm<Self::ConnectionHandler>) {
        match event {
            FromSwarm::DialFailure(DialFailure { peer_id, error, .. }) => {
                self.on_dial_failure(peer_id, error)
            }
            FromSwarm::ConnectionEstablished(_)
            | FromSwarm::ConnectionClosed(_)
            | FromSwarm::AddressChange(_)
            | FromSwarm::ListenFailure(_)
            | FromSwarm::NewListener(_)
            | FromSwarm::NewListenAddr(_)
            | FromSwarm::ExpiredListenAddr(_)
            | FromSwarm::ListenerError(_)
            | FromSwarm::ListenerClosed(_)
            | FromSwarm::NewExternalAddr(_)
            | FromSwarm::ExpiredExternalAddr(_) => {}
        }
    }
}

impl Discovery {
    fn on_dial_failure(&mut self, peer_id: Option<PeerId>, error: &DialError) {
        if let Some(peer_id) = peer_id {
            match error {
                DialError::LocalPeerId { .. }
                | DialError::Denied { .. }
                | DialError::NoAddresses
                | DialError::Transport(_)
                | DialError::WrongPeerId { .. } => {
                    slog::debug!(self.log, "Dial failure"; "peer_id" => peer_id.to_string(), "error" => format!("{:?}", error));
                }
                DialError::DialPeerConditionFalse(_) | DialError::Aborted => {}
                #[allow(deprecated)]
                DialError::ConnectionLimit(_) => {}
                DialError::InvalidPeerId(_) => {}
                #[allow(deprecated)]
                DialError::Banned => {}
            }
        }
    }
}
