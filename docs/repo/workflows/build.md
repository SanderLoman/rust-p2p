# build.yml Documentation

This GitHub Actions workflow is designed to automate the build process for a Rust project. The workflow is triggered by pushes and pull requests to specific branches.

## Workflow Triggers

The workflow is triggered by:

-   Push events on the `unstable` and `stable` branches.
-   Pull requests targeting the `unstable` and `stable` branches.

## Environment Variables

-   `CARGO_TERM_COLOR`: Set to `always` to ensure cargo always uses colored output.

## Jobs Overview

The workflow consists of a single job: `build`.

### Build Job

-   **Runs on**: `ubuntu-latest`

**Steps**:

1. **Checkout the repository**:
    - Uses the `actions/checkout@v3` action to check out the repository.
2. **Install Rust**:
    - Uses the `actions-rs/toolchain@v1` action to install the stable Rust toolchain.
    - Specifies the `minimal` profile to install the minimal set of components.
    - Overrides any existing toolchain with the stable toolchain.
3. **Build the project**:
    - Runs the `cargo build` command to build the Rust project.

This workflow ensures that the Rust project is built automatically on relevant events, with the stable version of Rust.
