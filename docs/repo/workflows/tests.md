# tests.yml Documentation

This GitHub Actions workflow is designed to automatically run tests for a Rust project. The workflow is triggered by pushes and pull requests to specific branches.

## Workflow Triggers

The workflow is triggered by:

-   Push events on the `unstable` and `stable` branches.
-   Pull requests targeting the `unstable` and `stable` branches.

## Environment Variables

-   `CARGO_TERM_COLOR`: Set to `always` to ensure cargo always uses colored output.

## Jobs Overview

The workflow consists of a single job: `test`.

### Test Job

-   **Runs on**: `ubuntu-latest`

**Steps**:

1. **Checkout the repository**:
    - Uses the `actions/checkout@v3` action to check out the repository.
2. **Run tests**:
    - Runs the `cargo test --verbose` command to execute the tests for the Rust project with verbose output.

This workflow ensures that the Rust project's tests are run automatically on relevant events, providing continuous integration testing.
