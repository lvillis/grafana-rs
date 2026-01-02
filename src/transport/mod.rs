use std::time::Duration;

use http::{HeaderMap, Method, StatusCode};

use crate::{Auth, util::redact};

#[cfg(feature = "async")]
mod async_transport;
#[cfg(feature = "blocking")]
mod blocking_transport;

#[cfg(feature = "async")]
pub(crate) use async_transport::AsyncTransport;
#[cfg(feature = "blocking")]
pub(crate) use blocking_transport::BlockingTransport;

#[derive(Clone)]
pub(crate) struct TransportConfig {
    pub timeout: Duration,
    pub connect_timeout: Duration,
}

#[derive(Clone)]
pub(crate) struct RetryConfig {
    pub max_retries: usize,
    pub base_delay: Duration,
    pub max_delay: Duration,
}

#[derive(Clone)]
pub(crate) struct BodySnippetConfig {
    pub enabled: bool,
    pub byte_limit: usize,
}

#[derive(Clone, Copy)]
pub(crate) struct RequestContext<'a> {
    pub base_url: &'a url::Url,
    pub auth: &'a Auth,
    pub default_headers: &'a HeaderMap,
    pub retry: &'a RetryConfig,
    pub body_snippet: &'a BodySnippetConfig,
}

fn is_retryable_status(method: &Method, status: StatusCode) -> bool {
    if !is_idempotent_method(method) {
        return false;
    }

    matches!(
        status,
        StatusCode::TOO_MANY_REQUESTS
            | StatusCode::BAD_GATEWAY
            | StatusCode::SERVICE_UNAVAILABLE
            | StatusCode::GATEWAY_TIMEOUT
    )
}

fn is_retryable_transport_error(method: &Method, err: &reqwest::Error) -> bool {
    if !is_idempotent_method(method) {
        return false;
    }

    err.is_timeout() || err.is_connect()
}

fn is_idempotent_method(method: &Method) -> bool {
    matches!(
        *method,
        Method::GET | Method::HEAD | Method::PUT | Method::DELETE | Method::OPTIONS
    )
}

fn extract_request_id(headers: &HeaderMap) -> Option<String> {
    const CANDIDATES: [&str; 3] = ["x-request-id", "x-grafana-request-id", "x-amzn-trace-id"];

    for name in CANDIDATES {
        if let Some(value) = headers.get(name)
            && let Ok(value) = value.to_str()
        {
            let value = value.trim();
            if !value.is_empty() {
                return Some(value.to_owned());
            }
        }
    }
    None
}

fn extract_message(bytes: &[u8]) -> Option<String> {
    #[derive(serde::Deserialize)]
    struct ApiErrorBody {
        message: Option<String>,
        error: Option<String>,
    }

    let body = serde_json::from_slice::<ApiErrorBody>(bytes).ok()?;
    body.message.or(body.error)
}

fn capture_snippet(bytes: &[u8], config: &BodySnippetConfig) -> Option<String> {
    if !config.enabled {
        return None;
    }
    redact::redact_body_snippet(bytes, config.byte_limit)
}

fn extract_retry_after(headers: &HeaderMap) -> Option<Duration> {
    let value = headers.get(http::header::RETRY_AFTER)?;
    let value = value.to_str().ok()?.trim();
    if value.is_empty() {
        return None;
    }

    if let Ok(seconds) = value.parse::<u64>() {
        return Some(Duration::from_secs(seconds));
    }

    let when = httpdate::parse_http_date(value).ok()?;
    let now = std::time::SystemTime::now();
    let diff = when.duration_since(now).unwrap_or(Duration::ZERO);
    Some(diff)
}

fn compute_retry_delay(
    retry_after: Option<Duration>,
    retry: &RetryConfig,
    attempt: usize,
) -> Option<Duration> {
    if let Some(delay) = retry_after {
        return Some(delay);
    }

    if retry.base_delay.is_zero() {
        return Some(Duration::ZERO);
    }

    let base_ms = retry.base_delay.as_millis().min(u128::from(u64::MAX)) as u64;
    let max_ms = retry.max_delay.as_millis().min(u128::from(u64::MAX)) as u64;

    let shift = attempt.min(32) as u32;
    let exp_ms = base_ms.saturating_mul(1u64 << shift);
    let cap_ms = exp_ms.min(max_ms);

    if cap_ms == 0 {
        return Some(Duration::ZERO);
    }

    let jitter_ms = fastrand::u64(0..=cap_ms);
    Some(Duration::from_millis(jitter_ms))
}
