use grafana::{Auth, Result};

#[cfg(all(feature = "blocking", not(feature = "async")))]
use grafana::BlockingClient;
#[cfg(feature = "async")]
use grafana::Client;

fn main() -> Result<()> {
    let base_url = std::env::var("GRAFANA_URL").expect("GRAFANA_URL is required");
    let token = std::env::var("GRAFANA_TOKEN").ok();

    #[cfg(feature = "async")]
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to build tokio runtime");

    #[cfg(feature = "async")]
    {
        runtime.block_on(async move {
            let mut builder =
                Client::builder(base_url)?.timeout(std::time::Duration::from_secs(30));
            if let Some(token) = token {
                builder = builder.auth(Auth::bearer(token));
            }

            let client = builder.build()?;
            let health = client.health().get().await?;
            println!("{health:?}");
            Ok(())
        })
    }

    #[cfg(all(feature = "blocking", not(feature = "async")))]
    {
        let mut builder =
            BlockingClient::builder(base_url)?.timeout(std::time::Duration::from_secs(30));
        if let Some(token) = token {
            builder = builder.auth(Auth::bearer(token));
        }

        let client = builder.build()?;
        let health = client.health().get()?;
        println!("{health:?}");
        Ok(())
    }
}
