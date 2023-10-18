pub mod inbound;
pub mod methods;
pub mod outbound;
pub mod protocol;
pub mod transport;

use futures::{channel::mpsc, Future};

#[derive(Debug)]
pub struct NetworkService {
    network_recv: mpsc::UnboundedReceiver<()>,

    network_send: mpsc::UnboundedSender<()>,
}
