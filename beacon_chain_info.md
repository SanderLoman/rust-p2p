# Collection of Beacon Chain Information

## Table of Contents

1. [Discv5](#Discv5)
2. [libp2p](#libp2p)

## Discv5

- **Discv5**: Discv5 is a protocol used for node discovery. It allows Ethereum nodes to find each other on the network. Once nodes have discovered each other, they can establish connections and start sharing data. However, Discv5 doesn't dictate how these connections should be established or how data should be shared. It's only concerned with node discovery.

### What do we need for Discv5 and discovering peer?

- **ENR of the consensus client**
  - blah blah blah
- **Listenings address**
  - blah blah blah
- **Discv5 config**
  - blah blah blah
- ****

## libp2p

- **libp2p**: libp2p is a modular network stack that can be used to build peer-to-peer applications. It provides a suite of protocols for various aspects of peer-to-peer communication, including transport (how data is physically sent from one node to another), multiplexing (how multiple data streams can be sent over a single connection), peer discovery (how nodes find each other), and more. Ethereum 2.0 uses libp2p for most of its networking needs, including establishing connections between nodes and sharing blockchain data.

### What do we need for libp2p and managing the peers?

- ****