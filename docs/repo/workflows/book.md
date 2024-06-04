# book.yml Documentation

This GitHub Actions workflow is designed to automate testing, linting, building, and deploying an `mdBook` project. The workflow is triggered by various events such as pushes and pull requests to specific branches.

## Workflow Triggers

The workflow is triggered by:

-   Push events on the `unstable` and `stable` branches.
-   Pull requests targeting the `unstable` and `stable` branches.
-   Merge group events.

## Jobs Overview

The workflow consists of four main jobs: `test`, `lint`, `build`, and `deploy`.

### 1. Test Job

-   **Runs on**: `ubuntu-latest`
-   **Name**: `test`
-   **Timeout**: 60 minutes

**Steps**:

1. **Checkout the repository**:
    - Uses the `actions/checkout@v4` action.
2. **Install `mdbook`**:
    - Creates a directory named `mdbook`.
    - Downloads and extracts `mdbook` version `v0.4.14` to the `mdbook` directory.
    - Adds the `mdbook` directory to the `PATH`.
3. **Install `mdbook-template`**:
    - Creates a directory named `mdbook-template`.
    - Downloads and extracts the latest `mdbook-template` to the `mdbook-template` directory.
    - Adds the `mdbook-template` directory to the `PATH`.
4. **Run tests**:
    - Runs the `mdbook test` command.

### 2. Lint Job

-   **Runs on**: `ubuntu-latest`
-   **Name**: `lint`
-   **Timeout**: 60 minutes

**Steps**:

1. **Checkout the repository**:
    - Uses the `actions/checkout@v4` action.
2. **Install `mdbook-linkcheck`**:
    - Creates a directory named `mdbook-linkcheck`.
    - Downloads and extracts the latest `mdbook-linkcheck` to the `mdbook-linkcheck` directory.
    - Adds the `mdbook-linkcheck` directory to the `PATH`.
3. **Run linkcheck**:
    - Runs the `mdbook-linkcheck --standalone` command.

### 3. Build Job

-   **Runs on**: `ubuntu-latest`
-   **Timeout**: 60 minutes

**Steps**:

1. **Checkout the repository**:
    - Uses the `actions/checkout@v4` action.
2. **Install Rust toolchain**:
    - Uses the `dtolnay/rust-toolchain@nightly` action.
3. **Install `mdbook`**:
    - Creates a directory named `mdbook`.
    - Downloads and extracts `mdbook` version `v0.4.14` to the `mdbook` directory.
    - Adds the `mdbook` directory to the `PATH`.
4. **Install `mdbook-template`**:
    - Creates a directory named `mdbook-template`.
    - Downloads and extracts the latest `mdbook-template` to the `mdbook-template` directory.
    - Adds the `mdbook-template` directory to the `PATH`.
5. **Cache Rust build artifacts**:
    - Uses the `Swatinem/rust-cache@v2` action with caching enabled on failure.
6. **Build the book**:
    - Runs the `mdbook build` command.
7. **Build the documentation**:
    - Runs the `cargo doc --workspace --all-features --no-deps` command with specific `RUSTDOCFLAGS`.
8. **Move documentation to book folder**:
    - Moves the generated documentation from `target/doc` to `target/book/docs`.
9. **Archive the artifact**:
    - Sets appropriate file permissions.
    - Creates a tar archive of the `target/book` directory.
10. **Upload the artifact**:
    - Uses the `actions/upload-artifact@v3` action to upload the artifact with a retention period of 1 day.

### 4. Deploy Job

-   **Runs on**: `ubuntu-latest`
-   **Depends on**: `test`, `lint`, `build` jobs
-   **Timeout**: 60 minutes

**Conditions**:

-   Only runs on push events to the `stable` branch.

**Permissions**:

-   Grants `pages: write` and `id-token: write` permissions.

**Environment**:

-   Sets the environment name to `github-pages`.
-   Sets the environment URL to `${{ steps.deployment.outputs.page_url }}`.

**Steps**:

1. **Deploy to GitHub Pages**:
    - Uses the `actions/deploy-pages@v3` action to deploy the book to GitHub Pages.

This workflow ensures that the `mdBook` project is thoroughly tested, linted, built, and deployed automatically on relevant events.
