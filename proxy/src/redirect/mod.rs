use std::error::Error;

use slog::{debug, Logger};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub fn proxy() -> Result<(), Box<dyn Error>> {
    Ok(())
}
