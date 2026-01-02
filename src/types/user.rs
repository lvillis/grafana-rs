use serde::{Deserialize, Serialize};

use super::teams::Team;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserProfile {
    pub access_control: Option<serde_json::Value>,
    pub auth_labels: Option<Vec<String>>,
    pub avatar_url: Option<String>,
    pub created_at: Option<String>,
    pub email: Option<String>,
    pub id: Option<i64>,
    pub is_disabled: Option<bool>,
    pub is_external: Option<bool>,
    pub is_externally_synced: Option<bool>,
    pub is_grafana_admin: Option<bool>,
    pub is_grafana_admin_externally_synced: Option<bool>,
    pub is_provisioned: Option<bool>,
    pub login: Option<String>,
    pub name: Option<String>,
    pub org_id: Option<i64>,
    pub theme: Option<String>,
    pub uid: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserOrg {
    pub name: Option<String>,
    pub org_id: Option<i64>,
    pub role: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
}

pub type UserTeamsResponse = Vec<Team>;
