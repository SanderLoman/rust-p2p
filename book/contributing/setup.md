# Setup Developer Environment

## Prerequisites

Before you begin, ensure you have the following tools installed on your system:

-   [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
-   [Git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git) (version control system)
-   [Make](https://www.gnu.org/software/make/)
-   [Docker](https://docs.docker.com/get-docker/)

## Setup

1. **Install Rust**: Follow the [official Rust installation guide](https://www.rust-lang.org/tools/install) to install the latest stable version of Rust.

2. **Install Git**: Follow the [official Git installation guide](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git) to install Git on your system.

3. **Install Make**: Make is a build automation tool that simplifies the build process. You can install Make using your package manager:

    - **Debian/Ubuntu**:

        ```bash
        sudo apt-get install make
        ```

    - **Fedora**:

        ```bash
        sudo dnf install make
        ```

    - **Arch Linux**:

        ```bash
        sudo pacman -S make
        ```

    - **macOS**:

        ```bash
        brew install make
        ```

    - **Windows**:

        You can install Make using the [Chocolatey package manager](https://chocolatey.org/):

        ```bash
        choco install make
        ```

4. **Install Docker**: Docker is a containerization platform that allows you to package and run applications in isolated environments. Follow the [official Docker installation guide](https://docs.docker.com/get-docker/) to install Docker on your system.

5. Fork and setup the [Contower repository](https://github.com/nodura/contower) on GitHub. As shown in the [Forking documentation](https://github.com/nodura/contower/blob/stable/docs/repo/github/forking.md).

## Linting and Formatting

To maintain code quality and consistency, use `clippy` for linting and `rustfmt` for formatting. Install these tools if you haven't already:

```bash
rustup component add clippy
rustup component add rustfmt
```

## Getting Help

If you encounter issues that you can't resolve, feel free to reach out for help:

-   Join our [Discord](https://discord.gg/vHWpWsjCqx) for real-time support.
-   Open an issue on the [GitHub repository](https://github.com/nodura/contower/issues).

By following these steps, you should be well on your way to contributing effectively to Contower. Thank you for your contributions!
