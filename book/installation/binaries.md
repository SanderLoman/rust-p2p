# Binaries

## Available Binaries

We provide binaries for the following platforms:

-   `x86_64-unknown-linux-gnu`
-   `aarch64-unknown-linux-gnu`
-   `x86_64-apple-darwin`
-   `x86_64-windows`

You can find these binaries on our [GitHub Releases page](https://github.com/nodura/contower/releases). We recommend downloading the latest release for your platform.

## Download and Extract

### Download from GitHub

After downloading the binary from GitHub in a `tar.gz` file format, extract it using the following command:

    ```bash
    tar -xvzf contower-*version*-*platform*.tar.gz
    ```

Replace `*version*` and `*platform*` with the correct details.

### Add to PATH

To make the binary accessible from any directory, add it to your `PATH`:

    ```bash
    export PATH=$PATH:/path/to/extracted/binary
    ```

Replace `/path/to/extracted/binary` with the actual path to your extracted binary.

## Comand Line Download

For comand line downloads and extractions, follow these platform-specific instructions:

### Linux (x86_64 and aarch64)

1.  **Download the binary:**
    ```bash
        curl -LO https://github.com/nodura/contower/releases/download/latest/contower-x86_64-unknown-linux-gnu.tar.gz
    ```
    For a portable version:
    ```bash
        curl -LO https://github.com/nodura/contower/releases/download/latest/contower-x86_64-unknown-linux-gnu-portable.tar.gz
    ```
2.  **Extract the binary:**
    ```bash
        tar -xvzf contower-x86_64-unknown-linux-gnu.tar.gz
    ```
3.  **Add to PATH:**
    ```bash
        export PATH=$PATH:/path/to/extracted/binary
    ```
    Update `/path/to/extracted/binary` to your specific extraction location.

### macOS (x86_64)

1.  **Download the binary:**
    ```bash
        curl -LO https://github.com/nodura/contower/releases/download/latest/contower-x86_64-apple-darwin.tar.gz
    ```
    For a portable version:
    ```bash
        curl -LO https://github.com/nodura/contower/releases/download/latest/contower-x86_64-apple-darwin-portable.tar.gz
    ```
2.  **Extract the binary:**
    ```bash
        tar -xvzf contower-x86_64-apple-darwin.tar.gz
    ```
3.  **Add to PATH:**
    ```bash
        export PATH=$PATH:/path/to/extracted/binary
    ```
    Modify `/path/to/extracted/binary` with the correct path.

### Windows (x86_64)

1. **Download the binary:**

    Navigate to [GitHub Releases](https://github.com/nodura/contower/releases) and select `contower-x86_64-windows.tar.gz` for download.

2. **Extract the binary:**

    - Right-click the downloaded file and select `Extract All`.
    - Choose a destination folder and click `Extract`.

3. **Add to PATH:**

    - Press `Win + R`, type `sysdm.cpl`, and press Enter.
    - Navigate to the `Advanced` tab and click `Environment Variables`.
    - In the `System variables` section, select `Path` and click `Edit`.
    - Click `New` and add the path to the extracted binary.
    - Confirm the changes by clicking `OK`.

## Release Signing Key

<!-- TODO: Add release signing key -->

We are currently working on providing signed releases. Stay tuned for updates.
