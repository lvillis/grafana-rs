//! Async-first Rust SDK for the Grafana HTTP API.
//!
//! ## Quick start
//!
//! Async (default):
//! ```no_run
//! # #[cfg(feature = "async")]
//! # async fn demo() -> Result<(), grafana::Error> {
//! use grafana::{Auth, Client};
//!
//! let client = Client::builder("https://grafana.example.com")?
//!     .auth(Auth::bearer("TOKEN"))
//!     .build()?;
//!
//! let health = client.health().get().await?;
//! println!("{health:?}");
//! # Ok(())
//! # }
//! ```
//!
//! Blocking (feature = `blocking`):
//! ```no_run
//! # #[cfg(feature = "blocking")]
//! # fn demo() -> Result<(), grafana::Error> {
//! use grafana::{Auth, BlockingClient};
//!
//! let client = BlockingClient::builder("https://grafana.example.com")?
//!     .auth(Auth::bearer("TOKEN"))
//!     .build()?;
//!
//! let health = client.health().get()?;
//! println!("{health:?}");
//! # Ok(())
//! # }
//! ```

#[cfg(all(feature = "rustls", feature = "native-tls", not(feature = "multi-tls")))]
compile_error!("Enable only one of: rustls, native-tls");

#[cfg(not(any(feature = "async", feature = "blocking")))]
compile_error!("Enable at least one of: async, blocking");

#[cfg(any(feature = "async", feature = "blocking"))]
pub mod api;
pub mod auth;
#[cfg(any(feature = "async", feature = "blocking"))]
pub mod client;
pub mod error;
pub mod request_options;
pub mod response;
pub mod types;

#[cfg(any(feature = "async", feature = "blocking"))]
mod transport;
mod util;

pub use auth::Auth;
#[cfg(feature = "blocking")]
pub use client::BlockingClient;
#[cfg(feature = "async")]
pub use client::Client;
pub use error::Error;
pub use request_options::RequestOptions;
pub use response::ResponseBytes;

pub type Result<T> = std::result::Result<T, Error>;
