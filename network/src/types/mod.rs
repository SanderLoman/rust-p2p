// pub struct Forward {
//     pub requesting_peer: PeerId,
//     pub responding_peer: PeerId,
//     pub request: Request,
// }

// pub enum Request {
//     Ping(u64),
//     MetaData(u64),
//     BlocksByRoot(Vec<Hash256>),
//     BlocksByRange(Hash256, u64, u64),
//     Goodbye(u64),
//     Status(StatusMessage),
// }

// This is for my custom instruction that i will later update you with,
// word this better and make this within 1500 characters:

// Ok so i have an idea of making a sort of beacon node that will act as a relay or a proxy you could say. The main goals of this is to make sure that this "beacon node" is participating in the ethereum 2.0 network and that it will find all peers within the network, now that might sound hard to do but i think its possible. With that comes making connections to peers that we discover over time, however 1 beacon node can probably not handle millions of connections at once. So thats why i thought of this relay that where it would receive alot of request of other peers and then sort of "reflect" them to other peers, to make sure we dont overload our REAL beacon node (lighthouse). So it would look somewhat like this: Random_peer1 -> My_node -> Random_peer2, and then "Random_peer2" send it back the data (response) to "My_node" and we send it back to "Random_peer1". So like this Random_peer2 -> "My_node" -> Random_peer1.

// Load Balancing, i like this idea it kinda looks like the idea i had shown in the image from before.

// Cache Mechanism, i also really like this but i dont think this will work out for some requests, like the BlocksByRange that some peers request. It could differ in how many blocks that peer wants. I could be wrong tho maybe we could cache it and then extract the amount of blocks we need for that peer and send him that chunk?

// Rate Limiting and Throttling, good idea i think libp2p can handle this.

// Peer Prioritization, this also is a good idea, i think us having a good peer score is essential, but also if we somehow find a way to distinguish
