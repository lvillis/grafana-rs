use crate::{
    Client, Result,
    types::{
        DashboardUid, DeleteDashboardResponse, GetDashboardResponse, SaveDashboardRequest,
        SaveDashboardResponse,
    },
};

#[derive(Clone)]
pub struct DashboardsService {
    client: Client,
}

impl DashboardsService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get_by_uid(&self, uid: impl Into<DashboardUid>) -> Result<GetDashboardResponse> {
        let uid: DashboardUid = uid.into();
        let segments = ["dashboards", "uid", uid.0.as_str()];
        self.client.get_json(&segments, Option::<&()>::None).await
    }

    pub async fn save(&self, request: &SaveDashboardRequest) -> Result<SaveDashboardResponse> {
        self.client.post_json(&["dashboards", "db"], request).await
    }

    pub async fn delete_by_uid(
        &self,
        uid: impl Into<DashboardUid>,
    ) -> Result<DeleteDashboardResponse> {
        let uid: DashboardUid = uid.into();
        let segments = ["dashboards", "uid", uid.0.as_str()];
        self.client.delete_json(&segments).await
    }
}
