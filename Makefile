GIT_TAG := $(shell git describe --tags --candidates 1)
BIN_DIR = "bin"

X86_64_TAG = "x86_64-unknown-linux-gnu"
BUILD_PATH_X86_64 = "target/$(X86_64_TAG)/release"
AARCH64_TAG = "aarch64-unknown-linux-gnu"
BUILD_PATH_AARCH64 = "target/$(AARCH64_TAG)/release"

# List of features to use when building natively. Can be overridden via the environment.
# No jemalloc on Windows
ifeq ($(OS),Windows_NT)
    FEATURES?=
else
    FEATURES?=jemalloc
endif

# List of features to use when cross-compiling. Can be overridden via the environment.
CROSS_FEATURES ?= jemalloc

# Cargo profile for Cross builds. Default is for local builds, CI uses an override.
CROSS_PROFILE ?= release

# Cargo profile for regular builds.
PROFILE ?= release

# Extra flags for Cargo
CARGO_INSTALL_EXTRA_FLAGS?=

# Builds the contower binary in release (optimized).
#
# Binaries will most likely be found in `./target/release`
install:
	cargo install --path contower --force --locked \
		--features "$(FEATURES)" \
		--profile "$(PROFILE)"

# Function to check required commands and memberships. @ to suppress output.
check-required-tools:
	@which cross > /dev/null || (echo "cross is not installed, installing..." && cargo install cross)
	@docker --version > /dev/null || (echo "Docker is not installed. Please install Docker from https://www.docker.com." && exit 1)
	@groups | grep -q docker || (echo "The current user is not in the 'docker' group. Please add the user to the group using 'sudo usermod -aG docker $$USER' and restart your session." && exit 1)
	@cargo audit --version > /dev/null || (echo "cargo-audit is not installed, installing..." && cargo install cargo-audit)

# The *-portable options compile the blst library *without* the use of some
# optimized CPU functions that may not be available on some systems. This
# results in a more portable binary with ~20% slower BLS verification.

# TODO: 
# - Fix portable builds
# - Fix modern builds 
build-x86_64: check-required-tools
	cross build --bin contower --target $(X86_64_TAG) --features "$(CROSS_FEATURES)" --profile "$(CROSS_PROFILE)" --locked

build-x86_64-portable: check-required-tools
	cross build --bin contower --target $(X86_64_TAG) --features "portable,$(CROSS_FEATURES)" --profile "$(CROSS_PROFILE)" --locked

build-aarch64: check-required-tools
	cross build --bin contower --target $(AARCH64_TAG) --features "$(CROSS_FEATURES)" --profile "$(CROSS_PROFILE)" --locked

build-aarch64-portable: check-required-tools
	cross build --bin contower --target $(AARCH64_TAG) --features "portable,$(CROSS_FEATURES)" --profile "$(CROSS_PROFILE)" --locked

# Create a `.tar.gz` containing a binary for a specific target.
define tarball_release_binary
	cp $(1)/contower $(BIN_DIR)/contower
	cd $(BIN_DIR) && \
		tar -czf contower-$(GIT_TAG)-$(2)$(3).tar.gz contower && \
		rm contower
endef

# Create a series of `.tar.gz` files in the BIN_DIR directory, each containing
# a `contower` binary for a different target.
#
# The current git tag will be used as the version in the output file names. You
# will likely need to use `git tag` and create a semver tag (e.g., `v0.2.3`).
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

# Runs the full workspace tests in **release**, without downloading any additional
# test vectors.
test-release:
	cargo test --workspace --release

# Runs the full workspace tests in **debug**, without downloading any additional test
# vectors.
test-debug:
	cargo test --workspace

# Runs cargo-fmt (linter).
cargo-fmt:
	cargo fmt --all -- --check

# Typechecks benchmark code
check-benches:
	cargo check --workspace --benches

# Runs the full workspace tests in release, without downloading any additional
# test vectors.
test: test-release

# Runs the entire test suite, downloading test vectors if required.
test-full: cargo-fmt test-release test-debug test-ef

# Lints the code for bad style and potentially unsafe arithmetic using Clippy.
# Clippy lints are opt-in per-crate for now. By default, everything is allowed except for performance and correctness lints.
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

# Runs cargo audit (Audit Cargo.lock files for crates with security vulnerabilities reported to the RustSec Advisory Database).
audit: install-audit audit-CI

install-audit:
	cargo install --force cargo-audit

audit-CI: check-required-tools
	cargo audit

# Performs a `cargo` clean.
clean:
	cargo clean