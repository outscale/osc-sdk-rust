## **Outscale SDK for Rust** 

[![Project Incubating](https://docs.outscale.com/fr/userguide/_images/Project-Incubating-blue.svg)](https://docs.outscale.com/en/userguide/Open-Source-Projects.html) [![](https://dcbadge.limes.pink/api/server/HUVtY5gT6s?style=flat\&theme=default-inverted)](https://discord.gg/HUVtY5gT6s)

<p align="center">
  <img alt="Outscale Python SDK" src="https://img.icons8.com/?size=100&id=2866&format=png&color=000000" width="100px">
</p>

---

## 🌐 Links

* Documentation: [https://docs.outscale.com/en/](https://docs.outscale.com/en/)
* Project website: [https://github.com/outscale/osc-sdk-rust](https://github.com/outscale/osc-sdk-rust)
* Crate on crates.io: [https://crates.io/crates/outscale_api](https://crates.io/crates/outscale_api)
* Join our community on [Discord](https://discord.gg/HUVtY5gT6s)

---

## 📄 Table of Contents

* [Overview](#-overview)
* [Requirements](#-requirements)
* [Installation](#-installation)
* [Configuration](#-configuration)
* [Usage](#-usage)
* [Examples](#-examples)
* [License](#-license)
* [Contributing](#-contributing)

---

## 🧭 Overview

**Outscale SDK for Rust** is the official Rust SDK for the OUTSCALE API, based on the [Rust 2021 edition](https://doc.rust-lang.org/edition-guide/rust-2021/index.html) (stable).

Key features:

* Rust-first API client generated from OUTSCALE’s OpenAPI definition
* Strongly typed models for OUTSCALE resources
* HTTP client based on `reqwest` with configurable TLS backends (`default-tls` and `rustls-tls`)

---

## ✅ Requirements

* A working [Rust toolchain](https://www.rust-lang.org/) (Rust 2021 edition, stable)
* [Cargo](https://doc.rust-lang.org/cargo/) package manager
* Access to the OUTSCALE API (valid access key / secret key)
* Network access to the OUTSCALE API endpoints

---

## ⚙ Installation

### Option 1: Install from crates.io (recommended)

Add the `outscale_api` crate to your project using `cargo`:

```bash
cargo add outscale_api
```

Or manually add it to your `Cargo.toml`:

```toml
[dependencies]
outscale_api = "1"
```

See the crate page on [crates.io](https://crates.io/crates/outscale_api) for the latest version.

### Option 2: Install from source

```bash
git clone https://github.com/outscale/osc-sdk-rust.git
cd osc-sdk-rust
cargo build --release
```

---

## 🛠 Configuration

The SDK itself is a Rust library: you configure it directly from your code (for example through a configuration struct or builder).

Refer to the `examples/` directory in this repository for concrete examples of how to build and pass configuration to the client.

---

## 🚀 Usage

Add the crate to your `Cargo.toml` (see [Installation](#-installation)), then use it in your code.

> For real-world examples (including how to authenticate and call specific APIs), check the `examples/` directory.

### Working with Async runtime

Calls will block the current thread from executing, instead of returning futures that must be run on a runtime.
Conversely, it must not be executed within an async runtime, or it will panic when it tries to block.
Consider changing the caller to wrap those calls in `tokio::task::spawn_blocking`.

```rust
use outscale_api::apis::profile::Profile;
use outscale_api::apis::vm_api::read_vms;
use outscale_api::models::ReadVmsRequest;

let config = Profile::default().and_then(|p| p.try_into()).unwrap();

let res = task::spawn_blocking(move || {
    read_vms(&config, Some(ReadVmsRequest::new()))
}).await.unwrap();
```

---

## 💡 Examples

### Enable TLS features

The crate exposes features to select the TLS backend used by `reqwest`:

* `default`: enables the `default-tls` feature in `reqwest` (Rustls-based).
* `native-tls`: uses `OpenSSL` instead of the default Rustls backend.
  When using `native-tls`, you typically also want to disable default features to avoid pulling in `default-tls`:

```toml
[dependencies]
outscale_api = { version = "1", default-features = false, features = ["native-tls"] }
```

### Explore the examples

Clone the repository and run the examples:

```bash
git clone https://github.com/outscale/osc-sdk-rust.git
cd osc-sdk-rust
cargo run --example <example-name>
```

Examples are available in the [`examples/`](examples/) directory and are a good starting point to:

* Set up authentication
* Call common OUTSCALE API endpoints
* Inspect responses and work with the generated models

---

## 📜 License

**Outscale SDK for Rust** is released under the **BSD-3-Clause** license.

© 2026 Outscale SAS

See [LICENSE](./LICENSE) for full details.

This project is compliant with [REUSE](https://reuse.software/).

---

## 🤝 Contributing

We welcome contributions!

Please read our [Contributing Guidelines](CONTRIBUTING.md) and [Code of Conduct](CODE_OF_CONDUCT.md) before submitting a pull request.
