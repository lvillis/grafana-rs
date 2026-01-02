use http::Method;

use crate::{
    BlockingClient, Result,
    types::{OrgId, SuccessResponse, Team, UpdateUserRequest, UserOrg, UserProfile},
};

#[derive(Clone)]
pub struct BlockingUserService {
    client: BlockingClient,
}

impl BlockingUserService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    pub fn get_profile(&self) -> Result<UserProfile> {
        self.client.get_json(&["user"], Option::<&()>::None)
    }

    pub fn update_profile(&self, request: &UpdateUserRequest) -> Result<SuccessResponse> {
        self.client.put_json(&["user"], request)
    }

    pub fn orgs(&self) -> Result<Vec<UserOrg>> {
        self.client.get_json(&["user", "orgs"], Option::<&()>::None)
    }

    pub fn teams(&self) -> Result<Vec<Team>> {
        self.client
            .get_json(&["user", "teams"], Option::<&()>::None)
    }

    pub fn switch_org(&self, org_id: impl Into<OrgId>) -> Result<SuccessResponse> {
        let org_id: OrgId = org_id.into();
        let org_id_str = org_id.0.to_string();
        let segments = ["user", "using", org_id_str.as_str()];
        self.client
            .request_json::<SuccessResponse, (), ()>(Method::POST, &segments, None, None)
    }
}
