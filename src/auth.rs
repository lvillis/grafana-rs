use std::fmt;

use base64::Engine;
use http::{HeaderMap, HeaderValue};

use crate::{Error, Result};

#[derive(Clone)]
pub enum Auth {
    None,
    Bearer(String),
    Basic { username: String, password: String },
}

impl Auth {
    pub fn none() -> Self {
        Self::None
    }

    pub fn bearer(token: impl Into<String>) -> Self {
        Self::Bearer(token.into())
    }

    pub fn basic(username: impl Into<String>, password: impl Into<String>) -> Self {
        Self::Basic {
            username: username.into(),
            password: password.into(),
        }
    }
}

impl fmt::Debug for Auth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => f.debug_tuple("None").finish(),
            Self::Bearer(_) => f.debug_tuple("Bearer").field(&"<redacted>").finish(),
            Self::Basic { username, .. } => f
                .debug_struct("Basic")
                .field("username", username)
                .field("password", &"<redacted>")
                .finish(),
        }
    }
}

impl Auth {
    pub(crate) fn apply(&self, headers: &mut HeaderMap) -> Result<()> {
        match self {
            Self::None => Ok(()),
            Self::Bearer(token) => {
                let value = HeaderValue::from_str(&format!("Bearer {token}"))
                    .map_err(|e| Error::invalid_config(format!("invalid bearer token: {e}")))?;
                headers.insert(http::header::AUTHORIZATION, value);
                Ok(())
            }
            Self::Basic { username, password } => {
                let credentials = format!("{username}:{password}");
                let encoded = base64::engine::general_purpose::STANDARD.encode(credentials);
                let value = HeaderValue::from_str(&format!("Basic {encoded}"))
                    .map_err(|e| Error::invalid_config(format!("invalid basic auth: {e}")))?;
                headers.insert(http::header::AUTHORIZATION, value);
                Ok(())
            }
        }
    }
}
