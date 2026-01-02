#![cfg(feature = "async")]

use std::time::Duration;

use grafana::{Auth, Client};

fn run_async(test: impl std::future::Future<Output = ()>) {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime");
    runtime.block_on(test);
}

fn contract_client() -> Option<Client> {
    let base_url = std::env::var("GRAFANA_BASE_URL").ok()?;

    let auth = if let Ok(token) = std::env::var("GRAFANA_TOKEN") {
        Auth::bearer(token)
    } else {
        let username = std::env::var("GRAFANA_USERNAME").ok()?;
        let password = std::env::var("GRAFANA_PASSWORD").ok()?;
        Auth::basic(username, password)
    };

    let client = Client::builder(base_url)
        .ok()?
        .auth(auth)
        .timeout(Duration::from_secs(30))
        .connect_timeout(Duration::from_secs(10))
        .max_retries(3)
        .build()
        .ok()?;

    Some(client)
}

async fn wait_for_grafana(client: &Client) {
    for _ in 0..60 {
        if client.health().get().await.is_ok() {
            return;
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    panic!("Grafana did not become ready in time");
}

#[test]
#[ignore]
fn contract_health_and_profile() {
    run_async(async {
        let Some(client) = contract_client() else {
            return;
        };

        wait_for_grafana(&client).await;

        let health = client.health().get().await.expect("health call");
        assert!(health.database.is_some());
        assert!(health.version.is_some());

        let profile = client.user().get_profile().await.expect("user profile");
        assert!(!profile.login.as_deref().unwrap_or_default().is_empty());
    });
}

#[test]
#[ignore]
fn contract_folder_roundtrip() {
    run_async(async {
        let Some(client) = contract_client() else {
            return;
        };

        wait_for_grafana(&client).await;

        let title = format!("grafana-sdk-contract-{}", fastrand::u64(0..=u64::MAX));
        let created = client
            .folders()
            .create(&grafana::types::CreateFolderRequest::new(title.clone()))
            .await
            .expect("create folder");

        let fetched = client
            .folders()
            .get_by_uid(created.uid.as_str())
            .await
            .expect("get folder");
        assert_eq!(fetched.uid, created.uid);

        let updated_title = format!("{title}-updated");
        let updated = client
            .folders()
            .update(
                created.uid.as_str(),
                &grafana::types::UpdateFolderRequest::new(updated_title.clone()),
            )
            .await
            .expect("update folder");
        assert_eq!(updated.title, updated_title);

        let deleted = client
            .folders()
            .delete_by_uid(created.uid.as_str())
            .await
            .expect("delete folder");
        assert!(deleted.message.is_some());
    });
}

#[test]
#[ignore]
fn contract_openapi_smoke() {
    run_async(async {
        let Some(client) = contract_client() else {
            return;
        };

        wait_for_grafana(&client).await;

        let typed: grafana::types::UserProfile = client
            .openapi()
            .get_signed_in_user()
            .await
            .expect("openapi get_signed_in_user");

        let profile = client.user().get_profile().await.expect("user profile");
        assert_eq!(typed.login, profile.login);
    });
}
