use http::{HeaderValue, Method};
use serde::{Serialize, de::DeserializeOwned};

use crate::{
    Error, RequestOptions, ResponseBytes, Result,
    error::HttpError,
    transport::{RequestContext, TransportConfig},
    util::url as url_util,
};

pub(crate) struct AsyncTransport {
    client: reqwest::Client,
}

impl AsyncTransport {
    pub(crate) fn new(config: &TransportConfig) -> Result<Self> {
        #[cfg(feature = "rustls")]
        if rustls::crypto::CryptoProvider::get_default().is_none() {
            let _ = rustls::crypto::ring::default_provider().install_default();
        }

        let client = reqwest::Client::builder()
            .timeout(config.timeout)
            .connect_timeout(config.connect_timeout)
            .build()
            .map_err(|e| Error::transport("failed to build HTTP client", e))?;

        Ok(Self { client })
    }

    pub(crate) async fn send_json<Response, Query, Body>(
        &self,
        ctx: RequestContext<'_>,
        method: Method,
        segments: &[&str],
        query: Option<&Query>,
        body: Option<&Body>,
        options: Option<&RequestOptions>,
    ) -> Result<Response>
    where
        Response: DeserializeOwned,
        Query: Serialize + ?Sized,
        Body: Serialize + ?Sized,
    {
        let response = self
            .send_bytes(ctx, method.clone(), segments, query, body, options)
            .await?;

        let bytes = response.body();
        let decoded: Response = serde_json::from_slice(bytes).map_err(|e| {
            let url =
                url_util::endpoint(ctx.base_url, segments).unwrap_or_else(|_| ctx.base_url.clone());
            let http = HttpError::new(method, url.path().to_owned(), Some(response.status()))
                .with_request_id(super::extract_request_id(response.headers()))
                .with_body_snippet(super::capture_snippet(bytes, ctx.body_snippet));
            Error::decode(http, e)
        })?;

        Ok(decoded)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn send_json_text<Response, Query>(
        &self,
        ctx: RequestContext<'_>,
        method: Method,
        segments: &[&str],
        query: Option<&Query>,
        body: &str,
        content_type: &'static str,
        options: Option<&RequestOptions>,
    ) -> Result<Response>
    where
        Response: DeserializeOwned,
        Query: Serialize + ?Sized,
    {
        let response = self
            .send_bytes_text(
                ctx,
                method.clone(),
                segments,
                query,
                body,
                content_type,
                options,
            )
            .await?;

        let bytes = response.body();
        let decoded: Response = serde_json::from_slice(bytes).map_err(|e| {
            let url =
                url_util::endpoint(ctx.base_url, segments).unwrap_or_else(|_| ctx.base_url.clone());
            let http = HttpError::new(method, url.path().to_owned(), Some(response.status()))
                .with_request_id(super::extract_request_id(response.headers()))
                .with_body_snippet(super::capture_snippet(bytes, ctx.body_snippet));
            Error::decode(http, e)
        })?;

        Ok(decoded)
    }

    pub(crate) async fn send_bytes<Query, Body>(
        &self,
        ctx: RequestContext<'_>,
        method: Method,
        segments: &[&str],
        query: Option<&Query>,
        body: Option<&Body>,
        options: Option<&RequestOptions>,
    ) -> Result<ResponseBytes>
    where
        Query: Serialize + ?Sized,
        Body: Serialize + ?Sized,
    {
        let url = url_util::endpoint(ctx.base_url, segments)?;
        let path = url.path().to_owned();

        #[cfg(feature = "tracing")]
        let span = tracing::info_span!(
            "grafana.request",
            http.method = %method,
            http.host = %url.host_str().unwrap_or_default(),
            http.path = %path
        );

        #[cfg(feature = "tracing")]
        let _enter = span.enter();

        let mut attempt: usize = 0;
        loop {
            #[cfg(feature = "tracing")]
            tracing::debug!(attempt, "sending request");

            let mut headers = ctx.default_headers.clone();
            if let Some(options) = options {
                for (name, value) in options.headers().iter() {
                    headers.insert(name.clone(), value.clone());
                }
            }
            ctx.auth.apply(&mut headers)?;

            let mut request = self
                .client
                .request(method.clone(), url.clone())
                .headers(headers);
            if let Some(query) = query {
                request = request.query(query);
            }
            if let Some(body) = body {
                request = request.json(body);
            }
            if let Some(timeout) = options.and_then(RequestOptions::timeout_override) {
                request = request.timeout(timeout);
            }

            let response = request.send().await;
            match response {
                Ok(response) => {
                    let status = response.status();
                    let headers = response.headers().clone();

                    if status.is_success() {
                        let bytes = response
                            .bytes()
                            .await
                            .map_err(|e| Error::transport("failed to read response body", e))?;
                        return Ok(ResponseBytes::new(status, headers, bytes.to_vec()));
                    }

                    let bytes = response
                        .bytes()
                        .await
                        .map_err(|e| Error::transport("failed to read error response body", e))?;

                    let retry_after = super::extract_retry_after(&headers);
                    let http = HttpError::new(method.clone(), path.clone(), Some(status))
                        .with_request_id(super::extract_request_id(&headers))
                        .with_message(super::extract_message(&bytes))
                        .with_body_snippet(super::capture_snippet(&bytes, ctx.body_snippet))
                        .with_retry_after(retry_after);
                    if attempt < ctx.retry.max_retries
                        && super::is_retryable_status(&method, status)
                        && let Some(delay) =
                            super::compute_retry_delay(http.retry_after(), ctx.retry, attempt)
                    {
                        attempt += 1;
                        if !delay.is_zero() {
                            tokio::time::sleep(delay).await;
                        }
                        continue;
                    }

                    return Err(Error::from_http(http));
                }
                Err(err) => {
                    if attempt < ctx.retry.max_retries
                        && super::is_retryable_transport_error(&method, &err)
                        && let Some(delay) = super::compute_retry_delay(None, ctx.retry, attempt)
                    {
                        attempt += 1;
                        if !delay.is_zero() {
                            tokio::time::sleep(delay).await;
                        }
                        continue;
                    }

                    return Err(Error::transport("request failed", err));
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) async fn send_bytes_text<Query>(
        &self,
        ctx: RequestContext<'_>,
        method: Method,
        segments: &[&str],
        query: Option<&Query>,
        body: &str,
        content_type: &'static str,
        options: Option<&RequestOptions>,
    ) -> Result<ResponseBytes>
    where
        Query: Serialize + ?Sized,
    {
        let url = url_util::endpoint(ctx.base_url, segments)?;
        let path = url.path().to_owned();

        #[cfg(feature = "tracing")]
        let span = tracing::info_span!(
            "grafana.request",
            http.method = %method,
            http.host = %url.host_str().unwrap_or_default(),
            http.path = %path
        );

        #[cfg(feature = "tracing")]
        let _enter = span.enter();

        let mut attempt: usize = 0;
        loop {
            #[cfg(feature = "tracing")]
            tracing::debug!(attempt, "sending request");

            let mut headers = ctx.default_headers.clone();
            headers.insert(
                http::header::CONTENT_TYPE,
                HeaderValue::from_static(content_type),
            );
            if let Some(options) = options {
                for (name, value) in options.headers().iter() {
                    headers.insert(name.clone(), value.clone());
                }
            }
            ctx.auth.apply(&mut headers)?;

            let mut request = self
                .client
                .request(method.clone(), url.clone())
                .headers(headers)
                .body(body.to_owned());
            if let Some(query) = query {
                request = request.query(query);
            }
            if let Some(timeout) = options.and_then(RequestOptions::timeout_override) {
                request = request.timeout(timeout);
            }

            let response = request.send().await;
            match response {
                Ok(response) => {
                    let status = response.status();
                    let headers = response.headers().clone();

                    if status.is_success() {
                        let bytes = response
                            .bytes()
                            .await
                            .map_err(|e| Error::transport("failed to read response body", e))?;
                        return Ok(ResponseBytes::new(status, headers, bytes.to_vec()));
                    }

                    let bytes = response
                        .bytes()
                        .await
                        .map_err(|e| Error::transport("failed to read error response body", e))?;

                    let retry_after = super::extract_retry_after(&headers);
                    let http = HttpError::new(method.clone(), path.clone(), Some(status))
                        .with_request_id(super::extract_request_id(&headers))
                        .with_message(super::extract_message(&bytes))
                        .with_body_snippet(super::capture_snippet(&bytes, ctx.body_snippet))
                        .with_retry_after(retry_after);
                    if attempt < ctx.retry.max_retries
                        && super::is_retryable_status(&method, status)
                        && let Some(delay) =
                            super::compute_retry_delay(http.retry_after(), ctx.retry, attempt)
                    {
                        attempt += 1;
                        if !delay.is_zero() {
                            tokio::time::sleep(delay).await;
                        }
                        continue;
                    }

                    return Err(Error::from_http(http));
                }
                Err(err) => {
                    if attempt < ctx.retry.max_retries
                        && super::is_retryable_transport_error(&method, &err)
                        && let Some(delay) = super::compute_retry_delay(None, ctx.retry, attempt)
                    {
                        attempt += 1;
                        if !delay.is_zero() {
                            tokio::time::sleep(delay).await;
                        }
                        continue;
                    }

                    return Err(Error::transport("request failed", err));
                }
            }
        }
    }
}
