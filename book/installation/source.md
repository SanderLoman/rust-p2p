# Building From Source

Building Contower from source is a straightforward process that can be done on Linux, macOS, and Windows. This guide will walk you through the steps required to build Contower from source.

## Prerequisites

Before building Contower from source, ensure you have the following prerequisites installed on your system:

-   [Rust](https://www.rust-lang.org/tools/install)
-   [Git](https://git-scm.com/downloads)
-   [CMake](https://cmake.org/download)

We will show you how to install these prerequisites on different operating systems.

### Linux (ubuntu, debian)

To install the prerequisites on Linux, run the following commands:

```bash
sudo apt update && sudo apt install -y git gcc g++ make cmake pkg-config llvm-dev libclang-dev clang
```

### Linux (Fedora, RHEL, CentOS)

To install the prerequisites on Fedora, RHEL, or CentOS, run the following commands:

```bash
yum -y install git make perl clang cmake
```

### macOS

To install the prerequisites on macOS, you can use [Homebrew](https://brew.sh/). Run the following commands:

```bash
brew install cmake
```

### Windows

1. Install [Git](https://git-scm.com/download/win).
1. Install the [Chocolatey](https://chocolatey.org/install) package manager for Windows.
    > Tips:
    >
    > - Use PowerShell to install. In Windows, search for PowerShell and run as administrator.
    > - You must ensure `Get-ExecutionPolicy` is not Restricted. To test this, run `Get-ExecutionPolicy` in PowerShell. If it returns `restricted`, then run `Set-ExecutionPolicy AllSigned`, and then run
    ```bash
    Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))
    ```
    > - To verify that Chocolatey is ready, run `choco` and it should return the version.
1. Install Make, CMake and LLVM using Chocolatey:

    ```powershell
    choco install make
    ```

    ```powershell
    choco install cmake --installargs 'ADD_CMAKE_TO_PATH=System'
    ```

    ```powershell
    choco install llvm
    ```

## Building Contower

Once you have installed the prerequisites, you can build Contower from source by following these steps:

1. Clone the Contower repository:

    ```bash
    git clone https://github.com/nodura/contower.git
    ```

2. Change to the Contower directory:

    ```bash
    cd contower
    ```

3. Build Contower using Make:

    ```bash
    git checkout stable
    ```

    ```bash
    make
    ```

## Update Contower

To update Contower to the latest version, run the following commands:

```bash
cd contower
```

```bash
git fetch
```

```bash
git checkout ${version}
```

```bash
make
```

## Feature Flags

Contower uses feature flags to enable or disable certain features during the build process. You can enable or disable these flags by setting the `FEATURES` environment variable. For example, to enable the `jemalloc` and `portable` features, run the following command:

```bash
FEATURES="jemalloc,portable" make
```

We are currently still working on implementing different features, for now you can use the following flag:

-   `jemalloc` - Enables jemalloc memory allocator.
-   `portable` - Enables portable builds.

Default features are enabled by default. To disable default features, use the following command:

```bash
CARGO_INSTALL_EXTRA_FLAGS="--no-default-features" make
```

## Compilation Profiles

Contower supports different compilation profiles to optimize the build process for different use cases. You can set the `PROFILE` environment variable to one of the following values:

-   `release` - default for source builds, enables most optimisations while not taking too long to compile.
-   `maxperf` - default for binary releases, enables aggressive optimisations including full LTO. Although compiling with this profile improves some benchmarks by around 20% compared to `release`, it imposes a significant cost at compile time and is only recommended if you have a fast CPU.

To build Contower with the `maxperf` profile, run the following command:

```bash
PROFILE="maxperf" make
```
