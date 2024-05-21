# Forking the Repository

Forking a repository creates a copy of the repository in your own GitHub account. This allows you to make changes without affecting the original repository. Forking is commonly used when you want to contribute to a project or customize the code.

## How to Fork a Repository

Follow these steps to fork a repository:

1. **Navigate to the Repository**:
   Go to the GitHub repository you want to fork.

2. **Fork the Repository**:
   Click the "Fork" button in the top right corner of the repository page. This will create a copy of the repository in your GitHub account.

3. **Clone the Repository**:
   Once you have forked the repository, it will appear in your GitHub account. To work on the repository locally, clone it to your computer by running the following command in your terminal:

    ```bash
    git clone https://github.com/YOUR_GITHUB_USERNAME/contower.git
    cd contower
    ```

4. **Add an Upstream Remote**:
   To keep your fork up to date with the original repository, add an upstream remote. Track the `unstable` branch since it contains the latest code. This allows you to pull changes from the original repository into your fork. Run the following command in your terminal:

    ```bash
    git remote add upstream https://github.com/nodura/contower.git
    ```

5. **Fetch the `unstable` Branch**:
   Fetch the latest changes from the `unstable` branch of the upstream repository:

    ```bash
    git fetch upstream unstable
    ```

6. **Create a Feature Branch**:
   Create a new feature branch from the `unstable` branch for your work:

    ```bash
    git checkout -b your_feature_name upstream/unstable
    ```

    Choose a short and descriptive name for your branch. For example, if you're fixing a bug with serialization, you could name your branch `fix_serialization_bug`.

## Keeping Your Fork Updated

To keep your feature branch up-to-date with the latest changes from the `unstable` branch of the original repository, follow these steps:

1. **Fetch the Latest Changes from Upstream**:
   Fetch the latest changes from the `unstable` branch of the upstream repository:

    ```bash
    git fetch upstream
    ```

2. **Rebase Your Feature Branch**:
   Rebase your feature branch on top of the latest changes from `unstable` for a clean history:

    ```bash
    git checkout your_feature_name
    git rebase upstream/unstable
    ```

    If you encounter conflicts, resolve them and continue the rebase:

    ```bash
    git rebase --continue
    ```

3. **Alternatively, Merge `unstable` into Your Feature Branch**:
   If you prefer, you can merge the latest changes from `unstable` into your feature branch:

    ```bash
    git checkout your_feature_name
    git merge upstream/unstable
    ```

    Resolve any conflicts and commit the merge.

By following these steps, you can ensure that your fork stays up-to-date with the latest changes from the original repository, and your contributions can be easily integrated.
