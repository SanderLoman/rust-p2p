# GitHub Actions Workflow for Labeling Pull Requests

This GitHub Actions workflow is designed to automatically label pull requests when they are opened. The workflow is triggered by pull request events.

## Workflow Triggers

The workflow is triggered by:

-   Pull request events of type `opened`.

## Jobs Overview

The workflow consists of a single job: `label`.

### Label Job

-   **Runs on**: `ubuntu-latest`

**Permissions**:

-   Grants `write` permissions for issues and pull requests.

**Steps**:

1. **Checkout the repository**:
    - Uses the `actions/checkout@v4` action to check out the repository.
2. **Set up Node.js**:
    - Uses the `actions/setup-node@v3` action to set up Node.js.
    - Specifies the Node.js version as `18`.
3. **Install npm packages**:
    - Runs the `npm install @actions/github @actions/core` command to install the required GitHub Actions packages.
4. **Run the labeling script**:
    - Uses the `actions/github-script@v7` action to run a custom script.
    - The script executes the [`label_pr.js`](../../../.github/scripts/label_pr.js) file located in the [`.github/scripts/`](../../../.github/scripts/) directory.
    - The script is passed the `github` and `context` objects.

This workflow ensures that pull requests are automatically labeled when they are opened, using a custom script.
