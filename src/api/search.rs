use crate::{
    Client, Result,
    types::{SearchParams, SearchResult},
};

#[derive(Clone)]
pub struct SearchService {
    client: Client,
}

impl SearchService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn search(&self, params: &SearchParams) -> Result<Vec<SearchResult>> {
        self.client.get_json(&["search"], Some(params)).await
    }
}
