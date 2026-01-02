use crate::{
    Client, Result,
    types::{Datasource, DatasourceId, DeleteDatasourceResponse},
};

#[derive(Clone)]
pub struct DatasourcesService {
    client: Client,
}

impl DatasourcesService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<Vec<Datasource>> {
        self.client
            .get_json(&["datasources"], Option::<&()>::None)
            .await
    }

    pub async fn get_by_id(&self, id: impl Into<DatasourceId>) -> Result<Datasource> {
        let id: DatasourceId = id.into();
        let id_str = id.0.to_string();
        let segments = ["datasources", id_str.as_str()];
        self.client.get_json(&segments, Option::<&()>::None).await
    }

    pub async fn delete_by_id(
        &self,
        id: impl Into<DatasourceId>,
    ) -> Result<DeleteDatasourceResponse> {
        let id: DatasourceId = id.into();
        let id_str = id.0.to_string();
        let segments = ["datasources", id_str.as_str()];
        self.client.delete_json(&segments).await
    }
}
