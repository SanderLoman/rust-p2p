# Custom Ethereum Beacon Node in Rust

## Overview

This project aims to create a custom Ethereum beacon node using Rust. The node is designed to participate in the Ethereum 2.0 network post-merge. Unlike a full-fledged beacon node, this custom node does not maintain an internal database to keep track of the blockchain. Instead, it relies on another "real" Ethereum beacon node to fetch block data as needed.

## Goals

1. **Network Participation**: The node should be able to send and receive data from other peers in the network.
2. **Peer Discovery**: The node should find ALL available peers on the network.
3. **Block Request**: Each time a new block is received, the node will already start requesting the latest head block from all peers to find the lastest or newest block again.

## How It Works

1. **Initialization**: The node starts and initializes its network connections.
2. **Peer Discovery**: The node discovers peers in the Ethereum 2.0 network.
3. **Block Fetching**: The node sends a request to all discovered peers asking for their latest head block.
4. **Block Checking**: The node checks whether the "lastest" block is really the lastest one.
