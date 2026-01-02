use crate::{Client, Result, types::HealthResponse};

#[derive(Clone)]
pub struct HealthService {
    client: Client,
}

impl HealthService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get(&self) -> Result<HealthResponse> {
        self.client.get_json(&["health"], Option::<&()>::None).await
    }
}
