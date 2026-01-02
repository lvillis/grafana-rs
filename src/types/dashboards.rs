use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct GetDashboardResponse {
    pub meta: serde_json::Value,
    pub dashboard: serde_json::Value,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveDashboardRequest {
    pub dashboard: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_uid: Option<String>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub overwrite: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl SaveDashboardRequest {
    pub fn new(dashboard: serde_json::Value) -> Self {
        Self {
            dashboard,
            folder_id: None,
            folder_uid: None,
            overwrite: false,
            message: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveDashboardResponse {
    pub id: i64,
    pub uid: Option<String>,
    pub url: Option<String>,
    pub status: Option<String>,
    pub slug: Option<String>,
    pub version: Option<i64>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteDashboardResponse {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub message: Option<String>,
}

fn is_false(value: &bool) -> bool {
    !*value
}
