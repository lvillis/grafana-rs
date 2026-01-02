use serde::{Deserialize, Serialize, Serializer, ser::SerializeMap};

#[derive(Clone, Debug, Default)]
pub struct ServiceAccountSearchParams {
    pub disabled: Option<bool>,
    pub expired_tokens: Option<bool>,
    pub query: Option<String>,
    pub per_page: Option<i64>,
    pub page: Option<i64>,
}

impl Serialize for ServiceAccountSearchParams {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(None)?;
        if let Some(disabled) = self.disabled {
            map.serialize_entry("disabled", &disabled)?;
            map.serialize_entry("Disabled", &disabled)?;
        }
        if let Some(expired_tokens) = self.expired_tokens {
            map.serialize_entry("expiredTokens", &expired_tokens)?;
        }
        if let Some(query) = self.query.as_ref() {
            map.serialize_entry("query", query)?;
        }
        if let Some(per_page) = self.per_page {
            map.serialize_entry("perpage", &per_page)?;
        }
        if let Some(page) = self.page {
            map.serialize_entry("page", &page)?;
        }
        map.end()
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccountSearchResult {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub total_count: Option<i64>,
    pub service_accounts: Option<Vec<ServiceAccount>>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccount {
    pub access_control: Option<serde_json::Value>,
    pub avatar_url: Option<String>,
    pub id: Option<i64>,
    pub is_disabled: Option<bool>,
    pub is_external: Option<bool>,
    pub login: Option<String>,
    pub name: Option<String>,
    pub org_id: Option<i64>,
    pub role: Option<String>,
    pub tokens: Option<i64>,
    pub uid: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceAccountProfile {
    pub access_control: Option<serde_json::Value>,
    pub avatar_url: Option<String>,
    pub created_at: Option<String>,
    pub id: Option<i64>,
    pub is_disabled: Option<bool>,
    pub is_external: Option<bool>,
    pub login: Option<String>,
    pub name: Option<String>,
    pub org_id: Option<i64>,
    pub required_by: Option<String>,
    pub role: Option<String>,
    pub teams: Option<Vec<String>>,
    pub tokens: Option<i64>,
    pub uid: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateServiceAccountRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateServiceAccountRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_id: Option<i64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UpdateServiceAccountResponse {
    pub id: Option<i64>,
    pub message: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "serviceaccount")]
    pub service_account: Option<ServiceAccountProfile>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub created: Option<String>,
    pub expiration: Option<String>,
    pub has_expired: Option<bool>,
    pub id: Option<i64>,
    pub is_revoked: Option<bool>,
    pub last_used_at: Option<String>,
    pub name: Option<String>,
    pub seconds_until_expiration: Option<f64>,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateServiceAccountTokenRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seconds_to_live: Option<i64>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewApiKey {
    pub id: Option<i64>,
    pub key: Option<String>,
    pub name: Option<String>,
}
