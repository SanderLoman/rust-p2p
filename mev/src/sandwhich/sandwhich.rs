#![deny(unsafe_code)]
#![allow(unused)]

use eyre::Result;
use std::error::Error;

// probs going to end up merging all of these into one function

// normal sandwhich trade
pub async fn sandwhich() -> Result<(), Box<dyn Error>> {
    Ok(())
}

// sandwhich trade with backrun
pub async fn sandwhich_backrun() -> Result<(), Box<dyn Error>> {
    Ok(())
}

// sandwhich trade with arbitrage
pub async fn sandwhich_arb() -> Result<(), Box<dyn Error>> {
    Ok(())
}

// sandwhich trade with arbitrage and backrun
pub async fn sandwhich_arb_backrun() -> Result<(), Box<dyn Error>> {
    Ok(())
}

// sandwhich trade with multiple tokens
pub async fn sandwhich_multi() -> Result<(), Box<dyn Error>> {
    Ok(())
}

// sandwhich trade with multiple tokens and backrun
pub async fn sandwhich_multi_backrun() -> Result<(), Box<dyn Error>> {
    Ok(())
}

// sandwhich trade with multiple tokens and arbitrage
pub async fn sandwhich_multi_arb() -> Result<(), Box<dyn Error>> {
    Ok(())
}

// sandwhich trade with multiple tokens, arbitrage, and backrun
pub async fn sandwhich_multi_arb_backrun() -> Result<(), Box<dyn Error>> {
    Ok(())
}
