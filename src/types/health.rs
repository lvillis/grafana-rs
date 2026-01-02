use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthResponse {
    pub database: Option<String>,
    pub message: Option<String>,
    pub version: Option<String>,
    pub commit: Option<String>,
}
