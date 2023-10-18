use std::error::Error;

use slog::{debug, Logger};

use crate::network_manager::NetworkManager;

#[derive(Clone, Debug)]
pub struct Redirect {
    network_manager: NetworkManager,
    log: Logger,
}

impl Redirect {}

pub fn proxy() -> Result<(), Box<dyn Error>> {
    Ok(())
}
