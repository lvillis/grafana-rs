use grafana::{Auth, Result};

#[cfg(all(feature = "blocking", not(feature = "async")))]
use grafana::BlockingClient;
#[cfg(feature = "async")]
use grafana::Client;

fn main() -> Result<()> {
    let base_url = std::env::var("GRAFANA_URL").expect("GRAFANA_URL is required");
    let token = std::env::var("GRAFANA_TOKEN").expect("GRAFANA_TOKEN is required");
    let dashboard_uid =
        std::env::var("GRAFANA_DASHBOARD_UID").expect("GRAFANA_DASHBOARD_UID is required");

    #[cfg(feature = "async")]
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime");

    #[cfg(feature = "async")]
    {
        runtime.block_on(async move {
            let client = Client::builder(base_url)?
                .auth(Auth::bearer(token))
                .build()?;

            let dashboard = client.dashboards().get_by_uid(dashboard_uid).await?;
            println!("{}", dashboard.dashboard);
            Ok(())
        })
    }

    #[cfg(all(feature = "blocking", not(feature = "async")))]
    {
        let client = BlockingClient::builder(base_url)?
            .auth(Auth::bearer(token))
            .build()?;

        let dashboard = client.dashboards().get_by_uid(dashboard_uid)?;
        println!("{}", dashboard.dashboard);
        Ok(())
    }
}
