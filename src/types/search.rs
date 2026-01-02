use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starred: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tag: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    pub id: i64,
    pub uid: Option<String>,
    pub title: Option<String>,
    pub uri: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub tags: Option<Vec<String>>,
    pub folder_id: Option<i64>,
    pub folder_uid: Option<String>,
}
