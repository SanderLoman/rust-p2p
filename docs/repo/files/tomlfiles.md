# TOML files Documentation

This folder contains various TOML files used in the project for configuration and metadata purposes. Below is a list of TOML files currently available and their purposes.

## book.toml

This configuration file is used to set up the documentation for the Contower project.

### [book]

Defines general book metadata and settings.

```toml
[book]
authors = ["Contower Core Contributors"]
language = "en"
multilingual = false
src = "book"
title = "Contower Book"
description = "A book on all things Contower"
```

### [output.html]

Configures the HTML output settings.

```toml
[output.html]
theme = "book/theme"
git-repository-url = "https://github.com/nodura/contower"
default-theme = "Coal"
no-section-label = true
```

### [output.html.fold]

Enables folding for HTML output.

```toml
[output.html.fold]
enable = true
level = 1
```

### [build]

Specifies the build directory for the book.

```toml
[build]
build-dir = "target/book"
```

## Cross.toml

This configuration file is used to set up cross-compilation for the Contower project.

### [target.x86_64-unknown-linux-gnu]

Specifies pre-build commands for the x86_64-unknown-linux-gnu target.

```toml
[target.x86_64-unknown-linux-gnu]
pre-build = ["apt-get install -y cmake clang-5.0"]
```

### [target.aarch64-unknown-linux-gnu]

Specifies pre-build commands for the aarch64-unknown-linux-gnu target.

```toml
[target.aarch64-unknown-linux-gnu]
pre-build = ["apt-get install -y cmake clang-5.0"]
```

## .config/nextest.toml

This is the default configuration used by nextest for running tests.

### [store]

Defines the directory where nextest-related files are written.

```toml
[store]
dir = "target/nextest"
```

### [profile.default]

Defines the default profile settings for nextest.

```toml
[profile.default]
retries = 0
test-threads = "num-cpus"
threads-required = 1
status-level = "pass"
final-status-level = "flaky"
failure-output = "immediate"
success-output = "never"
fail-fast = true
slow-timeout = { period = "60s" }
leak-timeout = "100ms"
archive.include = []
```

### [profile.default.junit]

Configures JUnit report output for the default profile.

```toml
[profile.default.junit]
report-name = "contower-run"
store-success-output = false
store-failure-output = true
```

### [profile.default-miri]

Defines settings for the default-miri profile, activated if MIRI_SYSROOT is set.

```toml
[profile.default-miri]
test-threads = 4
```

## .cargo/config.toml

This configuration file is used to set environment variables and target-specific settings for Cargo.

### [env]

Sets the number of arenas to 16 when using jemalloc.

```toml
[env]
JEMALLOC_SYS_WITH_MALLOC_CONF = "abort_conf:true,narenas:16"
```

### [target.x86_64-pc-windows-msvc]

Specifies Rust flags for the x86_64-pc-windows-msvc target.

```toml
[target.x86_64-pc-windows-msvc]
rustflags = ["-Clink-arg=/STACK:10000000"]
```

### [target.i686-pc-windows-msvc]

Specifies Rust flags for the i686-pc-windows-msvc target.

```toml
[target.i686-pc-windows-msvc]
rustflags = ["-Clink-arg=/STACK:10000000"]
```

## Other general Cargo.toml files

These files configure various aspects of the Cargo build system for the Contower project, such as dependencies, features, and profiles. Each file contains specific settings tailored to different parts of the project.
