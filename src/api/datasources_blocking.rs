use crate::{
    BlockingClient, Result,
    types::{Datasource, DatasourceId, DeleteDatasourceResponse},
};

#[derive(Clone)]
pub struct BlockingDatasourcesService {
    client: BlockingClient,
}

impl BlockingDatasourcesService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    pub fn list(&self) -> Result<Vec<Datasource>> {
        self.client.get_json(&["datasources"], Option::<&()>::None)
    }

    pub fn get_by_id(&self, id: impl Into<DatasourceId>) -> Result<Datasource> {
        let id: DatasourceId = id.into();
        let id_str = id.0.to_string();
        let segments = ["datasources", id_str.as_str()];
        self.client.get_json(&segments, Option::<&()>::None)
    }

    pub fn delete_by_id(&self, id: impl Into<DatasourceId>) -> Result<DeleteDatasourceResponse> {
        let id: DatasourceId = id.into();
        let id_str = id.0.to_string();
        let segments = ["datasources", id_str.as_str()];
        self.client.delete_json(&segments)
    }
}
