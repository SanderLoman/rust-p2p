# Cross-compiling

Contower supports cross-compiling, allowing users to run a binary on one platform (e.g., aarch64) that was compiled on another platform (e.g., x86_64).

## Instructions

Cross-compiling requires [Docker](https://www.docker.com/products/docker-desktop/), [`rustembedded/cross`](https://github.com/cross-rs/cross), and for the current user to be in the Docker group.

The binaries will be created in the `target/` directory of the Contower project.

## Checking Docker Group Membership

To ensure you have the necessary permissions to run Docker commands without using `sudo`, follow these steps to check if you are already in the Docker group:

1. Open your terminal.
2. Type the following command:
    ```bash
    groups
    ```
3. Look for `docker` in the list of groups displayed. If `docker` is listed, you are already in the Docker group and no further action is needed.

If you are not in the Docker group, follow the steps below to add yourself to the Docker group.

## Adding User to Docker Group

To run Docker commands without `sudo`, add your user to the Docker group:

1.  **Create the Docker group (if it doesn't already exist):**

    ```bash
    sudo groupadd docker
    ```

2.  **Add your user to the Docker group:**

    ```bash
    sudo usermod -aG docker $USER
    ```

3.  **Restart the Docker service:**

    ```bash
    sudo systemctl restart docker
    ```

4.  **Log out and log back in so that your group membership is re-evaluated, or you can use:**
    ```bash
    newgrp docker
    ```

After following these steps, you will be able to run Docker commands without needing to prepend them with `sudo`.

## Targets

The Makefile in the project contains four targets for cross-compiling:

-   **build-x86_64**: builds an optimized version for x86_64 processors (suitable for most users).
-   **build-x86_64-portable**: builds a version for x86_64 processors which avoids using some modern CPU instructions that are incompatible with older CPUs.
-   **build-aarch64**: builds an optimized version for 64-bit ARM processors (suitable for Raspberry Pi 4).
-   **build-aarch64-portable**: builds a version for 64-bit ARM processors which avoids using some modern CPU instructions. In practice, very few ARM processors lack the instructions necessary to run the faster non-portable build.
<!-- TODO: Add link to portability -->

For more information about optimized vs portable builds see [Portability](#).

## Example

```bash
cd contower
make build-aarch64
```

The contower binary will be compiled inside a Docker container and placed in `contower/target/aarch64-unknown-linux-gnu/release`.

## Feature Flags

<!-- TODO -->

We are still working on this feature.

<!-- When using the Makefile, the set of features used for building can be controlled with the environment variable `CROSS_FEATURES`. See [Feature Flags](#) for available features. -->

## Compilation Profiles

<!-- TODO -->

We are still working on this feature.

<!-- When using the Makefile, the build profile can be controlled with the environment variable `CROSS_PROFILE`. See [Compilation Profiles](#) for available profiles. -->
