# lint.yml Documentation

This GitHub Actions workflow is designed to automate the process of formatting, generating documentation, and checking the success of these tasks for a Rust project. The workflow is triggered by pushes to specific branches.

## Workflow Triggers

The workflow is triggered by:

-   Push events on the `unstable` and `stable` branches.

## Environment Variables

-   `CARGO_TERM_COLOR`: Set to `always` to ensure cargo always uses colored output.

## Jobs Overview

The workflow consists of three jobs: `fmt`, `docs`, and `lint-success`.

### 1. Format Job (`fmt`)

-   **Name**: `fmt`
-   **Runs on**: `ubuntu-latest`
-   **Timeout**: 30 minutes

**Steps**:

1. **Checkout the repository**:
    - Uses the `actions/checkout@v4` action to check out the repository.
2. **Set up Rust toolchain**:
    - Uses the `dtolnay/rust-toolchain@nightly` action to set up the nightly Rust toolchain with the `rustfmt` component.
3. **Check code formatting**:
    - Runs the `cargo fmt --all --check` command to check the formatting of the code.

### 2. Documentation Job (`docs`)

-   **Name**: `docs`
-   **Runs on**: `ubuntu-latest`
-   **Timeout**: 30 minutes

**Steps**:

1. **Checkout the repository**:
    - Uses the `actions/checkout@v4` action to check out the repository.
2. **Set up Rust toolchain**:
    - Uses the `dtolnay/rust-toolchain@nightly` action to set up the nightly Rust toolchain.
3. **Cache Rust build artifacts**:
    - Uses the `Swatinem/rust-cache@v2` action with caching enabled on failure.
4. **Generate documentation**:
    - Runs the `cargo doc --document-private-items` command to generate documentation, including private items.
    - Sets `RUSTDOCFLAGS` to `-D warnings` to treat warnings as errors during documentation generation.

### 3. Lint Success Job (`lint-success`)

-   **Name**: `lint success`
-   **Runs on**: `ubuntu-latest`
-   **Runs Always**: `if: always()`
-   **Depends on**: `fmt`, `docs`
-   **Timeout**: 30 minutes

**Steps**:

1. **Check job success**:
    - Uses the `re-actors/alls-green@release/v1` action to decide whether the required jobs (`fmt` and `docs`) succeeded or failed.
    - Passes the `needs` context to the action to check the status of the dependent jobs.

This workflow ensures that the Rust project's code is properly formatted, documentation is generated, and the success of these tasks is checked automatically on relevant events.
