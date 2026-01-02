use http::Method;

use crate::{
    Client, Result,
    types::{OrgId, SuccessResponse, Team, UpdateUserRequest, UserOrg, UserProfile},
};

#[derive(Clone)]
pub struct UserService {
    client: Client,
}

impl UserService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get_profile(&self) -> Result<UserProfile> {
        self.client.get_json(&["user"], Option::<&()>::None).await
    }

    pub async fn update_profile(&self, request: &UpdateUserRequest) -> Result<SuccessResponse> {
        self.client.put_json(&["user"], request).await
    }

    pub async fn orgs(&self) -> Result<Vec<UserOrg>> {
        self.client
            .get_json(&["user", "orgs"], Option::<&()>::None)
            .await
    }

    pub async fn teams(&self) -> Result<Vec<Team>> {
        self.client
            .get_json(&["user", "teams"], Option::<&()>::None)
            .await
    }

    pub async fn switch_org(&self, org_id: impl Into<OrgId>) -> Result<SuccessResponse> {
        let org_id: OrgId = org_id.into();
        let org_id_str = org_id.0.to_string();
        let segments = ["user", "using", org_id_str.as_str()];
        self.client
            .request_json::<SuccessResponse, (), ()>(Method::POST, &segments, None, None)
            .await
    }
}
