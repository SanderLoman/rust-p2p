pub mod peer_manager;
pub mod requests;

use slog::{debug, Logger};
use tokio::sync::mpsc::{channel, Receiver, Sender};

// !!! Maybe not needed
//
// pub struct Proxy {
//     redirect: Redirect,
//     log: Logger,
// }

#[derive(Debug)]
pub struct NetworkManager {
    pub network_sender: Sender<()>,
    pub network_receiver: Receiver<()>,
    // requests: Vec<Request>,
    log: Logger,
}

impl NetworkManager {
    pub async fn new(log: Logger) -> Self {
        NetworkManager {
            network_sender: channel(1).0,
            network_receiver: channel(1).1,
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
