use crate::{BlockingClient, Result, types::HealthResponse};

#[derive(Clone)]
pub struct BlockingHealthService {
    client: BlockingClient,
}

impl BlockingHealthService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    pub fn get(&self) -> Result<HealthResponse> {
        self.client.get_json(&["health"], Option::<&()>::None)
    }
}
