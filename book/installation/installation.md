# Installation

Contower runs on Linux, macOS, and Windows.

Installation of Contower is straightforward and can be achieved through a few methods:

-   [Pre-built binaries](./binaries.md).
-   [Docker images](./docker.md).
-   [Building from source](./source.md).

We also offer additional guides for specific platforms and use cases:

-   [Raspberry Pi guide](./pi.md).
-   [Cross-compiling for developers](./cross-compiling.md).

## System Requirements

To ensure optimal performance of Contower, it's important to consider the hardware requirements. These requirements are tailored to accommodate the diverse functionalities of Contower, catering to its relay, execution, and consensus components.

|         | Relay Client                      | Execution Client                  | Consensus Client                  |
| ------- | --------------------------------- | --------------------------------- | --------------------------------- |
| CPU     | Quad-core AMD Ryzen or Intel Core | Quad-core AMD Ryzen or Intel Core | Quad-core AMD Ryzen or Intel Core |
| Memory  | 4 GB RAM                          | 8 GB RAM                          | 16 GB RAM                         |
| Storage | < 1 GB SSD                        | 1 TB SSD                          | 2 TB SSD                          |
| Network | Stable 40 Mbps+                   | Stable 40 Mbps+                   | Stable 40 Mbps+                   |

_Note: These specifications represent minimum recommendations. Performance may vary based on network conditions and specific configurations. It is advisable to review these requirements in light of your planned usage of Contower._

### Storage Considerations

When selecting a storage device, it's recommended to use SSDs for their speed and reliability, especially for the Full Node Client. NVMe SSDs are preferred for even better performance. HDDs can be used but may result in slower operation, particularly during syncing processes.

### Network Requirements

A stable and reliable internet connection is crucial for both the initial synchronization and ongoing operations. Ensure your internet service can consistently meet the recommended bandwidth requirements.

## Installation Process

The installation process of Contower is designed to be as smooth and user-friendly as possible. Here's a brief overview of the steps:

1. **Download**: Choose your preferred method of installation. For most users, pre-built binaries are the simplest option.

2. **Configuration**: After installation, configure Contower according to your needs. You can set it up as a Relay/Proxy client, a Full Node client, or both.

3. **Initialization**: Run Contower to initialize and start syncing with the Ethereum network. This process might take some time depending on your hardware and network connection.

4. **Operation**: Once synced, Contower will begin its operation, either relaying information, executing transactions, or both, depending on your configuration.

## Encountering Issues?

If you encounter any issues during the installation process, please join our [Discord](https://discord.gg/7hPv2T6) and ask for help in the `#🦾support` channel or make a post in the `#community-forum`. Our team and community members will be happy to assist you.
