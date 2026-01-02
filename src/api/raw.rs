use http::Method;
use serde::{Serialize, de::DeserializeOwned};

use crate::{Client, RequestOptions, ResponseBytes, Result};

#[derive(Clone)]
pub struct RawService {
    client: Client,
}

impl RawService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn request_json<Response, Query, Body>(
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
        self.client
            .request_json(method, segments, query, body)
            .await
    }

    pub async fn request_json_with_options<Response, Query, Body>(
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
        self.client
            .request_json_with_options(method, segments, query, body, options)
            .await
    }

    pub async fn request_bytes<Query, Body>(
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
        self.client
            .request_bytes(method, segments, query, body)
            .await
    }

    pub async fn request_bytes_with_options<Query, Body>(
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
        self.client
            .request_bytes_with_options(method, segments, query, body, options)
            .await
    }

    pub async fn request_json_text<Response, Query>(
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
        self.client
            .request_json_text(method, segments, query, body, content_type)
            .await
    }

    pub async fn request_json_text_with_options<Response, Query>(
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
        self.client
            .request_json_text_with_options(method, segments, query, body, content_type, options)
            .await
    }

    pub async fn request_bytes_text<Query>(
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
        self.client
            .request_bytes_text(method, segments, query, body, content_type)
            .await
    }

    pub async fn request_bytes_text_with_options<Query>(
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
        self.client
            .request_bytes_text_with_options(method, segments, query, body, content_type, options)
            .await
    }
}
