use std::fmt;

use http::{HeaderMap, StatusCode};

#[derive(Clone)]
pub struct ResponseBytes {
    status: StatusCode,
    headers: HeaderMap,
    body: Vec<u8>,
}

impl ResponseBytes {
    pub(crate) fn new(status: StatusCode, headers: HeaderMap, body: Vec<u8>) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }

    pub fn status(&self) -> StatusCode {
        self.status
    }

    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn body(&self) -> &[u8] {
        &self.body
    }

    pub fn into_body(self) -> Vec<u8> {
        self.body
    }
}

impl fmt::Debug for ResponseBytes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut redacted = HeaderMap::new();
        for (name, value) in self.headers.iter() {
            if name == http::header::SET_COOKIE {
                redacted.insert(name.clone(), http::HeaderValue::from_static("<redacted>"));
                continue;
            }
            redacted.insert(name.clone(), value.clone());
        }

        f.debug_struct("ResponseBytes")
            .field("status", &self.status)
            .field("headers", &redacted)
            .field("body_len", &self.body.len())
            .finish()
    }
}
