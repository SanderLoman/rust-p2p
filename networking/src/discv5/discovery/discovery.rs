#![deny(unsafe_code)]
#![deny(unsafe_code)]

use crate::create_logger;
use crate::discv5::enr::enr::generate_enr;
use discv5::*;
use discv5::{
    enr, handler, kbucket, metrics, packet, permit_ban, rpc, service, socket, Discv5, Discv5Config,
    Discv5ConfigBuilder, Discv5Event, Enr, ListenConfig,
};
use std::error::Error;

pub async fn setup_discovery_process() -> Result<(), Box<dyn Error>> {
    let log = create_logger();
    let enr = generate_enr().await?;


    Ok(())
}
