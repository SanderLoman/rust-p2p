pub mod discovery;
pub mod network_globals;
pub mod swarm;
pub mod task_executor;
pub mod transport;

use std::error::Error;
use std::pin::Pin;
use std::sync::Arc;

use libp2p::identity::Keypair;
#[allow(deprecated)]
use libp2p::swarm::SwarmBuilder;
use libp2p::{identify, Swarm};
use slog::Logger;

use crate::network::swarm::behaviour::Behaviour;
use crate::network::transport::build_transport;
use crate::version_with_platform;

use self::discovery::Discovery;
use self::network_globals::NetworkGlobals;

pub struct Network {
    // The libp2p Swarm, this will handle incoming and outgoing requests so that we can redirect them. Instead of sending data right back to them
    swarm: Swarm<Behaviour>,

    /// A collections of variables accessible outside the network service.
    network_globals: Arc<NetworkGlobals>,

    // The Logger for the network service.
    log: Logger,
}

impl Network {
    pub async fn new(local_keypair: Keypair, log: Logger) -> Result<Network, Box<dyn Error>> {
        let network_globals = {
            // Create an ENR or load from disk if appropriate
            let enr = crate::network::discovery::enr::generate_enr(log.clone()).await;
            // Create the network globals
            let globals = NetworkGlobals::new(enr, &log);
            Arc::new(globals)
        };

        let identify = {
            let local_public_key = local_keypair.public().clone().into();
            let identify_config = identify::Config::new("eth2/1.0.0".into(), local_public_key)
                .with_agent_version(version_with_platform())
                .with_cache_size(0);
            identify::Behaviour::new(identify_config)
        };

        let discovery = {
            // Build and start the discovery sub-behaviour
            let mut discovery = Discovery::new(local_keypair, log.clone()).await.unwrap();
            discovery
        };

        let behaviour = {
            Behaviour {
                discovery,
                identify,
            }
        };

        // might make network globals later on.
        let local_peer_id = network_globals.local_peer_id();

        println!("local peer id: {:?}", local_peer_id);

        let swarm = {
            // Set up the transport - tcp/ws with noise and mplex
            let transport = build_transport(local_keypair.clone().into())
                .map_err(|e| format!("Failed to build transport: {:?}", e))
                .unwrap();

            // use the executor for libp2p
            struct Executor(task_executor::TaskExecutor);
            impl libp2p::swarm::Executor for Executor {
                fn exec(&self, f: Pin<Box<dyn futures::Future<Output = ()> + Send>>) {
                    self.0.spawn(f, "libp2p");
                }
            }

            // sets up the libp2p connection limits
            // transport
            // behaviour
            // local_peer_id
            // executor
            #[allow(deprecated)]
            SwarmBuilder::with_executor(transport, behaviour, local_peer_id, Executor(executor))
                .build()
        };

        let mut network = Network {
            swarm,
            network_globals,
            log,
        };

        Ok(network)
    }
}
