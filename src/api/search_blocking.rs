use crate::{
    BlockingClient, Result,
    types::{SearchParams, SearchResult},
};

#[derive(Clone)]
pub struct BlockingSearchService {
    client: BlockingClient,
}

impl BlockingSearchService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    pub fn search(&self, params: &SearchParams) -> Result<Vec<SearchResult>> {
        self.client.get_json(&["search"], Some(params))
    }
}
