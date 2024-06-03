## Dockerfile Documentation

This Dockerfile is used to build and run the `contower` application. It consists of two stages: the `staging` stage for building the application and the final stage for running the application.

### Staging Stage

The staging stage is responsible for building the `contower` binary.

1. **Base Image**

    ```Dockerfile
    FROM rust:1.78.0-bullseye AS staging
    ```

    Uses the official Rust image based on Debian Bullseye.

2. **Install Dependencies**

    ```Dockerfile
    RUN apt-get update && apt-get install -y cmake libclang-dev && rm -rf /var/lib/apt/lists/\*
    ```

    Installs `cmake` and `libclang-dev` which are necessary for building the project. The cache is cleared to reduce the layer size.

3. **Copy Source Code**

    ```Dockerfile
    COPY . contower
    ```

    Copies the entire source code into the container.

4. **Set Build Arguments and Environment Variables**

    ```Dockerfile
    ARG FEATURES
    ARG PROFILE=release
    ARG CARGO_USE_GIT_CLI=true
    ENV FEATURES $FEATURES
    ENV PROFILE $PROFILE
    ENV CARGO_NET_GIT_FETCH_WITH_CLI=$CARGO_USE_GIT_CLI
    ```

    Sets up arguments and environment variables to control the build process.

5. **Build the Application**
    ```Dockerfile
    RUN cd contower && make
    ```
    Navigates to the source code directory and builds the application using `make`.

### Final Stage

The final stage sets up the environment to run the built `contower` binary.

1. **Base Image**

    ```Dockerfile
    FROM ubuntu:22.04
    ```

    Uses the official Ubuntu 22.04 image.

2. **Install Dependencies**

    ```Dockerfile
    RUN apt-get update && apt-get install -y --no-install-recommends libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/\*
    ```

    Installs necessary runtime dependencies (`libssl-dev` and `ca-certificates`) and clears the cache to reduce the layer size.

3. **Copy Built Binary**

    ```Dockerfile
    COPY --from=staging /usr/local/cargo/bin/contower /usr/local/bin/contower
    ```

    Copies the built `contower` binary from the staging stage.

4. **Create Non-Root User**
    ```Dockerfile
    RUN useradd -m contower
    USER contower
    ```
    Creates a non-root user named `contower` for security purposes and sets this user as the default.

## Dockerfile.cross

This Dockerfile is used for cross-platform builds. It expects the `contower` binary to be pre-compiled for the target architecture and placed in the `./bin` directory.

1. **Base Image**

    ```Dockerfile
    FROM --platform=$TARGETPLATFORM ubuntu:22.04
    ```

    Uses the Ubuntu 22.04 image for the specified target platform.

2. **Install Dependencies**

    ```Dockerfile
    RUN apt-get update && apt-get install -y --no-install-recommends libssl-dev ca-certificates && apt-get clean && rm -rf /var/lib/apt/lists/\*
    ```

    Installs necessary runtime dependencies (`libssl-dev` and `ca-certificates`) and clears the cache to reduce the layer size.

3. **Copy Pre-Compiled Binary**

    ```Dockerfile
    COPY ./bin/contower /usr/local/bin/contower
    ```

    Copies the pre-compiled `contower` binary into the container.

4. **Create Non-Root User**
    ```Dockerfile
    RUN useradd -m contower
    USER contower
    ```
    Creates a non-root user named `contower` for security purposes and sets this user as the default.
