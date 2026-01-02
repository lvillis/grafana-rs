//! Grafana API endpoint groups (service layer).

#[cfg(feature = "async")]
mod dashboards;
#[cfg(feature = "async")]
mod datasources;
#[cfg(feature = "async")]
mod folders;
#[cfg(feature = "async")]
mod health;
#[cfg(feature = "async")]
mod openapi;
#[cfg(feature = "async")]
mod org;
#[cfg(feature = "async")]
mod raw;
#[cfg(feature = "async")]
mod search;
#[cfg(feature = "async")]
mod service_accounts;
#[cfg(feature = "async")]
mod teams;
#[cfg(feature = "async")]
mod user;

#[cfg(feature = "blocking")]
mod dashboards_blocking;
#[cfg(feature = "blocking")]
mod datasources_blocking;
#[cfg(feature = "blocking")]
mod folders_blocking;
#[cfg(feature = "blocking")]
mod health_blocking;
#[cfg(feature = "blocking")]
mod openapi_blocking;
#[cfg(feature = "blocking")]
mod org_blocking;
#[cfg(feature = "blocking")]
mod raw_blocking;
#[cfg(feature = "blocking")]
mod search_blocking;
#[cfg(feature = "blocking")]
mod service_accounts_blocking;
#[cfg(feature = "blocking")]
mod teams_blocking;
#[cfg(feature = "blocking")]
mod user_blocking;

#[cfg(feature = "async")]
pub use dashboards::DashboardsService;
#[cfg(feature = "blocking")]
pub use dashboards_blocking::BlockingDashboardsService;

#[cfg(feature = "async")]
pub use datasources::DatasourcesService;
#[cfg(feature = "blocking")]
pub use datasources_blocking::BlockingDatasourcesService;

#[cfg(feature = "async")]
pub use folders::FoldersService;
#[cfg(feature = "blocking")]
pub use folders_blocking::BlockingFoldersService;

#[cfg(feature = "async")]
pub use health::HealthService;
#[cfg(feature = "blocking")]
pub use health_blocking::BlockingHealthService;

#[cfg(feature = "async")]
pub use openapi::OpenApi;
#[cfg(feature = "blocking")]
pub use openapi_blocking::BlockingOpenApi;

#[cfg(feature = "async")]
pub use org::OrgService;
#[cfg(feature = "blocking")]
pub use org_blocking::BlockingOrgService;

#[cfg(feature = "async")]
pub use raw::RawService;
#[cfg(feature = "blocking")]
pub use raw_blocking::BlockingRawService;

#[cfg(feature = "async")]
pub use search::SearchService;
#[cfg(feature = "blocking")]
pub use search_blocking::BlockingSearchService;

#[cfg(feature = "async")]
pub use service_accounts::ServiceAccountsService;
#[cfg(feature = "blocking")]
pub use service_accounts_blocking::BlockingServiceAccountsService;

#[cfg(feature = "async")]
pub use teams::TeamsService;
#[cfg(feature = "blocking")]
pub use teams_blocking::BlockingTeamsService;

#[cfg(feature = "async")]
pub use user::UserService;
#[cfg(feature = "blocking")]
pub use user_blocking::BlockingUserService;
