# Collection of Beacon Chain Information

## Table of Contents

1. [Consensus-spec](#consensus-spec)
2. [Discv5](#discv5)
3. [libp2p](#libp2p)
4. [Req/Res domain](#reqres-domain)

## Consensus-spec

- **Consensus-spec**: The consensus-spec is a document that describes the rules of the Ethereum 2.0 network. It specifies how the network should behave, how blocks should be created, how blocks should be validated, and more. The consensus-spec is used by all Ethereum 2.0 clients to ensure that they all behave in the same way. This is important because it allows all clients to agree on the state of the network. If clients behaved differently, they would disagree on the state of the network, which would lead to a network split.

---

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

---

## libp2p

- **libp2p**: libp2p is a modular network stack that can be used to build peer-to-peer applications. It provides a suite of protocols for various aspects of peer-to-peer communication, including transport (how data is physically sent from one node to another), multiplexing (how multiple data streams can be sent over a single connection), peer discovery (how nodes find each other), and more. Ethereum 2.0 uses libp2p for most of its networking needs, including establishing connections between nodes and sharing blockchain data.

### What do we need for libp2p and managing the peers?

- **Swarm**
- **Kademlia**

---

## Req/Res domain

### Protocol identification
- _Each message in this system is identified by a string with four parts, separated by slashes_:
  - **ProtocolPrefix**: This groups messages into families. For this specific system, the prefix is "/eth2/beacon_chain/req".
  - **MessageName**: This is an identifier for each request, made up of English letters, numbers, and underscores.
  - **SchemaVersion**: This is a version number that is used to manage compatibility between different versions of the protocol.
  - **Encoding**: This is a description of how the bytes in the message are arranged, or encoded.
  - **Req/Res**: Each request/response interaction between two peers uses a single stream, once the interaction is finished, the stream is closed.

### Req/Resp interaction
- _The requesting side and the responding side must both follow a series of steps_:
  - **Requesting side**: After negotiating a new stream, the peer should send the request immediately, close the write side of the stream, and wait for the response.
  - **Responding side**: The peer should process the incoming request, validate it, read and deserialize the expected data type, process the request, write the response, and close its side of the stream. If any of these steps fail, the responder must respond with an error.

### Response codes
- _These are single-byte values at the beginning of each response chunk that signal the status of the response_:
  - `0` indicates success
  - `1` means the request was invalid
  - `2` means there was a server error
  - `3` means the requested resource is unavailable
  - codes above 128 can be used for custom error responses.