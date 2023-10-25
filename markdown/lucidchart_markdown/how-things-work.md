# How the Beacon Node Relay Works

## Boot Node Connection for the Relay

This section describes how the boot node connections are set up, ensuring that the relay can discover other peers in the network.

- **Initial Setup**: When starting up the Relay, it will need to have at least one boot node connection. This could be an existing beacon node or a custom boot node. The boot node information is stored in a JSON file.

  - **Discovery Mechanism**: After reading the boot node data from the JSON file, the relay will start discovering other peers through a defined discovery mechanism.

- **Additional Peers**: When other peers are discovered, they should be added to the JSON file. This ensures that on subsequent restarts, the relay has more boot nodes to connect to.

**Flow**:

1. Relay starts up and establishes a connection to the boot node.
2. The discovery mechanism is activated, leading to the relay discovering more peers.

## How the Relay Operates

This section describes the relay's operational flow, particularly how it interacts with random peers and manages requests.

- **Random Peer Interaction**: The relay can use random peers that it has discovered to process different types of requests from other peers.

- **Handling Large Number of Peers**: In situations where there are millions of peers and only one beacon node, the relay manages stress and request handling. In these scenarios, it uses connections to the beacon node and forwards any requests to other already discovered peers using the relay's boot node or discovery mechanism.

**Flow**:

1. Relay receives requests from many peers.
2. Relay forwards these requests to its discovered peers.
3. The discovered peers send back their responses.

## How the Swarm Operates

[Content Needed]

## Redirect Mechanism

[Content Needed]
