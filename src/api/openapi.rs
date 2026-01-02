//! Full Grafana OpenAPI coverage (generated).
//!
//! Methods are generated from upstream Grafana `operationId`s.

#![allow(clippy::too_many_arguments)]

use http::Method;
use serde::{Serialize, de::DeserializeOwned};

use crate::types::openapi as oas;
use crate::{Client, ResponseBytes, Result};

#[derive(Clone)]
pub struct OpenApi {
    client: Client,
}

impl OpenApi {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// `DELETE /convert/api/prom/rules/{NamespaceTitle}`
    pub async fn route_convert_prometheus_cortex_delete_namespace<T>(
        &self,
        namespace_title: impl AsRef<str>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let namespace_title = namespace_title.as_ref();
        let segments = ["convert", "api", "prom", "rules", namespace_title];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `DELETE /convert/api/prom/rules/{NamespaceTitle}/{Group}`
    pub async fn route_convert_prometheus_cortex_delete_rule_group<T>(
        &self,
        namespace_title: impl AsRef<str>,
        group: impl AsRef<str>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let namespace_title = namespace_title.as_ref();
        let group = group.as_ref();
        let segments = ["convert", "api", "prom", "rules", namespace_title, group];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `GET /convert/api/prom/rules/{NamespaceTitle}`
    pub async fn route_convert_prometheus_cortex_get_namespace(
        &self,
        namespace_title: impl AsRef<str>,
    ) -> Result<ResponseBytes> {
        let namespace_title = namespace_title.as_ref();
        let segments = ["convert", "api", "prom", "rules", namespace_title];
        self.client
            .request_bytes::<(), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /convert/api/prom/rules/{NamespaceTitle}/{Group}`
    pub async fn route_convert_prometheus_cortex_get_rule_group(
        &self,
        namespace_title: impl AsRef<str>,
        group: impl AsRef<str>,
    ) -> Result<ResponseBytes> {
        let namespace_title = namespace_title.as_ref();
        let group = group.as_ref();
        let segments = ["convert", "api", "prom", "rules", namespace_title, group];
        self.client
            .request_bytes::<(), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /convert/api/prom/rules`
    pub async fn route_convert_prometheus_cortex_get_rules(&self) -> Result<ResponseBytes> {
        let segments = ["convert", "api", "prom", "rules"];
        self.client
            .request_bytes::<(), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `POST /convert/api/prom/rules/{NamespaceTitle}`
    pub async fn route_convert_prometheus_cortex_post_rule_group<T>(
        &self,
        namespace_title: impl AsRef<str>,
        body_yaml: Option<&str>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let namespace_title = namespace_title.as_ref();
        let segments = ["convert", "api", "prom", "rules", namespace_title];
        match body_yaml {
            Some(body_yaml) => {
                self.client
                    .request_json_text::<T, ()>(
                        Method::POST,
                        &segments,
                        None,
                        body_yaml,
                        "application/yaml",
                    )
                    .await
            }
            None => {
                self.client
                    .request_json::<T, (), ()>(Method::POST, &segments, None, None)
                    .await
            }
        }
    }

    /// `POST /convert/api/prom/rules`
    pub async fn route_convert_prometheus_cortex_post_rule_groups<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["convert", "api", "prom", "rules"];
        self.client
            .request_json::<T, (), ()>(Method::POST, &segments, None, None)
            .await
    }

    /// `DELETE /convert/prometheus/config/v1/rules/{NamespaceTitle}`
    pub async fn route_convert_prometheus_delete_namespace<T>(
        &self,
        namespace_title: impl AsRef<str>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let namespace_title = namespace_title.as_ref();
        let segments = [
            "convert",
            "prometheus",
            "config",
            "v1",
            "rules",
            namespace_title,
        ];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `DELETE /convert/prometheus/config/v1/rules/{NamespaceTitle}/{Group}`
    pub async fn route_convert_prometheus_delete_rule_group<T>(
        &self,
        namespace_title: impl AsRef<str>,
        group: impl AsRef<str>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let namespace_title = namespace_title.as_ref();
        let group = group.as_ref();
        let segments = [
            "convert",
            "prometheus",
            "config",
            "v1",
            "rules",
            namespace_title,
            group,
        ];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `GET /convert/prometheus/config/v1/rules/{NamespaceTitle}`
    pub async fn route_convert_prometheus_get_namespace(
        &self,
        namespace_title: impl AsRef<str>,
    ) -> Result<ResponseBytes> {
        let namespace_title = namespace_title.as_ref();
        let segments = [
            "convert",
            "prometheus",
            "config",
            "v1",
            "rules",
            namespace_title,
        ];
        self.client
            .request_bytes::<(), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /convert/prometheus/config/v1/rules/{NamespaceTitle}/{Group}`
    pub async fn route_convert_prometheus_get_rule_group(
        &self,
        namespace_title: impl AsRef<str>,
        group: impl AsRef<str>,
    ) -> Result<ResponseBytes> {
        let namespace_title = namespace_title.as_ref();
        let group = group.as_ref();
        let segments = [
            "convert",
            "prometheus",
            "config",
            "v1",
            "rules",
            namespace_title,
            group,
        ];
        self.client
            .request_bytes::<(), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /convert/prometheus/config/v1/rules`
    pub async fn route_convert_prometheus_get_rules(&self) -> Result<ResponseBytes> {
        let segments = ["convert", "prometheus", "config", "v1", "rules"];
        self.client
            .request_bytes::<(), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `POST /convert/prometheus/config/v1/rules/{NamespaceTitle}`
    pub async fn route_convert_prometheus_post_rule_group<T>(
        &self,
        namespace_title: impl AsRef<str>,
        body_yaml: Option<&str>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let namespace_title = namespace_title.as_ref();
        let segments = [
            "convert",
            "prometheus",
            "config",
            "v1",
            "rules",
            namespace_title,
        ];
        match body_yaml {
            Some(body_yaml) => {
                self.client
                    .request_json_text::<T, ()>(
                        Method::POST,
                        &segments,
                        None,
                        body_yaml,
                        "application/yaml",
                    )
                    .await
            }
            None => {
                self.client
                    .request_json::<T, (), ()>(Method::POST, &segments, None, None)
                    .await
            }
        }
    }

    /// `POST /convert/prometheus/config/v1/rules`
    pub async fn route_convert_prometheus_post_rule_groups<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["convert", "prometheus", "config", "v1", "rules"];
        self.client
            .request_json::<T, (), ()>(Method::POST, &segments, None, None)
            .await
    }

    /// `DELETE /v1/provisioning/alert-rules/{UID}`
    pub async fn route_delete_alert_rule(&self, uid: impl AsRef<str>) -> Result<()> {
        let uid = uid.as_ref();
        let segments = ["v1", "provisioning", "alert-rules", uid];
        let _ = self
            .client
            .request_bytes::<(), ()>(Method::DELETE, &segments, None, None)
            .await?;
        Ok(())
    }

    /// `DELETE /v1/provisioning/folder/{FolderUID}/rule-groups/{Group}`
    pub async fn route_delete_alert_rule_group(
        &self,
        folder_uid: impl AsRef<str>,
        group: impl AsRef<str>,
    ) -> Result<()> {
        let folder_uid = folder_uid.as_ref();
        let group = group.as_ref();
        let segments = [
            "v1",
            "provisioning",
            "folder",
            folder_uid,
            "rule-groups",
            group,
        ];
        let _ = self
            .client
            .request_bytes::<(), ()>(Method::DELETE, &segments, None, None)
            .await?;
        Ok(())
    }

    /// `DELETE /v1/provisioning/contact-points/{UID}`
    pub async fn route_delete_contactpoints(&self, uid: impl AsRef<str>) -> Result<()> {
        let uid = uid.as_ref();
        let segments = ["v1", "provisioning", "contact-points", uid];
        let _ = self
            .client
            .request_bytes::<(), ()>(Method::DELETE, &segments, None, None)
            .await?;
        Ok(())
    }

    /// `DELETE /v1/provisioning/mute-timings/{name}`
    pub async fn route_delete_mute_timing(
        &self,
        name: impl AsRef<str>,
        query: Option<&oas::RouteDeleteMuteTimingQuery>,
    ) -> Result<()> {
        let name = name.as_ref();
        let segments = ["v1", "provisioning", "mute-timings", name];
        let _ = self
            .client
            .request_bytes::<oas::RouteDeleteMuteTimingQuery, ()>(
                Method::DELETE,
                &segments,
                query,
                None,
            )
            .await?;
        Ok(())
    }

    /// `DELETE /v1/provisioning/templates/{name}`
    pub async fn route_delete_template(
        &self,
        name: impl AsRef<str>,
        query: Option<&oas::RouteDeleteTemplateQuery>,
    ) -> Result<()> {
        let name = name.as_ref();
        let segments = ["v1", "provisioning", "templates", name];
        let _ = self
            .client
            .request_bytes::<oas::RouteDeleteTemplateQuery, ()>(
                Method::DELETE,
                &segments,
                query,
                None,
            )
            .await?;
        Ok(())
    }

    /// `GET /v1/provisioning/mute-timings/{name}/export`
    pub async fn route_export_mute_timing<T>(
        &self,
        name: impl AsRef<str>,
        query: Option<&oas::RouteExportMuteTimingQuery>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let name = name.as_ref();
        let segments = ["v1", "provisioning", "mute-timings", name, "export"];
        self.client
            .request_json::<T, oas::RouteExportMuteTimingQuery, ()>(
                Method::GET,
                &segments,
                query,
                None,
            )
            .await
    }

    /// `GET /v1/provisioning/mute-timings/export`
    pub async fn route_export_mute_timings<T>(
        &self,
        query: Option<&oas::RouteExportMuteTimingsQuery>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["v1", "provisioning", "mute-timings", "export"];
        self.client
            .request_json::<T, oas::RouteExportMuteTimingsQuery, ()>(
                Method::GET,
                &segments,
                query,
                None,
            )
            .await
    }

    /// `GET /v1/provisioning/alert-rules/{UID}`
    pub async fn route_get_alert_rule<T>(&self, uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uid = uid.as_ref();
        let segments = ["v1", "provisioning", "alert-rules", uid];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /v1/provisioning/alert-rules/{UID}/export`
    pub async fn route_get_alert_rule_export<T>(
        &self,
        uid: impl AsRef<str>,
        query: Option<&oas::RouteGetAlertRuleExportQuery>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uid = uid.as_ref();
        let segments = ["v1", "provisioning", "alert-rules", uid, "export"];
        self.client
            .request_json::<T, oas::RouteGetAlertRuleExportQuery, ()>(
                Method::GET,
                &segments,
                query,
                None,
            )
            .await
    }

    /// `GET /v1/provisioning/folder/{FolderUID}/rule-groups/{Group}`
    pub async fn route_get_alert_rule_group<T>(
        &self,
        folder_uid: impl AsRef<str>,
        group: impl AsRef<str>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let folder_uid = folder_uid.as_ref();
        let group = group.as_ref();
        let segments = [
            "v1",
            "provisioning",
            "folder",
            folder_uid,
            "rule-groups",
            group,
        ];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /v1/provisioning/folder/{FolderUID}/rule-groups/{Group}/export`
    pub async fn route_get_alert_rule_group_export<T>(
        &self,
        folder_uid: impl AsRef<str>,
        group: impl AsRef<str>,
        query: Option<&oas::RouteGetAlertRuleGroupExportQuery>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let folder_uid = folder_uid.as_ref();
        let group = group.as_ref();
        let segments = [
            "v1",
            "provisioning",
            "folder",
            folder_uid,
            "rule-groups",
            group,
            "export",
        ];
        self.client
            .request_json::<T, oas::RouteGetAlertRuleGroupExportQuery, ()>(
                Method::GET,
                &segments,
                query,
                None,
            )
            .await
    }

    /// `GET /v1/provisioning/alert-rules`
    pub async fn route_get_alert_rules<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["v1", "provisioning", "alert-rules"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /v1/provisioning/alert-rules/export`
    pub async fn route_get_alert_rules_export<T>(
        &self,
        query: Option<&oas::RouteGetAlertRulesExportQuery>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["v1", "provisioning", "alert-rules", "export"];
        self.client
            .request_json::<T, oas::RouteGetAlertRulesExportQuery, ()>(
                Method::GET,
                &segments,
                query,
                None,
            )
            .await
    }

    /// `GET /v1/provisioning/contact-points`
    pub async fn route_get_contactpoints<T>(
        &self,
        query: Option<&oas::RouteGetContactpointsQuery>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["v1", "provisioning", "contact-points"];
        self.client
            .request_json::<T, oas::RouteGetContactpointsQuery, ()>(
                Method::GET,
                &segments,
                query,
                None,
            )
            .await
    }

    /// `GET /v1/provisioning/contact-points/export`
    pub async fn route_get_contactpoints_export<T>(
        &self,
        query: Option<&oas::RouteGetContactpointsExportQuery>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["v1", "provisioning", "contact-points", "export"];
        self.client
            .request_json::<T, oas::RouteGetContactpointsExportQuery, ()>(
                Method::GET,
                &segments,
                query,
                None,
            )
            .await
    }

    /// `GET /v1/provisioning/mute-timings/{name}`
    pub async fn route_get_mute_timing<T>(&self, name: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let name = name.as_ref();
        let segments = ["v1", "provisioning", "mute-timings", name];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /v1/provisioning/mute-timings`
    pub async fn route_get_mute_timings<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["v1", "provisioning", "mute-timings"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /v1/provisioning/policies`
    pub async fn route_get_policy_tree<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["v1", "provisioning", "policies"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /v1/provisioning/policies/export`
    pub async fn route_get_policy_tree_export<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["v1", "provisioning", "policies", "export"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /v1/provisioning/templates/{name}`
    pub async fn route_get_template<T>(&self, name: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let name = name.as_ref();
        let segments = ["v1", "provisioning", "templates", name];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /v1/provisioning/templates`
    pub async fn route_get_templates<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["v1", "provisioning", "templates"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `POST /v1/provisioning/alert-rules`
    pub async fn route_post_alert_rule<T, B>(&self, body: Option<&B>) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["v1", "provisioning", "alert-rules"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, body)
            .await
    }

    /// `POST /v1/provisioning/contact-points`
    pub async fn route_post_contactpoints<T, B>(&self, body: Option<&B>) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["v1", "provisioning", "contact-points"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, body)
            .await
    }

    /// `POST /v1/provisioning/mute-timings`
    pub async fn route_post_mute_timing<T, B>(&self, body: Option<&B>) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["v1", "provisioning", "mute-timings"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, body)
            .await
    }

    /// `PUT /v1/provisioning/alert-rules/{UID}`
    pub async fn route_put_alert_rule<T, B>(
        &self,
        uid: impl AsRef<str>,
        body: Option<&B>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let uid = uid.as_ref();
        let segments = ["v1", "provisioning", "alert-rules", uid];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, body)
            .await
    }

    /// `PUT /v1/provisioning/folder/{FolderUID}/rule-groups/{Group}`
    pub async fn route_put_alert_rule_group<T, B>(
        &self,
        folder_uid: impl AsRef<str>,
        group: impl AsRef<str>,
        body: Option<&B>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let folder_uid = folder_uid.as_ref();
        let group = group.as_ref();
        let segments = [
            "v1",
            "provisioning",
            "folder",
            folder_uid,
            "rule-groups",
            group,
        ];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, body)
            .await
    }

    /// `PUT /v1/provisioning/contact-points/{UID}`
    pub async fn route_put_contactpoint<T, B>(
        &self,
        uid: impl AsRef<str>,
        body: Option<&B>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let uid = uid.as_ref();
        let segments = ["v1", "provisioning", "contact-points", uid];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, body)
            .await
    }

    /// `PUT /v1/provisioning/mute-timings/{name}`
    pub async fn route_put_mute_timing<T, B>(
        &self,
        name: impl AsRef<str>,
        body: Option<&B>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let name = name.as_ref();
        let segments = ["v1", "provisioning", "mute-timings", name];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, body)
            .await
    }

    /// `PUT /v1/provisioning/policies`
    pub async fn route_put_policy_tree<T, B>(&self, body: Option<&B>) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["v1", "provisioning", "policies"];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, body)
            .await
    }

    /// `PUT /v1/provisioning/templates/{name}`
    pub async fn route_put_template<T, B>(
        &self,
        name: impl AsRef<str>,
        body: Option<&B>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let name = name.as_ref();
        let segments = ["v1", "provisioning", "templates", name];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, body)
            .await
    }

    /// `DELETE /v1/provisioning/policies`
    pub async fn route_reset_policy_tree<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["v1", "provisioning", "policies"];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `GET /anonymous/search`
    pub async fn search_devices<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["anonymous", "search"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `POST /datasources`
    pub async fn add_data_source<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["datasources"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /org/invites`
    pub async fn add_org_invite<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["org", "invites"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /orgs/{org_id}/users`
    pub async fn add_org_user<T, B>(&self, org_id: i64, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let org_id_str = org_id.to_string();
        let segments = ["orgs", org_id_str.as_str(), "users"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /org/users`
    pub async fn add_org_user_to_current_org<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["org", "users"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /teams/{teamId}/groups`
    pub async fn add_team_group_api<T, B>(&self, team_id: impl AsRef<str>, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let team_id = team_id.as_ref();
        let segments = ["teams", team_id, "groups"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /teams/{team_id}/members`
    pub async fn add_team_member<T, B>(&self, team_id: impl AsRef<str>, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let team_id = team_id.as_ref();
        let segments = ["teams", team_id, "members"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /access-control/teams/{teamId}/roles`
    pub async fn add_team_role<T, B>(&self, team_id: i64, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let team_id_str = team_id.to_string();
        let segments = ["access-control", "teams", team_id_str.as_str(), "roles"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /access-control/users/{userId}/roles`
    pub async fn add_user_role<T, B>(&self, user_id: i64, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let user_id_str = user_id.to_string();
        let segments = ["access-control", "users", user_id_str.as_str(), "roles"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /admin/users`
    pub async fn admin_create_user<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["admin", "users"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `DELETE /admin/users/{user_id}`
    pub async fn admin_delete_user<T>(&self, user_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let user_id_str = user_id.to_string();
        let segments = ["admin", "users", user_id_str.as_str()];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `POST /admin/users/{user_id}/disable`
    pub async fn admin_disable_user<T>(&self, user_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let user_id_str = user_id.to_string();
        let segments = ["admin", "users", user_id_str.as_str(), "disable"];
        self.client
            .request_json::<T, (), ()>(Method::POST, &segments, None, None)
            .await
    }

    /// `POST /admin/users/{user_id}/enable`
    pub async fn admin_enable_user<T>(&self, user_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let user_id_str = user_id.to_string();
        let segments = ["admin", "users", user_id_str.as_str(), "enable"];
        self.client
            .request_json::<T, (), ()>(Method::POST, &segments, None, None)
            .await
    }

    /// `GET /admin/settings`
    pub async fn admin_get_settings<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["admin", "settings"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /admin/stats`
    pub async fn admin_get_stats<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["admin", "stats"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /admin/users/{user_id}/auth-tokens`
    pub async fn admin_get_user_auth_tokens<T>(&self, user_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let user_id_str = user_id.to_string();
        let segments = ["admin", "users", user_id_str.as_str(), "auth-tokens"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `POST /admin/users/{user_id}/logout`
    pub async fn admin_logout_user<T>(&self, user_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let user_id_str = user_id.to_string();
        let segments = ["admin", "users", user_id_str.as_str(), "logout"];
        self.client
            .request_json::<T, (), ()>(Method::POST, &segments, None, None)
            .await
    }

    /// `POST /admin/provisioning/access-control/reload`
    pub async fn admin_provisioning_reload_access_control<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["admin", "provisioning", "access-control", "reload"];
        self.client
            .request_json::<T, (), ()>(Method::POST, &segments, None, None)
            .await
    }

    /// `POST /admin/provisioning/dashboards/reload`
    pub async fn admin_provisioning_reload_dashboards<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["admin", "provisioning", "dashboards", "reload"];
        self.client
            .request_json::<T, (), ()>(Method::POST, &segments, None, None)
            .await
    }

    /// `POST /admin/provisioning/datasources/reload`
    pub async fn admin_provisioning_reload_datasources<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["admin", "provisioning", "datasources", "reload"];
        self.client
            .request_json::<T, (), ()>(Method::POST, &segments, None, None)
            .await
    }

    /// `POST /admin/provisioning/plugins/reload`
    pub async fn admin_provisioning_reload_plugins<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["admin", "provisioning", "plugins", "reload"];
        self.client
            .request_json::<T, (), ()>(Method::POST, &segments, None, None)
            .await
    }

    /// `POST /admin/users/{user_id}/revoke-auth-token`
    pub async fn admin_revoke_user_auth_token<T, B>(&self, user_id: i64, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let user_id_str = user_id.to_string();
        let segments = ["admin", "users", user_id_str.as_str(), "revoke-auth-token"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `PUT /admin/users/{user_id}/password`
    pub async fn admin_update_user_password<T, B>(&self, user_id: i64, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let user_id_str = user_id.to_string();
        let segments = ["admin", "users", user_id_str.as_str(), "password"];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `PUT /admin/users/{user_id}/permissions`
    pub async fn admin_update_user_permissions<T, B>(&self, user_id: i64, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let user_id_str = user_id.to_string();
        let segments = ["admin", "users", user_id_str.as_str(), "permissions"];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `GET /datasources/{id}/resources/{datasource_proxy_route}`
    pub async fn call_datasource_resource_by_id<T>(
        &self,
        datasource_proxy_route: impl AsRef<str>,
        id: impl AsRef<str>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let datasource_proxy_route = datasource_proxy_route.as_ref();
        let id = id.as_ref();
        let segments = ["datasources", id, "resources", datasource_proxy_route];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /datasources/uid/{uid}/resources/{datasource_proxy_route}`
    pub async fn call_datasource_resource_with_uid<T>(
        &self,
        datasource_proxy_route: impl AsRef<str>,
        uid: impl AsRef<str>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let datasource_proxy_route = datasource_proxy_route.as_ref();
        let uid = uid.as_ref();
        let segments = [
            "datasources",
            "uid",
            uid,
            "resources",
            datasource_proxy_route,
        ];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `POST /cloudmigration/migration/{uid}/snapshot/{snapshotUid}/cancel`
    pub async fn cancel_snapshot(
        &self,
        uid: impl AsRef<str>,
        snapshot_uid: impl AsRef<str>,
    ) -> Result<()> {
        let uid = uid.as_ref();
        let snapshot_uid = snapshot_uid.as_ref();
        let segments = [
            "cloudmigration",
            "migration",
            uid,
            "snapshot",
            snapshot_uid,
            "cancel",
        ];
        let _ = self
            .client
            .request_bytes::<(), ()>(Method::POST, &segments, None, None)
            .await?;
        Ok(())
    }

    /// `PUT /user/password`
    pub async fn change_user_password<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["user", "password"];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `GET /datasources/{id}/health`
    pub async fn check_datasource_health_by_id<T>(&self, id: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let id = id.as_ref();
        let segments = ["datasources", id, "health"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /datasources/uid/{uid}/health`
    pub async fn check_datasource_health_with_uid<T>(&self, uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uid = uid.as_ref();
        let segments = ["datasources", "uid", uid, "health"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `POST /datasources/{dataSourceUID}/cache/clean`
    pub async fn clean_data_source_cache<T>(&self, data_source_uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let data_source_uid = data_source_uid.as_ref();
        let segments = ["datasources", data_source_uid, "cache", "clean"];
        self.client
            .request_json::<T, (), ()>(Method::POST, &segments, None, None)
            .await
    }

    /// `GET /user/helpflags/clear`
    pub async fn clear_help_flags<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["user", "helpflags", "clear"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `POST /cloudmigration/token`
    pub async fn create_cloud_migration_token<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["cloudmigration", "token"];
        self.client
            .request_json::<T, (), ()>(Method::POST, &segments, None, None)
            .await
    }

    /// `POST /datasources/uid/{sourceUID}/correlations`
    pub async fn create_correlation<T, B>(&self, source_uid: impl AsRef<str>, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let source_uid = source_uid.as_ref();
        let segments = ["datasources", "uid", source_uid, "correlations"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /snapshots`
    pub async fn create_dashboard_snapshot<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["snapshots"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /folders`
    pub async fn create_folder<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["folders"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /groupsync/groups/{group_id}`
    pub async fn create_group_mappings<T, B>(
        &self,
        group_id: impl AsRef<str>,
        body: &B,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let group_id = group_id.as_ref();
        let segments = ["groupsync", "groups", group_id];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /library-elements`
    pub async fn create_library_element<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["library-elements"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /orgs`
    pub async fn create_org<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["orgs"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /playlists`
    pub async fn create_playlist<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["playlists"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /dashboards/uid/{dashboardUid}/public-dashboards`
    pub async fn create_public_dashboard<T, B>(
        &self,
        dashboard_uid: impl AsRef<str>,
        body: &B,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let dashboard_uid = dashboard_uid.as_ref();
        let segments = ["dashboards", "uid", dashboard_uid, "public-dashboards"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /query-history`
    pub async fn create_query<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["query-history"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /recording-rules`
    pub async fn create_recording_rule<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["recording-rules"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /recording-rules/writer`
    pub async fn create_recording_rule_write_target<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["recording-rules", "writer"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /reports`
    pub async fn create_report<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["reports"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /access-control/roles`
    pub async fn create_role<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["access-control", "roles"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /serviceaccounts`
    pub async fn create_service_account<T, B>(&self, body: Option<&B>) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["serviceaccounts"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, body)
            .await
    }

    /// `POST /cloudmigration/migration`
    pub async fn create_session<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["cloudmigration", "migration"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /cloudmigration/migration/{uid}/snapshot`
    pub async fn create_snapshot<T, B>(&self, uid: impl AsRef<str>, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let uid = uid.as_ref();
        let segments = ["cloudmigration", "migration", uid, "snapshot"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /teams`
    pub async fn create_team<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["teams"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /serviceaccounts/{serviceAccountId}/tokens`
    pub async fn create_token<T, B>(&self, service_account_id: i64, body: Option<&B>) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let service_account_id_str = service_account_id.to_string();
        let segments = ["serviceaccounts", service_account_id_str.as_str(), "tokens"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, body)
            .await
    }

    /// `DELETE /datasources/proxy/uid/{uid}/{datasource_proxy_route}`
    pub async fn datasource_proxy_delete_by_ui_dcalls(
        &self,
        uid: impl AsRef<str>,
        datasource_proxy_route: impl AsRef<str>,
    ) -> Result<()> {
        let uid = uid.as_ref();
        let datasource_proxy_route = datasource_proxy_route.as_ref();
        let segments = ["datasources", "proxy", "uid", uid, datasource_proxy_route];
        let _ = self
            .client
            .request_bytes::<(), ()>(Method::DELETE, &segments, None, None)
            .await?;
        Ok(())
    }

    /// `DELETE /datasources/proxy/{id}/{datasource_proxy_route}`
    pub async fn datasource_proxy_delet_ecalls(
        &self,
        id: impl AsRef<str>,
        datasource_proxy_route: impl AsRef<str>,
    ) -> Result<()> {
        let id = id.as_ref();
        let datasource_proxy_route = datasource_proxy_route.as_ref();
        let segments = ["datasources", "proxy", id, datasource_proxy_route];
        let _ = self
            .client
            .request_bytes::<(), ()>(Method::DELETE, &segments, None, None)
            .await?;
        Ok(())
    }

    /// `GET /datasources/proxy/uid/{uid}/{datasource_proxy_route}`
    pub async fn datasource_proxy_get_by_ui_dcalls(
        &self,
        datasource_proxy_route: impl AsRef<str>,
        uid: impl AsRef<str>,
    ) -> Result<()> {
        let datasource_proxy_route = datasource_proxy_route.as_ref();
        let uid = uid.as_ref();
        let segments = ["datasources", "proxy", "uid", uid, datasource_proxy_route];
        let _ = self
            .client
            .request_bytes::<(), ()>(Method::GET, &segments, None, None)
            .await?;
        Ok(())
    }

    /// `GET /datasources/proxy/{id}/{datasource_proxy_route}`
    pub async fn datasource_proxy_ge_tcalls(
        &self,
        datasource_proxy_route: impl AsRef<str>,
        id: impl AsRef<str>,
    ) -> Result<()> {
        let datasource_proxy_route = datasource_proxy_route.as_ref();
        let id = id.as_ref();
        let segments = ["datasources", "proxy", id, datasource_proxy_route];
        let _ = self
            .client
            .request_bytes::<(), ()>(Method::GET, &segments, None, None)
            .await?;
        Ok(())
    }

    /// `POST /datasources/proxy/uid/{uid}/{datasource_proxy_route}`
    pub async fn datasource_proxy_post_by_ui_dcalls<B>(
        &self,
        datasource_proxy_route: impl AsRef<str>,
        uid: impl AsRef<str>,
        body: &B,
    ) -> Result<()>
    where
        B: Serialize + ?Sized,
    {
        let datasource_proxy_route = datasource_proxy_route.as_ref();
        let uid = uid.as_ref();
        let segments = ["datasources", "proxy", "uid", uid, datasource_proxy_route];
        let _ = self
            .client
            .request_bytes::<(), B>(Method::POST, &segments, None, Some(body))
            .await?;
        Ok(())
    }

    /// `POST /datasources/proxy/{id}/{datasource_proxy_route}`
    pub async fn datasource_proxy_pos_tcalls<B>(
        &self,
        datasource_proxy_route: impl AsRef<str>,
        id: impl AsRef<str>,
        body: &B,
    ) -> Result<()>
    where
        B: Serialize + ?Sized,
    {
        let datasource_proxy_route = datasource_proxy_route.as_ref();
        let id = id.as_ref();
        let segments = ["datasources", "proxy", id, datasource_proxy_route];
        let _ = self
            .client
            .request_bytes::<(), B>(Method::POST, &segments, None, Some(body))
            .await?;
        Ok(())
    }

    /// `DELETE /annotations/{annotation_id}`
    pub async fn delete_annotation_by_id<T>(&self, annotation_id: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let annotation_id = annotation_id.as_ref();
        let segments = ["annotations", annotation_id];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `DELETE /cloudmigration/token/{uid}`
    pub async fn delete_cloud_migration_token(&self, uid: impl AsRef<str>) -> Result<()> {
        let uid = uid.as_ref();
        let segments = ["cloudmigration", "token", uid];
        let _ = self
            .client
            .request_bytes::<(), ()>(Method::DELETE, &segments, None, None)
            .await?;
        Ok(())
    }

    /// `DELETE /datasources/uid/{uid}/correlations/{correlationUID}`
    pub async fn delete_correlation<T>(
        &self,
        uid: impl AsRef<str>,
        correlation_uid: impl AsRef<str>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uid = uid.as_ref();
        let correlation_uid = correlation_uid.as_ref();
        let segments = ["datasources", "uid", uid, "correlations", correlation_uid];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `DELETE /dashboards/uid/{uid}`
    pub async fn delete_dashboard_by_uid<T>(&self, uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uid = uid.as_ref();
        let segments = ["dashboards", "uid", uid];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `DELETE /snapshots/{key}`
    pub async fn delete_dashboard_snapshot<T>(&self, key: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let key = key.as_ref();
        let segments = ["snapshots", key];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `GET /snapshots-delete/{deleteKey}`
    pub async fn delete_dashboard_snapshot_by_delete_key<T>(
        &self,
        delete_key: impl AsRef<str>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let delete_key = delete_key.as_ref();
        let segments = ["snapshots-delete", delete_key];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `DELETE /datasources/{id}`
    pub async fn delete_data_source_by_id<T>(&self, id: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let id = id.as_ref();
        let segments = ["datasources", id];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `DELETE /datasources/name/{name}`
    pub async fn delete_data_source_by_name<T>(&self, name: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let name = name.as_ref();
        let segments = ["datasources", "name", name];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `DELETE /datasources/uid/{uid}`
    pub async fn delete_data_source_by_uid<T>(&self, uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uid = uid.as_ref();
        let segments = ["datasources", "uid", uid];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `DELETE /folders/{folder_uid}`
    pub async fn delete_folder<T>(
        &self,
        folder_uid: impl AsRef<str>,
        query: Option<&oas::DeleteFolderQuery>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let folder_uid = folder_uid.as_ref();
        let segments = ["folders", folder_uid];
        self.client
            .request_json::<T, oas::DeleteFolderQuery, ()>(Method::DELETE, &segments, query, None)
            .await
    }

    /// `DELETE /groupsync/groups/{group_id}`
    pub async fn delete_group_mappings<T>(&self, group_id: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let group_id = group_id.as_ref();
        let segments = ["groupsync", "groups", group_id];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `DELETE /library-elements/{library_element_uid}`
    pub async fn delete_library_element_by_uid<T>(
        &self,
        library_element_uid: impl AsRef<str>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let library_element_uid = library_element_uid.as_ref();
        let segments = ["library-elements", library_element_uid];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `DELETE /licensing/token`
    pub async fn delete_license_token<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["licensing", "token"];
        self.client
            .request_json::<T, (), B>(Method::DELETE, &segments, None, Some(body))
            .await
    }

    /// `DELETE /orgs/{org_id}`
    pub async fn delete_org_by_id<T>(&self, org_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let org_id_str = org_id.to_string();
        let segments = ["orgs", org_id_str.as_str()];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `DELETE /playlists/{uid}`
    pub async fn delete_playlist<T>(&self, uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uid = uid.as_ref();
        let segments = ["playlists", uid];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `DELETE /dashboards/uid/{dashboardUid}/public-dashboards/{uid}`
    pub async fn delete_public_dashboard<T>(
        &self,
        dashboard_uid: impl AsRef<str>,
        uid: impl AsRef<str>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let dashboard_uid = dashboard_uid.as_ref();
        let uid = uid.as_ref();
        let segments = ["dashboards", "uid", dashboard_uid, "public-dashboards", uid];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `DELETE /query-history/{query_history_uid}`
    pub async fn delete_query<T>(&self, query_history_uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let query_history_uid = query_history_uid.as_ref();
        let segments = ["query-history", query_history_uid];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `DELETE /recording-rules/{recordingRuleID}`
    pub async fn delete_recording_rule<T>(&self, recording_rule_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let recording_rule_id_str = recording_rule_id.to_string();
        let segments = ["recording-rules", recording_rule_id_str.as_str()];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `DELETE /recording-rules/writer`
    pub async fn delete_recording_rule_write_target<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["recording-rules", "writer"];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `DELETE /reports/{id}`
    pub async fn delete_report<T>(&self, id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let id_str = id.to_string();
        let segments = ["reports", id_str.as_str()];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `DELETE /access-control/roles/{roleUID}`
    pub async fn delete_role<T>(
        &self,
        role_uid: impl AsRef<str>,
        query: Option<&oas::DeleteRoleQuery>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let role_uid = role_uid.as_ref();
        let segments = ["access-control", "roles", role_uid];
        self.client
            .request_json::<T, oas::DeleteRoleQuery, ()>(Method::DELETE, &segments, query, None)
            .await
    }

    /// `DELETE /serviceaccounts/{serviceAccountId}`
    pub async fn delete_service_account<T>(&self, service_account_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let service_account_id_str = service_account_id.to_string();
        let segments = ["serviceaccounts", service_account_id_str.as_str()];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `DELETE /cloudmigration/migration/{uid}`
    pub async fn delete_session<T>(&self, uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uid = uid.as_ref();
        let segments = ["cloudmigration", "migration", uid];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `DELETE /teams/{team_id}`
    pub async fn delete_team_by_id<T>(&self, team_id: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let team_id = team_id.as_ref();
        let segments = ["teams", team_id];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `DELETE /serviceaccounts/{serviceAccountId}/tokens/{tokenId}`
    pub async fn delete_token<T>(&self, token_id: i64, service_account_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let token_id_str = token_id.to_string();
        let service_account_id_str = service_account_id.to_string();
        let segments = [
            "serviceaccounts",
            service_account_id_str.as_str(),
            "tokens",
            token_id_str.as_str(),
        ];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `POST /datasources/{dataSourceUID}/cache/disable`
    pub async fn disable_data_source_cache<T>(&self, data_source_uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let data_source_uid = data_source_uid.as_ref();
        let segments = ["datasources", data_source_uid, "cache", "disable"];
        self.client
            .request_json::<T, (), ()>(Method::POST, &segments, None, None)
            .await
    }

    /// `POST /datasources/{dataSourceUID}/cache/enable`
    pub async fn enable_data_source_cache<T>(&self, data_source_uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let data_source_uid = data_source_uid.as_ref();
        let segments = ["datasources", data_source_uid, "cache", "enable"];
        self.client
            .request_json::<T, (), ()>(Method::POST, &segments, None, None)
            .await
    }

    /// `GET /access-control/status`
    pub async fn get_access_control_status<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["access-control", "status"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /annotations/{annotation_id}`
    pub async fn get_annotation_by_id<T>(&self, annotation_id: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let annotation_id = annotation_id.as_ref();
        let segments = ["annotations", annotation_id];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /annotations/tags`
    pub async fn get_annotation_tags<T>(
        &self,
        query: Option<&oas::GetAnnotationTagsQuery>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["annotations", "tags"];
        self.client
            .request_json::<T, oas::GetAnnotationTagsQuery, ()>(Method::GET, &segments, query, None)
            .await
    }

    /// `GET /annotations`
    pub async fn get_annotations<T>(&self, query: Option<&oas::GetAnnotationsQuery>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["annotations"];
        self.client
            .request_json::<T, oas::GetAnnotationsQuery, ()>(Method::GET, &segments, query, None)
            .await
    }

    /// `GET /cloudmigration/token`
    pub async fn get_cloud_migration_token<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["cloudmigration", "token"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /datasources/uid/{sourceUID}/correlations/{correlationUID}`
    pub async fn get_correlation<T>(
        &self,
        source_uid: impl AsRef<str>,
        correlation_uid: impl AsRef<str>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let source_uid = source_uid.as_ref();
        let correlation_uid = correlation_uid.as_ref();
        let segments = [
            "datasources",
            "uid",
            source_uid,
            "correlations",
            correlation_uid,
        ];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /datasources/correlations`
    pub async fn get_correlations<T>(&self, query: Option<&oas::GetCorrelationsQuery>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["datasources", "correlations"];
        self.client
            .request_json::<T, oas::GetCorrelationsQuery, ()>(Method::GET, &segments, query, None)
            .await
    }

    /// `GET /datasources/uid/{sourceUID}/correlations`
    pub async fn get_correlations_by_source_uid<T>(&self, source_uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let source_uid = source_uid.as_ref();
        let segments = ["datasources", "uid", source_uid, "correlations"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /org`
    pub async fn get_current_org<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["org"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /org/quotas`
    pub async fn get_current_org_quota<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["org", "quotas"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /licensing/custom-permissions-csv`
    pub async fn get_custom_permissions_csv<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["licensing", "custom-permissions-csv"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /licensing/custom-permissions`
    pub async fn get_custom_permissions_report<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["licensing", "custom-permissions"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /dashboards/uid/{uid}`
    pub async fn get_dashboard_by_uid<T>(&self, uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uid = uid.as_ref();
        let segments = ["dashboards", "uid", uid];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /dashboards/id/{DashboardID}/permissions`
    pub async fn get_dashboard_permissions_list_by_id<T>(&self, dashboard_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let dashboard_id_str = dashboard_id.to_string();
        let segments = ["dashboards", "id", dashboard_id_str.as_str(), "permissions"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /dashboards/uid/{uid}/permissions`
    pub async fn get_dashboard_permissions_list_by_uid<T>(&self, uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uid = uid.as_ref();
        let segments = ["dashboards", "uid", uid, "permissions"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /snapshots/{key}`
    pub async fn get_dashboard_snapshot(&self, key: impl AsRef<str>) -> Result<()> {
        let key = key.as_ref();
        let segments = ["snapshots", key];
        let _ = self
            .client
            .request_bytes::<(), ()>(Method::GET, &segments, None, None)
            .await?;
        Ok(())
    }

    /// `GET /dashboards/tags`
    pub async fn get_dashboard_tags<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["dashboards", "tags"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /dashboards/id/{DashboardID}/versions/{DashboardVersionID}`
    pub async fn get_dashboard_version_by_id<T>(
        &self,
        dashboard_id: i64,
        dashboard_version_id: i64,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let dashboard_id_str = dashboard_id.to_string();
        let dashboard_version_id_str = dashboard_version_id.to_string();
        let segments = [
            "dashboards",
            "id",
            dashboard_id_str.as_str(),
            "versions",
            dashboard_version_id_str.as_str(),
        ];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /dashboards/uid/{uid}/versions/{DashboardVersionID}`
    pub async fn get_dashboard_version_by_uid<T>(
        &self,
        dashboard_version_id: i64,
        uid: impl AsRef<str>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let dashboard_version_id_str = dashboard_version_id.to_string();
        let uid = uid.as_ref();
        let segments = [
            "dashboards",
            "uid",
            uid,
            "versions",
            dashboard_version_id_str.as_str(),
        ];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /dashboards/id/{DashboardID}/versions`
    pub async fn get_dashboard_versions_by_id<T>(&self, dashboard_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let dashboard_id_str = dashboard_id.to_string();
        let segments = ["dashboards", "id", dashboard_id_str.as_str(), "versions"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /dashboards/uid/{uid}/versions`
    pub async fn get_dashboard_versions_by_uid<T>(
        &self,
        uid: impl AsRef<str>,
        query: Option<&oas::GetDashboardVersionsByUidQuery>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uid = uid.as_ref();
        let segments = ["dashboards", "uid", uid, "versions"];
        self.client
            .request_json::<T, oas::GetDashboardVersionsByUidQuery, ()>(
                Method::GET,
                &segments,
                query,
                None,
            )
            .await
    }

    /// `GET /datasources/{id}`
    pub async fn get_data_source_by_id<T>(&self, id: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let id = id.as_ref();
        let segments = ["datasources", id];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /datasources/name/{name}`
    pub async fn get_data_source_by_name<T>(&self, name: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let name = name.as_ref();
        let segments = ["datasources", "name", name];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /datasources/uid/{uid}`
    pub async fn get_data_source_by_uid<T>(&self, uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uid = uid.as_ref();
        let segments = ["datasources", "uid", uid];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /datasources/{dataSourceUID}/cache`
    pub async fn get_data_source_cache_config<T>(
        &self,
        data_source_uid: impl AsRef<str>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let data_source_uid = data_source_uid.as_ref();
        let segments = ["datasources", data_source_uid, "cache"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /datasources/id/{name}`
    pub async fn get_data_source_id_by_name<T>(&self, name: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let name = name.as_ref();
        let segments = ["datasources", "id", name];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /datasources`
    pub async fn get_data_sources<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["datasources"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /folders/id/{folder_id}`
    pub async fn get_folder_by_id<T>(&self, folder_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let folder_id_str = folder_id.to_string();
        let segments = ["folders", "id", folder_id_str.as_str()];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /folders/{folder_uid}`
    pub async fn get_folder_by_uid<T>(&self, folder_uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let folder_uid = folder_uid.as_ref();
        let segments = ["folders", folder_uid];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /folders/{folder_uid}/counts`
    pub async fn get_folder_descendant_counts<T>(&self, folder_uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let folder_uid = folder_uid.as_ref();
        let segments = ["folders", folder_uid, "counts"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /folders/{folder_uid}/permissions`
    pub async fn get_folder_permission_list<T>(&self, folder_uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let folder_uid = folder_uid.as_ref();
        let segments = ["folders", folder_uid, "permissions"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /folders`
    pub async fn get_folders<T>(&self, query: Option<&oas::GetFoldersQuery>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["folders"];
        self.client
            .request_json::<T, oas::GetFoldersQuery, ()>(Method::GET, &segments, query, None)
            .await
    }

    /// `GET /groupsync/groups/{group_id}/roles`
    pub async fn get_group_roles<T>(&self, group_id: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let group_id = group_id.as_ref();
        let segments = ["groupsync", "groups", group_id, "roles"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /health`
    pub async fn get_health<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["health"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /dashboards/home`
    pub async fn get_home_dashboard<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["dashboards", "home"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /admin/ldap/status`
    pub async fn get_ldap_status<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["admin", "ldap", "status"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /library-elements/name/{library_element_name}`
    pub async fn get_library_element_by_name<T>(
        &self,
        library_element_name: impl AsRef<str>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let library_element_name = library_element_name.as_ref();
        let segments = ["library-elements", "name", library_element_name];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /library-elements/{library_element_uid}`
    pub async fn get_library_element_by_uid<T>(
        &self,
        library_element_uid: impl AsRef<str>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let library_element_uid = library_element_uid.as_ref();
        let segments = ["library-elements", library_element_uid];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /library-elements/{library_element_uid}/connections/`
    pub async fn get_library_element_connections<T>(
        &self,
        library_element_uid: impl AsRef<str>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let library_element_uid = library_element_uid.as_ref();
        let segments = ["library-elements", library_element_uid, "connections"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /library-elements`
    pub async fn get_library_elements<T>(
        &self,
        query: Option<&oas::GetLibraryElementsQuery>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["library-elements"];
        self.client
            .request_json::<T, oas::GetLibraryElementsQuery, ()>(
                Method::GET,
                &segments,
                query,
                None,
            )
            .await
    }

    /// `GET /licensing/token`
    pub async fn get_license_token<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["licensing", "token"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /groupsync/groups`
    pub async fn get_mapped_groups<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["groupsync", "groups"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /saml/metadata`
    pub async fn get_metadata<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["saml", "metadata"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /orgs/{org_id}`
    pub async fn get_org_by_id<T>(&self, org_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let org_id_str = org_id.to_string();
        let segments = ["orgs", org_id_str.as_str()];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /orgs/name/{org_name}`
    pub async fn get_org_by_name<T>(&self, org_name: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let org_name = org_name.as_ref();
        let segments = ["orgs", "name", org_name];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /org/preferences`
    pub async fn get_org_preferences<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["org", "preferences"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /orgs/{org_id}/quotas`
    pub async fn get_org_quota<T>(&self, org_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let org_id_str = org_id.to_string();
        let segments = ["orgs", org_id_str.as_str(), "quotas"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /orgs/{org_id}/users`
    pub async fn get_org_users<T>(&self, org_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let org_id_str = org_id.to_string();
        let segments = ["orgs", org_id_str.as_str(), "users"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /org/users`
    pub async fn get_org_users_for_current_org<T>(
        &self,
        query: Option<&oas::GetOrgUsersForCurrentOrgQuery>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["org", "users"];
        self.client
            .request_json::<T, oas::GetOrgUsersForCurrentOrgQuery, ()>(
                Method::GET,
                &segments,
                query,
                None,
            )
            .await
    }

    /// `GET /org/users/lookup`
    pub async fn get_org_users_for_current_org_lookup<T>(
        &self,
        query: Option<&oas::GetOrgUsersForCurrentOrgLookupQuery>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["org", "users", "lookup"];
        self.client
            .request_json::<T, oas::GetOrgUsersForCurrentOrgLookupQuery, ()>(
                Method::GET,
                &segments,
                query,
                None,
            )
            .await
    }

    /// `GET /org/invites`
    pub async fn get_pending_org_invites<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["org", "invites"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /playlists/{uid}`
    pub async fn get_playlist<T>(&self, uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uid = uid.as_ref();
        let segments = ["playlists", uid];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /playlists/{uid}/items`
    pub async fn get_playlist_items<T>(&self, uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uid = uid.as_ref();
        let segments = ["playlists", uid, "items"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /v1/sso-settings/{key}`
    pub async fn get_provider_settings<T>(&self, key: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let key = key.as_ref();
        let segments = ["v1", "sso-settings", key];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /public/dashboards/{accessToken}/annotations`
    pub async fn get_public_annotations<T>(&self, access_token: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let access_token = access_token.as_ref();
        let segments = ["public", "dashboards", access_token, "annotations"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /dashboards/uid/{dashboardUid}/public-dashboards`
    pub async fn get_public_dashboard<T>(&self, dashboard_uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let dashboard_uid = dashboard_uid.as_ref();
        let segments = ["dashboards", "uid", dashboard_uid, "public-dashboards"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /recording-rules/writer`
    pub async fn get_recording_rule_write_target<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["recording-rules", "writer"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /reports/{id}`
    pub async fn get_report<T>(&self, id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let id_str = id.to_string();
        let segments = ["reports", id_str.as_str()];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /reports/settings`
    pub async fn get_report_settings<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["reports", "settings"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /reports`
    pub async fn get_reports<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["reports"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /reports/dashboards/{uid}`
    pub async fn get_reports_by_dashboard_uid<T>(&self, uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uid = uid.as_ref();
        let segments = ["reports", "dashboards", uid];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /cloudmigration/resources/dependencies`
    pub async fn get_resource_dependencies<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["cloudmigration", "resources", "dependencies"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /access-control/{resource}/description`
    pub async fn get_resource_description<T>(&self, resource: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let resource = resource.as_ref();
        let segments = ["access-control", resource, "description"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /access-control/{resource}/{resourceID}`
    pub async fn get_resource_permissions<T>(
        &self,
        resource: impl AsRef<str>,
        resource_id: impl AsRef<str>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let resource = resource.as_ref();
        let resource_id = resource_id.as_ref();
        let segments = ["access-control", resource, resource_id];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /access-control/roles/{roleUID}`
    pub async fn get_role<T>(&self, role_uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let role_uid = role_uid.as_ref();
        let segments = ["access-control", "roles", role_uid];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /access-control/roles/{roleUID}/assignments`
    pub async fn get_role_assignments<T>(&self, role_uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let role_uid = role_uid.as_ref();
        let segments = ["access-control", "roles", role_uid, "assignments"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /logout/saml`
    pub async fn get_saml_logout<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["logout", "saml"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /saml/slo`
    pub async fn get_slo<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["saml", "slo"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /cloudmigration/migration/{uid}`
    pub async fn get_session<T>(&self, uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uid = uid.as_ref();
        let segments = ["cloudmigration", "migration", uid];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /cloudmigration/migration`
    pub async fn get_session_list<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["cloudmigration", "migration"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /reports/images/:image`
    pub async fn get_settings_image<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["reports", "images", ":image"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /cloudmigration/migration/{uid}/snapshots`
    pub async fn get_shapshot_list<T>(
        &self,
        uid: impl AsRef<str>,
        query: Option<&oas::GetShapshotListQuery>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uid = uid.as_ref();
        let segments = ["cloudmigration", "migration", uid, "snapshots"];
        self.client
            .request_json::<T, oas::GetShapshotListQuery, ()>(Method::GET, &segments, query, None)
            .await
    }

    /// `GET /snapshot/shared-options`
    pub async fn get_sharing_options<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["snapshot", "shared-options"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /user`
    pub async fn get_signed_in_user<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["user"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /user/orgs`
    pub async fn get_signed_in_user_org_list<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["user", "orgs"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /user/teams`
    pub async fn get_signed_in_user_team_list<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["user", "teams"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /cloudmigration/migration/{uid}/snapshot/{snapshotUid}`
    pub async fn get_snapshot<T>(
        &self,
        uid: impl AsRef<str>,
        snapshot_uid: impl AsRef<str>,
        query: Option<&oas::GetSnapshotQuery>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uid = uid.as_ref();
        let snapshot_uid = snapshot_uid.as_ref();
        let segments = ["cloudmigration", "migration", uid, "snapshot", snapshot_uid];
        self.client
            .request_json::<T, oas::GetSnapshotQuery, ()>(Method::GET, &segments, query, None)
            .await
    }

    /// `GET /licensing/check`
    pub async fn get_status(&self) -> Result<()> {
        let segments = ["licensing", "check"];
        let _ = self
            .client
            .request_bytes::<(), ()>(Method::GET, &segments, None, None)
            .await?;
        Ok(())
    }

    /// `GET /admin/ldap-sync-status`
    pub async fn get_sync_status<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["admin", "ldap-sync-status"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /teams/{team_id}`
    pub async fn get_team_by_id<T>(
        &self,
        team_id: impl AsRef<str>,
        query: Option<&oas::GetTeamByIdQuery>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let team_id = team_id.as_ref();
        let segments = ["teams", team_id];
        self.client
            .request_json::<T, oas::GetTeamByIdQuery, ()>(Method::GET, &segments, query, None)
            .await
    }

    /// `GET /teams/{teamId}/groups`
    pub async fn get_team_groups_api<T>(&self, team_id: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let team_id = team_id.as_ref();
        let segments = ["teams", team_id, "groups"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /datasources/uid/{uid}/lbac/teams`
    pub async fn get_team_lbac_rules_api<T>(&self, uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let uid = uid.as_ref();
        let segments = ["datasources", "uid", uid, "lbac", "teams"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /teams/{team_id}/members`
    pub async fn get_team_members<T>(&self, team_id: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let team_id = team_id.as_ref();
        let segments = ["teams", team_id, "members"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /teams/{team_id}/preferences`
    pub async fn get_team_preferences<T>(&self, team_id: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let team_id = team_id.as_ref();
        let segments = ["teams", team_id, "preferences"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /user/auth-tokens`
    pub async fn get_user_auth_tokens<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["user", "auth-tokens"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /users/{user_id}`
    pub async fn get_user_by_id<T>(&self, user_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let user_id_str = user_id.to_string();
        let segments = ["users", user_id_str.as_str()];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /users/lookup`
    pub async fn get_user_by_login_or_email<T>(&self, login_or_email: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let login_or_email = login_or_email.as_ref();
        #[derive(Serialize)]
        struct Query<'a> {
            #[serde(rename = "loginOrEmail")]
            login_or_email: &'a str,
        }
        let query = Query { login_or_email };
        let segments = ["users", "lookup"];
        self.client.get_json(&segments, Some(&query)).await
    }

    /// `GET /admin/ldap/{user_name}`
    pub async fn get_user_from_ldap<T>(&self, user_name: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let user_name = user_name.as_ref();
        let segments = ["admin", "ldap", user_name];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /users/{user_id}/orgs`
    pub async fn get_user_org_list<T>(&self, user_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let user_id_str = user_id.to_string();
        let segments = ["users", user_id_str.as_str(), "orgs"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /user/preferences`
    pub async fn get_user_preferences<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["user", "preferences"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /admin/users/{user_id}/quotas`
    pub async fn get_user_quota<T>(&self, user_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let user_id_str = user_id.to_string();
        let segments = ["admin", "users", user_id_str.as_str(), "quotas"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /user/quotas`
    pub async fn get_user_quotas<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["user", "quotas"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /users/{user_id}/teams`
    pub async fn get_user_teams<T>(&self, user_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let user_id_str = user_id.to_string();
        let segments = ["users", user_id_str.as_str(), "teams"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `POST /dashboards/import`
    pub async fn import_dashboard<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["dashboards", "import"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /dashboards/interpolate`
    pub async fn interpolate_dashboard<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["dashboards", "interpolate"];
        self.client
            .request_json::<T, (), ()>(Method::POST, &segments, None, None)
            .await
    }

    /// `GET /v1/sso-settings`
    pub async fn list_all_providers_settings<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["v1", "sso-settings"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /anonymous/devices`
    pub async fn list_devices<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["anonymous", "devices"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /dashboards/public-dashboards`
    pub async fn list_public_dashboards<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["dashboards", "public-dashboards"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /recording-rules`
    pub async fn list_recording_rules<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["recording-rules"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /access-control/roles`
    pub async fn list_roles<T>(&self, query: Option<&oas::ListRolesQuery>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["access-control", "roles"];
        self.client
            .request_json::<T, oas::ListRolesQuery, ()>(Method::GET, &segments, query, None)
            .await
    }

    /// `GET /search/sorting`
    pub async fn list_sort_options<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["search", "sorting"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /access-control/teams/{teamId}/roles`
    pub async fn list_team_roles<T>(&self, team_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let team_id_str = team_id.to_string();
        let segments = ["access-control", "teams", team_id_str.as_str(), "roles"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `POST /access-control/teams/roles/search`
    pub async fn list_teams_roles<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["access-control", "teams", "roles", "search"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `GET /serviceaccounts/{serviceAccountId}/tokens`
    pub async fn list_tokens<T>(&self, service_account_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let service_account_id_str = service_account_id.to_string();
        let segments = ["serviceaccounts", service_account_id_str.as_str(), "tokens"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /access-control/users/{userId}/roles`
    pub async fn list_user_roles<T>(&self, user_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let user_id_str = user_id.to_string();
        let segments = ["access-control", "users", user_id_str.as_str(), "roles"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `POST /access-control/users/roles/search`
    pub async fn list_users_roles<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["access-control", "users", "roles", "search"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /annotations/mass-delete`
    pub async fn mass_delete_annotations<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["annotations", "mass-delete"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /folders/{folder_uid}/move`
    pub async fn move_folder<T, B>(&self, folder_uid: impl AsRef<str>, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let folder_uid = folder_uid.as_ref();
        let segments = ["folders", folder_uid, "move"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `PATCH /annotations/{annotation_id}`
    pub async fn patch_annotation<T, B>(
        &self,
        annotation_id: impl AsRef<str>,
        body: &B,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let annotation_id = annotation_id.as_ref();
        let segments = ["annotations", annotation_id];
        self.client
            .request_json::<T, (), B>(Method::PATCH, &segments, None, Some(body))
            .await
    }

    /// `PATCH /org/preferences`
    pub async fn patch_org_preferences<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["org", "preferences"];
        self.client
            .request_json::<T, (), B>(Method::PATCH, &segments, None, Some(body))
            .await
    }

    /// `PATCH /query-history/{query_history_uid}`
    pub async fn patch_query_comment<T, B>(
        &self,
        query_history_uid: impl AsRef<str>,
        body: &B,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let query_history_uid = query_history_uid.as_ref();
        let segments = ["query-history", query_history_uid];
        self.client
            .request_json::<T, (), B>(Method::PATCH, &segments, None, Some(body))
            .await
    }

    /// `PATCH /user/preferences`
    pub async fn patch_user_preferences<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["user", "preferences"];
        self.client
            .request_json::<T, (), B>(Method::PATCH, &segments, None, Some(body))
            .await
    }

    /// `POST /saml/acs`
    pub async fn post_acs<T>(&self, query: Option<&oas::PostAcsQuery>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["saml", "acs"];
        self.client
            .request_json::<T, oas::PostAcsQuery, ()>(Method::POST, &segments, query, None)
            .await
    }

    /// `POST /annotations`
    pub async fn post_annotation<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["annotations"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /dashboards/db`
    pub async fn post_dashboard<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["dashboards", "db"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /annotations/graphite`
    pub async fn post_graphite_annotation<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["annotations", "graphite"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /licensing/token`
    pub async fn post_license_token<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["licensing", "token"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /licensing/token/renew`
    pub async fn post_renew_license_token<B>(&self, body: &B) -> Result<()>
    where
        B: Serialize + ?Sized,
    {
        let segments = ["licensing", "token", "renew"];
        let _ = self
            .client
            .request_bytes::<(), B>(Method::POST, &segments, None, Some(body))
            .await?;
        Ok(())
    }

    /// `POST /saml/slo`
    pub async fn post_slo<T>(&self, query: Option<&oas::PostSloQuery>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["saml", "slo"];
        self.client
            .request_json::<T, oas::PostSloQuery, ()>(Method::POST, &segments, query, None)
            .await
    }

    /// `POST /admin/ldap/sync/{user_id}`
    pub async fn post_sync_user_with_ldap<T>(&self, user_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let user_id_str = user_id.to_string();
        let segments = ["admin", "ldap", "sync", user_id_str.as_str()];
        self.client
            .request_json::<T, (), ()>(Method::POST, &segments, None, None)
            .await
    }

    /// `POST /ds/query`
    pub async fn query_metrics_with_expressions<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["ds", "query"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /public/dashboards/{accessToken}/panels/{panelId}/query`
    pub async fn query_public_dashboard<T>(
        &self,
        access_token: impl AsRef<str>,
        panel_id: i64,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let access_token = access_token.as_ref();
        let panel_id_str = panel_id.to_string();
        let segments = [
            "public",
            "dashboards",
            access_token,
            "panels",
            panel_id_str.as_str(),
            "query",
        ];
        self.client
            .request_json::<T, (), ()>(Method::POST, &segments, None, None)
            .await
    }

    /// `GET /licensing/refresh-stats`
    pub async fn refresh_license_stats<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["licensing", "refresh-stats"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `POST /admin/ldap/reload`
    pub async fn reload_ldap_cfg<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["admin", "ldap", "reload"];
        self.client
            .request_json::<T, (), ()>(Method::POST, &segments, None, None)
            .await
    }

    /// `DELETE /orgs/{org_id}/users/{user_id}`
    pub async fn remove_org_user<T>(&self, org_id: i64, user_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let org_id_str = org_id.to_string();
        let user_id_str = user_id.to_string();
        let segments = ["orgs", org_id_str.as_str(), "users", user_id_str.as_str()];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `DELETE /org/users/{user_id}`
    pub async fn remove_org_user_for_current_org<T>(&self, user_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let user_id_str = user_id.to_string();
        let segments = ["org", "users", user_id_str.as_str()];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `DELETE /v1/sso-settings/{key}`
    pub async fn remove_provider_settings<T>(&self, key: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let key = key.as_ref();
        let segments = ["v1", "sso-settings", key];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `DELETE /teams/{teamId}/groups`
    pub async fn remove_team_group_api_query<T>(
        &self,
        team_id: impl AsRef<str>,
        query: Option<&oas::RemoveTeamGroupApiQueryQuery>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let team_id = team_id.as_ref();
        let segments = ["teams", team_id, "groups"];
        self.client
            .request_json::<T, oas::RemoveTeamGroupApiQueryQuery, ()>(
                Method::DELETE,
                &segments,
                query,
                None,
            )
            .await
    }

    /// `DELETE /teams/{team_id}/members/{user_id}`
    pub async fn remove_team_member<T>(&self, team_id: impl AsRef<str>, user_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let team_id = team_id.as_ref();
        let user_id_str = user_id.to_string();
        let segments = ["teams", team_id, "members", user_id_str.as_str()];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `DELETE /access-control/teams/{teamId}/roles/{roleUID}`
    pub async fn remove_team_role<T>(&self, role_uid: impl AsRef<str>, team_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let role_uid = role_uid.as_ref();
        let team_id_str = team_id.to_string();
        let segments = [
            "access-control",
            "teams",
            team_id_str.as_str(),
            "roles",
            role_uid,
        ];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `DELETE /access-control/users/{userId}/roles/{roleUID}`
    pub async fn remove_user_role<T>(
        &self,
        role_uid: impl AsRef<str>,
        user_id: i64,
        query: Option<&oas::RemoveUserRoleQuery>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let role_uid = role_uid.as_ref();
        let user_id_str = user_id.to_string();
        let segments = [
            "access-control",
            "users",
            user_id_str.as_str(),
            "roles",
            role_uid,
        ];
        self.client
            .request_json::<T, oas::RemoveUserRoleQuery, ()>(Method::DELETE, &segments, query, None)
            .await
    }

    /// `GET /reports/render/csvs`
    pub async fn render_report_cs_vs<T>(
        &self,
        query: Option<&oas::RenderReportCsVsQuery>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["reports", "render", "csvs"];
        self.client
            .request_json::<T, oas::RenderReportCsVsQuery, ()>(Method::GET, &segments, query, None)
            .await
    }

    /// `GET /reports/render/pdfs`
    pub async fn render_report_pd_fs<T>(
        &self,
        query: Option<&oas::RenderReportPdFsQuery>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["reports", "render", "pdfs"];
        self.client
            .request_json::<T, oas::RenderReportPdFsQuery, ()>(Method::GET, &segments, query, None)
            .await
    }

    /// `POST /dashboards/uid/{uid}/restore`
    pub async fn restore_dashboard_version_by_uid<T, B>(
        &self,
        uid: impl AsRef<str>,
        body: &B,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let uid = uid.as_ref();
        let segments = ["dashboards", "uid", uid, "restore"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `GET /signing-keys/keys`
    pub async fn retrieve_jwks<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["signing-keys", "keys"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /serviceaccounts/{serviceAccountId}`
    pub async fn retrieve_service_account<T>(&self, service_account_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let service_account_id_str = service_account_id.to_string();
        let segments = ["serviceaccounts", service_account_id_str.as_str()];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `DELETE /org/invites/{invitation_code}/revoke`
    pub async fn revoke_invite<T>(&self, invitation_code: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let invitation_code = invitation_code.as_ref();
        let segments = ["org", "invites", invitation_code, "revoke"];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `POST /user/revoke-auth-token`
    pub async fn revoke_user_auth_token<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["user", "revoke-auth-token"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /reports/settings`
    pub async fn save_report_settings<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["reports", "settings"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `GET /search`
    pub async fn search<T>(&self, query: Option<&oas::SearchQuery>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["search"];
        self.client
            .request_json::<T, oas::SearchQuery, ()>(Method::GET, &segments, query, None)
            .await
    }

    /// `GET /dashboard/snapshots`
    pub async fn search_dashboard_snapshots<T>(
        &self,
        query: Option<&oas::SearchDashboardSnapshotsQuery>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["dashboard", "snapshots"];
        self.client
            .request_json::<T, oas::SearchDashboardSnapshotsQuery, ()>(
                Method::GET,
                &segments,
                query,
                None,
            )
            .await
    }

    /// `GET /serviceaccounts/search`
    pub async fn search_org_service_accounts_with_paging<T>(
        &self,
        query: Option<&oas::SearchOrgServiceAccountsWithPagingQuery>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["serviceaccounts", "search"];
        self.client
            .request_json::<T, oas::SearchOrgServiceAccountsWithPagingQuery, ()>(
                Method::GET,
                &segments,
                query,
                None,
            )
            .await
    }

    /// `GET /orgs/{org_id}/users/search`
    pub async fn search_org_users<T>(&self, org_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let org_id_str = org_id.to_string();
        let segments = ["orgs", org_id_str.as_str(), "users", "search"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `GET /orgs`
    pub async fn search_orgs<T>(&self, query: Option<&oas::SearchOrgsQuery>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["orgs"];
        self.client
            .request_json::<T, oas::SearchOrgsQuery, ()>(Method::GET, &segments, query, None)
            .await
    }

    /// `GET /playlists`
    pub async fn search_playlists<T>(&self, query: Option<&oas::SearchPlaylistsQuery>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["playlists"];
        self.client
            .request_json::<T, oas::SearchPlaylistsQuery, ()>(Method::GET, &segments, query, None)
            .await
    }

    /// `GET /query-history`
    pub async fn search_queries<T>(&self, query: Option<&oas::SearchQueriesQuery>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["query-history"];
        self.client
            .request_json::<T, oas::SearchQueriesQuery, ()>(Method::GET, &segments, query, None)
            .await
    }

    /// `POST /access-control/assignments/search`
    pub async fn search_result<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["access-control", "assignments", "search"];
        self.client
            .request_json::<T, (), ()>(Method::POST, &segments, None, None)
            .await
    }

    /// `GET /teams/{teamId}/groups/search`
    pub async fn search_team_groups<T>(
        &self,
        team_id: i64,
        query: Option<&oas::SearchTeamGroupsQuery>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let team_id_str = team_id.to_string();
        let segments = ["teams", team_id_str.as_str(), "groups", "search"];
        self.client
            .request_json::<T, oas::SearchTeamGroupsQuery, ()>(Method::GET, &segments, query, None)
            .await
    }

    /// `GET /teams/search`
    pub async fn search_teams<T>(&self, query: Option<&oas::SearchTeamsQuery>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["teams", "search"];
        self.client
            .request_json::<T, oas::SearchTeamsQuery, ()>(Method::GET, &segments, query, None)
            .await
    }

    /// `GET /users`
    pub async fn search_users<T>(&self, query: Option<&oas::SearchUsersQuery>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["users"];
        self.client
            .request_json::<T, oas::SearchUsersQuery, ()>(Method::GET, &segments, query, None)
            .await
    }

    /// `GET /users/search`
    pub async fn search_users_with_paging<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["users", "search"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `POST /reports/email`
    pub async fn send_report<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["reports", "email"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /reports/test-email`
    pub async fn send_test_email<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["reports", "test-email"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /datasources/{dataSourceUID}/cache`
    pub async fn set_data_source_cache_config<T, B>(
        &self,
        data_source_uid: impl AsRef<str>,
        body: &B,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let data_source_uid = data_source_uid.as_ref();
        let segments = ["datasources", data_source_uid, "cache"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `PUT /user/helpflags/{flag_id}`
    pub async fn set_help_flag<T>(&self, flag_id: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let flag_id = flag_id.as_ref();
        let segments = ["user", "helpflags", flag_id];
        self.client
            .request_json::<T, (), ()>(Method::PUT, &segments, None, None)
            .await
    }

    /// `POST /access-control/{resource}/{resourceID}`
    pub async fn set_resource_permissions<T, B>(
        &self,
        resource: impl AsRef<str>,
        resource_id: impl AsRef<str>,
        body: &B,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let resource = resource.as_ref();
        let resource_id = resource_id.as_ref();
        let segments = ["access-control", resource, resource_id];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /access-control/{resource}/{resourceID}/builtInRoles/{builtInRole}`
    pub async fn set_resource_permissions_for_built_in_role<T, B>(
        &self,
        resource: impl AsRef<str>,
        resource_id: impl AsRef<str>,
        built_in_role: impl AsRef<str>,
        body: &B,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let resource = resource.as_ref();
        let resource_id = resource_id.as_ref();
        let built_in_role = built_in_role.as_ref();
        let segments = [
            "access-control",
            resource,
            resource_id,
            "builtInRoles",
            built_in_role,
        ];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /access-control/{resource}/{resourceID}/teams/{teamID}`
    pub async fn set_resource_permissions_for_team<T, B>(
        &self,
        resource: impl AsRef<str>,
        resource_id: impl AsRef<str>,
        team_id: i64,
        body: &B,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let resource = resource.as_ref();
        let resource_id = resource_id.as_ref();
        let team_id_str = team_id.to_string();
        let segments = [
            "access-control",
            resource,
            resource_id,
            "teams",
            team_id_str.as_str(),
        ];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /access-control/{resource}/{resourceID}/users/{userID}`
    pub async fn set_resource_permissions_for_user<T, B>(
        &self,
        resource: impl AsRef<str>,
        resource_id: impl AsRef<str>,
        user_id: i64,
        body: &B,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let resource = resource.as_ref();
        let resource_id = resource_id.as_ref();
        let user_id_str = user_id.to_string();
        let segments = [
            "access-control",
            resource,
            resource_id,
            "users",
            user_id_str.as_str(),
        ];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `PUT /access-control/roles/{roleUID}/assignments`
    pub async fn set_role_assignments<T, B>(&self, role_uid: impl AsRef<str>, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let role_uid = role_uid.as_ref();
        let segments = ["access-control", "roles", role_uid, "assignments"];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `PUT /teams/{team_id}/members`
    pub async fn set_team_memberships<T, B>(&self, team_id: impl AsRef<str>, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let team_id = team_id.as_ref();
        let segments = ["teams", team_id, "members"];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `PUT /access-control/teams/{teamId}/roles`
    pub async fn set_team_roles<T, B>(&self, team_id: i64, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let team_id_str = team_id.to_string();
        let segments = ["access-control", "teams", team_id_str.as_str(), "roles"];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `PUT /access-control/users/{userId}/roles`
    pub async fn set_user_roles<T, B>(&self, user_id: i64, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let user_id_str = user_id.to_string();
        let segments = ["access-control", "users", user_id_str.as_str(), "roles"];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `POST /user/stars/dashboard/uid/{dashboard_uid}`
    pub async fn star_dashboard_by_uid<T>(&self, dashboard_uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let dashboard_uid = dashboard_uid.as_ref();
        let segments = ["user", "stars", "dashboard", "uid", dashboard_uid];
        self.client
            .request_json::<T, (), ()>(Method::POST, &segments, None, None)
            .await
    }

    /// `POST /query-history/star/{query_history_uid}`
    pub async fn star_query<T>(&self, query_history_uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let query_history_uid = query_history_uid.as_ref();
        let segments = ["query-history", "star", query_history_uid];
        self.client
            .request_json::<T, (), ()>(Method::POST, &segments, None, None)
            .await
    }

    /// `POST /recording-rules/test`
    pub async fn test_create_recording_rule<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["recording-rules", "test"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `DELETE /user/stars/dashboard/uid/{dashboard_uid}`
    pub async fn unstar_dashboard_by_uid<T>(&self, dashboard_uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let dashboard_uid = dashboard_uid.as_ref();
        let segments = ["user", "stars", "dashboard", "uid", dashboard_uid];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `DELETE /query-history/star/{query_history_uid}`
    pub async fn unstar_query<T>(&self, query_history_uid: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let query_history_uid = query_history_uid.as_ref();
        let segments = ["query-history", "star", query_history_uid];
        self.client
            .request_json::<T, (), ()>(Method::DELETE, &segments, None, None)
            .await
    }

    /// `PUT /annotations/{annotation_id}`
    pub async fn update_annotation<T, B>(
        &self,
        annotation_id: impl AsRef<str>,
        body: &B,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let annotation_id = annotation_id.as_ref();
        let segments = ["annotations", annotation_id];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `PATCH /datasources/uid/{sourceUID}/correlations/{correlationUID}`
    pub async fn update_correlation<T, B>(
        &self,
        source_uid: impl AsRef<str>,
        correlation_uid: impl AsRef<str>,
        body: Option<&B>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let source_uid = source_uid.as_ref();
        let correlation_uid = correlation_uid.as_ref();
        let segments = [
            "datasources",
            "uid",
            source_uid,
            "correlations",
            correlation_uid,
        ];
        self.client
            .request_json::<T, (), B>(Method::PATCH, &segments, None, body)
            .await
    }

    /// `PUT /org`
    pub async fn update_current_org<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["org"];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `PUT /org/address`
    pub async fn update_current_org_address<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["org", "address"];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `POST /dashboards/id/{DashboardID}/permissions`
    pub async fn update_dashboard_permissions_by_id<T, B>(
        &self,
        dashboard_id: i64,
        body: &B,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let dashboard_id_str = dashboard_id.to_string();
        let segments = ["dashboards", "id", dashboard_id_str.as_str(), "permissions"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `POST /dashboards/uid/{uid}/permissions`
    pub async fn update_dashboard_permissions_by_uid<T, B>(
        &self,
        uid: impl AsRef<str>,
        body: &B,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let uid = uid.as_ref();
        let segments = ["dashboards", "uid", uid, "permissions"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `PUT /datasources/{id}`
    pub async fn update_data_source_by_id<T, B>(&self, id: impl AsRef<str>, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let id = id.as_ref();
        let segments = ["datasources", id];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `PUT /datasources/uid/{uid}`
    pub async fn update_data_source_by_uid<T, B>(&self, uid: impl AsRef<str>, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let uid = uid.as_ref();
        let segments = ["datasources", "uid", uid];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `PUT /folders/{folder_uid}`
    pub async fn update_folder<T, B>(&self, folder_uid: impl AsRef<str>, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let folder_uid = folder_uid.as_ref();
        let segments = ["folders", folder_uid];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `POST /folders/{folder_uid}/permissions`
    pub async fn update_folder_permissions<T, B>(
        &self,
        folder_uid: impl AsRef<str>,
        body: &B,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let folder_uid = folder_uid.as_ref();
        let segments = ["folders", folder_uid, "permissions"];
        self.client
            .request_json::<T, (), B>(Method::POST, &segments, None, Some(body))
            .await
    }

    /// `PUT /groupsync/groups/{group_id}`
    pub async fn update_group_mappings<T, B>(
        &self,
        group_id: impl AsRef<str>,
        body: &B,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let group_id = group_id.as_ref();
        let segments = ["groupsync", "groups", group_id];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `PATCH /library-elements/{library_element_uid}`
    pub async fn update_library_element<T, B>(
        &self,
        library_element_uid: impl AsRef<str>,
        body: &B,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let library_element_uid = library_element_uid.as_ref();
        let segments = ["library-elements", library_element_uid];
        self.client
            .request_json::<T, (), B>(Method::PATCH, &segments, None, Some(body))
            .await
    }

    /// `PUT /orgs/{org_id}`
    pub async fn update_org<T, B>(&self, org_id: i64, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let org_id_str = org_id.to_string();
        let segments = ["orgs", org_id_str.as_str()];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `PUT /orgs/{org_id}/address`
    pub async fn update_org_address<T, B>(&self, org_id: i64, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let org_id_str = org_id.to_string();
        let segments = ["orgs", org_id_str.as_str(), "address"];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `PUT /org/preferences`
    pub async fn update_org_preferences<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["org", "preferences"];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `PUT /orgs/{org_id}/quotas/{quota_target}`
    pub async fn update_org_quota<T, B>(
        &self,
        quota_target: impl AsRef<str>,
        org_id: i64,
        body: &B,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let quota_target = quota_target.as_ref();
        let org_id_str = org_id.to_string();
        let segments = ["orgs", org_id_str.as_str(), "quotas", quota_target];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `PATCH /orgs/{org_id}/users/{user_id}`
    pub async fn update_org_user<T, B>(&self, org_id: i64, user_id: i64, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let org_id_str = org_id.to_string();
        let user_id_str = user_id.to_string();
        let segments = ["orgs", org_id_str.as_str(), "users", user_id_str.as_str()];
        self.client
            .request_json::<T, (), B>(Method::PATCH, &segments, None, Some(body))
            .await
    }

    /// `PATCH /org/users/{user_id}`
    pub async fn update_org_user_for_current_org<T, B>(&self, user_id: i64, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let user_id_str = user_id.to_string();
        let segments = ["org", "users", user_id_str.as_str()];
        self.client
            .request_json::<T, (), B>(Method::PATCH, &segments, None, Some(body))
            .await
    }

    /// `PUT /playlists/{uid}`
    pub async fn update_playlist<T, B>(&self, uid: impl AsRef<str>, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let uid = uid.as_ref();
        let segments = ["playlists", uid];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `PUT /v1/sso-settings/{key}`
    pub async fn update_provider_settings<T, B>(&self, key: impl AsRef<str>, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let key = key.as_ref();
        let segments = ["v1", "sso-settings", key];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `PATCH /dashboards/uid/{dashboardUid}/public-dashboards/{uid}`
    pub async fn update_public_dashboard<T, B>(
        &self,
        dashboard_uid: impl AsRef<str>,
        uid: impl AsRef<str>,
        body: &B,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let dashboard_uid = dashboard_uid.as_ref();
        let uid = uid.as_ref();
        let segments = ["dashboards", "uid", dashboard_uid, "public-dashboards", uid];
        self.client
            .request_json::<T, (), B>(Method::PATCH, &segments, None, Some(body))
            .await
    }

    /// `PUT /recording-rules`
    pub async fn update_recording_rule<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["recording-rules"];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `PUT /reports/{id}`
    pub async fn update_report<T, B>(&self, id: i64, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let id_str = id.to_string();
        let segments = ["reports", id_str.as_str()];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `PUT /access-control/roles/{roleUID}`
    pub async fn update_role<T, B>(&self, role_uid: impl AsRef<str>, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let role_uid = role_uid.as_ref();
        let segments = ["access-control", "roles", role_uid];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `PATCH /serviceaccounts/{serviceAccountId}`
    pub async fn update_service_account<T, B>(
        &self,
        service_account_id: i64,
        body: Option<&B>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let service_account_id_str = service_account_id.to_string();
        let segments = ["serviceaccounts", service_account_id_str.as_str()];
        self.client
            .request_json::<T, (), B>(Method::PATCH, &segments, None, body)
            .await
    }

    /// `PUT /user`
    pub async fn update_signed_in_user<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["user"];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `PUT /teams/{team_id}`
    pub async fn update_team<T, B>(&self, team_id: impl AsRef<str>, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let team_id = team_id.as_ref();
        let segments = ["teams", team_id];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `PUT /datasources/uid/{uid}/lbac/teams`
    pub async fn update_team_lbac_rules_api<T, B>(
        &self,
        uid: impl AsRef<str>,
        body: Option<&B>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let uid = uid.as_ref();
        let segments = ["datasources", "uid", uid, "lbac", "teams"];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, body)
            .await
    }

    /// `PUT /teams/{team_id}/members/{user_id}`
    pub async fn update_team_member<T, B>(
        &self,
        team_id: impl AsRef<str>,
        user_id: i64,
        body: &B,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let team_id = team_id.as_ref();
        let user_id_str = user_id.to_string();
        let segments = ["teams", team_id, "members", user_id_str.as_str()];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `PUT /teams/{team_id}/preferences`
    pub async fn update_team_preferences<T, B>(
        &self,
        team_id: impl AsRef<str>,
        body: &B,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let team_id = team_id.as_ref();
        let segments = ["teams", team_id, "preferences"];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `PUT /users/{user_id}`
    pub async fn update_user<T, B>(&self, user_id: i64, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let user_id_str = user_id.to_string();
        let segments = ["users", user_id_str.as_str()];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `GET /user/email/update`
    pub async fn update_user_email<T>(&self) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let segments = ["user", "email", "update"];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }

    /// `PUT /user/preferences`
    pub async fn update_user_preferences<T, B>(&self, body: &B) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let segments = ["user", "preferences"];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `PUT /admin/users/{user_id}/quotas/{quota_target}`
    pub async fn update_user_quota<T, B>(
        &self,
        quota_target: impl AsRef<str>,
        user_id: i64,
        body: &B,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        let quota_target = quota_target.as_ref();
        let user_id_str = user_id.to_string();
        let segments = [
            "admin",
            "users",
            user_id_str.as_str(),
            "quotas",
            quota_target,
        ];
        self.client
            .request_json::<T, (), B>(Method::PUT, &segments, None, Some(body))
            .await
    }

    /// `POST /cloudmigration/migration/{uid}/snapshot/{snapshotUid}/upload`
    pub async fn upload_snapshot(
        &self,
        uid: impl AsRef<str>,
        snapshot_uid: impl AsRef<str>,
    ) -> Result<()> {
        let uid = uid.as_ref();
        let snapshot_uid = snapshot_uid.as_ref();
        let segments = [
            "cloudmigration",
            "migration",
            uid,
            "snapshot",
            snapshot_uid,
            "upload",
        ];
        let _ = self
            .client
            .request_bytes::<(), ()>(Method::POST, &segments, None, None)
            .await?;
        Ok(())
    }

    /// `POST /user/using/{org_id}`
    pub async fn user_set_using_org<T>(&self, org_id: i64) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let org_id_str = org_id.to_string();
        let segments = ["user", "using", org_id_str.as_str()];
        self.client
            .request_json::<T, (), ()>(Method::POST, &segments, None, None)
            .await
    }

    /// `GET /public/dashboards/{accessToken}`
    pub async fn view_public_dashboard<T>(&self, access_token: impl AsRef<str>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let access_token = access_token.as_ref();
        let segments = ["public", "dashboards", access_token];
        self.client
            .request_json::<T, (), ()>(Method::GET, &segments, None, None)
            .await
    }
}
