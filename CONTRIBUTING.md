# Contributing to bifrost

Thanks for your interest in contributing. This document covers the basics.

## Dev setup

Requires a recent stable Rust toolchain (install via [rustup](https://rustup.rs)).

```sh
git clone https://github.com/Delian-Alliance-Industries/bifrost.git
cd bifrost
cargo build
cargo run
```

## Before submitting a PR

Run the same checks CI runs:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Commit style

- Write imperative, present-tense subjects: "add fuzzy filter", not "added".
- Keep the subject under ~72 characters.
- Explain the *why* in the body when it isn't obvious from the diff.

## Pull requests

- One logical change per PR.
- Link related issues.
- Keep the diff focused — unrelated reformatting belongs in its own PR.
- Expect review feedback; iterate until green.

## Reporting bugs / requesting features

Open an issue using the provided templates. Include reproduction steps, your
platform, and the `bifrost --version` (once `--version` exists) or the commit
SHA you built from.

## License of contributions

Unless you state otherwise, contributions are dual-licensed under
Apache-2.0 and MIT, matching the rest of the project.
