[![Project Incubating](https://docs.outscale.com/fr/userguide/_images/Project-Incubating-blue.svg)](https://docs.outscale.com/en/userguide/Open-Source-Projects.html)

# Outscale SDK for Rust

Welcome to Outscale SDK for [Rust](https://www.rust-lang.org/).

It is based on rust [2021 edition](https://doc.rust-lang.org/edition-guide/rust-2021/index.html) (stable).

## How to use the SDK ?

Add `outscale_api` Package to you Cargo.toml using `cargo add outscale_api` or manually add it (visit [crates.io](https://crates.io/crates/outscale_api) for more details).

See [examples](examples/) folder to jump straight into the code!

## Features

The following features can be enabled through `Cargo.toml`:
- `default`: enable `default-tls` feature in reqwest library.
- `rustls-tls`: will use rustls instead of default openssl in reqwest library. You will also need to also set [`default-features = false`] to avoid using `default-tls` feature.

# Contributing

Check [contributing documentation](CONTRIBUTING.md).

# License

> Copyright Outscale SAS
>
> BSD-3-Clause

This project is compliant with [REUSE](https://reuse.software/).
