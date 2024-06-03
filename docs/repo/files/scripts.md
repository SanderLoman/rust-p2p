# Scripts Folder Documentation

The [scripts](../../../.github/scripts/) folder contains various scripts used in the project to automate and facilitate different tasks. Below is a list of scripts currently available and their purposes.

## Contents

-   [label_pr.js](../../../.github/scripts/label_pr.js)

## label_pr.js

### Description

`label_pr.js` is a script that labels Pull Requests (PRs) based on the labels of linked issues. It is designed to automate the process of copying labels from issues to the corresponding PRs.

### Usage

This script is used in conjunction with `label_pr.yml`, which provides the necessary configuration for its execution.

### How It Works

1. The `label_pr.yml` file triggers the execution of `label_pr.js`.
2. `label_pr.js` identifies the PRs linked to specific issues.
3. It copies the labels from the linked issues and applies them to the PRs.

### Future Scripts

As more scripts are added to the `./script` folder, this documentation will be updated to reflect their purposes and usage.
