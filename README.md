# grafana

Async-first Rust SDK for the Grafana HTTP API.

This crate provides:
- A production-hardened `Client` / `BlockingClient` (base URL validation, auth injection, timeouts, retries, redaction).
- Typed wrappers for common endpoint groups (and `raw()` for full API coverage).

## Install

```toml
[dependencies]
grafana = { path = ".", default-features = true }
```

Default features: `async` + `rustls`.

### Feature flags

- `async` (default): Async client built on `tokio` + `reqwest`.
- `blocking`: Synchronous client built on `reqwest::blocking` (no Tokio required).
- `rustls` (default): `reqwest` TLS via `rustls` (ring provider installed automatically).
- `native-tls`: `reqwest` TLS via the platform/native TLS stack.
- `tracing`: Adds request spans via `tracing`.

Only enable one of `rustls` / `native-tls` in production.
If you are running inside Tokio, call blocking APIs from `spawn_blocking` or a dedicated thread pool.

## Quick start (async)

```rust
use grafana::{Auth, Client};

# async fn demo() -> Result<(), grafana::Error> {
let client = Client::builder("https://grafana.example.com")?
    .auth(Auth::bearer("TOKEN"))
    .build()?;

let health = client.health().get().await?;
println!("{health:?}");
# Ok(())
# }
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

## Base URL behavior

Pass the Grafana root URL (optionally with a subpath), for example:
- `https://grafana.example.com`
- `https://example.com/grafana`

The client automatically targets the HTTP API under `/api`.

## Raw API (escape hatch)

If an endpoint is not yet wrapped, use `client.raw()`:

```rust
use http::Method;
use grafana::Client;

# async fn demo(client: Client) -> Result<(), grafana::Error> {
let value: serde_json::Value = client
    .raw()
    .request_json(Method::GET, &["health"], Option::<&()>::None, Option::<&()>::None)
    .await?;
# Ok(())
# }
```

## Full OpenAPI coverage

Use `client.openapi()` for method-per-operation coverage based on Grafana `operationId`s:

```rust
use grafana::Client;

# async fn demo(client: Client) -> Result<(), grafana::Error> {
let value: serde_json::Value = client
    .openapi()
    .get_user_by_login_or_email("alice@example.com")
    .await?;
# Ok(())
# }
```

## Compatibility

Endpoint wrappers follow the latest Grafana OpenAPI spec (`grafana/grafana` `public/openapi3.json`).
For full coverage across Grafana versions, use `raw()` for unwrapped endpoints.

## Contract tests (optional)

Run the ignored contract tests against a real Grafana:

```bash
# Bearer token auth
GRAFANA_BASE_URL="https://grafana.example.com" GRAFANA_TOKEN="..." \
  cargo test --test contract -- --ignored

# Basic auth
GRAFANA_BASE_URL="http://localhost:3000" GRAFANA_USERNAME="admin" GRAFANA_PASSWORD="admin" \
  cargo test --test contract -- --ignored
```
