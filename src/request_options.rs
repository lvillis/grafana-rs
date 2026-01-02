use std::time::Duration;

use http::{HeaderMap, HeaderName, HeaderValue};

use crate::{Error, Result};

#[derive(Clone, Default)]
pub struct RequestOptions {
    headers: HeaderMap,
    timeout: Option<Duration>,
}

impl RequestOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn header(mut self, name: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(name, value);
        self
    }

    pub fn try_header(mut self, name: impl AsRef<str>, value: impl AsRef<str>) -> Result<Self> {
        let name = HeaderName::from_bytes(name.as_ref().as_bytes())
            .map_err(|e| Error::invalid_config(format!("invalid header name: {e}")))?;
        let value = HeaderValue::from_str(value.as_ref())
            .map_err(|e| Error::invalid_config(format!("invalid header value: {e}")))?;
        self.headers.insert(name, value);
        Ok(self)
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn timeout_override(&self) -> Option<Duration> {
        self.timeout
    }
}
