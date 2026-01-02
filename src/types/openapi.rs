//! OpenAPI-derived query parameter models.

use serde::Serialize;

#[derive(Clone, Debug, Default, Serialize)]
pub struct RouteDeleteMuteTimingQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct RouteDeleteTemplateQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct RouteExportMuteTimingQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct RouteExportMuteTimingsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct RouteGetAlertRuleExportQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct RouteGetAlertRuleGroupExportQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct RouteGetAlertRulesExportQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "folderUid")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub folder_uid: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "ruleUid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_uid: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct RouteGetContactpointsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct RouteGetContactpointsExportQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decrypt: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct DeleteFolderQuery {
    #[serde(rename = "forceDeleteRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_delete_rules: Option<bool>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct DeleteRoleQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global: Option<bool>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct GetAnnotationTagsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct GetAnnotationsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(rename = "alertId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_id: Option<i64>,
    #[serde(rename = "alertUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alert_uid: Option<String>,
    #[serde(rename = "dashboardId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_id: Option<i64>,
    #[serde(rename = "dashboardUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_uid: Option<String>,
    #[serde(rename = "panelId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub panel_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "matchAny")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_any: Option<bool>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct GetCorrelationsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "sourceUID")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_uid: Vec<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct GetDashboardVersionsByUidQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i64>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct GetFoldersQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "parentUid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct GetLibraryElementsQuery {
    #[serde(rename = "searchString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<i64>,
    #[serde(rename = "sortDirection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_direction: Option<String>,
    #[serde(rename = "typeFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_filter: Option<String>,
    #[serde(rename = "excludeUid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_uid: Option<String>,
    #[serde(rename = "folderFilter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_filter: Option<String>,
    #[serde(rename = "perPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct GetOrgUsersForCurrentOrgQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct GetOrgUsersForCurrentOrgLookupQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct GetShapshotListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct GetSnapshotQuery {
    #[serde(rename = "resultPage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_page: Option<i64>,
    #[serde(rename = "resultLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_limit: Option<i64>,
    #[serde(rename = "resultSortColumn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_sort_column: Option<String>,
    #[serde(rename = "resultSortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_sort_order: Option<String>,
    #[serde(rename = "errorsOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors_only: Option<bool>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct GetTeamByIdQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accesscontrol: Option<bool>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct ListRolesQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delegatable: Option<bool>,
    #[serde(rename = "includeHidden")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_hidden: Option<bool>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct PostAcsQuery {
    #[serde(rename = "RelayState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relay_state: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct PostSloQuery {
    #[serde(rename = "SAMLRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saml_request: Option<String>,
    #[serde(rename = "SAMLResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saml_response: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct RemoveTeamGroupApiQueryQuery {
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct RemoveUserRoleQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global: Option<bool>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct RenderReportCsVsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboards: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct RenderReportPdFsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboards: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orientation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layout: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "scaleFactor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scale_factor: Option<String>,
    #[serde(rename = "includeTables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_tables: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SearchQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tag: Vec<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "dashboardIds")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dashboard_ids: Vec<i64>,
    #[serde(rename = "dashboardUIDs")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dashboard_ui_ds: Vec<String>,
    #[serde(rename = "folderIds")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub folder_ids: Vec<i64>,
    #[serde(rename = "folderUIDs")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub folder_ui_ds: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starred: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SearchDashboardSnapshotsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SearchOrgServiceAccountsWithPagingQuery {
    #[serde(rename = "Disabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(rename = "expiredTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired_tokens: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perpage: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SearchOrgsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perpage: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SearchPlaylistsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SearchQueriesQuery {
    #[serde(rename = "datasourceUid")]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub datasource_uid: Vec<String>,
    #[serde(rename = "searchString")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_string: Option<String>,
    #[serde(rename = "onlyStarred")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub only_starred: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SearchTeamGroupsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perpage: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SearchTeamsQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perpage: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accesscontrol: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct SearchUsersQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perpage: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
}
