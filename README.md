<h1 align="center"><code>grafana-rs</code></h1>

<p align="center">
Ergonomic Rust SDK for Grafana's HTTP API, with async and blocking clients.
</p>

<div align="center">

[![Crates.io](https://img.shields.io/crates/v/grafana.svg)](https://crates.io/crates/grafana)&nbsp;
[![Downloads](https://img.shields.io/crates/d/grafana.svg)](https://crates.io/crates/grafana)&nbsp;
[![Docs.rs](https://img.shields.io/docsrs/grafana)](https://docs.rs/grafana)&nbsp;
[![MSRV](https://img.shields.io/badge/MSRV-1.92-blue)](Cargo.toml)&nbsp;
[![CI](https://github.com/lvillis/grafana-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/lvillis/grafana-rs/actions/workflows/ci.yml)&nbsp;
[![Repo Size](https://img.shields.io/github/repo-size/lvillis/grafana-rs?color=328657)](https://github.com/lvillis/grafana-rs)&nbsp;
[![License](https://img.shields.io/crates/l/grafana.svg)](LICENSE)

</div>

---


## Install

```toml
[dependencies]
grafana = "0.1" # async + rustls (default)
```

Blocking-only (no Tokio):

```toml
[dependencies]
grafana = { version = "0.1", default-features = false, features = ["blocking", "rustls"] }
```

Native TLS: replace `rustls` with `native-tls` (and set `default-features = false`).

### Features

- `async` (default): `tokio` + `reqwest`.
- `blocking`: `reqwest::blocking`.
- `rustls` (default) / `native-tls`: pick one TLS backend.
- `tracing`: request spans.

If you run inside Tokio, call blocking APIs from `spawn_blocking` or a dedicated thread pool.

## Quick start (async)

```rust
use grafana::{Auth, Client};

async fn demo() -> Result<(), grafana::Error> {
    let client = Client::builder("https://grafana.example.com")?
        .auth(Auth::bearer("TOKEN"))
        .build()?;

    let health = client.health().get().await?;
    println!("{health:?}");
    Ok(())
}
```

## Quick start (blocking)

```rust
use grafana::{Auth, BlockingClient};

fn demo() -> Result<(), grafana::Error> {
    let client = BlockingClient::builder("https://grafana.example.com")?
        .auth(Auth::bearer("TOKEN"))
        .build()?;

    let health = client.health().get()?;
    println!("{health:?}");
    Ok(())
}
```

## Base URL

Pass the Grafana root URL (optionally with a subpath). The client automatically targets `/api`.

Examples:
- `https://grafana.example.com`
- `https://example.com/grafana`

## API coverage

- Hand-written wrappers: `client.dashboards()`, `client.folders()`, `client.user()`, ...
- Generated wrappers: `client.openapi()` (method-per-operation, keyed by Grafana `operationId`).
- Escape hatch: `client.raw()` for custom requests.

## Compatibility

Endpoint wrappers follow the latest Grafana OpenAPI spec (`grafana/grafana` `public/openapi3.json`).
For full coverage across Grafana versions, use `raw()` for unwrapped endpoints.

## Contract tests (repo)

Run the ignored contract tests against a real Grafana:

```bash
# Bearer token auth
GRAFANA_BASE_URL="https://grafana.example.com" GRAFANA_TOKEN="..." \
  cargo test --test contract -- --ignored

# Basic auth
GRAFANA_BASE_URL="http://localhost:3000" GRAFANA_USERNAME="admin" GRAFANA_PASSWORD="admin" \
  cargo test --test contract -- --ignored
```
