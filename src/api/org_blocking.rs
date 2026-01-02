use http::Method;

use crate::{
    BlockingClient, Result,
    types::{
        AddOrgUserRequest, OrgDetails, OrgUser, SuccessResponse, UpdateOrgRequest,
        UpdateOrgUserRequest, UserId,
    },
};

#[derive(Clone)]
pub struct BlockingOrgService {
    client: BlockingClient,
}

impl BlockingOrgService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    pub fn get(&self) -> Result<OrgDetails> {
        self.client.get_json(&["org"], Option::<&()>::None)
    }

    pub fn update(&self, request: &UpdateOrgRequest) -> Result<SuccessResponse> {
        self.client.put_json(&["org"], request)
    }

    pub fn users(&self) -> Result<Vec<OrgUser>> {
        self.client.get_json(&["org", "users"], Option::<&()>::None)
    }

    pub fn add_user(&self, request: &AddOrgUserRequest) -> Result<SuccessResponse> {
        self.client.post_json(&["org", "users"], request)
    }

    pub fn update_user_role(
        &self,
        user_id: impl Into<UserId>,
        request: &UpdateOrgUserRequest,
    ) -> Result<SuccessResponse> {
        let user_id: UserId = user_id.into();
        let user_id_str = user_id.0.to_string();
        let segments = ["org", "users", user_id_str.as_str()];
        self.client
            .request_json::<SuccessResponse, (), UpdateOrgUserRequest>(
                Method::PATCH,
                &segments,
                None,
                Some(request),
            )
    }

    pub fn remove_user(&self, user_id: impl Into<UserId>) -> Result<SuccessResponse> {
        let user_id: UserId = user_id.into();
        let user_id_str = user_id.0.to_string();
        let segments = ["org", "users", user_id_str.as_str()];
        self.client.delete_json(&segments)
    }
}
