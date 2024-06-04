# stale.yml Documentation

This GitHub Actions workflow is designed to automatically mark and close stale issues and pull requests in a GitHub repository. The workflow is triggered by a scheduled cron job and manual dispatch.

## Workflow Triggers

The workflow is triggered by:

-   A cron schedule that runs at midnight every day (`0 0 * * *`).
-   Manual dispatch using the `workflow_dispatch` event.

## Jobs Overview

The workflow consists of a single job: `stale`.

### Stale Job

-   **Runs on**: `ubuntu-latest`

**Permissions**:

-   Grants `write` permissions for issues and pull requests.

**Steps**:

1. **Mark and close stale issues and PRs**:
    - Uses the `actions/stale@v9` action to mark and close stale issues and pull requests.
    - Configurations for the `actions/stale` action:
        - `days-before-stale`: 30 days of inactivity before an issue or PR is marked as stale.
        - `days-before-close`: 7 days after being marked as stale before an issue or PR is closed.
        - `stale-issue-label`: Label used to mark stale issues (`LS-Stale`).
        - `stale-pr-label`: Label used to mark stale pull requests (`LS-Stale`).
        - `stale-issue-message`: Message added to stale issues.
        - `stale-pr-message`: Message added to stale pull requests.
        - `close-issue-message`: Message added when an issue is closed due to inactivity.
        - `close-pr-message`: Message added when a pull request is closed due to inactivity.
        - `exempt-issue-labels`: Label that prevents issues from being marked as stale (`P-Prevent-stale`).
        - `exempt-pr-labels`: Label that prevents pull requests from being marked as stale (`P-Prevent-stale`).
        - `exempt-all-assignees`: Exempts issues and PRs with any assignees from being marked as stale.
        - `exempt-all-milestones`: Exempts issues and PRs with any milestones from being marked as stale.
        - `repo-token`: Uses the repository's GitHub token for authentication (`${{ secrets.GITHUB_TOKEN }}`).

This workflow ensures that issues and pull requests that have been inactive for a specified period are automatically marked as stale and eventually closed if there is no further activity.
