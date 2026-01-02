use crate::{
    BlockingClient, Result,
    types::{
        DashboardUid, DeleteDashboardResponse, GetDashboardResponse, SaveDashboardRequest,
        SaveDashboardResponse,
    },
};

#[derive(Clone)]
pub struct BlockingDashboardsService {
    client: BlockingClient,
}

impl BlockingDashboardsService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    pub fn get_by_uid(&self, uid: impl Into<DashboardUid>) -> Result<GetDashboardResponse> {
        let uid: DashboardUid = uid.into();
        let segments = ["dashboards", "uid", uid.0.as_str()];
        self.client.get_json(&segments, Option::<&()>::None)
    }

    pub fn save(&self, request: &SaveDashboardRequest) -> Result<SaveDashboardResponse> {
        self.client.post_json(&["dashboards", "db"], request)
    }

    pub fn delete_by_uid(&self, uid: impl Into<DashboardUid>) -> Result<DeleteDashboardResponse> {
        let uid: DashboardUid = uid.into();
        let segments = ["dashboards", "uid", uid.0.as_str()];
        self.client.delete_json(&segments)
    }
}
