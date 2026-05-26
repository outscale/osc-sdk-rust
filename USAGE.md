# Outscale SDK for Rust — Usage Guide

## Table of Contents

- [Installation](#installation)
- [Feature Flags](#feature-flags)
- [Offline Mode](#offline-mode)
  - [How the Generator Pipeline Works](#how-the-generator-pipeline-works)
  - [Pre-building the Generator](#pre-building-the-generator)
  - [Offline Build with GENERATOR_BIN](#offline-build-with-generator_bin)
  - [Offline Build with Nix](#offline-build-with-nix)

---

## Installation

Add the SDK to your `Cargo.toml`:

```toml
[dependencies]
osc-sdk-rust = { git = "https://github.com/outscale/osc-sdk-rust.git", branch = "v2" }
```

**Requirements:**

- Rust stable toolchain (2024 edition)
- Go toolchain (for building the code generator; see [Offline Mode](#offline-mode) for pre-built alternatives)
- Valid OUTSCALE API credentials

---

## Feature Flags

| Feature | Default | Description                                  |
| ------- | ------- | -------------------------------------------- |
| `osc`   | Yes     | Enables the OSC (compute) API client         |
| `oks`   | Yes     | Enables the OKS (Kubernetes) API client      |
| `default-tls` | Yes | Uses reqwest's default Rustls TLS backend |
| `native-tls`  | No  | Uses OpenSSL via reqwest's native-tls feature |

To enable only the OSC API:

```toml
[dependencies]
osc-sdk-rust = { git = "...", branch = "v2", default-features = false, features = ["osc"] }
```

---

## Offline Mode

By default, building `osc-sdk-rust` requires network access for two reasons:

1. **Rust crate dependencies** — fetched from crates.io (solved by `cargo vendor` or a pre-populated `Cargo.lock`)
2. **The code generator binary** — the `osc-sdk-rust-build` crate acquires the Go generator at build time

This section covers how to eliminate requirement #2. Once the generator is locally available, you can build the SDK without network access to GitHub or any Go module proxy.

### How the Generator Pipeline Works

The generator is a Go program that reads the OpenAPI specification (`config/{osc,oks}/api.yaml`) and its overlay patches (`config/{osc,oks}/patch.yaml`), then emits Rust code (structs, enums, the `Api` trait, and the `Client` implementation).

### Pre-building the Generator

The simplest offline strategy: build the generator once, then point `GENERATOR_BIN` at it.

```bash
# Build the generator from source
cd hacks/generator
CGO_ENABLED=0 go build -trimpath -ldflags="-s -w" -o ~/bin/osc-sdk-generator .

# Verify
~/bin/osc-sdk-generator --help
```

Now any subsequent `cargo build` of the SDK can use this binary:

```bash
GENERATOR_BIN=$HOME/bin/osc-sdk-generator cargo build
```

The `sdk-build` crate checks `GENERATOR_BIN` first — if the file exists, it uses it and skips both the GitHub download and the local `go build` attempts. No network access needed for code generation.

### Offline Build with GENERATOR_BIN

For a fully offline build, combine a pre-built generator with vendored Rust dependencies:

```bash
# Step 1: Pre-build the generator (do once, with network)
cd hacks/generator
CGO_ENABLED=0 go build -trimpath -ldflags="-s -w" -o /tmp/osc-sdk-generator .

# Step 2: Vendor Rust dependencies (do once, with network)
cargo vendor
mkdir -p .cargo
cat > .cargo/config.toml <<EOF
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor"
EOF

# Step 3: Build offline (no network required)
GENERATOR_BIN=/tmp/osc-sdk-generator cargo build --offline
```

The `--offline` flag tells cargo to use only the vendored dependencies. `GENERATOR_BIN` tells `sdk-build` to use the pre-built generator without downloading or compiling anything.

### Offline Build with Nix

The repository includes a `flake.nix` that builds the Go generator as a Nix derivation and wires it into the Rust build, producing fully reproducible example binaries without network access at build time.

```bash
# Build the examples
nix build .#examples

# Run
./result/bin/vm
./result/bin/project
```

See `flake.nix` for the derivation details. The flake also provides a development shell:

```bash
nix develop
# Inside the shell, GENERATOR_BIN is already set:
echo $GENERATOR_BIN
cargo build --examples
```

---

## Examples

```bash
# OSC: list VMs by ID
export OSC_ACCESS_KEY=... OSC_SECRET_KEY=...
cargo run --example vm

# OKS: full project lifecycle (list, create, poll, delete)
export OSC_ACCESS_KEY=... OSC_SECRET_KEY=...
cargo run --example project
```
