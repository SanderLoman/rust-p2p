# Contributing to Contower

[![Chat Badge]][Chat Link]
[![Issues Badge](https://img.shields.io/github/issues/nodura/contower.svg)](https://github.com/nodura/contower/issues)

[Chat Badge]: https://img.shields.io/discord/1174374333062316032?logo=discord
[Chat Link]: https://discord.gg/vHWpWsjCqx

Contower is an evolving project in the Ethereum ecosystem, focused on bringing innovation and efficiency to blockchain operations. We welcome contributions from developers, writers, and enthusiasts who are interested in enhancing and expanding Contower's capabilities.

## Getting Started

1. Familiarize yourself with the [how to contribute](https://github.com/nodura/contower/blob/stable/CONTRIBUTING.md) guide.
2. Set up your development environment as described in [setup instructions](./setup.md).
3. Check out the [open issues](https://github.com/nodura/contower/issues) for areas where you can contribute.
    - Start with issues labeled [good first issue](https://github.com/nodura/contower/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22) if you're new.
4. Before starting work on an issue, please comment on it to let others know you're working on it.
5. Submit your contributions as a pull request for review.

Questions or discussions? Join us on [Discord](https://discord.gg/vHWpWsjCqx).

## Contribution Guidelines

### Branches

Contower will permenantly have two branches:

-   `stable`: Our primary development branch. This is where we merge new features and changes after they have been tested and are ready for release.
-   `unstable`: Our secondary development branch. This is where we merge new features and changes before they are moved to `stable`.

### Rust Development

Contower is developed in Rust, and we follow standard Rust conventions. Use [clippy](https://github.com/rust-lang/rust-clippy) and [rustfmt](https://github.com/rust-lang/rustfmt) for linting and formatting:

```bash
$ cargo clippy --all
$ cargo fmt --all --check
```

### Panics

It's crucial to avoid panics in a high-risk environment like the Internet. Panics in Contower represent a significant security vulnerability, especially if external users can trigger them.

Opt for `Result` or `Option` instead of panicking. For example, use `array.get(1)?` instead of `array[1]`.

In situations where a panic is unlikely but still needs to be communicated to the compiler, prefer `.expect("Descriptive message")` over `.unwrap()`. It's important to include a comment explaining why a panic is not expected in such cases.

### TODOs

Every `TODO` comment must be linked to a corresponding GitHub issue.

```rust
use std::error::Error;

struct MyExampleStruct;

impl MyExampleStruct {
    pub fn my_function(&mut self, _something: &[u8]) -> Result<String, Box<dyn Error>> {
        // TODO: Implement feature
        // Issue link: https://github.com/nodura/contower/issues/XX
        Ok("Example response".to_string())
    }
}
```

### Comments

**General Comments**

-   Use line comments (`//`) rather than block comments (`/* ... */`).
-   Comments can be placed either before the item they refer to or after a space on the same line.

```rust
// Description of the struct
struct Contower {}
fn make_blockchain() {} // Inline comment after a space
```

**Documentation Comments**

-   Use `///` for generating documentation comments.
-   Place these comments before attributes.

```rust
use std::path::PathBuf;

/// Configuration for the Contower instance, covering the core settings.
/// This general configuration can be extended by other components. #[derive(Clone)]
#[derive(Clone)]
pub struct ContowerConfig {
    pub data_dir: PathBuf,
    pub p2p_listen_port: u16,
}
```

### Rust Learning Resources

Rust is a powerful, low-level language offering great control and performance. The [Rust Book](https://doc.rust-lang.org/stable/book/) is an excellent guide to understanding Rust, including its style and usage.

Learning Rust can be challenging, but there are numerous resources available:

-   [Rust Book](https://doc.rust-lang.org/stable/book/) for a comprehensive introduction.
-   [Rust by Example](https://doc.rust-lang.org/stable/rust-by-example/) for practical learning.
-   [Learning Rust With Entirely Too Many Linked Lists](http://cglab.ca/~abeinges/blah/too-many-lists/book/) for a unique approach.
-   [Rustlings](https://github.com/rustlings/rustlings) for interactive exercises.
-   [Rust Exercism](https://exercism.io/tracks/rust) for coding challenges.
-   [Learn X in Y Minutes - Rust](https://learnxinyminutes.com/docs/rust/) for a quick overview.
