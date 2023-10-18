pub mod transport;
pub mod protocol;


use futures::{channel::mpsc, Future};

#[derive(Debug)]
pub struct NetworkService {
    network_recv: mpsc::UnboundedReceiver<()>,

    network_send: mpsc::UnboundedSender<()>,
}
