pub mod inbound;
pub mod outbound;

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
pub struct NetworkManager {
    // pub network_sender: NetworkSender,
    // pub network_receiver: NetworkReceiver,
    // requests: Vec<Request>,
    log: Logger,
}

impl NetworkManager {
    pub async fn new(log: Logger) -> Self {
        NetworkManager {
            // network_sender: NetworkSender::new(log.clone()),
            // network_receiver: NetworkReceiver::new(log.clone()),
            // requests: Vec::new(),
            log,
        }
    }

    // pub fn add_request(&mut self, request: Request<N>) {
    //     debug!(self.log, "Adding request"; "request" => format!("{:?}", request));
    //     self.requests.push(request);
    // }

    // pub fn process_requests(&mut self) {
    //     // for request in self.requests.drain(..) {
    //     //     debug!(self.log, "Processing request"; "request" => format!("{:?}", request));
    //     // }
    // }
}
