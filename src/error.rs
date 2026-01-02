use std::{fmt, time::Duration};

use http::{Method, StatusCode};

pub(crate) type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Clone, Debug)]
pub struct HttpError {
    method: Method,
    path: String,
    status: Option<StatusCode>,
    request_id: Option<String>,
    message: Option<String>,
    body_snippet: Option<String>,
    retry_after: Option<Duration>,
}

impl HttpError {
    pub(crate) fn new(method: Method, path: String, status: Option<StatusCode>) -> Self {
        Self {
            method,
            path,
            status,
            request_id: None,
            message: None,
            body_snippet: None,
            retry_after: None,
        }
    }

    pub(crate) fn with_request_id(mut self, request_id: Option<String>) -> Self {
        self.request_id = request_id;
        self
    }

    pub(crate) fn with_message(mut self, message: Option<String>) -> Self {
        self.message = message;
        self
    }

    pub(crate) fn with_body_snippet(mut self, body_snippet: Option<String>) -> Self {
        self.body_snippet = body_snippet;
        self
    }

    pub(crate) fn with_retry_after(mut self, retry_after: Option<Duration>) -> Self {
        self.retry_after = retry_after;
        self
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn status(&self) -> Option<StatusCode> {
        self.status
    }

    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }

    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    pub fn body_snippet(&self) -> Option<&str> {
        self.body_snippet.as_deref()
    }

    pub fn retry_after(&self) -> Option<Duration> {
        self.retry_after
    }
}

impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.method, self.path)?;
        if let Some(status) = self.status {
            write!(f, " returned {status}")?;
        }

        if let Some(message) = &self.message
            && !message.trim().is_empty()
        {
            write!(f, ": {}", message.trim())?;
        }

        let mut details = Vec::with_capacity(2);
        if let Some(request_id) = &self.request_id
            && !request_id.trim().is_empty()
        {
            details.push(format!("request_id={}", request_id.trim()));
        }
        if let Some(retry_after) = self.retry_after {
            if retry_after.is_zero() {
                details.push("retry_after=0ms".to_owned());
            } else {
                details.push(format!("retry_after={}ms", retry_after.as_millis()));
            }
        }

        if !details.is_empty() {
            write!(f, " ({})", details.join(", "))?;
        }

        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("invalid configuration: {message}")]
    InvalidConfig { message: String },

    #[error("transport error: {message}")]
    Transport {
        message: &'static str,
        #[source]
        source: BoxError,
    },

    #[error("failed to decode response: {http}")]
    Decode {
        http: Box<HttpError>,
        #[source]
        source: BoxError,
    },

    #[error("{0} (auth error)")]
    Auth(Box<HttpError>),

    #[error("{0} (not found)")]
    NotFound(Box<HttpError>),

    #[error("{0} (conflict)")]
    Conflict(Box<HttpError>),

    #[error("{0} (rate limited)")]
    RateLimited(Box<HttpError>),

    #[error("{0}")]
    Api(Box<HttpError>),
}

impl Error {
    pub(crate) fn invalid_config(message: impl Into<String>) -> Self {
        Self::InvalidConfig {
            message: message.into(),
        }
    }

    pub(crate) fn transport(
        message: &'static str,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::Transport {
            message,
            source: Box::new(source),
        }
    }

    pub(crate) fn decode(
        http: HttpError,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self::Decode {
            http: Box::new(http),
            source: Box::new(source),
        }
    }

    pub(crate) fn from_http(http: HttpError) -> Self {
        let status = http.status();
        match status {
            Some(StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN) => Self::Auth(Box::new(http)),
            Some(StatusCode::NOT_FOUND) => Self::NotFound(Box::new(http)),
            Some(StatusCode::CONFLICT | StatusCode::PRECONDITION_FAILED) => {
                Self::Conflict(Box::new(http))
            }
            Some(StatusCode::TOO_MANY_REQUESTS) => Self::RateLimited(Box::new(http)),
            _ => Self::Api(Box::new(http)),
        }
    }

    pub fn status(&self) -> Option<StatusCode> {
        match self {
            Self::InvalidConfig { .. } => None,
            Self::Transport { .. } => None,
            Self::Decode { http, .. } => http.status(),
            Self::Auth(http)
            | Self::NotFound(http)
            | Self::Conflict(http)
            | Self::RateLimited(http)
            | Self::Api(http) => http.status(),
        }
    }

    pub fn request_id(&self) -> Option<&str> {
        match self {
            Self::InvalidConfig { .. } => None,
            Self::Transport { .. } => None,
            Self::Decode { http, .. } => http.request_id(),
            Self::Auth(http)
            | Self::NotFound(http)
            | Self::Conflict(http)
            | Self::RateLimited(http)
            | Self::Api(http) => http.request_id(),
        }
    }

    pub fn message(&self) -> Option<&str> {
        match self {
            Self::InvalidConfig { message } => Some(message.as_str()),
            Self::Transport { .. } => None,
            Self::Decode { http, .. } => http.message(),
            Self::Auth(http)
            | Self::NotFound(http)
            | Self::Conflict(http)
            | Self::RateLimited(http)
            | Self::Api(http) => http.message(),
        }
    }

    pub fn body_snippet(&self) -> Option<&str> {
        match self {
            Self::InvalidConfig { .. } => None,
            Self::Transport { .. } => None,
            Self::Decode { http, .. } => http.body_snippet(),
            Self::Auth(http)
            | Self::NotFound(http)
            | Self::Conflict(http)
            | Self::RateLimited(http)
            | Self::Api(http) => http.body_snippet(),
        }
    }

    pub fn is_auth_error(&self) -> bool {
        matches!(self, Self::Auth(_))
    }

    pub fn is_retryable(&self) -> bool {
        match self {
            Self::RateLimited(_) => true,
            Self::Api(http) => matches!(
                http.status(),
                Some(
                    StatusCode::BAD_GATEWAY
                        | StatusCode::SERVICE_UNAVAILABLE
                        | StatusCode::GATEWAY_TIMEOUT
                )
            ),
            _ => false,
        }
    }
}
