//! A collection of variables that are accessible outside of the network thread itself.
use crate::peer_manager::peerdb::PeerDB;
use crate::rpc::methods::{MetaData, MetaDataV2};
// use crate::types::{BackFillState, SyncState};
// use crate::Client;
// use crate::EnrExt;
use crate::types::{Enr, GossipTopic, Multiaddr, PeerId};
use parking_lot::RwLock;
use std::collections::HashSet;
use project_types::EthSpec;

pub struct NetworkGlobals<TSpec: EthSpec> {
    /// The current local ENR.
    pub local_enr: RwLock<Enr>,
    /// The local peer_id.
    pub peer_id: RwLock<PeerId>,
    /// Listening multiaddrs.
    pub listen_multiaddrs: RwLock<Vec<Multiaddr>>,
    /// The collection of known peers.
    pub peers: RwLock<PeerDB>,
    // The local meta data of our node.
    pub local_metadata: RwLock<MetaData<TSpec>>,
    /// The current gossipsub topic subscriptions.
    pub gossipsub_subscriptions: RwLock<HashSet<GossipTopic>>,
}

impl<TSpec: EthSpec> NetworkGlobals<TSpec> {
    pub fn new(
        enr: Enr,
        local_metadata: MetaData<TSpec>,
        trusted_peers: Vec<PeerId>,
        disable_peer_scoring: bool,
        log: &slog::Logger,
    ) -> Self {
        NetworkGlobals {
            local_enr: RwLock::new(enr.clone()),
            peer_id: RwLock::new(enr.peer_id()),
            listen_multiaddrs: RwLock::new(Vec::new()),
            local_metadata: RwLock::new(local_metadata),
            peers: RwLock::new(PeerDB::new(trusted_peers, disable_peer_scoring, log)),
            gossipsub_subscriptions: RwLock::new(HashSet::new()),
            sync_state: RwLock::new(SyncState::Stalled),
            backfill_state: RwLock::new(BackFillState::NotRequired),
        }
    }

    /// Returns the local ENR from the underlying Discv5 behaviour that external peers may connect
    /// to.
    pub fn local_enr(&self) -> Enr {
        self.local_enr.read().clone()
    }

    /// Returns the local libp2p PeerID.
    pub fn local_peer_id(&self) -> PeerId {
        *self.peer_id.read()
    }

    /// Returns the list of `Multiaddr` that the underlying libp2p instance is listening on.
    pub fn listen_multiaddrs(&self) -> Vec<Multiaddr> {
        self.listen_multiaddrs.read().clone()
    }

    /// Returns the number of libp2p connected peers.
    pub fn connected_peers(&self) -> usize {
        self.peers.read().connected_peer_ids().count()
    }

    /// Returns the number of libp2p connected peers with outbound-only connections.
    pub fn connected_outbound_only_peers(&self) -> usize {
        self.peers.read().connected_outbound_only_peers().count()
    }

    /// Returns the number of libp2p peers that are either connected or being dialed.
    pub fn connected_or_dialing_peers(&self) -> usize {
        self.peers.read().connected_or_dialing_peers().count()
    }

    /// Returns in the node is syncing.
    pub fn is_syncing(&self) -> bool {
        self.sync_state.read().is_syncing()
    }

    // /// Returns the current sync state of the peer.
    // pub fn sync_state(&self) -> SyncState {
    //     self.sync_state.read().clone()
    // }

    // /// Returns the current backfill state.
    // pub fn backfill_state(&self) -> BackFillState {
    //     self.backfill_state.read().clone()
    // }

    // /// Returns a `Client` type if one is known for the `PeerId`.
    // pub fn client(&self, peer_id: &PeerId) -> Client {
    //     self.peers
    //         .read()
    //         .peer_info(peer_id)
    //         .map(|info| info.client().clone())
    //         .unwrap_or_default()
    // }

    // /// Updates the syncing state of the node.
    // ///
    // /// The old state is returned
    // pub fn set_sync_state(&self, new_state: SyncState) -> SyncState {
    //     std::mem::replace(&mut *self.sync_state.write(), new_state)
    // }
}
