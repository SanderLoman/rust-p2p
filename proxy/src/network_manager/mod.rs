pub mod inbound;
pub mod outbound;

use crate::redirect::NetworkRequests;
use hyper::Request;
use slog::{debug, Logger};

use self::inbound::NetworkReceiver;
use self::outbound::NetworkSender;
// !!! Maybe not needed
//
// pub struct Proxy<N: NetworkRequests> {
//     redirect: Redirect<N>,
//     log: Logger,
// }

#[derive(Debug)]
pub struct NetworkManager<N: NetworkRequests> {
    pub network_sender: NetworkSender<N>,
    pub network_receiver: NetworkReceiver<N>,
    requests: Vec<Request<N>>,
    log: Logger,
}

impl<N: NetworkRequests + std::fmt::Debug> NetworkManager<N> {
    pub fn new(log: Logger) -> Self {
        NetworkManager {
            network_sender: NetworkSender::new(log.clone()),
            network_receiver: NetworkReceiver::new(log.clone()),
            requests: Vec::new(),
            log,
        }
    }

    pub fn add_request(&mut self, request: Request<N>) {
        debug!(self.log, "Adding request"; "request" => format!("{:?}", request));
        self.requests.push(request);
    }

    pub fn process_requests(&mut self) {
        // for request in self.requests.drain(..) {
        //     debug!(self.log, "Processing request"; "request" => format!("{:?}", request));
        // }
    }
}
