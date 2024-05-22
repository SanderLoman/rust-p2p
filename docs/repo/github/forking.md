# Forking the Repository

Forking a repository creates a copy of the repository in your own GitHub account. This allows you to make changes without affecting the original repository. Forking is commonly used when you want to contribute to a project or customize the code.

## How to Fork a Repository

Follow these steps to fork a repository:

1.  **Navigate to the Repository**:
    Go to the GitHub repository you want to fork.

2.  **Fork the Repository**:
    Click the "Fork" button in the top right corner of the repository page. This will create a copy of the repository in your GitHub account.

3.  **Clone the Repository**:
    Once you have forked the repository, it will appear in your GitHub account. To work on the repository locally, clone it to your computer by running the following command in your terminal:

        git clone https://github.com/YOUR_GITHUB_USERNAME/contower.git
        cd contower

4.  **Add an Upstream Remote**:
    To keep your fork up to date with the original repository, add an upstream remote. Track the `unstable` branch, as it contains the latest code. While this might seem counterintuitive since you typically want stable code, the `unstable` branch undergoes thorough testing before its changes are merged into the stable branch. This approach ensures you have the most recent updates for development purposes. Run the following command in your terminal:

        git remote add upstream https://github.com/nodura/contower.git

5.  **Fetch the `unstable` Branch**:
    Fetch the latest changes from the `unstable` branch of the upstream repository:

        git fetch upstream unstable

6.  **Create a Feature Branch**:
    Create a new feature branch from the `unstable` branch for your work:

        git checkout -b your_feature_name upstream/unstable

    Choose a short and descriptive name for your branch. For example, if you're fixing a bug with serialization, you could name your branch `fix_cli_bug`. Or something else that describes the work you're doing.

## Keeping Your Fork Updated

To keep your feature branch up-to-date with the latest changes from the `unstable` branch of the original repository, follow these steps:

1.  **Fetch the Latest Changes from Upstream**:
    Fetch the latest changes from the `unstable` branch of the upstream repository:

        git fetch upstream

2.  **Pull the Changes into Your Feature Branch**:
    Pull the latest changes from `unstable` into your feature branch to keep it updated:

        git checkout your_feature_name
        git pull upstream unstable

## Pushing Your Changes

Once you've made your changes and committed them locally, you need to push your feature branch to your fork on GitHub. Follow these steps:

1.  **Push Your Feature Branch**:
    Push your feature branch to your fork on GitHub:

        git push origin your_feature_name

2.  **Create a Pull Request**:
    After pushing your branch to your fork, go to your fork on GitHub, switch to the branch you just pushed, and click the "Compare & pull request" button to create a pull request. Fill in the necessary details and submit the pull request.

By following these steps, you can ensure that your fork stays up-to-date with the latest changes from the original repository, and your contributions can be easily integrated. This workflow allows you to work on new features or fixes while keeping your codebase in sync with the upstream repository.
