use http::Method;

use crate::{
    Client, Result,
    types::{
        CreateServiceAccountRequest, CreateServiceAccountTokenRequest, NewApiKey, ServiceAccount,
        ServiceAccountId, ServiceAccountSearchParams, ServiceAccountSearchResult, SuccessResponse,
        Token, TokenId, UpdateServiceAccountRequest, UpdateServiceAccountResponse,
    },
};

#[derive(Clone)]
pub struct ServiceAccountsService {
    client: Client,
}

impl ServiceAccountsService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn search(
        &self,
        params: &ServiceAccountSearchParams,
    ) -> Result<ServiceAccountSearchResult> {
        self.client
            .get_json(&["serviceaccounts", "search"], Some(params))
            .await
    }

    pub async fn create(&self, request: &CreateServiceAccountRequest) -> Result<ServiceAccount> {
        self.client.post_json(&["serviceaccounts"], request).await
    }

    pub async fn get_by_id(
        &self,
        service_account_id: impl Into<ServiceAccountId>,
    ) -> Result<ServiceAccount> {
        let service_account_id: ServiceAccountId = service_account_id.into();
        let id_str = service_account_id.0.to_string();
        let segments = ["serviceaccounts", id_str.as_str()];
        self.client.get_json(&segments, Option::<&()>::None).await
    }

    pub async fn update(
        &self,
        service_account_id: impl Into<ServiceAccountId>,
        request: &UpdateServiceAccountRequest,
    ) -> Result<UpdateServiceAccountResponse> {
        let service_account_id: ServiceAccountId = service_account_id.into();
        let id_str = service_account_id.0.to_string();
        let segments = ["serviceaccounts", id_str.as_str()];
        self.client
            .request_json::<UpdateServiceAccountResponse, (), UpdateServiceAccountRequest>(
                Method::PATCH,
                &segments,
                None,
                Some(request),
            )
            .await
    }

    pub async fn delete(
        &self,
        service_account_id: impl Into<ServiceAccountId>,
    ) -> Result<SuccessResponse> {
        let service_account_id: ServiceAccountId = service_account_id.into();
        let id_str = service_account_id.0.to_string();
        let segments = ["serviceaccounts", id_str.as_str()];
        self.client.delete_json(&segments).await
    }

    pub async fn tokens(
        &self,
        service_account_id: impl Into<ServiceAccountId>,
    ) -> Result<Vec<Token>> {
        let service_account_id: ServiceAccountId = service_account_id.into();
        let id_str = service_account_id.0.to_string();
        let segments = ["serviceaccounts", id_str.as_str(), "tokens"];
        self.client.get_json(&segments, Option::<&()>::None).await
    }

    pub async fn create_token(
        &self,
        service_account_id: impl Into<ServiceAccountId>,
        request: &CreateServiceAccountTokenRequest,
    ) -> Result<NewApiKey> {
        let service_account_id: ServiceAccountId = service_account_id.into();
        let id_str = service_account_id.0.to_string();
        let segments = ["serviceaccounts", id_str.as_str(), "tokens"];
        self.client.post_json(&segments, request).await
    }

    pub async fn delete_token(
        &self,
        service_account_id: impl Into<ServiceAccountId>,
        token_id: impl Into<TokenId>,
    ) -> Result<SuccessResponse> {
        let service_account_id: ServiceAccountId = service_account_id.into();
        let token_id: TokenId = token_id.into();
        let service_account_id_str = service_account_id.0.to_string();
        let token_id_str = token_id.0.to_string();
        let segments = [
            "serviceaccounts",
            service_account_id_str.as_str(),
            "tokens",
            token_id_str.as_str(),
        ];
        self.client.delete_json(&segments).await
    }
}
