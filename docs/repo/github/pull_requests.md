# Pull Request (PR) Guidelines

Creating a pull request is an essential part of the collaborative workflow in our project. It allows team members and contributors to review, discuss, and merge changes into the codebase. Follow these guidelines to ensure your pull requests are clear, organized, and efficient.

## Creating a Pull Request

### 1. Prepare Your Branch

Before creating a pull request, make sure your changes are committed to a branch. Follow these steps:

1. **Create a Branch:** Create a new branch for your changes. Name your branch descriptively, using a format like `feature/your-feature-name` or `bugfix/issue-number`.

    ```bash
    git checkout -b feature/your-feature-name
    ```

2. **Make Your Changes:** Implement your changes in the new branch.

3. **Commit Your Changes:** Commit your changes with a clear and descriptive commit message.

    ```bash
    git add .
    git commit -m "Add feature XYZ"
    ```

4. **Push Your Branch:** Push your branch to the remote repository.

    ```bash
    git push origin feature/your-feature-name
    ```

### 2. Create the Pull Request

To create a pull request, follow these steps:

1. **Navigate to the Repository:** Go to the [repository page](https://github.com/nodura/contower) on GitHub.

2. **Open the Pull Request Page:** Click on the "Pull requests" tab, then click on the "New pull request" button.

3. **Select Branches:** Choose the appropriate branches to merge from and to:

    - **Base:** Select the `unstable` branch as the base branch.
    - **Compare:** Select the branch that contains your changes.

4. **Fill in the PR Form:** Provide a clear and descriptive title for your pull request. In the description, include:

    - A brief summary of the changes.
    - Any related issue numbers (e.g., "Fixes #123").
    - Relevant details or screenshots that help explain the changes.

5. **Apply Labels:** Add relevant labels to your pull request. Labels help categorize the PR and make it easier for reviewers to understand its context. Examples include:

    - https://github.com/nodura/contower/labels/C-Enhancement
    - https://github.com/nodura/contower/labels/C-Bug
    - https://github.com/nodura/contower/labels/C-Documentation

6. **Link Issues:** If your pull request is related to an issue, link the issue in the description. This can be done by using keywords like "Fixes," "Closes," or "Resolves" followed by the issue number (e.g., "Fixes #123").

7. **Submit the Pull Request:** Click the "Create pull request" button to submit your pull request for review.

## Reviewing and Merging

Once your pull request is submitted, it will be reviewed by project maintainers or other contributors. Here are the steps involved:

1. **Review Process:** Reviewers will examine your code, provide feedback, and may request changes. Address any comments or requested changes promptly.

2. **Approval:** Once the pull request has been reviewed and approved, a maintainer will merge it into the base branch.

3. **Merging:** If you have write access, do not merge your own pull request unless you have explicit permission from the project maintainers.

## Best Practices for Pull Requests

-   **Keep It Small:** Try to keep your pull requests small and focused. Large pull requests can be difficult to review.
-   **Be Descriptive:** Provide a clear description and context for your changes. This helps reviewers understand the purpose and impact of your changes.
-   **Test Your Changes:** Ensure that your changes have been tested and do not break existing functionality.

By following these guidelines, we can ensure that our pull requests are clear, organized, and facilitate effective collaboration among all contributors.

For more information, refer to our [Issues Documentation](./issues).
