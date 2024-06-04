# coverage.yml Documentation

This GitHub Actions workflow is designed to automate the process of running tests with coverage and uploading the results to Codecov for a Rust project. The workflow is triggered by pushes and pull requests to specific branches.

## Workflow Triggers

The workflow is triggered by:

-   Push events on the `unstable` and `stable` branches.
-   Pull requests targeting the `unstable` and `stable` branches.

## Environment Variables

-   `CARGO_TERM_COLOR`: Set to `always` to ensure cargo always uses colored output.

## Jobs Overview

The workflow consists of a single job: `coverage`.

### Coverage Job

-   **Runs on**: `ubuntu-latest`

**Steps**:

1. **Checkout the repository**:
    - Uses the `actions/checkout@v4` action to check out the repository.
2. **Set up Rust**:
    - Uses the `actions-rs/toolchain@v1` action to install the stable Rust toolchain.
    - Overrides any existing toolchain with the stable toolchain.
3. **Cache cargo and cargo-tarpaulin**:
    - Uses the `actions/cache@v3` action to cache the cargo and cargo-tarpaulin directories.
    - Caches:
        - `~/.cargo/bin`
        - `~/.cargo/registry/index`
        - `~/.cargo/registry/cache`
        - `~/.cargo/git/db`
    - Sets the cache key based on the OS and the hash of `Cargo.lock` files.
4. **Check if cargo-tarpaulin is installed**:
    - Runs a script to check if `cargo-tarpaulin` is installed.
    - If not installed, installs `cargo-tarpaulin`.
5. **Run tests with coverage**:
    - Runs the `cargo tarpaulin --out Xml --output-dir ./target/tarpaulin` command to execute tests and generate a coverage report in XML format.
6. **Upload coverage to Codecov**:
    - Uses the `codecov/codecov-action@v4.0.1` action to upload the coverage report to Codecov.
    - Specifies:
        - The Codecov token from the repository secrets.
        - The repository slug (`nodura/contower`).
        - The path to the coverage report file (`./target/tarpaulin/cobertura.xml`).
        - Flags the upload as `unittests`.
        - Names the upload `codecov-umbrella`.
        - Fails the CI if there is an error in the upload.
        - Enables verbose logging for the upload.

This workflow ensures that test coverage is measured and reported to Codecov automatically on relevant events.
