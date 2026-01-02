use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize)]
pub struct TeamSearchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "perpage")]
    pub per_page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "accesscontrol")]
    pub access_control: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamSearchResult {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub total_count: Option<i64>,
    pub teams: Option<Vec<Team>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub access_control: Option<serde_json::Value>,
    pub avatar_url: Option<String>,
    pub email: Option<String>,
    #[serde(rename = "externalUID")]
    pub external_uid: Option<String>,
    pub id: i64,
    pub is_provisioned: bool,
    pub member_count: i64,
    pub name: String,
    pub org_id: i64,
    pub permission: Option<i64>,
    pub uid: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamMember {
    #[serde(rename = "auth_module")]
    pub auth_module: Option<String>,
    pub avatar_url: Option<String>,
    pub email: Option<String>,
    pub labels: Option<Vec<String>>,
    pub login: Option<String>,
    pub name: Option<String>,
    pub org_id: Option<i64>,
    pub permission: Option<i64>,
    pub team_id: Option<i64>,
    #[serde(rename = "teamUID")]
    pub team_uid: Option<String>,
    pub uid: Option<String>,
    pub user_id: Option<i64>,
    #[serde(rename = "userUID")]
    pub user_uid: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTeamRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl CreateTeamRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            email: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTeamResponse {
    pub message: Option<String>,
    pub team_id: Option<i64>,
    pub uid: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTeamRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddTeamMemberRequest {
    pub user_id: i64,
}

impl AddTeamMemberRequest {
    pub fn new(user_id: i64) -> Self {
        Self { user_id }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTeamMemberRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<i64>,
}
