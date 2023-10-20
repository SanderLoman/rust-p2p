use std::time::Duration;

use futures::future::Either;
use hyper::header::ACCEPT;
use hyper::{Client, HeaderMap, Uri};
use libp2p::core::{multiaddr::Multiaddr, muxing::StreamMuxerBox, transport::Boxed};

use libp2p::identity::Keypair;
use libp2p::multiaddr::Protocol;
use libp2p::{core, noise, yamux, PeerId, Transport};
use libp2p_quic;
use serde_derive::Serialize;
use serde_json::Value;
use slog::{debug, error, warn, Logger};
use ssz_derive::{Decode, Encode};
use ssz_types::BitVector;
use std::error::Error;
use superstruct::superstruct;

// use crate::network::methods::{MetaData, MetaDataV1, MetaDataV2};
// type SubnetBitfieldLength: Unsigned + Clone + Sync + Send + Debug + PartialEq + Default;
// pub type EnrAttestationBitfield = BitVector<SubnetBitfieldLength>;
// type SyncCommitteeSubnetCount: Unsigned + Clone + Sync + Send + Debug + PartialEq;
// pub type EnrSyncCommitteeBitfield = BitVector<SyncCommitteeSubnetCount>;

type BoxedTransport = Boxed<(PeerId, StreamMuxerBox)>;

/// The implementation supports TCP/IP, QUIC (experimental) over UDP, noise as the encryption layer, and
/// mplex/yamux as the multiplexing layer (when using TCP).
pub fn build_transport(local_private_key: Keypair) -> std::io::Result<(BoxedTransport)> {
    // mplex config
    let mut mplex_config = libp2p_mplex::MplexConfig::new();
    mplex_config.set_max_buffer_size(256);
    mplex_config.set_max_buffer_behaviour(libp2p_mplex::MaxBufferBehaviour::Block);

    // yamux config
    let mut yamux_config = yamux::Config::default();
    yamux_config.set_window_update_mode(yamux::WindowUpdateMode::on_read());

    // Creates the TCP transport layer
    let tcp = libp2p::tcp::tokio::Transport::new(libp2p::tcp::Config::default().nodelay(true))
        .upgrade(core::upgrade::Version::V1)
        .authenticate(generate_noise_config(&local_private_key))
        .multiplex(core::upgrade::SelectUpgrade::new(
            yamux_config,
            mplex_config,
        ))
        .timeout(Duration::from_secs(10));

    let transport = {
        // Enables Quic
        // The default quic configuration suits us for now.
        let quic_config = libp2p_quic::Config::new(&local_private_key);
        tcp.or_transport(libp2p_quic::tokio::Transport::new(quic_config))
            .map(|either_output, _| match either_output {
                Either::Left((peer_id, muxer)) => (peer_id, StreamMuxerBox::new(muxer)),
                Either::Right((peer_id, muxer)) => (peer_id, StreamMuxerBox::new(muxer)),
            })
    };

    // // Enables DNS over the transport.
    let transport = libp2p::dns::tokio::Transport::system(transport)?.boxed();

    Ok(transport)
}

/// Generate authenticated XX Noise config from identity keys
fn generate_noise_config(identity_keypair: &Keypair) -> noise::Config {
    noise::Config::new(identity_keypair).expect("signing can fail only once during starting a node")
}

/// For a multiaddr that ends with a peer id, this strips this suffix. Rust-libp2p
/// only supports dialing to an address without providing the peer id.
pub fn strip_peer_id(addr: &mut Multiaddr) {
    let last = addr.pop();
    match last {
        Some(Protocol::P2p(_)) => {}
        Some(other) => addr.push(other),
        _ => {}
    }
}

// /// The METADATA response structure.
// #[superstruct(
//     variants(V1, V2),
//     variant_attributes(derive(Encode, Decode, Clone, Debug, PartialEq, Serialize))
// )]
// #[derive(Clone, Debug, PartialEq, Serialize)]
// pub struct MetaData {
//     /// A sequential counter indicating when data gets modified.
//     pub seq_number: u64,
//     /// The persistent attestation subnet bitfield.
//     pub attnets: EnrAttestationBitfield,
//     /// The persistent sync committee bitfield.
//     #[superstruct(only(V2))]
//     pub syncnets: EnrSyncCommitteeBitfield,
// }

// impl MetaData {
//     /// Returns a V1 MetaData response from self.
//     pub fn metadata_v1(&self) -> Self {
//         match self {
//             md @ MetaData::V1(_) => md.clone(),
//             MetaData::V2(metadata) => MetaData::V1(MetaDataV1 {
//                 seq_number: metadata.seq_number,
//                 attnets: metadata.attnets.clone(),
//             }),
//         }
//     }

//     /// Returns a V2 MetaData response from self by filling unavailable fields with default.
//     pub fn metadata_v2(&self) -> Self {
//         match self {
//             MetaData::V1(metadata) => MetaData::V2(MetaDataV2 {
//                 seq_number: metadata.seq_number,
//                 attnets: metadata.attnets.clone(),
//                 syncnets: Default::default(),
//             }),
//             md @ MetaData::V2(_) => md.clone(),
//         }
//     }

//     pub fn as_ssz_bytes(&self) -> Vec<u8> {
//         match self {
//             MetaData::V1(md) => md.as_ssz_bytes(),
//             MetaData::V2(md) => md.as_ssz_bytes(),
//         }
//     }
// }

// pub async fn get_metadata(log: &Logger) -> Result<MetaData, Box<dyn Error>> {
//     let client = Client::new();

//     // Set up headers for the request
//     let mut headers = HeaderMap::new();
//     headers.insert(ACCEPT, "application/json".parse().unwrap());

//     // Convert the string to a Uri
//     let uri = Uri::from_static("http://127.0.0.1:5052/eth/v1/node/identity");

//     // Make the request to the local node
//     let res = client.get(uri).await?;

//     // Check for a successful response
//     if res.status().is_success() {
//         // Parse the response body as JSON
//         let body_bytes = hyper::body::to_bytes(res.into_body()).await?;
//         let json: Value = serde_json::from_slice(&body_bytes)?;

//         // Navigate to the metadata field in the JSON object
//         if let Some(metadata) = json.get("data").and_then(|data| data.get("metadata")) {
//             // Extract the required fields from the metadata field
//             let seq_number = metadata
//                 .get("seq_number")
//                 .and_then(|v| v.as_str())
//                 .unwrap_or("")
//                 .to_string();
//             let attnets = metadata
//                 .get("attnets")
//                 .and_then(|v| v.as_str())
//                 .unwrap_or("")
//                 .to_string();
//             let syncnets = metadata
//                 .get("syncnets")
//                 .and_then(|v| v.as_str())
//                 .unwrap_or("")
//                 .to_string();

//             Ok(MetaData {
//                 seq_number,
//                 attnets,
//                 syncnets,
//             })
//         } else {
//             let err_msg = "Missing metadata";
//             error!(log, "{}", err_msg);
//             Err(Box::new(std::io::Error::new(
//                 std::io::ErrorKind::InvalidData,
//                 err_msg,
//             )))
//         }
//     } else {
//         let err_msg = format!("Failed to retrieve metadata: {}", res.status());
//         error!(log, "{}", err_msg);
//         Err(Box::new(std::io::Error::new(
//             std::io::ErrorKind::Other,
//             err_msg,
//         )))
//     }
// }
