# bifrost

> Cross the rainbow bridge between Kubernetes contexts.

`bifrost` is a small, fast, interactive Kubernetes context switcher written in
Rust. It reads your kubeconfig, presents an ergonomic picker with fuzzy filter,
and writes the selection back to `current-context`.

Think `kubectx`, but with a nicer prompt and a single static binary.

## Features

- Interactive context picker powered by [`cliclack`](https://crates.io/crates/cliclack)
- Type-to-filter fuzzy search across context names
- Shows cluster and namespace next to each context
- Marks the current context with `*`
- Respects `$KUBECONFIG` (uses first entry if colon-separated)
- Preserves unknown kubeconfig fields on write

## Install

### Homebrew (macOS and Linux)

```sh
brew install Delian-Alliance-Industries/tap/bifrost
```

### From source

```sh
cargo install --git https://github.com/Delian-Alliance-Industries/bifrost.git
```

Or clone and build locally:

```sh
git clone https://github.com/Delian-Alliance-Industries/bifrost.git
cd bifrost
cargo install --path .
```

### Prebuilt binaries

See the [Releases](https://github.com/Delian-Alliance-Industries/bifrost/releases)
page for Linux, macOS, and Windows binaries.

## Usage

```sh
bifrost
```

Use arrow keys to navigate, start typing to filter, `Enter` to select, `Esc` to
cancel. The chosen context is written to the kubeconfig as `current-context`.

### Kubeconfig location

`bifrost` uses `$KUBECONFIG` when set. Otherwise it falls back to
`~/.kube/config`. If `$KUBECONFIG` contains multiple paths separated by `:`,
only the first is used.

## Comparison to `kubectx`

| Feature             | `bifrost` | `kubectx` |
|---------------------|-----------|-----------|
| Static binary       | yes       | no (bash) |
| Fuzzy filter        | built-in  | via `fzf` |
| Cluster + ns hints  | yes       | no        |
| Rename / delete ctx | no        | yes       |

`bifrost` is intentionally small: it only switches contexts. For everything
else, keep using `kubectl`.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). By participating you agree to the
[Code of Conduct](CODE_OF_CONDUCT.md).

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
