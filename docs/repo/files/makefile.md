## Makefile Documentation

The makefile in the repository is used to automate various build, test, and deployment tasks for the `contower` application. Below is a detailed explanation of each target and its purpose.

### Variables

1. **GIT_TAG**

    ```makefile
    GIT_TAG := $(shell git describe --tags --candidates 1)
    ```

    Gets the latest Git tag. e.g., `v1.0.0`.

    This variable is used to name the tarballs.

2. **BIN_DIR**

    ```makefile
    BIN_DIR = "bin"
    ```

    Directory where binary files are stored.

3. **Build Path Variables**

    ```makefile
    X86_64_TAG = "x86_64-unknown-linux-gnu"
    BUILD_PATH_X86_64 = "target/$(X86_64_TAG)/release"
    AARCH64_TAG = "aarch64-unknown-linux-gnu"
    BUILD_PATH_AARCH64 = "target/$(AARCH64_TAG)/release"
    ```

    Specifies the build paths for different target architectures.

4. **Feature Flags**

    ```makefile
    ifeq ($(OS),Windows_NT)
    FEATURES?=
    else
    FEATURES?=jemalloc
    endif
    CROSS_FEATURES ?= jemalloc
    ```

    Defines features to use during the build process, depending on the operating system.

5. **Profiles**

    ```makefile
    CROSS_PROFILE ?= release
    PROFILE ?= release
    ```

    Specifies the profiles for cross-compilation and regular builds.

6. **Extra Flags**

    ```makefile
    CARGO_INSTALL_EXTRA_FLAGS?=
    ```

    Allows passing extra flags to Cargo.

### Targets

1. **Install**

    ```makefile
    install:
    cargo install --path contower --force --locked \
     --features "$(FEATURES)" \
            --profile "$(PROFILE)"
    ```

    Builds and installs the `contower` binary in release mode.

2. **Check Required Tools**

    ```makefile
    check-required-tools:
    @which cross > /dev/null || (echo "cross is not installed, installing..." && cargo install cross)
    @docker --version > /dev/null || (echo "Docker is not installed. Please install Docker from https://www.docker.com." && exit 1)
    @groups | grep -q docker || (echo "The current user is not in the 'docker' group. Please add the user to the group using 'sudo usermod -aG docker $$USER' and restart your session." && exit 1)
    @cargo audit --version > /dev/null || (echo "cargo-audit is not installed, installing..." && cargo install cargo-audit)
    ```

    Checks for required tools (`cross`, Docker, and `cargo-audit`) and installs them if necessary.

3. **Build Targets**

    ```makefile
    build-x86_64: check-required-tools
    cross build --bin contower --target $(X86_64_TAG) --features "$(CROSS_FEATURES)" --profile "$(CROSS_PROFILE)" --locked

    build-x86_64-portable: check-required-tools
    cross build --bin contower --target $(X86_64_TAG) --features "portable,$(CROSS_FEATURES)" --profile "$(CROSS_PROFILE)" --locked

    build-aarch64: check-required-tools
    cross build --bin contower --target $(AARCH64_TAG) --features "$(CROSS_FEATURES)" --profile "$(CROSS_PROFILE)" --locked

    build-aarch64-portable: check-required-tools
    cross build --bin contower --target $(AARCH64_TAG) --features "portable,$$(CROSS_FEATURES)" --profile "$(CROSS_PROFILE)" --locked
    ```

    Builds the `contower` binary for different architectures, both standard and portable builds.

4. **Tarball Release Binary**

    ```makefile
    define tarball_release_binary
    cp $(1)/contower $(BIN_DIR)/contower
        cd $(BIN_DIR) && \
            tar -czf contower-$(GIT_TAG)-$(2)$(3).tar.gz contower && \
     rm contower
    endef
    ```

    Defines a function to create a tarball containing the `contower` binary.

5. **Build Release Tarballs**

    ```makefile
    build-release-tarballs:
    [ -d $(BIN_DIR) ] || mkdir -p $(BIN_DIR)
        $(MAKE) build-x86_64
        $(call tarball_release_binary,$(BUILD_PATH_X86_64),$(X86_64_TAG),"")
        $(MAKE) build-x86_64-portable
        $(call tarball_release_binary,$(BUILD_PATH_X86_64),$(X86_64_TAG),"-portable")
        $(MAKE) build-aarch64
        $(call tarball_release_binary,$(BUILD_PATH_AARCH64),$(AARCH64_TAG),"")
        $(MAKE) build-aarch64-portable
        $(call tarball_release_binary,$(BUILD_PATH_AARCH64),$(AARCH64_TAG),"-portable")
    ```

    Creates tarballs for different build targets and stores them in the `BIN_DIR` directory.

6. **Test Targets**

    ```makefile
    test-release:
    cargo test --workspace --release

    test-debug:
    cargo test --workspace
    ```

    Runs the test suite in release and debug modes.

7. **Formatting and Linting**

    ```makefile
    cargo-fmt:
    cargo fmt --all -- --check

    check-benches:
    cargo check --workspace --benches

    lint:
    cargo clippy --workspace -- \
     -D clippy::fn_to_numeric_cast_any \
     -D clippy::manual_let_else \
     -D warnings \
     -A clippy::derive_partial_eq_without_eq \
     -A clippy::from-over-into \
     -A clippy::upper-case-acronyms \
     -A clippy::vec-init-then-push \
     -A clippy::question-mark \
     -A clippy::uninlined-format-args \
     -A clippy::enum_variant_names
    ```

    Runs `cargo fmt` for formatting checks and `cargo clippy` for linting.

8. **Security Audit**

    ```makefile
    audit: install-audit audit-CI

    install-audit:
    cargo install --force cargo-audit

    audit-CI: check-required-tools
    cargo audit
    ```

    Installs and runs `cargo-audit` to check for security vulnerabilities.

9. **Clean**

    ```makefile
    clean:
    cargo clean
    ```

    Cleans the build artifacts using `cargo clean`.

### Summary

-   **Install:** Builds and installs the `contower` binary.
-   **Check Required Tools:** Ensures all necessary tools are installed.
-   **Build Targets:** Compiles the binary for various architectures.
-   **Tarball Release:** Packages the binary into a tarball.
-   **Testing:** Runs tests in release and debug modes.
-   **Formatting and Linting:** Checks code formatting and lints the code.
-   **Security Audit:** Runs a security audit on the dependencies.
-   **Clean:** Removes build artifacts.

This makefile streamlines the development workflow by automating common tasks, ensuring consistency, and improving productivity. If you have any questions or need further assistance, feel free to ask!
