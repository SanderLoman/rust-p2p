# Binaries

## Available Binaries

We provide binaries for the following platforms:

-   `x86_64-unknown-linux-gnu`
-   `aarch64-unknown-linux-gnu`
-   `x86_64-apple-darwin`
-   `x86_64-windows`

You can find these binaries on our [GitHub Releases page](https://github.com/nodura/contower/releases). We recommend downloading the latest release for your platform.

## Download and Extract

To download and extract the binaries, follow the instructions for your platform:

### Linux (x86_64 and aarch64)

1. **Download the binary:**

    ```bash
    curl -LO https://github.com/nodura/contower/releases/download/<tag>/binary-x86_64-unknown-linux-gnu.tar.gz
    ```

    Replace `<tag>` with the appropriate release tag.

2. **Extract the binary:**
    ```bash
    tar -xvzf binary-x86_64-unknown-linux-gnu.tar.gz
    ```
3. **Add to PATH:**

    ```bash
    export PATH=$PATH:/path/to/extracted/binary
    ```

    Replace `/path/to/extracted/binary` with the path where you extracted the binary.

### macOS (x86_64)

1. **Download the binary:**
    ```bash
    curl -LO https://github.com/your-repo/releases/download/<tag>/binary-x86_64-apple-darwin.tar.gz
    ```
    Replace `<tag>` with the appropriate release tag.
2. **Extract the binary:**

    ```bash
    tar -xvzf binary-x86_64-apple-darwin.tar.gz
    ```

3. **Add to PATH:**

    ```bash
    export PATH=$PATH:/path/to/extracted/binary
    ```

    Replace `/path/to/extracted/binary` with the path where you extracted the binary.

### Windows (x86_64)

1. **Download the binary:**
   Go to [GitHub Releases](https://github.com/your-repo/releases) and download `binary-x86_64-windows.zip`.

2. **Extract the binary:**
   Use any archive tool (e.g., 7-Zip) to extract the `.zip` file.

3. **Add to PATH:**
    - Press `Win + R`, type `sysdm.cpl`, and press Enter.
    - Go to the `Advanced` tab and click `Environment Variables`.
    - In the `System variables` section, select `Path` and click `Edit`.
    - Click `New` and add the path to the extracted binary.
    - Click `OK` to save the changes.
