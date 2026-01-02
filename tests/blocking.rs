#![cfg(feature = "blocking")]

use std::time::Duration;

use grafana::{Auth, BlockingClient, Error};
use http::StatusCode;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{body_string, header, method, path, query_param},
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
                "message": "ok"
            })))
            .expect(1)
            .mount(&server)
            .await;

        let base_url = format!("{}/grafana", server.uri());
        let health = tokio::task::spawn_blocking(move || {
            let client = BlockingClient::builder(base_url)
                .expect("client builder")
                .auth(Auth::bearer("TOKEN"))
                .build()
                .expect("client build");
            client.health().get()
        })
        .await
        .expect("join blocking task")
        .expect("health call");
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

        let base_url = server.uri();
        let response = tokio::task::spawn_blocking(move || {
            let client = BlockingClient::builder(base_url)
                .expect("client builder")
                .build()
                .expect("client build");
            client.dashboards().get_by_uid("a/b")
        })
        .await
        .expect("join blocking task")
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

        let base_url = server.uri();
        let error = tokio::task::spawn_blocking(move || {
            let client = BlockingClient::builder(base_url)
                .expect("client builder")
                .build()
                .expect("client build");
            client.health().get()
        })
        .await
        .expect("join blocking task")
        .expect_err("expected error");
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

        let base_url = server.uri();
        let health = tokio::task::spawn_blocking(move || {
            let client = BlockingClient::builder(base_url)
                .expect("client builder")
                .max_retries(3)
                .build()
                .expect("client build");
            client.health().get()
        })
        .await
        .expect("join blocking task")
        .expect("health call after retry");
        assert_eq!(health.message.as_deref(), Some("ok"));
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

        let base_url = server.uri();
        let err = tokio::task::spawn_blocking(move || {
            let client = BlockingClient::builder(base_url)
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
            client.dashboards().save(&request)
        })
        .await
        .expect("join blocking task")
        .expect_err("expected non-retryable error for POST");
        assert_eq!(err.status(), Some(StatusCode::SERVICE_UNAVAILABLE));
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

        let base_url = server.uri();
        let value: serde_json::Value = tokio::task::spawn_blocking(move || {
            let client = BlockingClient::builder(base_url)
                .expect("client builder")
                .build()
                .expect("client build");
            client
                .openapi()
                .get_user_by_login_or_email("alice@example.com")
        })
        .await
        .expect("join blocking task")
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

        let base_url = server.uri();
        let value: serde_json::Value = tokio::task::spawn_blocking(move || {
            let client = BlockingClient::builder(base_url)
                .expect("client builder")
                .build()
                .expect("client build");
            client
                .openapi()
                .route_convert_prometheus_post_rule_group("ns1", Some(yaml))
        })
        .await
        .expect("join blocking task")
        .expect("openapi yaml body call");
        assert_eq!(
            value.get("message").and_then(|v| v.as_str()),
            Some("accepted")
        );
    });
}
