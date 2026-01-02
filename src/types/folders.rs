use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub id: i64,
    pub uid: String,
    pub title: String,
    pub url: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFolderRequest {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

impl CreateFolderRequest {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            uid: None,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateFolderRequest {
    pub title: String,
}

impl UpdateFolderRequest {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteFolderResponse {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub message: Option<String>,
}
