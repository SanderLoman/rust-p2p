use hyper::Request;
use slog::{debug, Logger};
use tokio::sync::mpsc;

use crate::redirect::NetworkRequests;

#[derive(Debug)]
pub struct NetworkReceiver<N: NetworkRequests> {
    pub receiver: mpsc::UnboundedReceiver<Request<N>>,
    pub log: Logger,
}

impl<N: NetworkRequests> NetworkReceiver<N> {
    pub fn new(log: Logger) -> Self {
        let (_, receiver) = mpsc::unbounded_channel();
        NetworkReceiver { receiver, log }
    }

    pub async fn receive_request(&mut self) -> Option<Request<N>> {
        debug!(self.log, "Receiving request from network");
        self.receiver.recv().await
    }
}

// /* Inbound upgrade */
// // The inbound protocol reads the request, decodes it and returns the stream to the protocol
// // handler to respond to once ready.

// use std::time::Duration;

// use futures::future::BoxFuture;
// use libp2p::{InboundUpgrade, kad::InboundRequest};
// use tokio::io::{AsyncWrite, AsyncRead};

// use super::protocol::{Encoding, ProtocolId, REQUEST_TIMEOUT};

// pub type InboundOutput<TSocket, TSpec> = (InboundRequest<TSpec>, InboundFramed<TSocket, TSpec>);
// pub type InboundFramed<TSocket, TSpec> =
//     Framed<std::pin::Pin<Box<TimeoutStream<Compat<TSocket>>>>, InboundCodec<TSpec>>;

// impl<TSocket, TSpec> InboundUpgrade<TSocket> for RPCProtocol<TSpec>
// where
//     TSocket: AsyncRead + AsyncWrite + Unpin + Send + 'static,
//     TSpec: EthSpec,
// {
//     type Output = InboundOutput<TSocket, TSpec>;
//     type Error = RPCError;
//     type Future = BoxFuture<'static, Result<Self::Output, Self::Error>>;

//     fn upgrade_inbound(self, socket: TSocket, protocol: ProtocolId) -> Self::Future {
//         async move {
//             let versioned_protocol = protocol.versioned_protocol;
//             // convert the socket to tokio compatible socket
//             let socket = socket.compat();
//             let codec = match protocol.encoding {
//                 Encoding::SSZSnappy => {
//                     let ssz_snappy_codec = BaseInboundCodec::new(SSZSnappyInboundCodec::new(
//                         protocol,
//                         self.max_rpc_size,
//                         self.fork_context.clone(),
//                     ));
//                     InboundCodec::SSZSnappy(ssz_snappy_codec)
//                 }
//             };
//             let mut timed_socket = TimeoutStream::new(socket);
//             timed_socket.set_read_timeout(Some(self.ttfb_timeout));

//             let socket = Framed::new(Box::pin(timed_socket), codec);

//             // MetaData requests should be empty, return the stream
//             match versioned_protocol {
//                 SupportedProtocol::MetaDataV1 => {
//                     Ok((InboundRequest::MetaData(MetadataRequest::new_v1()), socket))
//                 }
//                 SupportedProtocol::MetaDataV2 => {
//                     Ok((InboundRequest::MetaData(MetadataRequest::new_v2()), socket))
//                 }
//                 _ => {
//                     match tokio::time::timeout(
//                         Duration::from_secs(REQUEST_TIMEOUT),
//                         socket.into_future(),
//                     )
//                     .await
//                     {
//                         Err(e) => Err(RPCError::from(e)),
//                         Ok((Some(Ok(request)), stream)) => Ok((request, stream)),
//                         Ok((Some(Err(e)), _)) => Err(e),
//                         Ok((None, _)) => Err(RPCError::IncompleteStream),
//                     }
//                 }
//             }
//         }
//         .boxed()
//     }
// }
