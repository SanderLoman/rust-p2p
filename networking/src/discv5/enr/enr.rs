#![deny(unsafe_code)]

// I dont know if im going to keep this code or not, but I want to keep it around for now.


use discv5::enr::{CombinedKey, CombinedPublicKey};
use discv5::Enr;
use libp2p::core::{identity::Keypair, multiaddr::Protocol};
use libp2p::{Multiaddr, PeerId};

/// Extend ENR for libp2p types.
pub trait EnrExt {
    /// The libp2p `PeerId` for the record.
    fn peer_id(&self) -> PeerId;

    /// Returns a list of multiaddrs if the ENR has an `ip` and either a `tcp` or `udp` key **or** an `ip6` and either a `tcp6` or `udp6`.
    /// The vector remains empty if these fields are not defined.
    fn multiaddr(&self) -> Vec<Multiaddr>;

    /// Returns a list of multiaddrs with the `PeerId` prepended.
    fn multiaddr_p2p(&self) -> Vec<Multiaddr>;

    /// Returns any multiaddrs that contain the TCP protocol with the `PeerId` prepended.
    fn multiaddr_p2p_tcp(&self) -> Vec<Multiaddr>;

    /// Returns any multiaddrs that contain the UDP protocol with the `PeerId` prepended.
    fn multiaddr_p2p_udp(&self) -> Vec<Multiaddr>;

    /// Returns any multiaddrs that contain the TCP protocol.
    fn multiaddr_tcp(&self) -> Vec<Multiaddr>;
}

/// Extend ENR CombinedPublicKey for libp2p types.
pub trait CombinedKeyPublicExt {
    /// Converts the publickey into a peer id, without consuming the key.
    fn as_peer_id(&self) -> PeerId;
}

/// Extend ENR CombinedKey for conversion to libp2p keys.
pub trait CombinedKeyExt {
    /// Converts a libp2p key into an ENR combined key.
    fn from_libp2p(key: &libp2p::core::identity::Keypair) -> Result<CombinedKey, &'static str>;
}

impl EnrExt for Enr {
    /// The libp2p `PeerId` for the record.
    fn peer_id(&self) -> PeerId {
        self.public_key().as_peer_id()
    }

    /// Returns a list of multiaddrs if the ENR has an `ip` and either a `tcp` or `udp` key **or** an `ip6` and either a `tcp6` or `udp6`.
    /// The vector remains empty if these fields are not defined.
    fn multiaddr(&self) -> Vec<Multiaddr> {
        let mut multiaddrs: Vec<Multiaddr> = Vec::new();
        if let Some(ip) = self.ip4() {
            if let Some(udp) = self.udp4() {
                let mut multiaddr: Multiaddr = ip.into();
                multiaddr.push(Protocol::Udp(udp));
                multiaddrs.push(multiaddr);
            }

            if let Some(tcp) = self.tcp4() {
                let mut multiaddr: Multiaddr = ip.into();
                multiaddr.push(Protocol::Tcp(tcp));
                multiaddrs.push(multiaddr);
            }
        }
        if let Some(ip6) = self.ip6() {
            if let Some(udp6) = self.udp6() {
                let mut multiaddr: Multiaddr = ip6.into();
                multiaddr.push(Protocol::Udp(udp6));
                multiaddrs.push(multiaddr);
            }

            if let Some(tcp6) = self.tcp6() {
                let mut multiaddr: Multiaddr = ip6.into();
                multiaddr.push(Protocol::Tcp(tcp6));
                multiaddrs.push(multiaddr);
            }
        }
        multiaddrs
    }

    /// Returns a list of multiaddrs if the ENR has an `ip` and either a `tcp` or `udp` key **or** an `ip6` and either a `tcp6` or `udp6`.
    /// The vector remains empty if these fields are not defined.
    ///
    /// This also prepends the `PeerId` into each multiaddr with the `P2p` protocol.
    fn multiaddr_p2p(&self) -> Vec<Multiaddr> {
        let peer_id = self.peer_id();
        let mut multiaddrs: Vec<Multiaddr> = Vec::new();
        if let Some(ip) = self.ip4() {
            if let Some(udp) = self.udp4() {
                let mut multiaddr: Multiaddr = ip.into();
                multiaddr.push(Protocol::Udp(udp));
                multiaddr.push(Protocol::P2p(peer_id.into()));
                multiaddrs.push(multiaddr);
            }

            if let Some(tcp) = self.tcp4() {
                let mut multiaddr: Multiaddr = ip.into();
                multiaddr.push(Protocol::Tcp(tcp));
                multiaddr.push(Protocol::P2p(peer_id.into()));
                multiaddrs.push(multiaddr);
            }
        }
        if let Some(ip6) = self.ip6() {
            if let Some(udp6) = self.udp6() {
                let mut multiaddr: Multiaddr = ip6.into();
                multiaddr.push(Protocol::Udp(udp6));
                multiaddr.push(Protocol::P2p(peer_id.into()));
                multiaddrs.push(multiaddr);
            }

            if let Some(tcp6) = self.tcp6() {
                let mut multiaddr: Multiaddr = ip6.into();
                multiaddr.push(Protocol::Tcp(tcp6));
                multiaddr.push(Protocol::P2p(peer_id.into()));
                multiaddrs.push(multiaddr);
            }
        }
        multiaddrs
    }

    /// Returns a list of multiaddrs if the ENR has an `ip` and a `tcp` key **or** an `ip6` and a `tcp6`.
    /// The vector remains empty if these fields are not defined.
    ///
    /// This also prepends the `PeerId` into each multiaddr with the `P2p` protocol.
    fn multiaddr_p2p_tcp(&self) -> Vec<Multiaddr> {
        let peer_id = self.peer_id();
        let mut multiaddrs: Vec<Multiaddr> = Vec::new();
        if let Some(ip) = self.ip4() {
            if let Some(tcp) = self.tcp4() {
                let mut multiaddr: Multiaddr = ip.into();
                multiaddr.push(Protocol::Tcp(tcp));
                multiaddr.push(Protocol::P2p(peer_id.into()));
                multiaddrs.push(multiaddr);
            }
        }
        if let Some(ip6) = self.ip6() {
            if let Some(tcp6) = self.tcp6() {
                let mut multiaddr: Multiaddr = ip6.into();
                multiaddr.push(Protocol::Tcp(tcp6));
                multiaddr.push(Protocol::P2p(peer_id.into()));
                multiaddrs.push(multiaddr);
            }
        }
        multiaddrs
    }

    /// Returns a list of multiaddrs if the ENR has an `ip` and a `udp` key **or** an `ip6` and a `udp6`.
    /// The vector remains empty if these fields are not defined.
    ///
    /// This also prepends the `PeerId` into each multiaddr with the `P2p` protocol.
    fn multiaddr_p2p_udp(&self) -> Vec<Multiaddr> {
        let peer_id = self.peer_id();
        let mut multiaddrs: Vec<Multiaddr> = Vec::new();
        if let Some(ip) = self.ip4() {
            if let Some(udp) = self.udp4() {
                let mut multiaddr: Multiaddr = ip.into();
                multiaddr.push(Protocol::Udp(udp));
                multiaddr.push(Protocol::P2p(peer_id.into()));
                multiaddrs.push(multiaddr);
            }
        }
        if let Some(ip6) = self.ip6() {
            if let Some(udp6) = self.udp6() {
                let mut multiaddr: Multiaddr = ip6.into();
                multiaddr.push(Protocol::Udp(udp6));
                multiaddr.push(Protocol::P2p(peer_id.into()));
                multiaddrs.push(multiaddr);
            }
        }
        multiaddrs
    }

    /// Returns a list of multiaddrs if the ENR has an `ip` and either a `tcp` or `udp` key **or** an `ip6` and either a `tcp6` or `udp6`.
    /// The vector remains empty if these fields are not defined.
    fn multiaddr_tcp(&self) -> Vec<Multiaddr> {
        let mut multiaddrs: Vec<Multiaddr> = Vec::new();
        if let Some(ip) = self.ip4() {
            if let Some(tcp) = self.tcp4() {
                let mut multiaddr: Multiaddr = ip.into();
                multiaddr.push(Protocol::Tcp(tcp));
                multiaddrs.push(multiaddr);
            }
        }
        if let Some(ip6) = self.ip6() {
            if let Some(tcp6) = self.tcp6() {
                let mut multiaddr: Multiaddr = ip6.into();
                multiaddr.push(Protocol::Tcp(tcp6));
                multiaddrs.push(multiaddr);
            }
        }
        multiaddrs
    }
}

impl CombinedKeyPublicExt for CombinedPublicKey {
    /// Converts the publickey into a peer id, without consuming the key.
    ///
    /// This is only available with the `libp2p` feature flag.
    fn as_peer_id(&self) -> PeerId {
        match self {
            Self::Secp256k1(pk) => {
                let pk_bytes = pk.to_sec1_bytes();
                let libp2p_pk = libp2p::identity::PublicKey::try_into_secp256k1(
                    libp2p::core::identity::PublicKey::decode(&pk_bytes).expect("valid public key"),
                );
                PeerId::from_public_key(&libp2p_pk)
            }
            Self::Ed25519(pk) => {
                let pk_bytes = pk.to_bytes();
                let libp2p_pk = libp2p::identity::PublicKey::try_into_ed25519(
                    libp2p::core::identity::ed25519::PublicKey::decode(&pk_bytes)
                        .expect("valid public key"),
                );
                PeerId::from_public_key(&libp2p_pk)
            }
        }
    }
}

impl CombinedKeyExt for CombinedKey {
    fn from_libp2p(key: &libp2p::core::identity::Keypair) -> Result<CombinedKey, &'static str> {
        match key {
            Keypair::try_into_secp256k1(key) => {
                let secret =
                    discv5::enr::k256::ecdsa::SigningKey::from_slice(&key.secret().to_bytes())
                        .expect("libp2p key must be valid");
                Ok(CombinedKey::Secp256k1(secret))
            }
            Keypair::try_into_ed25519(key) => {
                let ed_keypair = discv5::enr::ed25519_dalek::SigningKey::from_bytes(
                    &(key.encode()[..32])
                        .try_into()
                        .expect("libp2p key must be valid"),
                );
                Ok(CombinedKey::from(ed_keypair))
            }
            Keypair::Ecdsa(_) => Err("Ecdsa keypairs not supported"),
        }
    }
}
