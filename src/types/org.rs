use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub zip_code: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrgDetails {
    pub address: Option<Address>,
    pub id: Option<i64>,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOrgRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrgUser {
    pub access_control: Option<serde_json::Value>,
    pub auth_labels: Option<Vec<String>>,
    pub avatar_url: Option<String>,
    pub email: Option<String>,
    pub is_disabled: Option<bool>,
    pub is_externally_synced: Option<bool>,
    pub is_provisioned: Option<bool>,
    pub last_seen_at: Option<String>,
    pub last_seen_at_age: Option<String>,
    pub login: Option<String>,
    pub name: Option<String>,
    pub org_id: Option<i64>,
    pub role: Option<String>,
    pub uid: Option<String>,
    pub user_id: Option<i64>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddOrgUserRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_or_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateOrgUserRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}
