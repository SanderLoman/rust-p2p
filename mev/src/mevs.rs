// this file will be used that combine all the code into 1 single file and make all the stuff happen.

#![deny(unsafe_code)]

use crate::arbitrage::*;
use crate::liquidations::*;
use crate::sandwhich::*;
use eyre::Result;
use std::error::Error;

pub async fn meving() -> Result<(), Box<dyn Error>> {
    let _ = sandwhich::sandwhich().await?;
    let _ = arbitrage::arbitrage().await?;
    let _ = liquidations::liquidations().await?;

    Ok(())
}
