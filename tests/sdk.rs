#![cfg(feature = "async")]

use std::time::Duration;

use grafana::{Auth, Client, Error};
use http::StatusCode;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_json, body_string, header, method, path, query_param},
};

fn run_async(test: impl std::future::Future<Output = ()>) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime");
    runtime.block_on(test);
}

#[test]
fn health_supports_base_path_and_bearer_auth() {
    run_async(async {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/grafana/api/health"))
            .and(header("authorization", "Bearer TOKEN"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "database": "ok",
                "message": "ok",
                "version": "10.0.0",
                "commit": "deadbeef"
            })))
            .expect(1)
            .mount(&server)
            .await;

        let client = Client::builder(format!("{}/grafana", server.uri()))
            .expect("client builder")
            .auth(Auth::bearer("TOKEN"))
            .build()
            .expect("client build");

        let health = client.health().get().await.expect("health call");
        assert_eq!(health.message.as_deref(), Some("ok"));
        assert_eq!(health.database.as_deref(), Some("ok"));
    });
}

#[test]
fn dashboard_uid_is_path_segment_encoded() {
    run_async(async {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/dashboards/uid/a%2Fb"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "meta": { "folderId": 0 },
                "dashboard": { "uid": "a/b" }
            })))
            .expect(1)
            .mount(&server)
            .await;

        let client = Client::builder(server.uri())
            .expect("client builder")
            .build()
            .expect("client build");

        let response = client
            .dashboards()
            .get_by_uid("a/b")
            .await
            .expect("get dashboard");

        assert_eq!(
            response.dashboard.get("uid").and_then(|v| v.as_str()),
            Some("a/b")
        );
    });
}

#[test]
fn error_is_classified_and_body_snippet_is_redacted() {
    run_async(async {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/health"))
            .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
                "message": "not found",
                "token": "SECRET"
            })))
            .expect(1)
            .mount(&server)
            .await;

        let client = Client::builder(server.uri())
            .expect("client builder")
            .build()
            .expect("client build");

        let error = client.health().get().await.expect_err("expected error");
        match &error {
            Error::NotFound(http) => {
                assert_eq!(http.status(), Some(StatusCode::NOT_FOUND));
                assert_eq!(http.message(), Some("not found"));
            }
            other => panic!("unexpected error variant: {other:?}"),
        }

        let snippet = error.body_snippet().expect("body snippet");
        assert!(snippet.contains("<redacted>"));
        assert!(!snippet.contains("SECRET"));
    });
}

#[test]
fn retry_after_is_honored_for_rate_limits() {
    run_async(async {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/health"))
            .respond_with(
                ResponseTemplate::new(429)
                    .insert_header("Retry-After", "0")
                    .set_body_json(serde_json::json!({ "message": "rate limited" })),
            )
            .with_priority(1)
            .up_to_n_times(1)
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/api/health"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "database": "ok",
                "message": "ok"
            })))
            .with_priority(2)
            .expect(1)
            .mount(&server)
            .await;

        let client = Client::builder(server.uri())
            .expect("client builder")
            .max_retries(3)
            .build()
            .expect("client build");

        let health = client
            .health()
            .get()
            .await
            .expect("health call after retry");
        assert_eq!(health.message.as_deref(), Some("ok"));
    });
}

#[test]
fn retry_is_honored_for_service_unavailable() {
    run_async(async {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/health"))
            .respond_with(
                ResponseTemplate::new(503)
                    .insert_header("Retry-After", "0")
                    .set_body_json(serde_json::json!({ "message": "temporarily unavailable" })),
            )
            .with_priority(1)
            .up_to_n_times(1)
            .mount(&server)
            .await;

        Mock::given(method("GET"))
            .and(path("/api/health"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "database": "ok",
                "message": "ok"
            })))
            .with_priority(2)
            .expect(1)
            .mount(&server)
            .await;

        let client = Client::builder(server.uri())
            .expect("client builder")
            .max_retries(3)
            .build()
            .expect("client build");

        let health = client
            .health()
            .get()
            .await
            .expect("health call after retry");
        assert_eq!(health.message.as_deref(), Some("ok"));
    });
}

#[test]
fn user_and_org_endpoints_work_with_base_path() {
    run_async(async {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/grafana/api/user"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "login": "alice",
                "orgId": 1
            })))
            .expect(1)
            .mount(&server)
            .await;

        Mock::given(method("POST"))
            .and(path("/grafana/api/user/using/42"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "message": "ok"
            })))
            .expect(1)
            .mount(&server)
            .await;

        let client = Client::builder(format!("{}/grafana", server.uri()))
            .expect("client builder")
            .build()
            .expect("client build");

        let profile = client.user().get_profile().await.expect("user profile");
        assert_eq!(profile.login.as_deref(), Some("alice"));

        let resp = client.user().switch_org(42).await.expect("switch org");
        assert_eq!(resp.message.as_deref(), Some("ok"));
    });
}

#[test]
fn teams_search_serializes_query_params() {
    run_async(async {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/teams/search"))
            .and(query_param("perpage", "5"))
            .and(query_param("page", "2"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "page": 2,
                "perPage": 5,
                "totalCount": 1,
                "teams": [{
                    "id": 1,
                    "uid": "t1",
                    "orgId": 1,
                    "name": "team-one",
                    "memberCount": 0,
                    "isProvisioned": false
                }]
            })))
            .expect(1)
            .mount(&server)
            .await;

        let client = Client::builder(server.uri())
            .expect("client builder")
            .build()
            .expect("client build");

        let params = grafana::types::TeamSearchParams {
            page: Some(2),
            per_page: Some(5),
            ..Default::default()
        };
        let result = client.teams().search(&params).await.expect("team search");
        assert_eq!(result.total_count, Some(1));
    });
}

#[test]
fn service_accounts_search_serializes_disabled_variants() {
    run_async(async {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/serviceaccounts/search"))
            .and(query_param("disabled", "true"))
            .and(query_param("Disabled", "true"))
            .and(query_param("perpage", "10"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "page": 1,
                "perPage": 10,
                "totalCount": 0,
                "serviceAccounts": []
            })))
            .expect(1)
            .mount(&server)
            .await;

        let client = Client::builder(server.uri())
            .expect("client builder")
            .build()
            .expect("client build");

        let params = grafana::types::ServiceAccountSearchParams {
            disabled: Some(true),
            per_page: Some(10),
            page: Some(1),
            ..Default::default()
        };
        let result = client
            .service_accounts()
            .search(&params)
            .await
            .expect("service account search");
        assert_eq!(result.total_count, Some(0));
    });
}

#[test]
fn org_user_role_update_uses_patch() {
    run_async(async {
        let server = MockServer::start().await;

        Mock::given(method("PATCH"))
            .and(path("/api/org/users/123"))
            .and(body_json(serde_json::json!({ "role": "Admin" })))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "message": "ok"
            })))
            .expect(1)
            .mount(&server)
            .await;

        let client = Client::builder(server.uri())
            .expect("client builder")
            .build()
            .expect("client build");

        let request = grafana::types::UpdateOrgUserRequest {
            role: Some("Admin".to_owned()),
        };
        let resp = client
            .org()
            .update_user_role(123, &request)
            .await
            .expect("update org user role");
        assert_eq!(resp.message.as_deref(), Some("ok"));
    });
}

#[test]
fn retry_does_not_apply_to_post_requests() {
    run_async(async {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/dashboards/db"))
            .respond_with(ResponseTemplate::new(503).set_body_json(serde_json::json!({
                "message": "temporarily unavailable"
            })))
            .expect(1)
            .mount(&server)
            .await;

        let client = Client::builder(server.uri())
            .expect("client builder")
            .max_retries(3)
            .retry_base_delay(Duration::ZERO)
            .build()
            .expect("client build");

        let request = grafana::types::SaveDashboardRequest::new(serde_json::json!({
            "uid": "t1",
            "title": "test",
            "schemaVersion": 1,
            "panels": []
        }));
        let err = client
            .dashboards()
            .save(&request)
            .await
            .expect_err("expected non-retryable error for POST");

        assert_eq!(err.status(), Some(StatusCode::SERVICE_UNAVAILABLE));
    });
}

#[test]
fn service_account_token_create_posts_json_body() {
    run_async(async {
        let server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/api/serviceaccounts/1/tokens"))
            .and(body_json(serde_json::json!({
                "name": "token",
                "secondsToLive": 3600
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "id": 1,
                "name": "token",
                "key": "secret"
            })))
            .expect(1)
            .mount(&server)
            .await;

        let client = Client::builder(server.uri())
            .expect("client builder")
            .build()
            .expect("client build");

        let request = grafana::types::CreateServiceAccountTokenRequest {
            name: Some("token".to_owned()),
            seconds_to_live: Some(3600),
        };

        let token = client
            .service_accounts()
            .create_token(1, &request)
            .await
            .expect("create token");
        assert_eq!(token.name.as_deref(), Some("token"));
    });
}

#[test]
fn openapi_get_user_by_login_or_email_serializes_required_query_param() {
    run_async(async {
        let server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/api/users/lookup"))
            .and(query_param("loginOrEmail", "alice@example.com"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "login": "alice"
            })))
            .expect(1)
            .mount(&server)
            .await;

        let client = Client::builder(server.uri())
            .expect("client builder")
            .build()
            .expect("client build");

        let value: serde_json::Value = client
            .openapi()
            .get_user_by_login_or_email("alice@example.com")
            .await
            .expect("openapi call");
        assert_eq!(value.get("login").and_then(|v| v.as_str()), Some("alice"));
    });
}

#[test]
fn openapi_supports_yaml_request_bodies_for_convert_prometheus() {
    run_async(async {
        let server = MockServer::start().await;

        let yaml = "groups: []\n";

        Mock::given(method("POST"))
            .and(path("/api/convert/prometheus/config/v1/rules/ns1"))
            .and(header("content-type", "application/yaml"))
            .and(body_string(yaml))
            .respond_with(ResponseTemplate::new(202).set_body_json(serde_json::json!({
                "message": "accepted"
            })))
            .expect(1)
            .mount(&server)
            .await;

        let client = Client::builder(server.uri())
            .expect("client builder")
            .build()
            .expect("client build");

        let value: serde_json::Value = client
            .openapi()
            .route_convert_prometheus_post_rule_group("ns1", Some(yaml))
            .await
            .expect("openapi yaml body call");
        assert_eq!(
            value.get("message").and_then(|v| v.as_str()),
            Some("accepted")
        );
    });
}
