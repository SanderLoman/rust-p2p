// use super::methods::*;
// use futures::future::BoxFuture;
// use futures::prelude::{AsyncRead, AsyncWrite};
// use futures::{FutureExt, StreamExt};
// use libp2p::core::{InboundUpgrade, UpgradeInfo};
// use ssz::Encode;
// use ssz_types::VariableList;
// use std::collections::HashMap;
// use std::io;
// use std::marker::PhantomData;
// use std::sync::Arc;
// use std::time::Duration;
// use strum::{AsRefStr, Display, EnumString, IntoStaticStr};
// use tokio::sync::RwLock;
// use tokio_io_timeout::TimeoutStream;
// use tokio_util::{
//     codec::Framed,
//     compat::{Compat, FuturesAsyncReadCompatExt},
// };

// /// The protocol prefix the RPC protocol id.
// const PROTOCOL_PREFIX: &str = "/eth2/beacon_chain/req";
// /// The number of seconds to wait for the first bytes of a request once a protocol has been
// /// established before the stream is terminated.
// const REQUEST_TIMEOUT: u64 = 15;

// /// Protocol names to be used.
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumString, AsRefStr, Display)]
// #[strum(serialize_all = "snake_case")]
// pub enum Protocol {
//     /// The Status protocol name.
//     Status,
//     /// The Goodbye protocol name.
//     Goodbye,
//     /// The `BlocksByRange` protocol name.
//     #[strum(serialize = "beacon_blocks_by_range")]
//     BlocksByRange,
//     /// The `BlocksByRoot` protocol name.
//     #[strum(serialize = "beacon_blocks_by_root")]
//     BlocksByRoot,
//     /// The `Ping` protocol name.
//     Ping,
//     /// The `LightClientBootstrap` protocol name.
//     #[strum(serialize = "light_client_bootstrap")]
//     LightClientBootstrap,
// }

// /// RPC Encondings supported.
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum Encoding {
//     SSZSnappy,
// }

// /// All valid protocol name and version combinations.
// #[derive(Debug, Clone, Copy, PartialEq)]
// pub enum SupportedProtocol {
//     StatusV1,
//     GoodbyeV1,
//     BlocksByRangeV1,
//     BlocksByRangeV2,
//     BlocksByRootV1,
//     BlocksByRootV2,
//     PingV1,
//     LightClientBootstrapV1,
// }

// impl SupportedProtocol {
//     pub fn version_string(&self) -> &'static str {
//         match self {
//             SupportedProtocol::StatusV1 => "1",
//             SupportedProtocol::GoodbyeV1 => "1",
//             SupportedProtocol::BlocksByRangeV1 => "1",
//             SupportedProtocol::BlocksByRangeV2 => "2",
//             SupportedProtocol::BlocksByRootV1 => "1",
//             SupportedProtocol::BlocksByRootV2 => "2",
//             SupportedProtocol::PingV1 => "1",
//             SupportedProtocol::LightClientBootstrapV1 => "1",
//         }
//     }

//     pub fn protocol(&self) -> Protocol {
//         match self {
//             SupportedProtocol::StatusV1 => Protocol::Status,
//             SupportedProtocol::GoodbyeV1 => Protocol::Goodbye,
//             SupportedProtocol::BlocksByRangeV1 => Protocol::BlocksByRange,
//             SupportedProtocol::BlocksByRangeV2 => Protocol::BlocksByRange,
//             SupportedProtocol::BlocksByRootV1 => Protocol::BlocksByRoot,
//             SupportedProtocol::BlocksByRootV2 => Protocol::BlocksByRoot,
//             SupportedProtocol::PingV1 => Protocol::Ping,
//             SupportedProtocol::LightClientBootstrapV1 => Protocol::LightClientBootstrap,
//         }
//     }

//     fn currently_supported() -> Vec<ProtocolId> {
//         vec![
//             ProtocolId::new(Self::StatusV1, Encoding::SSZSnappy),
//             ProtocolId::new(Self::GoodbyeV1, Encoding::SSZSnappy),
//             // V2 variants have higher preference then V1
//             ProtocolId::new(Self::BlocksByRangeV2, Encoding::SSZSnappy),
//             ProtocolId::new(Self::BlocksByRangeV1, Encoding::SSZSnappy),
//             ProtocolId::new(Self::BlocksByRootV2, Encoding::SSZSnappy),
//             ProtocolId::new(Self::BlocksByRootV1, Encoding::SSZSnappy),
//             ProtocolId::new(Self::PingV1, Encoding::SSZSnappy),
//         ]
//     }
// }

// impl std::fmt::Display for Encoding {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let repr = match self {
//             Encoding::SSZSnappy => "ssz_snappy",
//         };
//         f.write_str(repr)
//     }
// }
// pub enum ForkName {
//     Base,
//     Altair,
//     Merge,
//     Capella,
// }

// pub struct ForkContext {
//     current_fork: RwLock<ForkName>,
//     fork_to_digest: HashMap<ForkName, [u8; 4]>,
//     digest_to_fork: HashMap<[u8; 4], ForkName>,
// }

// #[derive(Debug, Clone)]
// pub struct RPCProtocol {
//     pub fork_context: Arc<ForkContext>,
//     pub max_rpc_size: usize,
//     pub enable_light_client_server: bool,
//     pub phantom: PhantomData<()>,
//     pub ttfb_timeout: Duration,
// }

// /// Represents the ssz length bounds for RPC messages.
// #[derive(Debug, PartialEq)]
// pub struct RpcLimits {
//     pub min: usize,
//     pub max: usize,
// }

// impl RpcLimits {
//     pub fn new(min: usize, max: usize) -> Self {
//         Self { min, max }
//     }

//     /// Returns true if the given length is greater than `max_rpc_size` or out of
//     /// bounds for the given ssz type, returns false otherwise.
//     pub fn is_out_of_bounds(&self, length: usize, max_rpc_size: usize) -> bool {
//         length > std::cmp::min(self.max, max_rpc_size) || length < self.min
//     }
// }

// /// Tracks the types in a protocol id.
// #[derive(Clone, Debug)]
// pub struct ProtocolId {
//     /// The protocol name and version
//     pub versioned_protocol: SupportedProtocol,

//     /// The encoding of the RPC.
//     pub encoding: Encoding,

//     /// The protocol id that is formed from the above fields.
//     protocol_id: String,
// }

// impl AsRef<str> for ProtocolId {
//     fn as_ref(&self) -> &str {
//         self.protocol_id.as_ref()
//     }
// }

// impl ProtocolId {
//     /// Returns min and max size for messages of given protocol id requests.
//     pub fn rpc_request_limits(&self) -> RpcLimits {
//         match self.versioned_protocol.protocol() {
//             Protocol::Status => RpcLimits::new(
//                 <StatusMessage as Encode>::ssz_fixed_len(),
//                 <StatusMessage as Encode>::ssz_fixed_len(),
//             ),
//             Protocol::Goodbye => RpcLimits::new(
//                 <GoodbyeReason as Encode>::ssz_fixed_len(),
//                 <GoodbyeReason as Encode>::ssz_fixed_len(),
//             ),
//             // V1 and V2 requests are the same
//             Protocol::BlocksByRange => RpcLimits::new(
//                 <OldBlocksByRangeRequestV2 as Encode>::ssz_fixed_len(),
//                 <OldBlocksByRangeRequestV2 as Encode>::ssz_fixed_len(),
//             ),
//             Protocol::BlocksByRoot => RpcLimits::new(0, 128),
//             Protocol::Ping => RpcLimits::new(
//                 <Ping as Encode>::ssz_fixed_len(),
//                 <Ping as Encode>::ssz_fixed_len(),
//             ),
//             Protocol::LightClientBootstrap => RpcLimits::new(
//                 <LightClientBootstrapRequest as Encode>::ssz_fixed_len(),
//                 <LightClientBootstrapRequest as Encode>::ssz_fixed_len(),
//             ),
//         }
//     }

//     /// Returns min and max size for messages of given protocol id responses.
//     pub fn rpc_response_limits(&self, fork_context: &ForkContext) -> RpcLimits {
//         match self.versioned_protocol.protocol() {
//             Protocol::Status => RpcLimits::new(
//                 <StatusMessage as Encode>::ssz_fixed_len(),
//                 <StatusMessage as Encode>::ssz_fixed_len(),
//             ),
//             Protocol::Goodbye => RpcLimits::new(0, 0), // Goodbye request has no response
//             Protocol::BlocksByRange => 128.as_ssz_bytes().len(),
//             Protocol::BlocksByRoot => 128.as_ssz_bytes().len(),
//             Protocol::Ping => RpcLimits::new(
//                 <Ping as Encode>::ssz_fixed_len(),
//                 <Ping as Encode>::ssz_fixed_len(),
//             ),

//             Protocol::LightClientBootstrap => RpcLimits::new(
//                 <LightClientBootstrapRequest as Encode>::ssz_fixed_len(),
//                 <LightClientBootstrapRequest as Encode>::ssz_fixed_len(),
//             ),
//         }
//     }

//     /// Returns `true` if the given `ProtocolId` should expect `context_bytes` in the
//     /// beginning of the stream, else returns `false`.
//     pub fn has_context_bytes(&self) -> bool {
//         match self.versioned_protocol {
//             SupportedProtocol::BlocksByRangeV2
//             | SupportedProtocol::BlocksByRootV2
//             | SupportedProtocol::LightClientBootstrapV1 => true,
//             SupportedProtocol::StatusV1
//             | SupportedProtocol::BlocksByRootV1
//             | SupportedProtocol::BlocksByRangeV1
//             | SupportedProtocol::PingV1
//             | SupportedProtocol::GoodbyeV1 => false,
//         }
//     }
// }

// /// An RPC protocol ID.
// impl ProtocolId {
//     pub fn new(versioned_protocol: SupportedProtocol, encoding: Encoding) -> Self {
//         let protocol_id = format!(
//             "{}/{}/{}/{}",
//             PROTOCOL_PREFIX,
//             versioned_protocol.protocol(),
//             versioned_protocol.version_string(),
//             encoding
//         );

//         ProtocolId {
//             versioned_protocol,
//             encoding,
//             protocol_id,
//         }
//     }
// }

// #[derive(Debug, Clone, PartialEq)]
// pub enum InboundRequest {
//     Status(StatusMessage),
//     Goodbye(GoodbyeReason),
//     BlocksByRange(OldBlocksByRangeRequest),
//     BlocksByRoot(BlocksByRootRequest),
//     LightClientBootstrap(LightClientBootstrapRequest),
//     Ping(Ping),
// }

// /// Implements the encoding per supported protocol for `RPCRequest`.
// impl InboundRequest {
//     /* These functions are used in the handler for stream management */

//     /// Number of responses expected for this request.
//     pub fn expected_responses(&self) -> u64 {
//         match self {
//             InboundRequest::Status(_) => 1,
//             InboundRequest::Goodbye(_) => 0,
//             InboundRequest::BlocksByRange(req) => *req.count(),
//             InboundRequest::BlocksByRoot(req) => req.block_roots().len() as u64,
//             InboundRequest::Ping(_) => 1,
//             InboundRequest::LightClientBootstrap(_) => 1,
//         }
//     }

//     /// Gives the corresponding `SupportedProtocol` to this request.
//     pub fn versioned_protocol(&self) -> SupportedProtocol {
//         match self {
//             InboundRequest::Status(_) => SupportedProtocol::StatusV1,
//             InboundRequest::Goodbye(_) => SupportedProtocol::GoodbyeV1,
//             InboundRequest::BlocksByRange(req) => match req {
//                 OldBlocksByRangeRequest::V1(_) => SupportedProtocol::BlocksByRangeV1,
//                 OldBlocksByRangeRequest::V2(_) => SupportedProtocol::BlocksByRangeV2,
//             },
//             InboundRequest::BlocksByRoot(req) => match req {
//                 BlocksByRootRequest::V1(_) => SupportedProtocol::BlocksByRootV1,
//                 BlocksByRootRequest::V2(_) => SupportedProtocol::BlocksByRootV2,
//             },
//             InboundRequest::Ping(_) => SupportedProtocol::PingV1,
//             InboundRequest::LightClientBootstrap(_) => SupportedProtocol::LightClientBootstrapV1,
//         }
//     }

//     /// Returns the `ResponseTermination` type associated with the request if a stream gets
//     /// terminated.
//     pub fn stream_termination(&self) -> ResponseTermination {
//         match self {
//             // this only gets called after `multiple_responses()` returns true. Therefore, only
//             // variants that have `multiple_responses()` can have values.
//             InboundRequest::BlocksByRange(_) => ResponseTermination::BlocksByRange,
//             InboundRequest::BlocksByRoot(_) => ResponseTermination::BlocksByRoot,
//             InboundRequest::Status(_) => unreachable!(),
//             InboundRequest::Goodbye(_) => unreachable!(),
//             InboundRequest::Ping(_) => unreachable!(),
//             InboundRequest::LightClientBootstrap(_) => unreachable!(),
//         }
//     }
// }

// /// Error in RPC Encoding/Decoding.
// #[derive(Debug, Clone, PartialEq, IntoStaticStr)]
// #[strum(serialize_all = "snake_case")]
// pub enum RPCError {
//     /// Error when decoding the raw buffer from ssz.
//     // NOTE: in the future a ssz::DecodeError should map to an InvalidData error
//     #[strum(serialize = "decode_error")]
//     SSZDecodeError(ssz::DecodeError),
//     /// IO Error.
//     IoError(String),
//     /// The peer returned a valid response but the response indicated an error.
//     ErrorResponse(RPCResponseErrorCode, String),
//     /// Timed out waiting for a response.
//     StreamTimeout,
//     /// Peer does not support the protocol.
//     UnsupportedProtocol,
//     /// Stream ended unexpectedly.
//     IncompleteStream,
//     /// Peer sent invalid data.
//     InvalidData(String),
//     /// An error occurred due to internal reasons. Ex: timer failure.
//     InternalError(&'static str),
//     /// Negotiation with this peer timed out.
//     NegotiationTimeout,
//     /// Handler rejected this request.
//     HandlerRejected,
//     /// We have intentionally disconnected.
//     Disconnected,
// }

// impl From<ssz::DecodeError> for RPCError {
//     #[inline]
//     fn from(err: ssz::DecodeError) -> Self {
//         RPCError::SSZDecodeError(err)
//     }
// }
// impl From<tokio::time::error::Elapsed> for RPCError {
//     fn from(_: tokio::time::error::Elapsed) -> Self {
//         RPCError::StreamTimeout
//     }
// }

// impl From<io::Error> for RPCError {
//     fn from(err: io::Error) -> Self {
//         RPCError::IoError(err.to_string())
//     }
// }

// // Error trait is required for `ProtocolsHandler`
// impl std::fmt::Display for RPCError {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match *self {
//             RPCError::SSZDecodeError(ref err) => write!(f, "Error while decoding ssz: {:?}", err),
//             RPCError::InvalidData(ref err) => write!(f, "Peer sent unexpected data: {}", err),
//             RPCError::IoError(ref err) => write!(f, "IO Error: {}", err),
//             RPCError::ErrorResponse(ref code, ref reason) => write!(
//                 f,
//                 "RPC response was an error: {} with reason: {}",
//                 code, reason
//             ),
//             RPCError::StreamTimeout => write!(f, "Stream Timeout"),
//             RPCError::UnsupportedProtocol => write!(f, "Peer does not support the protocol"),
//             RPCError::IncompleteStream => write!(f, "Stream ended unexpectedly"),
//             RPCError::InternalError(ref err) => write!(f, "Internal error: {}", err),
//             RPCError::NegotiationTimeout => write!(f, "Negotiation timeout"),
//             RPCError::HandlerRejected => write!(f, "Handler rejected the request"),
//             RPCError::Disconnected => write!(f, "Gracefully Disconnected"),
//         }
//     }
// }

// impl std::error::Error for RPCError {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         match *self {
//             // NOTE: this does have a source
//             RPCError::SSZDecodeError(_) => None,
//             RPCError::IoError(_) => None,
//             RPCError::StreamTimeout => None,
//             RPCError::UnsupportedProtocol => None,
//             RPCError::IncompleteStream => None,
//             RPCError::InvalidData(_) => None,
//             RPCError::InternalError(_) => None,
//             RPCError::ErrorResponse(_, _) => None,
//             RPCError::NegotiationTimeout => None,
//             RPCError::HandlerRejected => None,
//             RPCError::Disconnected => None,
//         }
//     }
// }

// impl RPCError {
//     /// Get a `str` representation of the error.
//     /// Used for metrics.
//     pub fn as_static_str(&self) -> &'static str {
//         match self {
//             RPCError::ErrorResponse(ref code, ..) => code.into(),
//             e => e.into(),
//         }
//     }
// }
