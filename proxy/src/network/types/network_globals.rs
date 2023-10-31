use discv5::enr::{CombinedKey, CombinedPublicKey};
use libp2p::{
    identity::{ed25519, secp256k1, KeyType, Keypair, PublicKey},
    Multiaddr, PeerId,
};
use parking_lot::RwLock;

pub type Enr = discv5::enr::Enr<discv5::enr::CombinedKey>;

pub struct NetworkGlobals {
    /// The current local ENR.
    pub local_enr: RwLock<Enr>,
    /// The local peer_id.
    pub peer_id: RwLock<PeerId>,
    /// Listening multiaddrs.
    pub listen_multiaddrs: RwLock<Vec<Multiaddr>>,
    // Dont know if we will need this, keeping it here for now.
    // pub gossipsub_subscriptions: RwLock<HashSet<GossipTopic>>,
}

impl NetworkGlobals {
    pub fn new(enr: Enr) -> Self {
        NetworkGlobals {
            local_enr: RwLock::new(enr.clone()),
            peer_id: RwLock::new(enr.peer_id()),
            listen_multiaddrs: RwLock::new(Vec::new()),
            // gossipsub_subscriptions: RwLock::new(HashSet::new()),
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
}

pub trait CombinedKeyPublicExt {
    /// Converts the publickey into a peer id, without consuming the key.
    fn as_peer_id(&self) -> PeerId;
}

impl CombinedKeyPublicExt for CombinedPublicKey {
    /// Converts the publickey into a peer id, without consuming the key.
    ///
    /// This is only available with the `libp2p` feature flag.
    fn as_peer_id(&self) -> PeerId {
        match self {
            Self::Secp256k1(pk) => {
                let pk_bytes = pk.to_sec1_bytes();
                let libp2p_pk: PublicKey = secp256k1::PublicKey::try_from_bytes(&pk_bytes)
                    .expect("valid public key")
                    .into();
                PeerId::from_public_key(&libp2p_pk)
            }
            Self::Ed25519(pk) => {
                let pk_bytes = pk.to_bytes();
                let libp2p_pk: PublicKey = ed25519::PublicKey::try_from_bytes(&pk_bytes)
                    .expect("valid public key")
                    .into();
                PeerId::from_public_key(&libp2p_pk)
            }
        }
    }
}

pub trait EnrExt {
    /// Returns the `PeerId` of the ENR.
    fn peer_id(&self) -> PeerId;
}

impl EnrExt for Enr {
    fn peer_id(&self) -> PeerId {
        self.public_key().as_peer_id()
    }
}

pub trait CombinedKeyExt {
    /// Converts a libp2p key into an ENR combined key.
    fn from_libp2p(key: Keypair) -> Result<CombinedKey, &'static str>;

    /// Converts a [`secp256k1::Keypair`] into and Enr [`CombinedKey`].
    fn from_secp256k1(key: &secp256k1::Keypair) -> CombinedKey;
}

impl CombinedKeyExt for CombinedKey {
    fn from_libp2p(key: Keypair) -> Result<CombinedKey, &'static str> {
        match key.key_type() {
            KeyType::Secp256k1 => {
                let key = key.try_into_secp256k1().expect("right key type");
                let secret =
                    discv5::enr::k256::ecdsa::SigningKey::from_slice(&key.secret().to_bytes())
                        .expect("libp2p key must be valid");
                Ok(CombinedKey::Secp256k1(secret))
            }
            KeyType::Ed25519 => {
                let key = key.try_into_ed25519().expect("right key type");
                let ed_keypair = discv5::enr::ed25519_dalek::SigningKey::from_bytes(
                    &(key.to_bytes()[..32])
                        .try_into()
                        .expect("libp2p key must be valid"),
                );
                Ok(CombinedKey::from(ed_keypair))
            }
            _ => Err("Unsupported keypair kind"),
        }
    }
    fn from_secp256k1(key: &secp256k1::Keypair) -> Self {
        let secret = discv5::enr::k256::ecdsa::SigningKey::from_slice(&key.secret().to_bytes())
            .expect("libp2p key must be valid");
        CombinedKey::Secp256k1(secret)
    }
}
