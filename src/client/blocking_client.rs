use std::{sync::Arc, time::Duration};

use http::{HeaderMap, HeaderName, HeaderValue, Method};
use serde::{Serialize, de::DeserializeOwned};
use url::Url;

use crate::{
    Auth, Error, RequestOptions, ResponseBytes, Result, api,
    transport::{
        BlockingTransport, BodySnippetConfig, RequestContext, RetryConfig, TransportConfig,
    },
    util,
};

#[derive(Clone)]
pub struct BlockingClient {
    inner: Arc<Inner>,
}

struct Inner {
    api_base_url: Url,
    auth: Auth,
    default_headers: HeaderMap,
    transport: BlockingTransport,
    retry: RetryConfig,
    body_snippet: BodySnippetConfig,
}

pub struct BlockingClientBuilder {
    base_url: Url,
    auth: Auth,
    default_headers: HeaderMap,
    transport: TransportConfig,
    retry: RetryConfig,
    body_snippet: BodySnippetConfig,
}

impl BlockingClient {
    pub fn builder(base_url: impl AsRef<str>) -> Result<BlockingClientBuilder> {
        let base_url = Url::parse(base_url.as_ref())
            .map_err(|e| Error::invalid_config(format!("invalid base_url: {e}")))?;

        if base_url.cannot_be_a_base() {
            return Err(Error::invalid_config("base_url must be hierarchical"));
        }
        if !matches!(base_url.scheme(), "http" | "https") {
            return Err(Error::invalid_config(
                "base_url scheme must be http or https",
            ));
        }
        if base_url.host_str().is_none() {
            return Err(Error::invalid_config("base_url must include a host"));
        }
        if base_url.query().is_some() || base_url.fragment().is_some() {
            return Err(Error::invalid_config(
                "base_url must not include query or fragment",
            ));
        }

        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            http::header::ACCEPT,
            HeaderValue::from_static("application/json"),
        );

        Ok(BlockingClientBuilder {
            base_url,
            auth: Auth::none(),
            default_headers,
            transport: TransportConfig {
                timeout: Duration::from_secs(30),
                connect_timeout: Duration::from_secs(10),
            },
            retry: RetryConfig {
                max_retries: 3,
                base_delay: Duration::from_millis(200),
                max_delay: Duration::from_secs(2),
            },
            body_snippet: BodySnippetConfig {
                enabled: true,
                byte_limit: 4096,
            },
        })
    }

    pub fn health(&self) -> api::BlockingHealthService {
        api::BlockingHealthService::new(self.clone())
    }

    pub fn dashboards(&self) -> api::BlockingDashboardsService {
        api::BlockingDashboardsService::new(self.clone())
    }

    pub fn folders(&self) -> api::BlockingFoldersService {
        api::BlockingFoldersService::new(self.clone())
    }

    pub fn datasources(&self) -> api::BlockingDatasourcesService {
        api::BlockingDatasourcesService::new(self.clone())
    }

    pub fn search(&self) -> api::BlockingSearchService {
        api::BlockingSearchService::new(self.clone())
    }

    pub fn user(&self) -> api::BlockingUserService {
        api::BlockingUserService::new(self.clone())
    }

    pub fn org(&self) -> api::BlockingOrgService {
        api::BlockingOrgService::new(self.clone())
    }

    pub fn teams(&self) -> api::BlockingTeamsService {
        api::BlockingTeamsService::new(self.clone())
    }

    pub fn service_accounts(&self) -> api::BlockingServiceAccountsService {
        api::BlockingServiceAccountsService::new(self.clone())
    }

    pub fn openapi(&self) -> api::BlockingOpenApi {
        api::BlockingOpenApi::new(self.clone())
    }

    pub fn raw(&self) -> api::BlockingRawService {
        api::BlockingRawService::new(self.clone())
    }

    pub(crate) fn request_json<Response, Query, Body>(
        &self,
        method: Method,
        segments: &[&str],
        query: Option<&Query>,
        body: Option<&Body>,
    ) -> Result<Response>
    where
        Response: DeserializeOwned,
        Query: Serialize + ?Sized,
        Body: Serialize + ?Sized,
    {
        let ctx = RequestContext {
            base_url: &self.inner.api_base_url,
            auth: &self.inner.auth,
            default_headers: &self.inner.default_headers,
            retry: &self.inner.retry,
            body_snippet: &self.inner.body_snippet,
        };

        self.inner
            .transport
            .send_json(ctx, method, segments, query, body, None)
    }

    pub(crate) fn request_json_with_options<Response, Query, Body>(
        &self,
        method: Method,
        segments: &[&str],
        query: Option<&Query>,
        body: Option<&Body>,
        options: &RequestOptions,
    ) -> Result<Response>
    where
        Response: DeserializeOwned,
        Query: Serialize + ?Sized,
        Body: Serialize + ?Sized,
    {
        let ctx = RequestContext {
            base_url: &self.inner.api_base_url,
            auth: &self.inner.auth,
            default_headers: &self.inner.default_headers,
            retry: &self.inner.retry,
            body_snippet: &self.inner.body_snippet,
        };

        self.inner
            .transport
            .send_json(ctx, method, segments, query, body, Some(options))
    }

    pub(crate) fn request_bytes<Query, Body>(
        &self,
        method: Method,
        segments: &[&str],
        query: Option<&Query>,
        body: Option<&Body>,
    ) -> Result<ResponseBytes>
    where
        Query: Serialize + ?Sized,
        Body: Serialize + ?Sized,
    {
        let ctx = RequestContext {
            base_url: &self.inner.api_base_url,
            auth: &self.inner.auth,
            default_headers: &self.inner.default_headers,
            retry: &self.inner.retry,
            body_snippet: &self.inner.body_snippet,
        };

        self.inner
            .transport
            .send_bytes(ctx, method, segments, query, body, None)
    }

    pub(crate) fn request_bytes_with_options<Query, Body>(
        &self,
        method: Method,
        segments: &[&str],
        query: Option<&Query>,
        body: Option<&Body>,
        options: &RequestOptions,
    ) -> Result<ResponseBytes>
    where
        Query: Serialize + ?Sized,
        Body: Serialize + ?Sized,
    {
        let ctx = RequestContext {
            base_url: &self.inner.api_base_url,
            auth: &self.inner.auth,
            default_headers: &self.inner.default_headers,
            retry: &self.inner.retry,
            body_snippet: &self.inner.body_snippet,
        };

        self.inner
            .transport
            .send_bytes(ctx, method, segments, query, body, Some(options))
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn request_json_text<Response, Query>(
        &self,
        method: Method,
        segments: &[&str],
        query: Option<&Query>,
        body: &str,
        content_type: &'static str,
    ) -> Result<Response>
    where
        Response: DeserializeOwned,
        Query: Serialize + ?Sized,
    {
        let ctx = RequestContext {
            base_url: &self.inner.api_base_url,
            auth: &self.inner.auth,
            default_headers: &self.inner.default_headers,
            retry: &self.inner.retry,
            body_snippet: &self.inner.body_snippet,
        };

        self.inner
            .transport
            .send_json_text(ctx, method, segments, query, body, content_type, None)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn request_json_text_with_options<Response, Query>(
        &self,
        method: Method,
        segments: &[&str],
        query: Option<&Query>,
        body: &str,
        content_type: &'static str,
        options: &RequestOptions,
    ) -> Result<Response>
    where
        Response: DeserializeOwned,
        Query: Serialize + ?Sized,
    {
        let ctx = RequestContext {
            base_url: &self.inner.api_base_url,
            auth: &self.inner.auth,
            default_headers: &self.inner.default_headers,
            retry: &self.inner.retry,
            body_snippet: &self.inner.body_snippet,
        };

        self.inner.transport.send_json_text(
            ctx,
            method,
            segments,
            query,
            body,
            content_type,
            Some(options),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn request_bytes_text<Query>(
        &self,
        method: Method,
        segments: &[&str],
        query: Option<&Query>,
        body: &str,
        content_type: &'static str,
    ) -> Result<ResponseBytes>
    where
        Query: Serialize + ?Sized,
    {
        let ctx = RequestContext {
            base_url: &self.inner.api_base_url,
            auth: &self.inner.auth,
            default_headers: &self.inner.default_headers,
            retry: &self.inner.retry,
            body_snippet: &self.inner.body_snippet,
        };

        self.inner
            .transport
            .send_bytes_text(ctx, method, segments, query, body, content_type, None)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn request_bytes_text_with_options<Query>(
        &self,
        method: Method,
        segments: &[&str],
        query: Option<&Query>,
        body: &str,
        content_type: &'static str,
        options: &RequestOptions,
    ) -> Result<ResponseBytes>
    where
        Query: Serialize + ?Sized,
    {
        let ctx = RequestContext {
            base_url: &self.inner.api_base_url,
            auth: &self.inner.auth,
            default_headers: &self.inner.default_headers,
            retry: &self.inner.retry,
            body_snippet: &self.inner.body_snippet,
        };

        self.inner.transport.send_bytes_text(
            ctx,
            method,
            segments,
            query,
            body,
            content_type,
            Some(options),
        )
    }

    pub(crate) fn get_json<Response, Query>(
        &self,
        segments: &[&str],
        query: Option<&Query>,
    ) -> Result<Response>
    where
        Response: DeserializeOwned,
        Query: Serialize + ?Sized,
    {
        self.request_json(Method::GET, segments, query, Option::<&()>::None)
    }

    pub(crate) fn post_json<Response, Body>(
        &self,
        segments: &[&str],
        body: &Body,
    ) -> Result<Response>
    where
        Response: DeserializeOwned,
        Body: Serialize + ?Sized,
    {
        self.request_json(Method::POST, segments, Option::<&()>::None, Some(body))
    }

    pub(crate) fn put_json<Response, Body>(
        &self,
        segments: &[&str],
        body: &Body,
    ) -> Result<Response>
    where
        Response: DeserializeOwned,
        Body: Serialize + ?Sized,
    {
        self.request_json(Method::PUT, segments, Option::<&()>::None, Some(body))
    }

    pub(crate) fn delete_json<Response>(&self, segments: &[&str]) -> Result<Response>
    where
        Response: DeserializeOwned,
    {
        self.request_json(
            Method::DELETE,
            segments,
            Option::<&()>::None,
            Option::<&()>::None,
        )
    }
}

impl BlockingClientBuilder {
    pub fn auth(mut self, auth: Auth) -> Self {
        self.auth = auth;
        self
    }

    pub fn default_header(mut self, name: HeaderName, value: HeaderValue) -> Self {
        self.default_headers.insert(name, value);
        self
    }

    pub fn default_headers(mut self, headers: HeaderMap) -> Self {
        self.default_headers = headers;
        self
    }

    pub fn user_agent(mut self, user_agent: impl AsRef<str>) -> Result<Self> {
        let user_agent = HeaderValue::from_str(user_agent.as_ref())
            .map_err(|e| Error::invalid_config(format!("invalid user agent: {e}")))?;
        self.default_headers
            .insert(http::header::USER_AGENT, user_agent);
        Ok(self)
    }

    pub fn org_id(mut self, org_id: i64) -> Result<Self> {
        let value = HeaderValue::from_str(&org_id.to_string())
            .map_err(|e| Error::invalid_config(format!("invalid org id: {e}")))?;
        self.default_headers
            .insert(HeaderName::from_static("x-grafana-org-id"), value);
        Ok(self)
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.transport.timeout = timeout;
        self
    }

    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.transport.connect_timeout = timeout;
        self
    }

    pub fn max_retries(mut self, max_retries: usize) -> Self {
        self.retry.max_retries = max_retries;
        self
    }

    pub fn retry_base_delay(mut self, delay: Duration) -> Self {
        self.retry.base_delay = delay;
        self
    }

    pub fn retry_max_delay(mut self, delay: Duration) -> Self {
        self.retry.max_delay = delay;
        self
    }

    pub fn capture_body_snippet(mut self, enabled: bool) -> Self {
        self.body_snippet.enabled = enabled;
        self
    }

    pub fn body_snippet_limit(mut self, byte_limit: usize) -> Self {
        self.body_snippet.byte_limit = byte_limit;
        self
    }

    pub fn build(self) -> Result<BlockingClient> {
        let api_base_url = match self
            .base_url
            .path_segments()
            .and_then(|mut segs| segs.rfind(|s| !s.is_empty()))
        {
            Some("api") => self.base_url,
            _ => util::url::endpoint(&self.base_url, &["api"])?,
        };

        let transport = BlockingTransport::new(&self.transport)?;

        Ok(BlockingClient {
            inner: Arc::new(Inner {
                api_base_url,
                auth: self.auth,
                default_headers: self.default_headers,
                transport,
                retry: self.retry,
                body_snippet: self.body_snippet,
            }),
        })
    }
}
