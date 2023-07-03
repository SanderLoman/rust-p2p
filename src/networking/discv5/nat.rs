use if_addrs::get_if_addrs;
use slog::{debug, info, error};
use std::net::{IpAddr, SocketAddr, SocketAddrV4};
use igd;
use igd::SearchOptions;
use std::time::Duration;

/// Configuration required to construct the UPnP port mappings.
pub struct UPnPConfig {
    /// The local tcp port.
    pub tcp_port: u16,
    /// The local udp port.
    pub udp_port: u16,
}

impl UPnPConfig {
    pub fn set_upnp_mappings(config: UPnPConfig, log: slog::Logger) {
        construct_upnp_mappings(config, log);
    }
}

/// Attempts to construct external port mappings with UPnP.
pub fn construct_upnp_mappings(config: UPnPConfig, log: slog::Logger) {
    info!(log, "UPnP Attempting to initialise routes");
    let opts = SearchOptions {
        timeout: Some(Duration::from_secs(30)),
        ..Default::default()
    };
    match igd::search_gateway(opts) {
        Err(e) => error!(log, "UPnP not available"; "error" => %e),
        Ok(gateway) => {
            // Need to find the local listening address matched with the router subnet
            let interfaces = match get_if_addrs() {
                Ok(v) => v,
                Err(e) => {
                    info!(log, "UPnP failed to get local interfaces"; "error" => %e);
                    return;
                }
            };
            let local_ip = interfaces.iter().find_map(|interface| {
                if !interface.is_loopback() {
                    interface.ip().is_ipv4().then(|| interface.ip())
                } else {
                    None
                }
            });

            let local_ip = match local_ip {
                None => {
                    info!(log, "UPnP failed to find local IP address");
                    return;
                }
                Some(v) => v,
            };

            debug!(log, "UPnP Local IP Discovered"; "ip" => ?local_ip);

            match local_ip {
                IpAddr::V4(address) => {
                    let libp2p_socket = SocketAddrV4::new(address, config.tcp_port);
                    let external_ip = gateway.get_external_ip();
                    let tcp = add_port_mapping(
                        &gateway,
                        igd::PortMappingProtocol::TCP,
                        libp2p_socket,
                        "tcp",
                        &log,
                    ).and_then(|_| {
                        let external_socket = external_ip.as_ref().map(|ip| SocketAddr::new((*ip).into(), config.tcp_port)).map_err(|_| ());
                        info!(log, "UPnP TCP route established"; "external_socket" => format!("{}:{}", external_socket.as_ref().map(|ip| ip.to_string()).unwrap_or_else(|_| "".into()), config.tcp_port));
                        external_socket
                    }).ok();

                    let udp = {
                        let discovery_socket = SocketAddrV4::new(address, config.udp_port);
                        add_port_mapping(
                            &gateway,
                            igd::PortMappingProtocol::UDP,
                            discovery_socket,
                            "udp",
                            &log,
                        ).and_then(|_| {
                            let external_socket = external_ip
                                    .map(|ip| SocketAddr::new(ip.into(), config.udp_port)).map_err(|_| ());
                        info!(log, "UPnP UDP route established"; "external_socket" => format!("{}:{}", external_socket.as_ref().map(|ip| ip.to_string()).unwrap_or_else(|_| "".into()), config.udp_port));
                        external_socket
                    }).ok()
                    };

                    info!(log, "UPnP routes established"; "tcp_socket" => ?tcp, "udp_socket" => ?udp)
                }
                _ => debug!(log, "UPnP no routes constructed. IPv6 not supported"),
            }
        }
    };
}

/// Sets up a port mapping for a protocol returning the mapped port if successful.
fn add_port_mapping(
    gateway: &igd::Gateway,
    protocol: igd::PortMappingProtocol,
    socket: SocketAddrV4,
    protocol_string: &'static str,
    log: &slog::Logger,
) -> Result<(), ()> {
    let mapping_string = &format!("lighthouse-{}", protocol_string);
    for _ in 0..2 {
        match gateway.add_port(protocol, socket.port(), socket, 0, mapping_string) {
            Err(e) => {
                match e {
                    igd::AddPortError::PortInUse => {
                        // Try and remove and re-create
                        debug!(log, "UPnP port in use, attempting to remap"; "protocol" => protocol_string, "port" => socket.port());
                        match gateway.remove_port(protocol, socket.port()) {
                            Ok(()) => {
                                debug!(log, "UPnP Removed port mapping"; "protocol" => protocol_string,  "port" => socket.port())
                            }
                            Err(e) => {
                                debug!(log, "UPnP Port remove failure"; "protocol" => protocol_string, "port" => socket.port(), "error" => %e);
                                return Err(());
                            }
                        }
                    }
                    e => {
                        info!(log, "UPnP TCP route not set"; "error" => %e);
                        return Err(());
                    }
                }
            }
            Ok(_) => {
                return Ok(());
            }
        }
    }
    Err(())
}

// !!! MAYBE NOT NEEDED !!!
// Removes the specified TCP and UDP port mappings.
// pub fn remove_mappings(tcp_port: Option<u16>, udp_port: Option<u16>, log: &slog::Logger) {
//     if tcp_port.is_some() || udp_port.is_some() {
//         debug!(log, "Removing UPnP port mappings");
//         match igd::search_gateway(Default::default()) {
//             Ok(gateway) => {
//                 if let Some(tcp_port) = tcp_port {
//                     match gateway.remove_port(igd::PortMappingProtocol::TCP, tcp_port) {
//                         Ok(()) => debug!(log, "UPnP Removed TCP port mapping"; "port" => tcp_port),
//                         Err(e) => {
//                             debug!(log, "UPnP Failed to remove TCP port mapping"; "port" => tcp_port, "error" => %e)
//                         }
//                     }
//                 }
//                 if let Some(udp_port) = udp_port {
//                     match gateway.remove_port(igd::PortMappingProtocol::UDP, udp_port) {
//                         Ok(()) => debug!(log, "UPnP Removed UDP port mapping"; "port" => udp_port),
//                         Err(e) => {
//                             debug!(log, "UPnP Failed to remove UDP port mapping"; "port" => udp_port, "error" => %e)
//                         }
//                     }
//                 }
//             }
//             Err(e) => debug!(log, "UPnP failed to remove mappings"; "error" => %e),
//         }
//     }
// }
