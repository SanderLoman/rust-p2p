⚠️ **This project is still in development and not ready for production use.** ⚠️

# Contower: A New All-in-One Ethereum Client

![Contower Banner](assets/repo_banner.webp)

[![build](https://github.com/nodura/conTower/actions/workflows/build.yml/badge.svg)](https://github.com/nodura/conTower/actions/workflows/build.yml)
[![tests](https://github.com/nodura/conTower/actions/workflows/tests.yml/badge.svg)](https://github.com/nodura/conTower/actions/workflows/tests.yml)
[![Codecov](https://img.shields.io/codecov/c/github/nodura/conTower?token=JT1850HR9J)](https://app.codecov.io/gh/nodura/conTower)
[![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

## Overview

Contower is our latest development for the Ethereum network, designed to bring flexibility and efficiency to blockchain operations. It uniquely integrates execution and consensus clients into a versatile relay/proxy networking client. Developed with Rust, Contower stands out for its optional reliance on a traditional database, allowing for streamlined and adaptable operations.

This client serves as an adaptable intermediary, capable of either facilitating relay communication and keeping track of the latest blockchain traffic with transient caching or functioning as a full node with its own database. Users have the choice to run Contower as a lean relay/proxy client, a complete node client incorporating execution and consensus mechanisms, or even both simultaneously. This flexibility ensures that Contower can meet various user needs, enhancing network functionality, efficiency, and decentralization, tailored to individual preferences and requirements.

### Execution, Consensus

The Exection and Consensus layer are the core components of Ethereum clients, responsible for processing transactions and maintaining the blockchain. Contower's execution and consensus clients are designed to be modular, allowing for easy integration with other Ethereum clients. This flexibility enables users to customize their client to suit their specific needs, whether they require a full node or a lightweight relay client.

### Relay Functionality

Contower's relay functionality acts as a bridge between various nodes, facilitating the flow of information without the need for storing the entire blockchain. It's ideal for users who wish to participate in the network with minimal resource usage, providing a lightweight option for staying connected and updated with the latest blockchain activities.

Contower's versatile design allows for these components to be used in combination or individually, providing a flexible solution that can integrate seamlessly with other Ethereum clients. Whether you require a full node capability, lightweight relay operations, or specific functionalities like execution or consensus, Contower offers a customizable solution to fit your needs.

## Community and Support

Engage with our community for discussions, support, and collaboration.

-   [GitHub Issues](https://github.com/nodura/conTower/issues)
-   [Discord](https://discord.gg/vHWpWsjCqx)

## Documentation

For comprehensive information, you can refer to two key resources:

1. [Book](https://nodura.github.io/Contower/): This resource provides an overview and installation guide for Contower, helping you get started with the project.

2. [docs](docs/) directory: This directory contains detailed documentation about the project's inner workings, including explanations of the static files and folders used, and how they function.

## Contributing

Eager to witness your contributions and innovations!

See [CONTRIBUTING.md](CONTRIBUTING.md) for more information.

## Security

See [SECURITY.md](SECURITY.md) for more information.
