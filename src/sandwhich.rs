#![deny(unsafe_code)]

use dotenv::dotenv;
use ethers::prelude::*;
use ethers_flashbots::*;
use eyre::Result;

// probs going to end up merging all of these into one function

// normal sandwhich trade
pub async fn sandwhich() {}

// sandwhich trade with backrun
pub async fn sandwhich_backrun() {}

// sandwhich trade with arbitrage
pub async fn sandwhich_arb() {}

// sandwhich trade with arbitrage and backrun
pub async fn sandwhich_arb_backrun() {}

// sandwhich trade with multiple tokens
pub async fn sandwhich_multi() {}

// sandwhich trade with multiple tokens and backrun
pub async fn sandwhich_multi_backrun() {}

// sandwhich trade with multiple tokens and arbitrage
pub async fn sandwhich_multi_arb() {}

// sandwhich trade with multiple tokens, arbitrage, and backrun
pub async fn sandwhich_multi_arb_backrun() {}
