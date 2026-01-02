use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Datasource {
    pub id: i64,
    pub uid: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub url: Option<String>,
    pub access: Option<String>,
    pub is_default: Option<bool>,
    pub json_data: Option<serde_json::Value>,
    pub secure_json_fields: Option<serde_json::Value>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteDatasourceResponse {
    pub message: Option<String>,
}
