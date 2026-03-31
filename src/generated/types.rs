//! Generated types from OpenAPI specification
//!
//! This file contains all the generated types for the API.
//! Do not edit manually - regenerate using the appropriate script.
#![allow(clippy::large_enum_variant)]
#![allow(clippy::format_in_format_args)]
#![allow(clippy::let_unit_value)]
#![allow(unreachable_patterns)]
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VersionVariables {
    #[serde(rename = "buildVariables", skip_serializing_if = "Option::is_none")]
    pub build_variables: Option<Vec<RestVariableDefinitionContext>>,
    #[serde(rename = "incrementableVariables", skip_serializing_if = "Option::is_none")]
    pub incrementable_variables: Option<Vec<RestVariableDefinitionContext>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestVariableDefinitionContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "variableType", skip_serializing_if = "Option::is_none")]
    pub variable_type: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VersionPreview {
    #[serde(rename = "commitCount", skip_serializing_if = "Option::is_none")]
    pub commit_count: Option<i32>,
    #[serde(rename = "commitUrl", skip_serializing_if = "Option::is_none")]
    pub commit_url: Option<String>,
    #[serde(
        rename = "differentBranchAsPreviousRelease",
        skip_serializing_if = "Option::is_none"
    )]
    pub different_branch_as_previous_release: Option<bool>,
    #[serde(rename = "differentBuildPlan", skip_serializing_if = "Option::is_none")]
    pub different_build_plan: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "issueCount", skip_serializing_if = "Option::is_none")]
    pub issue_count: Option<i32>,
    #[serde(rename = "issueUrl", skip_serializing_if = "Option::is_none")]
    pub issue_url: Option<String>,
    #[serde(rename = "lastCreatedVersionId", skip_serializing_if = "Option::is_none")]
    pub last_created_version_id: Option<i64>,
    #[serde(rename = "lastCreatedVersionName", skip_serializing_if = "Option::is_none")]
    pub last_created_version_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "numEnvironments", skip_serializing_if = "Option::is_none")]
    pub num_environments: Option<i32>,
    #[serde(
        rename = "numEnvironmentsDeployedFailed",
        skip_serializing_if = "Option::is_none"
    )]
    pub num_environments_deployed_failed: Option<i32>,
    #[serde(
        rename = "numEnvironmentsDeployedSuccessful",
        skip_serializing_if = "Option::is_none"
    )]
    pub num_environments_deployed_successful: Option<i32>,
    #[serde(rename = "planBranchName", skip_serializing_if = "Option::is_none")]
    pub plan_branch_name: Option<String>,
    #[serde(rename = "previousVersionId", skip_serializing_if = "Option::is_none")]
    pub previous_version_id: Option<i64>,
    #[serde(rename = "previousVersionName", skip_serializing_if = "Option::is_none")]
    pub previous_version_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback: Option<bool>,
    #[serde(rename = "versionGeneralState", skip_serializing_if = "Option::is_none")]
    pub version_general_state: Option<VersionPreviewVersionGeneralState>,
    #[serde(rename = "versionStatuses", skip_serializing_if = "Option::is_none")]
    pub version_statuses: Option<Vec<RestDeploymentVersionStatus>>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum VersionPreviewVersionGeneralState {
    #[default]
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Approved")]
    Approved,
    #[serde(rename = "Broken")]
    Broken,
    #[serde(rename = "Incomplete")]
    Incomplete,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchResultsList {
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(rename = "mayHaveMore", skip_serializing_if = "Option::is_none")]
    pub may_have_more: Option<bool>,
    #[serde(rename = "searchResults", skip_serializing_if = "Option::is_none")]
    pub search_results: Option<Vec<SearchResult>>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity: Option<SearchResultEntity>,
    #[serde(rename = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SearchResultEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestVcsLocationSpecsStatus {
    #[serde(rename = "inProgress", skip_serializing_if = "Option::is_none")]
    pub in_progress: Option<bool>,
    #[serde(rename = "specsLogs", skip_serializing_if = "Option::is_none")]
    pub specs_logs: Option<Vec<RestVcsLocationBambooSpecsState>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestVcsLocationBambooSpecsState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "logFilename", skip_serializing_if = "Option::is_none")]
    pub log_filename: Option<String>,
    #[serde(rename = "logUrl", skip_serializing_if = "Option::is_none")]
    pub log_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    #[serde(rename = "specImportState", skip_serializing_if = "Option::is_none")]
    pub spec_import_state: Option<RestVcsLocationBambooSpecsStateSpecImportState>,
    #[serde(rename = "specsExecutionDate", skip_serializing_if = "Option::is_none")]
    pub specs_execution_date: Option<String>,
    #[serde(rename = "specsNotFound", skip_serializing_if = "Option::is_none")]
    pub specs_not_found: Option<bool>,
    #[serde(rename = "vcsLocationId", skip_serializing_if = "Option::is_none")]
    pub vcs_location_id: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestVcsLocationBambooSpecsStateSpecImportState {
    #[default]
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "ERROR")]
    Error,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestVariableList {
    #[serde(rename = "allElements", skip_serializing_if = "Option::is_none")]
    pub all_elements: Option<Vec<RestVariable>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackRestVariable>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<RestVariable>>,
}
pub type ListWrapperCallbackRestVariable = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestUpdateDeploymentProjectRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "planKey", skip_serializing_if = "Option::is_none")]
    pub plan_key: Option<PlanKey>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestStageResultList {
    #[serde(rename = "allElements", skip_serializing_if = "Option::is_none")]
    pub all_elements: Option<Vec<RestStageResult>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackRestStageResult>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stages: Option<Vec<RestStageResult>>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
pub type ListWrapperCallbackRestStageResult = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestStageResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<RestResultList>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestServerStatusInfo {
    #[serde(rename = "askedNodeState", skip_serializing_if = "Option::is_none")]
    pub asked_node_state: Option<RestServerStatusInfoAskedNodeState>,
    #[serde(rename = "clusterState", skip_serializing_if = "Option::is_none")]
    pub cluster_state: Option<RestServerStatusInfoClusterState>,
    #[serde(rename = "reindexInProgress", skip_serializing_if = "Option::is_none")]
    pub reindex_in_progress: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestServerStatusInfoClusterState {
    #[default]
    #[serde(rename = "STARTING")]
    Starting,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "PAUSING")]
    Pausing,
    #[serde(rename = "PAUSED")]
    Paused,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestServerStatusInfoAskedNodeState {
    #[default]
    #[serde(rename = "SETUP")]
    Setup,
    #[serde(rename = "STARTING")]
    Starting,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "RUNNING_AS_SECONDARY")]
    RunningAsSecondary,
    #[serde(rename = "PAUSING")]
    Pausing,
    #[serde(rename = "PAUSED")]
    Paused,
    #[serde(rename = "PREPARING_FOR_RESTART")]
    PreparingForRestart,
    #[serde(rename = "READY_FOR_RESTART")]
    ReadyForRestart,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestServerNodesInfo {
    #[serde(rename = "askedNodeState", skip_serializing_if = "Option::is_none")]
    pub asked_node_state: Option<RestServerNodesInfoAskedNodeState>,
    #[serde(rename = "clusterState", skip_serializing_if = "Option::is_none")]
    pub cluster_state: Option<RestServerNodesInfoClusterState>,
    #[serde(rename = "nodeStatuses", skip_serializing_if = "Option::is_none")]
    pub node_statuses: Option<Vec<RestNodeStatus>>,
    #[serde(rename = "reindexInProgress", skip_serializing_if = "Option::is_none")]
    pub reindex_in_progress: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestServerNodesInfoClusterState {
    #[default]
    #[serde(rename = "STARTING")]
    Starting,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "PAUSING")]
    Pausing,
    #[serde(rename = "PAUSED")]
    Paused,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestServerNodesInfoAskedNodeState {
    #[default]
    #[serde(rename = "SETUP")]
    Setup,
    #[serde(rename = "STARTING")]
    Starting,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "RUNNING_AS_SECONDARY")]
    RunningAsSecondary,
    #[serde(rename = "PAUSING")]
    Pausing,
    #[serde(rename = "PAUSED")]
    Paused,
    #[serde(rename = "PREPARING_FOR_RESTART")]
    PreparingForRestart,
    #[serde(rename = "READY_FOR_RESTART")]
    ReadyForRestart,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestNodeStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alive: Option<bool>,
    #[serde(rename = "buildNumber", skip_serializing_if = "Option::is_none")]
    pub build_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(
        rename = "internalCommunicationPort",
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_communication_port: Option<i32>,
    #[serde(rename = "lastHeartbeat", skip_serializing_if = "Option::is_none")]
    pub last_heartbeat: Option<String>,
    #[serde(rename = "nodeId", skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "nodeName", skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestSecuritySettings {
    #[serde(
        rename = "agentAssignmentModificationByUsersAllowed",
        skip_serializing_if = "Option::is_none"
    )]
    pub agent_assignment_modification_by_users_allowed: Option<bool>,
    #[serde(rename = "bruteForceProtection", skip_serializing_if = "Option::is_none")]
    pub brute_force_protection: Option<RestBruteForceProtection>,
    #[serde(
        rename = "displayContactDetailsEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub display_contact_details_enabled: Option<bool>,
    #[serde(
        rename = "manageAcceptedSshHostKeys",
        skip_serializing_if = "Option::is_none"
    )]
    pub manage_accepted_ssh_host_keys: Option<bool>,
    #[serde(
        rename = "manualEncryptionConfiguration",
        skip_serializing_if = "Option::is_none"
    )]
    pub manual_encryption_configuration: Option<RestManualEncryptionConfiguration>,
    #[serde(
        rename = "personalAccessTokensExpirationConfiguration",
        skip_serializing_if = "Option::is_none"
    )]
    pub personal_access_tokens_expiration_configuration: Option<
        RestPersonalAccessTokensExpirationConfiguration,
    >,
    #[serde(
        rename = "resolveArtifactsContentTypeByExtensionEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub resolve_artifacts_content_type_by_extension_enabled: Option<bool>,
    #[serde(
        rename = "restrictedAdministratorRoleEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub restricted_administrator_role_enabled: Option<bool>,
    #[serde(
        rename = "rssSecurityConfiguration",
        skip_serializing_if = "Option::is_none"
    )]
    pub rss_security_configuration: Option<RestRssSecurityConfiguration>,
    #[serde(
        rename = "showAuthorsDetailsToUsers",
        skip_serializing_if = "Option::is_none"
    )]
    pub show_authors_details_to_users: Option<bool>,
    #[serde(rename = "signUp", skip_serializing_if = "Option::is_none")]
    pub sign_up: Option<RestSignUp>,
    #[serde(
        rename = "soxComplianceModeEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub sox_compliance_mode_enabled: Option<bool>,
    #[serde(
        rename = "unauthenticatedRemoteTriggerAllowed",
        skip_serializing_if = "Option::is_none"
    )]
    pub unauthenticated_remote_trigger_allowed: Option<bool>,
    #[serde(rename = "xsrfProtection", skip_serializing_if = "Option::is_none")]
    pub xsrf_protection: Option<RestXsrfProtection>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestXsrfProtection {
    #[serde(rename = "disableForHTTPGET", skip_serializing_if = "Option::is_none")]
    pub disable_for_httpget: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestSignUp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "enabledCaptcha", skip_serializing_if = "Option::is_none")]
    pub enabled_captcha: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestRssSecurityConfiguration {
    #[serde(rename = "dockerImage", skip_serializing_if = "Option::is_none")]
    pub docker_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "executeSpecsInDocker", skip_serializing_if = "Option::is_none")]
    pub execute_specs_in_docker: Option<bool>,
    #[serde(rename = "localMavenDirectory", skip_serializing_if = "Option::is_none")]
    pub local_maven_directory: Option<String>,
    #[serde(
        rename = "mountLocalMavenDirectory",
        skip_serializing_if = "Option::is_none"
    )]
    pub mount_local_maven_directory: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPersonalAccessTokensExpirationConfiguration {
    #[serde(rename = "expirationRequired", skip_serializing_if = "Option::is_none")]
    pub expiration_required: Option<bool>,
    #[serde(rename = "maxDaysUntilExpiry", skip_serializing_if = "Option::is_none")]
    pub max_days_until_expiry: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestBruteForceProtection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "loginAttempts", skip_serializing_if = "Option::is_none")]
    pub login_attempts: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestResults {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<RestResultList>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestResultStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished: Option<bool>,
    #[serde(rename = "planResultKey", skip_serializing_if = "Option::is_none")]
    pub plan_result_key: Option<String>,
    #[serde(rename = "prettyQueuedTime", skip_serializing_if = "Option::is_none")]
    pub pretty_queued_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<RestProgress>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestProgress {
    #[serde(rename = "averageBuildDuration", skip_serializing_if = "Option::is_none")]
    pub average_build_duration: Option<i64>,
    #[serde(rename = "buildTime", skip_serializing_if = "Option::is_none")]
    pub build_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(rename = "percentageCompleted", skip_serializing_if = "Option::is_none")]
    pub percentage_completed: Option<f64>,
    #[serde(
        rename = "percentageCompletedPretty",
        skip_serializing_if = "Option::is_none"
    )]
    pub percentage_completed_pretty: Option<String>,
    #[serde(
        rename = "prettyAverageBuildDuration",
        skip_serializing_if = "Option::is_none"
    )]
    pub pretty_average_build_duration: Option<String>,
    #[serde(rename = "prettyBuildTime", skip_serializing_if = "Option::is_none")]
    pub pretty_build_time: Option<String>,
    #[serde(rename = "prettyStartedTime", skip_serializing_if = "Option::is_none")]
    pub pretty_started_time: Option<String>,
    #[serde(rename = "prettyTimeRemaining", skip_serializing_if = "Option::is_none")]
    pub pretty_time_remaining: Option<String>,
    #[serde(rename = "prettyTimeRemainingLong", skip_serializing_if = "Option::is_none")]
    pub pretty_time_remaining_long: Option<String>,
    #[serde(rename = "startedTime", skip_serializing_if = "Option::is_none")]
    pub started_time: Option<String>,
    #[serde(rename = "startedTimeFormatted", skip_serializing_if = "Option::is_none")]
    pub started_time_formatted: Option<String>,
    #[serde(rename = "underAverageTime", skip_serializing_if = "Option::is_none")]
    pub under_average_time: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestResultList {
    #[serde(rename = "allElements", skip_serializing_if = "Option::is_none")]
    pub all_elements: Option<Vec<BuildResult>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackResult>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<BuildResult>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
pub type ListWrapperCallbackResult = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BuildResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<RestBuildArtifactList>,
    #[serde(rename = "buildCompletedTime", skip_serializing_if = "Option::is_none")]
    pub build_completed_time: Option<String>,
    #[serde(rename = "buildDuration", skip_serializing_if = "Option::is_none")]
    pub build_duration: Option<i64>,
    #[serde(
        rename = "buildDurationDescription",
        skip_serializing_if = "Option::is_none"
    )]
    pub build_duration_description: Option<String>,
    #[serde(rename = "buildDurationInSeconds", skip_serializing_if = "Option::is_none")]
    pub build_duration_in_seconds: Option<i64>,
    #[serde(rename = "buildNumber", skip_serializing_if = "Option::is_none")]
    pub build_number: Option<i32>,
    #[serde(rename = "buildReason", skip_serializing_if = "Option::is_none")]
    pub build_reason: Option<String>,
    #[serde(rename = "buildRelativeTime", skip_serializing_if = "Option::is_none")]
    pub build_relative_time: Option<String>,
    #[serde(rename = "buildStartedTime", skip_serializing_if = "Option::is_none")]
    pub build_started_time: Option<String>,
    #[serde(rename = "buildState", skip_serializing_if = "Option::is_none")]
    pub build_state: Option<String>,
    #[serde(rename = "buildSummary", skip_serializing_if = "Option::is_none")]
    pub build_summary: Option<Box<ImmutableResultsSummary>>,
    #[serde(rename = "buildTestSummary", skip_serializing_if = "Option::is_none")]
    pub build_test_summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<RestCommentList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(rename = "failedTestCount", skip_serializing_if = "Option::is_none")]
    pub failed_test_count: Option<i32>,
    #[serde(rename = "hasExecutableAgents", skip_serializing_if = "Option::is_none")]
    pub has_executable_agents: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "jiraIssues", skip_serializing_if = "Option::is_none")]
    pub jira_issues: Option<RestJiraIssueList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<RestBuildLabelList>,
    #[serde(rename = "lifeCycleState", skip_serializing_if = "Option::is_none")]
    pub life_cycle_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
    #[serde(rename = "onceOff", skip_serializing_if = "Option::is_none")]
    pub once_off: Option<bool>,
    #[serde(rename = "parentLink", skip_serializing_if = "Option::is_none")]
    pub parent_link: Option<Link>,
    #[serde(rename = "planName", skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[serde(rename = "planResultKey", skip_serializing_if = "Option::is_none")]
    pub plan_result_key: Option<PlanResultKey>,
    #[serde(
        rename = "prettyBuildCompletedTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub pretty_build_completed_time: Option<String>,
    #[serde(rename = "prettyBuildStartedTime", skip_serializing_if = "Option::is_none")]
    pub pretty_build_started_time: Option<String>,
    #[serde(rename = "prettyQueueDuration", skip_serializing_if = "Option::is_none")]
    pub pretty_queue_duration: Option<String>,
    #[serde(rename = "prettyQueueStartedTime", skip_serializing_if = "Option::is_none")]
    pub pretty_queue_started_time: Option<String>,
    #[serde(rename = "prettyVcsUpdateDuration", skip_serializing_if = "Option::is_none")]
    pub pretty_vcs_update_duration: Option<String>,
    #[serde(
        rename = "prettyVcsUpdateStartedTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub pretty_vcs_update_started_time: Option<String>,
    #[serde(rename = "projectName", skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(rename = "quarantinedTestCount", skip_serializing_if = "Option::is_none")]
    pub quarantined_test_count: Option<i32>,
    #[serde(rename = "queueDuration", skip_serializing_if = "Option::is_none")]
    pub queue_duration: Option<i64>,
    #[serde(rename = "queueStartedTime", skip_serializing_if = "Option::is_none")]
    pub queue_started_time: Option<String>,
    #[serde(rename = "queueTimeInSeconds", skip_serializing_if = "Option::is_none")]
    pub queue_time_in_seconds: Option<i64>,
    #[serde(rename = "restartCount", skip_serializing_if = "Option::is_none")]
    pub restart_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restartable: Option<bool>,
    #[serde(rename = "skippedTestCount", skip_serializing_if = "Option::is_none")]
    pub skipped_test_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    #[serde(rename = "stageResult", skip_serializing_if = "Option::is_none")]
    pub stage_result: Option<Box<ChainStageResult>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stages: Option<RestStageResultList>,
    #[serde(rename = "successfulTestCount", skip_serializing_if = "Option::is_none")]
    pub successful_test_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<RestVariableList>,
    #[serde(rename = "vcsRevisionKey", skip_serializing_if = "Option::is_none")]
    pub vcs_revision_key: Option<String>,
    #[serde(rename = "vcsUpdateDuration", skip_serializing_if = "Option::is_none")]
    pub vcs_update_duration: Option<i64>,
    #[serde(rename = "vcsUpdateInSeconds", skip_serializing_if = "Option::is_none")]
    pub vcs_update_in_seconds: Option<i64>,
    #[serde(rename = "vcsUpdateStartedTime", skip_serializing_if = "Option::is_none")]
    pub vcs_update_started_time: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestResources {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<RestResourceList>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestResourceList {
    #[serde(rename = "allElements", skip_serializing_if = "Option::is_none")]
    pub all_elements: Option<Vec<RestResource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackRestResource>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Vec<RestResource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
pub type ListWrapperCallbackRestResource = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestRepositoryUsageModel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environments: Option<Vec<RestEnvironment>>,
    #[serde(
        rename = "inaccessibleEnvironmentsCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub inaccessible_environments_count: Option<i32>,
    #[serde(rename = "inaccessiblePlansCount", skip_serializing_if = "Option::is_none")]
    pub inaccessible_plans_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plans: Option<Vec<RestPlanIdentifier>>,
    #[serde(rename = "totalEnvironments", skip_serializing_if = "Option::is_none")]
    pub total_environments: Option<i64>,
    #[serde(rename = "totalPlans", skip_serializing_if = "Option::is_none")]
    pub total_plans: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestRepositoryList {
    #[serde(rename = "allElements", skip_serializing_if = "Option::is_none")]
    pub all_elements: Option<Vec<RestRepository>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackRestRepository>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(rename = "searchResults", skip_serializing_if = "Option::is_none")]
    pub search_results: Option<Vec<RestRepository>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
pub type ListWrapperCallbackRestRepository = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestReports {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reports: Option<RestReportList>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestReportList {
    #[serde(rename = "allElements", skip_serializing_if = "Option::is_none")]
    pub all_elements: Option<Vec<RestReport>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackRestReport>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report: Option<Vec<RestReport>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestReport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
pub type ListWrapperCallbackRestReport = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestQueuedDeployments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
    #[serde(rename = "queuedDeployments", skip_serializing_if = "Option::is_none")]
    pub queued_deployments: Option<RestQueuedDeploymentList>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestQueuedDeploymentList {
    #[serde(rename = "allElements", skip_serializing_if = "Option::is_none")]
    pub all_elements: Option<Vec<RestQueuedDeployment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackRestQueuedDeployment>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(rename = "queuedDeployments", skip_serializing_if = "Option::is_none")]
    pub queued_deployments: Option<Vec<RestQueuedDeployment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestQueuedDeployment {
    #[serde(rename = "deploymentResultId", skip_serializing_if = "Option::is_none")]
    pub deployment_result_id: Option<i64>,
}
pub type ListWrapperCallbackRestQueuedDeployment = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestQueuedBuilds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
    #[serde(rename = "queuedBuilds", skip_serializing_if = "Option::is_none")]
    pub queued_builds: Option<RestQueuedBuildList>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestQueuedBuildList {
    #[serde(rename = "allElements", skip_serializing_if = "Option::is_none")]
    pub all_elements: Option<Vec<RestQueuedBuild>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackRestQueuedBuild>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(rename = "queuedBuild", skip_serializing_if = "Option::is_none")]
    pub queued_build: Option<Vec<RestQueuedBuild>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
pub type ListWrapperCallbackRestQueuedBuild = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestQueuedBuild {
    #[serde(rename = "buildNumber", skip_serializing_if = "Option::is_none")]
    pub build_number: Option<i32>,
    #[serde(rename = "buildResultKey", skip_serializing_if = "Option::is_none")]
    pub build_result_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changes: Option<RestChangeList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changesets: Option<Vec<Box<RepositoryChangeset>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
    #[serde(rename = "planKey", skip_serializing_if = "Option::is_none")]
    pub plan_key: Option<String>,
    #[serde(rename = "triggerReason", skip_serializing_if = "Option::is_none")]
    pub trigger_reason: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestProjects {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projects: Option<RestProjectList>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestProjectList {
    #[serde(rename = "allElements", skip_serializing_if = "Option::is_none")]
    pub all_elements: Option<Vec<Box<RestProject>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackRestProject>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Vec<Box<RestProject>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
pub type ListWrapperCallbackRestProject = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPlans {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plans: Option<Box<RestPlanList>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPlanLabels {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<RestPlanLabelList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPlanLabelList {
    #[serde(rename = "allElements", skip_serializing_if = "Option::is_none")]
    pub all_elements: Option<Vec<RestPlanLabel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackRestPlanLabel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Vec<RestPlanLabel>>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPlanLabel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
pub type ListWrapperCallbackRestPlanLabel = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPlanIdentifier {
    #[serde(rename = "buildKey", skip_serializing_if = "Option::is_none")]
    pub build_key: Option<String>,
    #[serde(rename = "buildName", skip_serializing_if = "Option::is_none")]
    pub build_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "masterId", skip_serializing_if = "Option::is_none")]
    pub master_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "planKey", skip_serializing_if = "Option::is_none")]
    pub plan_key: Option<String>,
    #[serde(rename = "planType", skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<RestProjectIdentifier>,
    #[serde(rename = "suspendedFromBuilding", skip_serializing_if = "Option::is_none")]
    pub suspended_from_building: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestProjectIdentifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPlanConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<RestKey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<RestProjectCreate>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stages: Option<RestPageModelRestStage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspended: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestProjectCreate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "publicAccess", skip_serializing_if = "Option::is_none")]
    pub public_access: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPageModelRestUserPermission {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestUserPermission>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPageModelRestUserDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestUserDetails>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPageModelRestUserAlias {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestUserAlias>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPageModelRestUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestUser>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPageModelRestStage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<Box<RestStage>>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPageModelRestSharedCredential {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestSharedCredential>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPageModelRestRolePermission {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestRolePermission>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPageModelRestProjectRepository {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestProjectRepository>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPageModelRestJob {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestJob>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPageModelRestGroupPermission {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestGroupPermission>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPageModelRestGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestGroup>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPageModelRestEphemeralAgentTemplate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestEphemeralAgentTemplate>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPageModelRestEnvironmentForExecutablesView {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestEnvironmentForExecutablesView>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPageModelRestDeploymentProject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestDeploymentProject>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPageModelRestCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestCapability>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPageModelRestBrokenPlan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestBrokenPlan>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPageModelRestAgent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestAgent>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPageModelRestAccessToken {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestAccessToken>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPageModelPlanWithCustomExpirySettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<PlanWithCustomExpirySettings>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestManualEncryptionConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "limitUnit", skip_serializing_if = "Option::is_none")]
    pub limit_unit: Option<RestManualEncryptionConfigurationLimitUnit>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestManualEncryptionConfigurationLimitUnit {
    #[default]
    #[serde(rename = "NANOSECONDS")]
    Nanoseconds,
    #[serde(rename = "MICROSECONDS")]
    Microseconds,
    #[serde(rename = "MILLISECONDS")]
    Milliseconds,
    #[serde(rename = "SECONDS")]
    Seconds,
    #[serde(rename = "MINUTES")]
    Minutes,
    #[serde(rename = "HOURS")]
    Hours,
    #[serde(rename = "DAYS")]
    Days,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestJiraIssueRelatedDeploymentProjects {
    #[serde(rename = "deploymentProjects", skip_serializing_if = "Option::is_none")]
    pub deployment_projects: Option<Vec<RestDeploymentProjectLink>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestJiraIssueList {
    #[serde(rename = "allElements", skip_serializing_if = "Option::is_none")]
    pub all_elements: Option<Vec<RestJiraIssue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackRestJiraIssue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issue: Option<Vec<RestJiraIssue>>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
pub type ListWrapperCallbackRestJiraIssue = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestJiraIssue {
    #[serde(rename = "iconUrl", skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    #[serde(rename = "issueType", skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Link>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestInfo {
    #[serde(rename = "buildDate", skip_serializing_if = "Option::is_none")]
    pub build_date: Option<String>,
    #[serde(rename = "buildNumber", skip_serializing_if = "Option::is_none")]
    pub build_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<RestInfoState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestInfoState {
    #[default]
    #[serde(rename = "SETUP")]
    Setup,
    #[serde(rename = "STARTING")]
    Starting,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "RUNNING_AS_SECONDARY")]
    RunningAsSecondary,
    #[serde(rename = "PAUSING")]
    Pausing,
    #[serde(rename = "PAUSED")]
    Paused,
    #[serde(rename = "PREPARING_FOR_RESTART")]
    PreparingForRestart,
    #[serde(rename = "READY_FOR_RESTART")]
    ReadyForRestart,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestGlobalVariables {
    #[serde(rename = "globalVariables", skip_serializing_if = "Option::is_none")]
    pub global_variables: Option<RestGlobalVariableList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestGlobalVariableList {
    #[serde(rename = "allElements", skip_serializing_if = "Option::is_none")]
    pub all_elements: Option<Vec<RestGlobalVariable>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackRestGlobalVariable>,
    #[serde(rename = "globalVariables", skip_serializing_if = "Option::is_none")]
    pub global_variables: Option<Vec<RestGlobalVariable>>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestGlobalVariable {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
pub type ListWrapperCallbackRestGlobalVariable = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestEnvironmentWithConfigCounts {
    #[serde(rename = "compatibleAgentsCount", skip_serializing_if = "Option::is_none")]
    pub compatible_agents_count: Option<i32>,
    #[serde(rename = "configurationState", skip_serializing_if = "Option::is_none")]
    pub configuration_state: Option<RestEnvironmentWithConfigCountsConfigurationState>,
    #[serde(rename = "deploymentProjectId", skip_serializing_if = "Option::is_none")]
    pub deployment_project_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<RestKey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "notificationsCount", skip_serializing_if = "Option::is_none")]
    pub notifications_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<RestOperations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(
        rename = "releaseApprovalPrerequisite",
        skip_serializing_if = "Option::is_none"
    )]
    pub release_approval_prerequisite: Option<
        RestEnvironmentWithConfigCountsReleaseApprovalPrerequisite,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspended: Option<bool>,
    #[serde(rename = "taskDefinitions", skip_serializing_if = "Option::is_none")]
    pub task_definitions: Option<Vec<TaskDefinition>>,
    #[serde(rename = "triggerDefinitions", skip_serializing_if = "Option::is_none")]
    pub trigger_definitions: Option<Vec<TriggerDefinition>>,
    #[serde(rename = "triggersCount", skip_serializing_if = "Option::is_none")]
    pub triggers_count: Option<i32>,
    #[serde(rename = "variablesCount", skip_serializing_if = "Option::is_none")]
    pub variables_count: Option<i32>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestEnvironmentWithConfigCountsReleaseApprovalPrerequisite {
    #[default]
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "NOT_BROKEN")]
    NotBroken,
    #[serde(rename = "APPROVED")]
    Approved,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestEnvironmentWithConfigCountsConfigurationState {
    #[default]
    #[serde(rename = "CREATED")]
    Created,
    #[serde(rename = "DETAILED")]
    Detailed,
    #[serde(rename = "TASKED")]
    Tasked,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestEnvironmentPrerequisites {
    #[serde(
        rename = "releaseApprovalPrerequisite",
        skip_serializing_if = "Option::is_none"
    )]
    pub release_approval_prerequisite: Option<
        RestEnvironmentPrerequisitesReleaseApprovalPrerequisite,
    >,
    #[serde(
        rename = "releaseApprovalPrerequisitePresent",
        skip_serializing_if = "Option::is_none"
    )]
    pub release_approval_prerequisite_present: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestEnvironmentPrerequisitesReleaseApprovalPrerequisite {
    #[default]
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "NOT_BROKEN")]
    NotBroken,
    #[serde(rename = "APPROVED")]
    Approved,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestElasticConfiguration {
    #[serde(rename = "accessKeyId", skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(
        rename = "allocatePublicIpToVpcInstances",
        skip_serializing_if = "Option::is_none"
    )]
    pub allocate_public_ip_to_vpc_instances: Option<bool>,
    #[serde(rename = "awsCredentialsType", skip_serializing_if = "Option::is_none")]
    pub aws_credentials_type: Option<RestElasticConfigurationAwsCredentialsType>,
    #[serde(rename = "certificateFile", skip_serializing_if = "Option::is_none")]
    pub certificate_file: Option<String>,
    #[serde(
        rename = "elasticInstanceManagement",
        skip_serializing_if = "Option::is_none"
    )]
    pub elastic_instance_management: Option<RestElasticInstanceManagement>,
    #[serde(
        rename = "elasticTerminationConfiguration",
        skip_serializing_if = "Option::is_none"
    )]
    pub elastic_termination_configuration: Option<RestElasticTerminationConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "maxNumOfElasticInstances",
        skip_serializing_if = "Option::is_none"
    )]
    pub max_num_of_elastic_instances: Option<i32>,
    #[serde(rename = "privateKeyFile", skip_serializing_if = "Option::is_none")]
    pub private_key_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "secretAccessKey", skip_serializing_if = "Option::is_none")]
    pub secret_access_key: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "uploadAwsIdentifier", skip_serializing_if = "Option::is_none")]
    pub upload_aws_identifier: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestElasticTerminationConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "shutdownDelay", skip_serializing_if = "Option::is_none")]
    pub shutdown_delay: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestElasticInstanceManagement {
    #[serde(
        rename = "allowedNonBambooInstances",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_non_bamboo_instances: Option<i32>,
    #[serde(rename = "avgQueueTimeMinutes", skip_serializing_if = "Option::is_none")]
    pub avg_queue_time_minutes: Option<i32>,
    #[serde(
        rename = "idleAgentShutdownDelayMinutes",
        skip_serializing_if = "Option::is_none"
    )]
    pub idle_agent_shutdown_delay_minutes: Option<i32>,
    #[serde(rename = "maxNumOfInstancesStart", skip_serializing_if = "Option::is_none")]
    pub max_num_of_instances_start: Option<i32>,
    #[serde(rename = "numOfBuildsInQueue", skip_serializing_if = "Option::is_none")]
    pub num_of_builds_in_queue: Option<i32>,
    #[serde(
        rename = "numOfElasticBuildsInQueue",
        skip_serializing_if = "Option::is_none"
    )]
    pub num_of_elastic_builds_in_queue: Option<i32>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestElasticConfigurationAwsCredentialsType {
    #[default]
    #[serde(rename = "INSTANCE_PROFILE")]
    InstanceProfile,
    #[serde(rename = "ACCESS_KEY")]
    AccessKey,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestDockerPipelineConfiguration {
    #[serde(rename = "additionalArguments", skip_serializing_if = "Option::is_none")]
    pub additional_arguments: Option<Vec<String>>,
    #[serde(rename = "dataVolumes", skip_serializing_if = "Option::is_none")]
    pub data_volumes: Option<Vec<RestDataVolume>>,
    #[serde(rename = "dockerImage", skip_serializing_if = "Option::is_none")]
    pub docker_image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestDataVolume {
    #[serde(rename = "containerDirectory", skip_serializing_if = "Option::is_none")]
    pub container_directory: Option<String>,
    #[serde(rename = "hostDirectory", skip_serializing_if = "Option::is_none")]
    pub host_directory: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestDeploymentVersionStatuses {
    #[serde(rename = "currentUserState", skip_serializing_if = "Option::is_none")]
    pub current_user_state: Option<RestDeploymentVersionStatusesCurrentUserState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statuses: Option<Vec<RestDeploymentVersionStatus>>,
    #[serde(rename = "versionGeneralState", skip_serializing_if = "Option::is_none")]
    pub version_general_state: Option<RestDeploymentVersionStatusesVersionGeneralState>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestDeploymentVersionStatusesVersionGeneralState {
    #[default]
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Approved")]
    Approved,
    #[serde(rename = "Broken")]
    Broken,
    #[serde(rename = "Incomplete")]
    Incomplete,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestDeploymentVersionStatusesCurrentUserState {
    #[default]
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Approved")]
    Approved,
    #[serde(rename = "Broken")]
    Broken,
    #[serde(rename = "Incomplete")]
    Incomplete,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestDeploymentVersionAndPlanResult {
    #[serde(rename = "deploymentVersion", skip_serializing_if = "Option::is_none")]
    pub deployment_version: Option<RestDeploymentVersion>,
    #[serde(rename = "planResultKey", skip_serializing_if = "Option::is_none")]
    pub plan_result_key: Option<PlanResultKey>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestDeploymentResultList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestDeploymentResult>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestDeploymentProjectStatusForJiraIssue {
    #[serde(rename = "deploymentProject", skip_serializing_if = "Option::is_none")]
    pub deployment_project: Option<RestDeploymentProjectLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environments: Option<Vec<RestEnvironmentStatusOfIssue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<RestDeploymentVersionLink>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestEnvironmentStatusOfIssue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "issueStatus", skip_serializing_if = "Option::is_none")]
    pub issue_status: Option<RestEnvironmentStatusOfIssueIssueStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "statusOk", skip_serializing_if = "Option::is_none")]
    pub status_ok: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<RestDeploymentVersionLink>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestEnvironmentStatusOfIssueIssueStatus {
    #[default]
    #[serde(rename = "NOT_AVAILABLE")]
    NotAvailable,
    #[serde(rename = "PARTIALLY_AVAILABLE")]
    PartiallyAvailable,
    #[serde(rename = "AVAILABLE")]
    Available,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestDeploymentVersionLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestDeploymentProjectLink {
    #[serde(rename = "environmentCount", skip_serializing_if = "Option::is_none")]
    pub environment_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "upToDateEnvironmentCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub up_to_date_environment_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestDependencies {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Box<RestPlanList>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestCreateDeploymentProjectRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "planKey", skip_serializing_if = "Option::is_none")]
    pub plan_key: Option<PlanKey>,
    #[serde(rename = "publicAccess", skip_serializing_if = "Option::is_none")]
    pub public_access: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestComments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<RestCommentList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestCommentList {
    #[serde(rename = "allElements", skip_serializing_if = "Option::is_none")]
    pub all_elements: Option<Vec<RestComment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackRestComment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<Vec<RestComment>>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestComment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(rename = "creationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "modificationDate", skip_serializing_if = "Option::is_none")]
    pub modification_date: Option<String>,
}
pub type ListWrapperCallbackRestComment = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestCombinedExpiryStatus {
    #[serde(rename = "buildExpiryStatus", skip_serializing_if = "Option::is_none")]
    pub build_expiry_status: Option<RestExpiryStatus>,
    #[serde(rename = "deploymentExpiryStatus", skip_serializing_if = "Option::is_none")]
    pub deployment_expiry_status: Option<RestExpiryStatus>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestExpiryStatus {
    #[serde(rename = "lastFinished", skip_serializing_if = "Option::is_none")]
    pub last_finished: Option<i64>,
    #[serde(rename = "lastFinishedDate", skip_serializing_if = "Option::is_none")]
    pub last_finished_date: Option<String>,
    #[serde(rename = "lastRun", skip_serializing_if = "Option::is_none")]
    pub last_run: Option<i64>,
    #[serde(rename = "lastRunDate", skip_serializing_if = "Option::is_none")]
    pub last_run_date: Option<String>,
    #[serde(rename = "lastRunSuccessful", skip_serializing_if = "Option::is_none")]
    pub last_run_successful: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub running: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestChangeList {
    #[serde(rename = "allElements", skip_serializing_if = "Option::is_none")]
    pub all_elements: Option<Vec<RestChange>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackRestChange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change: Option<Vec<RestChange>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
pub type ListWrapperCallbackRestChange = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestChange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(rename = "changeFiles", skip_serializing_if = "Option::is_none")]
    pub change_files: Option<RestChangeFileList>,
    #[serde(rename = "changesetId", skip_serializing_if = "Option::is_none")]
    pub changeset_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(rename = "commitUrl", skip_serializing_if = "Option::is_none")]
    pub commit_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(rename = "fullName", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(rename = "userName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestChangeFileList {
    #[serde(rename = "allElements", skip_serializing_if = "Option::is_none")]
    pub all_elements: Option<Vec<RestChangeFile>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackRestChangeFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<Vec<RestChangeFile>>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestChangeFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}
pub type ListWrapperCallbackRestChangeFile = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestBuildLabels {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<RestBuildLabelList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestBuildLabelList {
    #[serde(rename = "allElements", skip_serializing_if = "Option::is_none")]
    pub all_elements: Option<Vec<RestBuildLabel>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackRestBuildLabel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Vec<RestBuildLabel>>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestBuildLabel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
pub type ListWrapperCallbackRestBuildLabel = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestBuildArtifactList {
    #[serde(rename = "allElements", skip_serializing_if = "Option::is_none")]
    pub all_elements: Option<Vec<RestBuildArtifact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<RestBuildArtifact>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackRestBuildArtifact>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
pub type ListWrapperCallbackRestBuildArtifact = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestBuildArtifact {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "prettySizeDescription", skip_serializing_if = "Option::is_none")]
    pub pretty_size_description: Option<String>,
    #[serde(rename = "producerJobKey", skip_serializing_if = "Option::is_none")]
    pub producer_job_key: Option<PlanResultKey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestBranchList {
    #[serde(rename = "allElements", skip_serializing_if = "Option::is_none")]
    pub all_elements: Option<Vec<RestBranch>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackRestBranch>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(rename = "searchResults", skip_serializing_if = "Option::is_none")]
    pub search_results: Option<Vec<RestBranch>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestBranch {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
pub type ListWrapperCallbackRestBranch = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestArtifactDefinitions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<RestArtifactDefinitionList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestArtifactDefinitionList {
    #[serde(rename = "allElements", skip_serializing_if = "Option::is_none")]
    pub all_elements: Option<Vec<RestArtifactDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifacts: Option<Vec<RestArtifactDefinition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackRestArtifactDefinition>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestArtifactDefinition {
    #[serde(rename = "copyPatterns", skip_serializing_if = "Option::is_none")]
    pub copy_patterns: Option<Vec<String>>,
    #[serde(rename = "exclusionPatterns", skip_serializing_if = "Option::is_none")]
    pub exclusion_patterns: Option<Vec<String>>,
    #[serde(rename = "httpCompressionOn", skip_serializing_if = "Option::is_none")]
    pub http_compression_on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
}
pub type ListWrapperCallbackRestArtifactDefinition = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestAnonymousServerStatusInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<NodeLifecycleState>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestAgentInformation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<RestAgentCapabilities>,
    #[serde(rename = "executableEnvironments", skip_serializing_if = "Option::is_none")]
    pub executable_environments: Option<RestExecutableEnvironmentList>,
    #[serde(rename = "executableJobs", skip_serializing_if = "Option::is_none")]
    pub executable_jobs: Option<RestExecutableJobList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general: Option<RestBuildAgent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
    #[serde(
        rename = "restAgentExecutableEnvironments",
        skip_serializing_if = "Option::is_none"
    )]
    pub rest_agent_executable_environments: Option<RestExecutableEnvironmentList>,
    #[serde(rename = "restAgentExecutableJobs", skip_serializing_if = "Option::is_none")]
    pub rest_agent_executable_jobs: Option<RestExecutableJobList>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestExecutableJobList {
    #[serde(rename = "allElements", skip_serializing_if = "Option::is_none")]
    pub all_elements: Option<Vec<RestJob>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackRestJob>,
    #[serde(rename = "executableJobs", skip_serializing_if = "Option::is_none")]
    pub executable_jobs: Option<Vec<RestJob>>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestJob {
    #[serde(rename = "agentSummary", skip_serializing_if = "Option::is_none")]
    pub agent_summary: Option<RestAgentSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<RestKey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "sourceJobKey", skip_serializing_if = "Option::is_none")]
    pub source_job_key: Option<RestKey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<Box<RestStage>>,
    #[serde(rename = "stageId", skip_serializing_if = "Option::is_none")]
    pub stage_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspended: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestStage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plans: Option<Box<RestPlanList>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPlanList {
    #[serde(rename = "allElements", skip_serializing_if = "Option::is_none")]
    pub all_elements: Option<Vec<Box<RestPlan>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackRestPlan>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<Vec<Box<RestPlan>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPlan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<RestPlanActionList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(
        rename = "averageBuildTimeInSeconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub average_build_time_in_seconds: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branches: Option<RestPlanBranchList>,
    #[serde(rename = "buildName", skip_serializing_if = "Option::is_none")]
    pub build_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub building: Option<bool>,
    #[serde(rename = "currentRestUser", skip_serializing_if = "Option::is_none")]
    pub current_rest_user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favourite: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master: Option<Box<RestPlan>>,
    #[serde(rename = "parentKey", skip_serializing_if = "Option::is_none")]
    pub parent_key: Option<String>,
    #[serde(rename = "parentLink", skip_serializing_if = "Option::is_none")]
    pub parent_link: Option<Link>,
    #[serde(rename = "parentName", skip_serializing_if = "Option::is_none")]
    pub parent_name: Option<String>,
    #[serde(rename = "planKey", skip_serializing_if = "Option::is_none")]
    pub plan_key: Option<RestKey>,
    #[serde(rename = "planName", skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Box<RestProject>>,
    #[serde(rename = "projectKey", skip_serializing_if = "Option::is_none")]
    pub project_key: Option<String>,
    #[serde(rename = "projectName", skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(rename = "restPlanKey", skip_serializing_if = "Option::is_none")]
    pub rest_plan_key: Option<RestKey>,
    #[serde(rename = "shortKey", skip_serializing_if = "Option::is_none")]
    pub short_key: Option<String>,
    #[serde(rename = "shortName", skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    #[serde(rename = "stageName", skip_serializing_if = "Option::is_none")]
    pub stage_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stages: Option<Box<RestStageList>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "variableContext", skip_serializing_if = "Option::is_none")]
    pub variable_context: Option<RestVariableDefinitionContextList>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestVariableDefinitionContextList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackRestVariableDefinitionContext>,
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "pagingCallback", skip_serializing_if = "Option::is_none")]
    pub paging_callback: Option<ListWrapperCallbackRestVariableDefinitionContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
pub type ListWrapperCallbackRestVariableDefinitionContext = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestStageList {
    #[serde(rename = "allElements", skip_serializing_if = "Option::is_none")]
    pub all_elements: Option<Vec<Box<RestStage>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackRestStage>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<Vec<Box<RestStage>>>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
pub type ListWrapperCallbackRestStage = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestProject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<Link>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plans: Option<Box<RestPlanList>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Box<Project>>,
    #[serde(rename = "uriInfo", skip_serializing_if = "Option::is_none")]
    pub uri_info: Option<Box<RestProjectUriInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<User>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "fullName", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestProjectUriInfo {
    #[serde(rename = "absolutePath", skip_serializing_if = "Option::is_none")]
    pub absolute_path: Option<String>,
    #[serde(rename = "absolutePathBuilder", skip_serializing_if = "Option::is_none")]
    pub absolute_path_builder: Option<serde_json::Value>,
    #[serde(rename = "baseUri", skip_serializing_if = "Option::is_none")]
    pub base_uri: Option<String>,
    #[serde(rename = "baseUriBuilder", skip_serializing_if = "Option::is_none")]
    pub base_uri_builder: Option<serde_json::Value>,
    #[serde(rename = "matchedResources", skip_serializing_if = "Option::is_none")]
    pub matched_resources: Option<Vec<Box<RestProjectMatchedResourcesItem>>>,
    #[serde(rename = "matchedURIs", skip_serializing_if = "Option::is_none")]
    pub matched_uris: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "pathParameters", skip_serializing_if = "Option::is_none")]
    pub path_parameters: Option<serde_json::Value>,
    #[serde(rename = "pathSegments", skip_serializing_if = "Option::is_none")]
    pub path_segments: Option<Vec<Box<RestProjectPathSegmentsItem>>>,
    #[serde(rename = "queryParameters", skip_serializing_if = "Option::is_none")]
    pub query_parameters: Option<serde_json::Value>,
    #[serde(rename = "requestUri", skip_serializing_if = "Option::is_none")]
    pub request_uri: Option<String>,
    #[serde(rename = "requestUriBuilder", skip_serializing_if = "Option::is_none")]
    pub request_uri_builder: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestProjectPathSegmentsItem {
    #[serde(rename = "matrixParameters", skip_serializing_if = "Option::is_none")]
    pub matrix_parameters: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestProjectMatchedResourcesItem {}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPlanBranchList {
    #[serde(rename = "allElements", skip_serializing_if = "Option::is_none")]
    pub all_elements: Option<Vec<RestPlanBranch>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<Vec<RestPlanBranch>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackRestPlanBranch>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPlanBranch {
    #[serde(rename = "branchKey", skip_serializing_if = "Option::is_none")]
    pub branch_key: Option<String>,
    #[serde(rename = "branchName", skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favourite: Option<bool>,
}
pub type ListWrapperCallbackRestPlanBranch = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPlanActionList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<Vec<RestPlanAction>>,
    #[serde(rename = "allElements", skip_serializing_if = "Option::is_none")]
    pub all_elements: Option<Vec<RestPlanAction>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackRestPlanAction>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestPlanAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
pub type ListWrapperCallbackRestPlanAction = serde_json::Value;
pub type ListWrapperCallbackRestPlan = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestAgentSummary {
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
pub type ListWrapperCallbackRestJob = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestExecutableEnvironmentList {
    #[serde(rename = "allElements", skip_serializing_if = "Option::is_none")]
    pub all_elements: Option<Vec<RestEnvironment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackRestEnvironment>,
    #[serde(rename = "executableEnvironments", skip_serializing_if = "Option::is_none")]
    pub executable_environments: Option<Vec<RestEnvironment>>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
pub type ListWrapperCallbackRestEnvironment = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestAgentCapabilities {
    #[serde(rename = "allElements", skip_serializing_if = "Option::is_none")]
    pub all_elements: Option<Vec<RestCapability>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<ListWrapperCallbackRestCapability>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<RestCapability>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<String>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
pub type ListWrapperCallbackRestCapability = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NodeLifecycleState {
    #[serde(rename = "PAUSED", skip_serializing_if = "Option::is_none")]
    pub paused: Option<NodeLifecycleStatePAUSED>,
    #[serde(rename = "PAUSING", skip_serializing_if = "Option::is_none")]
    pub pausing: Option<NodeLifecycleStatePAUSING>,
    #[serde(rename = "PREPARING_FOR_RESTART", skip_serializing_if = "Option::is_none")]
    pub preparing_for_restart: Option<NodeLifecycleStatePREPARINGFORRESTART>,
    #[serde(rename = "READY_FOR_RESTART", skip_serializing_if = "Option::is_none")]
    pub ready_for_restart: Option<NodeLifecycleStateREADYFORRESTART>,
    #[serde(rename = "RUNNING", skip_serializing_if = "Option::is_none")]
    pub running: Option<NodeLifecycleStateRUNNING>,
    #[serde(rename = "RUNNING_AS_SECONDARY", skip_serializing_if = "Option::is_none")]
    pub running_as_secondary: Option<NodeLifecycleStateRUNNINGASSECONDARY>,
    #[serde(rename = "SETUP", skip_serializing_if = "Option::is_none")]
    pub setup: Option<NodeLifecycleStateSETUP>,
    #[serde(rename = "STARTING", skip_serializing_if = "Option::is_none")]
    pub starting: Option<NodeLifecycleStateSTARTING>,
    #[serde(
        rename = "correspondingClusterState",
        skip_serializing_if = "Option::is_none"
    )]
    pub corresponding_cluster_state: Option<NodeLifecycleStateCorrespondingClusterState>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum NodeLifecycleStateSTARTING {
    #[default]
    #[serde(rename = "SETUP")]
    Setup,
    #[serde(rename = "STARTING")]
    Starting,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "RUNNING_AS_SECONDARY")]
    RunningAsSecondary,
    #[serde(rename = "PAUSING")]
    Pausing,
    #[serde(rename = "PAUSED")]
    Paused,
    #[serde(rename = "PREPARING_FOR_RESTART")]
    PreparingForRestart,
    #[serde(rename = "READY_FOR_RESTART")]
    ReadyForRestart,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum NodeLifecycleStateSETUP {
    #[default]
    #[serde(rename = "SETUP")]
    Setup,
    #[serde(rename = "STARTING")]
    Starting,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "RUNNING_AS_SECONDARY")]
    RunningAsSecondary,
    #[serde(rename = "PAUSING")]
    Pausing,
    #[serde(rename = "PAUSED")]
    Paused,
    #[serde(rename = "PREPARING_FOR_RESTART")]
    PreparingForRestart,
    #[serde(rename = "READY_FOR_RESTART")]
    ReadyForRestart,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum NodeLifecycleStateRUNNINGASSECONDARY {
    #[default]
    #[serde(rename = "SETUP")]
    Setup,
    #[serde(rename = "STARTING")]
    Starting,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "RUNNING_AS_SECONDARY")]
    RunningAsSecondary,
    #[serde(rename = "PAUSING")]
    Pausing,
    #[serde(rename = "PAUSED")]
    Paused,
    #[serde(rename = "PREPARING_FOR_RESTART")]
    PreparingForRestart,
    #[serde(rename = "READY_FOR_RESTART")]
    ReadyForRestart,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum NodeLifecycleStateRUNNING {
    #[default]
    #[serde(rename = "SETUP")]
    Setup,
    #[serde(rename = "STARTING")]
    Starting,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "RUNNING_AS_SECONDARY")]
    RunningAsSecondary,
    #[serde(rename = "PAUSING")]
    Pausing,
    #[serde(rename = "PAUSED")]
    Paused,
    #[serde(rename = "PREPARING_FOR_RESTART")]
    PreparingForRestart,
    #[serde(rename = "READY_FOR_RESTART")]
    ReadyForRestart,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum NodeLifecycleStateREADYFORRESTART {
    #[default]
    #[serde(rename = "SETUP")]
    Setup,
    #[serde(rename = "STARTING")]
    Starting,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "RUNNING_AS_SECONDARY")]
    RunningAsSecondary,
    #[serde(rename = "PAUSING")]
    Pausing,
    #[serde(rename = "PAUSED")]
    Paused,
    #[serde(rename = "PREPARING_FOR_RESTART")]
    PreparingForRestart,
    #[serde(rename = "READY_FOR_RESTART")]
    ReadyForRestart,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum NodeLifecycleStatePREPARINGFORRESTART {
    #[default]
    #[serde(rename = "SETUP")]
    Setup,
    #[serde(rename = "STARTING")]
    Starting,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "RUNNING_AS_SECONDARY")]
    RunningAsSecondary,
    #[serde(rename = "PAUSING")]
    Pausing,
    #[serde(rename = "PAUSED")]
    Paused,
    #[serde(rename = "PREPARING_FOR_RESTART")]
    PreparingForRestart,
    #[serde(rename = "READY_FOR_RESTART")]
    ReadyForRestart,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum NodeLifecycleStatePAUSING {
    #[default]
    #[serde(rename = "SETUP")]
    Setup,
    #[serde(rename = "STARTING")]
    Starting,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "RUNNING_AS_SECONDARY")]
    RunningAsSecondary,
    #[serde(rename = "PAUSING")]
    Pausing,
    #[serde(rename = "PAUSED")]
    Paused,
    #[serde(rename = "PREPARING_FOR_RESTART")]
    PreparingForRestart,
    #[serde(rename = "READY_FOR_RESTART")]
    ReadyForRestart,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum NodeLifecycleStatePAUSED {
    #[default]
    #[serde(rename = "SETUP")]
    Setup,
    #[serde(rename = "STARTING")]
    Starting,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "RUNNING_AS_SECONDARY")]
    RunningAsSecondary,
    #[serde(rename = "PAUSING")]
    Pausing,
    #[serde(rename = "PAUSED")]
    Paused,
    #[serde(rename = "PREPARING_FOR_RESTART")]
    PreparingForRestart,
    #[serde(rename = "READY_FOR_RESTART")]
    ReadyForRestart,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum NodeLifecycleStateCorrespondingClusterState {
    #[default]
    #[serde(rename = "STARTING")]
    Starting,
    #[serde(rename = "RUNNING")]
    Running,
    #[serde(rename = "PAUSING")]
    Pausing,
    #[serde(rename = "PAUSED")]
    Paused,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListUserPermissionsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestUserPermission>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListUserPermissions6Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestUserPermission>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListUserPermissions5Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestUserPermission>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListUserPermissions4Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestUserPermission>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListUserPermissions3Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestUserPermission>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListUserPermissions2Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestUserPermission>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListUserPermissions1Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestUserPermission>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestUserPermission {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "fullName", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "sanitizedName", skip_serializing_if = "Option::is_none")]
    pub sanitized_name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListRolePermissionsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestRolePermission>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListRolePermissions6Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestRolePermission>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListRolePermissions5Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestRolePermission>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListRolePermissions4Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestRolePermission>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListRolePermissions3Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestRolePermission>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListRolePermissions2Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestRolePermission>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListRolePermissions1Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestRolePermission>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestRolePermission {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListGroupPermissionsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestGroupPermission>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListGroupPermissions6Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestGroupPermission>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListGroupPermissions5Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestGroupPermission>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListGroupPermissions4Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestGroupPermission>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListGroupPermissions3Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestGroupPermission>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListGroupPermissions2Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestGroupPermission>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListGroupPermissions1Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestGroupPermission>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestGroupPermission {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
pub type ListAssignedRepositoriesResponse = Vec<RestRepository>;
pub type ListAssignedRepositories1Response = Vec<RestRepository>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsonSearchResultsList {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json: Option<JsonElement>,
    #[serde(rename = "maxResult", skip_serializing_if = "Option::is_none")]
    pub max_result: Option<i32>,
    #[serde(rename = "searchResults", skip_serializing_if = "Option::is_none")]
    pub search_results: Option<Vec<JsonElement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "startIndex", skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsonElement {
    #[serde(rename = "asBigDecimal", skip_serializing_if = "Option::is_none")]
    pub as_big_decimal: Option<f64>,
    #[serde(rename = "asBigInteger", skip_serializing_if = "Option::is_none")]
    pub as_big_integer: Option<i64>,
    #[serde(rename = "asBoolean", skip_serializing_if = "Option::is_none")]
    pub as_boolean: Option<bool>,
    #[serde(rename = "asByte", skip_serializing_if = "Option::is_none")]
    pub as_byte: Option<String>,
    #[serde(rename = "asCharacter", skip_serializing_if = "Option::is_none")]
    pub as_character: Option<String>,
    #[serde(rename = "asDouble", skip_serializing_if = "Option::is_none")]
    pub as_double: Option<f64>,
    #[serde(rename = "asFloat", skip_serializing_if = "Option::is_none")]
    pub as_float: Option<f32>,
    #[serde(rename = "asInt", skip_serializing_if = "Option::is_none")]
    pub as_int: Option<i32>,
    #[serde(rename = "asJsonArray", skip_serializing_if = "Option::is_none")]
    pub as_json_array: Option<Box<JsonArray>>,
    #[serde(rename = "asJsonNull", skip_serializing_if = "Option::is_none")]
    pub as_json_null: Option<Box<JsonNull>>,
    #[serde(rename = "asJsonObject", skip_serializing_if = "Option::is_none")]
    pub as_json_object: Option<Box<JsonObject>>,
    #[serde(rename = "asJsonPrimitive", skip_serializing_if = "Option::is_none")]
    pub as_json_primitive: Option<Box<JsonPrimitive>>,
    #[serde(rename = "asLong", skip_serializing_if = "Option::is_none")]
    pub as_long: Option<i64>,
    #[serde(rename = "asNumber", skip_serializing_if = "Option::is_none")]
    pub as_number: Option<f64>,
    #[serde(rename = "asShort", skip_serializing_if = "Option::is_none")]
    pub as_short: Option<i32>,
    #[serde(rename = "asString", skip_serializing_if = "Option::is_none")]
    pub as_string: Option<String>,
    #[serde(rename = "jsonArray", skip_serializing_if = "Option::is_none")]
    pub json_array: Option<bool>,
    #[serde(rename = "jsonNull", skip_serializing_if = "Option::is_none")]
    pub json_null: Option<bool>,
    #[serde(rename = "jsonObject", skip_serializing_if = "Option::is_none")]
    pub json_object: Option<bool>,
    #[serde(rename = "jsonPrimitive", skip_serializing_if = "Option::is_none")]
    pub json_primitive: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsonArray {
    #[serde(rename = "asBigDecimal", skip_serializing_if = "Option::is_none")]
    pub as_big_decimal: Option<f64>,
    #[serde(rename = "asBigInteger", skip_serializing_if = "Option::is_none")]
    pub as_big_integer: Option<i64>,
    #[serde(rename = "asBoolean", skip_serializing_if = "Option::is_none")]
    pub as_boolean: Option<bool>,
    #[serde(rename = "asByte", skip_serializing_if = "Option::is_none")]
    pub as_byte: Option<String>,
    #[serde(rename = "asCharacter", skip_serializing_if = "Option::is_none")]
    pub as_character: Option<String>,
    #[serde(rename = "asDouble", skip_serializing_if = "Option::is_none")]
    pub as_double: Option<f64>,
    #[serde(rename = "asFloat", skip_serializing_if = "Option::is_none")]
    pub as_float: Option<f32>,
    #[serde(rename = "asInt", skip_serializing_if = "Option::is_none")]
    pub as_int: Option<i32>,
    #[serde(rename = "asJsonArray", skip_serializing_if = "Option::is_none")]
    pub as_json_array: Option<Box<JsonArray>>,
    #[serde(rename = "asJsonNull", skip_serializing_if = "Option::is_none")]
    pub as_json_null: Option<Box<JsonNull>>,
    #[serde(rename = "asJsonObject", skip_serializing_if = "Option::is_none")]
    pub as_json_object: Option<Box<JsonObject>>,
    #[serde(rename = "asJsonPrimitive", skip_serializing_if = "Option::is_none")]
    pub as_json_primitive: Option<Box<JsonPrimitive>>,
    #[serde(rename = "asLong", skip_serializing_if = "Option::is_none")]
    pub as_long: Option<i64>,
    #[serde(rename = "asNumber", skip_serializing_if = "Option::is_none")]
    pub as_number: Option<f64>,
    #[serde(rename = "asShort", skip_serializing_if = "Option::is_none")]
    pub as_short: Option<i32>,
    #[serde(rename = "asString", skip_serializing_if = "Option::is_none")]
    pub as_string: Option<String>,
    #[serde(rename = "jsonArray", skip_serializing_if = "Option::is_none")]
    pub json_array: Option<bool>,
    #[serde(rename = "jsonNull", skip_serializing_if = "Option::is_none")]
    pub json_null: Option<bool>,
    #[serde(rename = "jsonObject", skip_serializing_if = "Option::is_none")]
    pub json_object: Option<bool>,
    #[serde(rename = "jsonPrimitive", skip_serializing_if = "Option::is_none")]
    pub json_primitive: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsonNull {
    #[serde(rename = "asBigDecimal", skip_serializing_if = "Option::is_none")]
    pub as_big_decimal: Option<f64>,
    #[serde(rename = "asBigInteger", skip_serializing_if = "Option::is_none")]
    pub as_big_integer: Option<i64>,
    #[serde(rename = "asBoolean", skip_serializing_if = "Option::is_none")]
    pub as_boolean: Option<bool>,
    #[serde(rename = "asByte", skip_serializing_if = "Option::is_none")]
    pub as_byte: Option<String>,
    #[serde(rename = "asCharacter", skip_serializing_if = "Option::is_none")]
    pub as_character: Option<String>,
    #[serde(rename = "asDouble", skip_serializing_if = "Option::is_none")]
    pub as_double: Option<f64>,
    #[serde(rename = "asFloat", skip_serializing_if = "Option::is_none")]
    pub as_float: Option<f32>,
    #[serde(rename = "asInt", skip_serializing_if = "Option::is_none")]
    pub as_int: Option<i32>,
    #[serde(rename = "asJsonArray", skip_serializing_if = "Option::is_none")]
    pub as_json_array: Option<Box<JsonArray>>,
    #[serde(rename = "asJsonNull", skip_serializing_if = "Option::is_none")]
    pub as_json_null: Option<Box<JsonNull>>,
    #[serde(rename = "asJsonObject", skip_serializing_if = "Option::is_none")]
    pub as_json_object: Option<Box<JsonObject>>,
    #[serde(rename = "asJsonPrimitive", skip_serializing_if = "Option::is_none")]
    pub as_json_primitive: Option<Box<JsonPrimitive>>,
    #[serde(rename = "asLong", skip_serializing_if = "Option::is_none")]
    pub as_long: Option<i64>,
    #[serde(rename = "asNumber", skip_serializing_if = "Option::is_none")]
    pub as_number: Option<f64>,
    #[serde(rename = "asShort", skip_serializing_if = "Option::is_none")]
    pub as_short: Option<i32>,
    #[serde(rename = "asString", skip_serializing_if = "Option::is_none")]
    pub as_string: Option<String>,
    #[serde(rename = "jsonArray", skip_serializing_if = "Option::is_none")]
    pub json_array: Option<bool>,
    #[serde(rename = "jsonNull", skip_serializing_if = "Option::is_none")]
    pub json_null: Option<bool>,
    #[serde(rename = "jsonObject", skip_serializing_if = "Option::is_none")]
    pub json_object: Option<bool>,
    #[serde(rename = "jsonPrimitive", skip_serializing_if = "Option::is_none")]
    pub json_primitive: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsonObject {
    #[serde(rename = "asBigDecimal", skip_serializing_if = "Option::is_none")]
    pub as_big_decimal: Option<f64>,
    #[serde(rename = "asBigInteger", skip_serializing_if = "Option::is_none")]
    pub as_big_integer: Option<i64>,
    #[serde(rename = "asBoolean", skip_serializing_if = "Option::is_none")]
    pub as_boolean: Option<bool>,
    #[serde(rename = "asByte", skip_serializing_if = "Option::is_none")]
    pub as_byte: Option<String>,
    #[serde(rename = "asCharacter", skip_serializing_if = "Option::is_none")]
    pub as_character: Option<String>,
    #[serde(rename = "asDouble", skip_serializing_if = "Option::is_none")]
    pub as_double: Option<f64>,
    #[serde(rename = "asFloat", skip_serializing_if = "Option::is_none")]
    pub as_float: Option<f32>,
    #[serde(rename = "asInt", skip_serializing_if = "Option::is_none")]
    pub as_int: Option<i32>,
    #[serde(rename = "asJsonArray", skip_serializing_if = "Option::is_none")]
    pub as_json_array: Option<Box<JsonArray>>,
    #[serde(rename = "asJsonNull", skip_serializing_if = "Option::is_none")]
    pub as_json_null: Option<Box<JsonNull>>,
    #[serde(rename = "asJsonObject", skip_serializing_if = "Option::is_none")]
    pub as_json_object: Option<Box<JsonObject>>,
    #[serde(rename = "asJsonPrimitive", skip_serializing_if = "Option::is_none")]
    pub as_json_primitive: Option<Box<JsonPrimitive>>,
    #[serde(rename = "asLong", skip_serializing_if = "Option::is_none")]
    pub as_long: Option<i64>,
    #[serde(rename = "asNumber", skip_serializing_if = "Option::is_none")]
    pub as_number: Option<f64>,
    #[serde(rename = "asShort", skip_serializing_if = "Option::is_none")]
    pub as_short: Option<i32>,
    #[serde(rename = "asString", skip_serializing_if = "Option::is_none")]
    pub as_string: Option<String>,
    #[serde(rename = "jsonArray", skip_serializing_if = "Option::is_none")]
    pub json_array: Option<bool>,
    #[serde(rename = "jsonNull", skip_serializing_if = "Option::is_none")]
    pub json_null: Option<bool>,
    #[serde(rename = "jsonObject", skip_serializing_if = "Option::is_none")]
    pub json_object: Option<bool>,
    #[serde(rename = "jsonPrimitive", skip_serializing_if = "Option::is_none")]
    pub json_primitive: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JsonPrimitive {
    #[serde(rename = "asBigDecimal", skip_serializing_if = "Option::is_none")]
    pub as_big_decimal: Option<f64>,
    #[serde(rename = "asBigInteger", skip_serializing_if = "Option::is_none")]
    pub as_big_integer: Option<i64>,
    #[serde(rename = "asBoolean", skip_serializing_if = "Option::is_none")]
    pub as_boolean: Option<bool>,
    #[serde(rename = "asByte", skip_serializing_if = "Option::is_none")]
    pub as_byte: Option<String>,
    #[serde(rename = "asCharacter", skip_serializing_if = "Option::is_none")]
    pub as_character: Option<String>,
    #[serde(rename = "asDouble", skip_serializing_if = "Option::is_none")]
    pub as_double: Option<f64>,
    #[serde(rename = "asFloat", skip_serializing_if = "Option::is_none")]
    pub as_float: Option<f32>,
    #[serde(rename = "asInt", skip_serializing_if = "Option::is_none")]
    pub as_int: Option<i32>,
    #[serde(rename = "asJsonArray", skip_serializing_if = "Option::is_none")]
    pub as_json_array: Option<Box<JsonArray>>,
    #[serde(rename = "asJsonNull", skip_serializing_if = "Option::is_none")]
    pub as_json_null: Option<Box<JsonNull>>,
    #[serde(rename = "asJsonObject", skip_serializing_if = "Option::is_none")]
    pub as_json_object: Option<Box<JsonObject>>,
    #[serde(rename = "asJsonPrimitive", skip_serializing_if = "Option::is_none")]
    pub as_json_primitive: Option<Box<JsonPrimitive>>,
    #[serde(rename = "asLong", skip_serializing_if = "Option::is_none")]
    pub as_long: Option<i64>,
    #[serde(rename = "asNumber", skip_serializing_if = "Option::is_none")]
    pub as_number: Option<f64>,
    #[serde(rename = "asShort", skip_serializing_if = "Option::is_none")]
    pub as_short: Option<i32>,
    #[serde(rename = "asString", skip_serializing_if = "Option::is_none")]
    pub as_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boolean: Option<bool>,
    #[serde(rename = "jsonArray", skip_serializing_if = "Option::is_none")]
    pub json_array: Option<bool>,
    #[serde(rename = "jsonNull", skip_serializing_if = "Option::is_none")]
    pub json_null: Option<bool>,
    #[serde(rename = "jsonObject", skip_serializing_if = "Option::is_none")]
    pub json_object: Option<bool>,
    #[serde(rename = "jsonPrimitive", skip_serializing_if = "Option::is_none")]
    pub json_primitive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<bool>,
}
pub type GetVisibleFiltersResponse = Vec<RestQuickFilter>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetUsersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestUserDetails>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetUserTokensResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestAccessToken>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetUserRepositoryAliasesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestUserAlias>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetUserAccessTokensResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestAccessToken>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestAccessToken {
    #[serde(rename = "creationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "daysUntilExpiry", skip_serializing_if = "Option::is_none")]
    pub days_until_expiry: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "rawToken", skip_serializing_if = "Option::is_none")]
    pub raw_token: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "tokenId", skip_serializing_if = "Option::is_none")]
    pub token_id: Option<String>,
}
pub type GetTemplateConfigurationsPageResponse = Vec<RestEphemeralAgentTemplate>;
pub type GetRssRepositoriesAllowedToAccessRepositoryResponse = Vec<RestRepository>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestRepository {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "projectKey", skip_serializing_if = "Option::is_none")]
    pub project_key: Option<String>,
    #[serde(rename = "projectName", skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
    #[serde(rename = "rssEnabled", skip_serializing_if = "Option::is_none")]
    pub rss_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
pub type GetRequirementsForEnvironmentResponse = Vec<RestRequirement>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestRequirement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "matchValue", skip_serializing_if = "Option::is_none")]
    pub match_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<RequirementOperations>,
    #[serde(rename = "readonlyData", skip_serializing_if = "Option::is_none")]
    pub readonly_data: Option<RequirementReadonlyData>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "typeOfMatch", skip_serializing_if = "Option::is_none")]
    pub type_of_match: Option<RestRequirementTypeOfMatch>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestRequirementTypeOfMatch {
    #[default]
    #[serde(rename = "EXISTS")]
    Exists,
    #[serde(rename = "EQUALS")]
    Equals,
    #[serde(rename = "MATCHES")]
    Matches,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequirementReadonlyData {
    #[serde(rename = "matchingAgents", skip_serializing_if = "Option::is_none")]
    pub matching_agents: Option<i32>,
    #[serde(rename = "matchingImages", skip_serializing_if = "Option::is_none")]
    pub matching_images: Option<i32>,
    #[serde(rename = "matchingTemplates", skip_serializing_if = "Option::is_none")]
    pub matching_templates: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequirementOperations {
    #[serde(rename = "canDelete", skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<bool>,
}
pub type GetRemoteAgentsResponse = Vec<RestBuildAgent>;
pub type GetQuickFiltersResponse = Vec<RestQuickFilter>;
pub type GetProjectVariablesResponse = Vec<RestVariable>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestVariable {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetPaginatedProjectSharedCredentialsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestSharedCredential>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestSharedCredential {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "projectKey", skip_serializing_if = "Option::is_none")]
    pub project_key: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetPaginatedProjectRepositoriesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestProjectRepository>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestProjectRepository {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetPaginateDeploymentProjectResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestDeploymentProject>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetMyBrokenBuildsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestBrokenPlan>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
pub type GetJobsResponse = Vec<RestScheduledJob>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestScheduledJob {
    #[serde(rename = "groupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nextScheduledTime", skip_serializing_if = "Option::is_none")]
    pub next_scheduled_time: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetGroupsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestGroup>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
pub type GetGroupResponse = Vec<RestGroup>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetEphemeralAgentPodLogsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestEphemeralPodLogs>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestEphemeralPodLogs {
    #[serde(rename = "containerName", skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[serde(rename = "lastTimeStamp", skip_serializing_if = "Option::is_none")]
    pub last_time_stamp: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<String>>,
    #[serde(rename = "podName", skip_serializing_if = "Option::is_none")]
    pub pod_name: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetEnvironmentsExecutableByEphemeralAgentTemplateResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestEnvironmentForExecutablesView>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetEnvironmentsExecutableByElasticConfigurationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestEnvironmentForExecutablesView>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetEnvironmentsExecutableByAgentResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestEnvironmentForExecutablesView>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestEnvironmentForExecutablesView {
    #[serde(rename = "environmentId", skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<i64>,
    #[serde(rename = "environmentName", skip_serializing_if = "Option::is_none")]
    pub environment_name: Option<String>,
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
    #[serde(rename = "projectName", skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,
}
pub type GetEnvironmentStatutesResponse = Vec<RestEnvironmentStatusForDashboard>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestEnvironmentStatusForDashboard {
    #[serde(rename = "deploymentResultId", skip_serializing_if = "Option::is_none")]
    pub deployment_result_id: Option<i64>,
    #[serde(rename = "deploymentState", skip_serializing_if = "Option::is_none")]
    pub deployment_state: Option<RestEnvironmentStatusForDashboardDeploymentState>,
    #[serde(rename = "deploymentVersionId", skip_serializing_if = "Option::is_none")]
    pub deployment_version_id: Option<i64>,
    #[serde(rename = "deploymentVersionName", skip_serializing_if = "Option::is_none")]
    pub deployment_version_name: Option<String>,
    #[serde(
        rename = "deploymentVersionRelatedBranchName",
        skip_serializing_if = "Option::is_none"
    )]
    pub deployment_version_related_branch_name: Option<String>,
    #[serde(
        rename = "deploymentVersionStatuses",
        skip_serializing_if = "Option::is_none"
    )]
    pub deployment_version_statuses: Option<
        Vec<RestDeploymentVersionStatusForDashboard>,
    >,
    #[serde(rename = "environmentId", skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<i64>,
    #[serde(rename = "executedDate", skip_serializing_if = "Option::is_none")]
    pub executed_date: Option<String>,
    #[serde(rename = "finishedDate", skip_serializing_if = "Option::is_none")]
    pub finished_date: Option<String>,
    #[serde(rename = "lifeCycleState", skip_serializing_if = "Option::is_none")]
    pub life_cycle_state: Option<RestEnvironmentStatusForDashboardLifeCycleState>,
    #[serde(rename = "queuedDate", skip_serializing_if = "Option::is_none")]
    pub queued_date: Option<String>,
    #[serde(rename = "startedDate", skip_serializing_if = "Option::is_none")]
    pub started_date: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestEnvironmentStatusForDashboardLifeCycleState {
    #[default]
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Queued")]
    Queued,
    #[serde(rename = "InProgress")]
    InProgress,
    #[serde(rename = "Finished")]
    Finished,
    #[serde(rename = "NotBuilt")]
    NotBuilt,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestEnvironmentStatusForDashboardDeploymentState {
    #[default]
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Successful")]
    Successful,
    #[serde(rename = "Failed")]
    Failed,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestDeploymentVersionStatusForDashboard {
    #[serde(rename = "deploymentVersionState", skip_serializing_if = "Option::is_none")]
    pub deployment_version_state: Option<
        RestDeploymentVersionStatusForDashboardDeploymentVersionState,
    >,
    #[serde(
        rename = "versionStatusSanitizedUserName",
        skip_serializing_if = "Option::is_none"
    )]
    pub version_status_sanitized_user_name: Option<String>,
    #[serde(
        rename = "versionStatusUserDisplayName",
        skip_serializing_if = "Option::is_none"
    )]
    pub version_status_user_display_name: Option<String>,
    #[serde(rename = "versionStatusUserName", skip_serializing_if = "Option::is_none")]
    pub version_status_user_name: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestDeploymentVersionStatusForDashboardDeploymentVersionState {
    #[default]
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Approved")]
    Approved,
    #[serde(rename = "Broken")]
    Broken,
    #[serde(rename = "Incomplete")]
    Incomplete,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetDeploymentProjectsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestDeploymentProject>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
pub type GetDeploymentProjectsForPlanResponse = Vec<RestLinkedDeploymentProject>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestLinkedDeploymentProject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
pub type GetDeploymentProjectResponse = Vec<DashboardProjectWithEnvironmentStatus>;
pub type GetCapabilitiesResponse = Vec<RestEphemeralAgentTemplate>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestEphemeralAgentTemplate {
    #[serde(rename = "configurationId", skip_serializing_if = "Option::is_none")]
    pub configuration_id: Option<i64>,
    #[serde(rename = "configurationName", skip_serializing_if = "Option::is_none")]
    pub configuration_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetBrokenBuildsForUserResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestBrokenPlan>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestBrokenPlan {
    #[serde(rename = "planResultKey", skip_serializing_if = "Option::is_none")]
    pub plan_result_key: Option<String>,
    #[serde(rename = "responsibleUsers", skip_serializing_if = "Option::is_none")]
    pub responsible_users: Option<Vec<RestUserResponsible>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestUserResponsible {
    #[serde(rename = "assignedBy", skip_serializing_if = "Option::is_none")]
    pub assigned_by: Option<String>,
    #[serde(rename = "assignedUser", skip_serializing_if = "Option::is_none")]
    pub assigned_user: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetAvailableUsersResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestUser>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetAvailableUsers6Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestUser>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetAvailableUsers5Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestUser>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetAvailableUsers4Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestUser>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetAvailableUsers3Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestUser>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetAvailableUsers2Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestUser>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetAvailableUsers1Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestUser>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "fullName", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "sanitizedName", skip_serializing_if = "Option::is_none")]
    pub sanitized_name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetAvailableGroupsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestGroup>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetAvailableGroups6Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestGroup>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetAvailableGroups5Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestGroup>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetAvailableGroups4Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestGroup>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetAvailableGroups3Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestGroup>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetAvailableGroups2Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestGroup>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetAvailableGroups1Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestGroup>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
pub type GetAllResponse = Vec<RestDarkFeature>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestDarkFeature {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
pub type GetAllDeploymentProjectsResponse = Vec<DashboardProjectWithEnvironmentStatus>;
pub type GetAllDeploymentProjects1Response = Vec<RestDeploymentProject>;
pub type GetAll1Response = Vec<RestElasticImageConfig>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestElasticImageConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub architecture: Option<String>,
    #[serde(rename = "availabilityZone", skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(
        rename = "configurationDescription",
        skip_serializing_if = "Option::is_none"
    )]
    pub configuration_description: Option<String>,
    #[serde(rename = "configurationId", skip_serializing_if = "Option::is_none")]
    pub configuration_id: Option<i64>,
    #[serde(rename = "configurationName", skip_serializing_if = "Option::is_none")]
    pub configuration_name: Option<String>,
    #[serde(rename = "ebsEnabled", skip_serializing_if = "Option::is_none")]
    pub ebs_enabled: Option<bool>,
    #[serde(rename = "ebsOptimised", skip_serializing_if = "Option::is_none")]
    pub ebs_optimised: Option<bool>,
    #[serde(rename = "ebsSnapshotId", skip_serializing_if = "Option::is_none")]
    pub ebs_snapshot_id: Option<String>,
    #[serde(
        rename = "iamInstanceProfileArnOrName",
        skip_serializing_if = "Option::is_none"
    )]
    pub iam_instance_profile_arn_or_name: Option<String>,
    #[serde(rename = "imageFilesVersion", skip_serializing_if = "Option::is_none")]
    pub image_files_version: Option<String>,
    #[serde(rename = "imageId", skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "instanceType", skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(
        rename = "legacyEbsHandlingEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub legacy_ebs_handling_enabled: Option<bool>,
    #[serde(rename = "perSecondBillingEnabled", skip_serializing_if = "Option::is_none")]
    pub per_second_billing_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "rootDeviceType", skip_serializing_if = "Option::is_none")]
    pub root_device_type: Option<String>,
    #[serde(rename = "rootFsSizeOverride", skip_serializing_if = "Option::is_none")]
    pub root_fs_size_override: Option<i32>,
    #[serde(rename = "startupScripts", skip_serializing_if = "Option::is_none")]
    pub startup_scripts: Option<Vec<String>>,
    #[serde(rename = "subnetId", skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    #[serde(rename = "useLegacyEbsHandling", skip_serializing_if = "Option::is_none")]
    pub use_legacy_ebs_handling: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetAgentsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestAgent>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestAgent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
pub type GetAgents1Response = Vec<RestBuildAgent>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestBuildAgent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub busy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<RestBuildAgentType>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestBuildAgentType {
    #[default]
    #[serde(rename = "LOCAL")]
    Local,
    #[serde(rename = "REMOTE")]
    Remote,
    #[serde(rename = "ELASTIC")]
    Elastic,
    #[serde(rename = "EPHEMERAL")]
    Ephemeral,
}
pub type GetAgentCapabilitiesResponse = Vec<RestCapability>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
pub type GetAgentAuthenticationsResponse = Vec<RestRemoteAgentAuthentication>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestRemoteAgentAuthentication {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(rename = "ipPatterns", skip_serializing_if = "Option::is_none")]
    pub ip_patterns: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}
pub type GetAgentAssignmentsResponse = Vec<RestDedicatedAgent>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestDedicatedAgent {
    #[serde(rename = "executableId", skip_serializing_if = "Option::is_none")]
    pub executable_id: Option<i64>,
    #[serde(rename = "executableType", skip_serializing_if = "Option::is_none")]
    pub executable_type: Option<RestDedicatedAgentExecutableType>,
    #[serde(rename = "executorId", skip_serializing_if = "Option::is_none")]
    pub executor_id: Option<i64>,
    #[serde(rename = "executorType", skip_serializing_if = "Option::is_none")]
    pub executor_type: Option<RestDedicatedAgentExecutorType>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestDedicatedAgentExecutorType {
    #[default]
    #[serde(rename = "AGENT")]
    Agent,
    #[serde(rename = "IMAGE")]
    Image,
    #[serde(rename = "EPHEMERAL")]
    Ephemeral,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestDedicatedAgentExecutableType {
    #[default]
    #[serde(rename = "Build project")]
    BuildProject,
    #[serde(rename = "Build plan")]
    BuildPlan,
    #[serde(rename = "Build job")]
    BuildJob,
    #[serde(rename = "Deployment project")]
    DeploymentProject,
    #[serde(rename = "Deployment environment")]
    DeploymentEnvironment,
}
pub type GetActiveFiltersResponse = Vec<RestQuickFilter>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestQuickFilter {
    #[serde(rename = "hasRules", skip_serializing_if = "Option::is_none")]
    pub has_rules: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FindUsersNotInGroupResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestUserDetails>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FindUsersInGroupResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestUserDetails>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestUserDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "fullName", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "sanitizedName", skip_serializing_if = "Option::is_none")]
    pub sanitized_name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FindUnassignedUserRepositoryAliasesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestUserAlias>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestUserAlias {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FindUnassignedGroupsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestGroup>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FindPlansWithCustomExpirySettingsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<PlanWithCustomExpirySettings>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlanWithCustomExpirySettings {
    #[serde(rename = "configLink", skip_serializing_if = "Option::is_none")]
    pub config_link: Option<Link>,
    #[serde(rename = "expiryConfig", skip_serializing_if = "Option::is_none")]
    pub expiry_config: Option<ExpiryConfig>,
    #[serde(rename = "planKey", skip_serializing_if = "Option::is_none")]
    pub plan_key: Option<String>,
    #[serde(rename = "planName", skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Link {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rel: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExpiryConfig {
    #[serde(rename = "buildsToKeep", skip_serializing_if = "Option::is_none")]
    pub builds_to_keep: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "expiryBuildLog", skip_serializing_if = "Option::is_none")]
    pub expiry_build_log: Option<bool>,
    #[serde(rename = "expiryTypeArtifact", skip_serializing_if = "Option::is_none")]
    pub expiry_type_artifact: Option<bool>,
    #[serde(rename = "expiryTypeNothing", skip_serializing_if = "Option::is_none")]
    pub expiry_type_nothing: Option<bool>,
    #[serde(rename = "expiryTypeResult", skip_serializing_if = "Option::is_none")]
    pub expiry_type_result: Option<bool>,
    #[serde(rename = "labelsList", skip_serializing_if = "Option::is_none")]
    pub labels_list: Option<String>,
    #[serde(rename = "maximumBuildsToKeep", skip_serializing_if = "Option::is_none")]
    pub maximum_builds_to_keep: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FindAssignedGroupsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<RestGroup>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
pub type FindAssignedAgentsByJobResponse = Vec<RestAgentAssignmentExecutorDetails>;
pub type FindAssignedAgentsByEnvironmentResponse = Vec<
    RestAgentAssignmentExecutorDetails,
>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestAgentAssignmentExecutorDetails {
    #[serde(rename = "agentType", skip_serializing_if = "Option::is_none")]
    pub agent_type: Option<RestAgentAssignmentExecutorDetailsAgentType>,
    #[serde(rename = "capabilitiesMatch", skip_serializing_if = "Option::is_none")]
    pub capabilities_match: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removable: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<RestAgentAssignmentExecutorDetailsType>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestAgentAssignmentExecutorDetailsType {
    #[default]
    #[serde(rename = "AGENT")]
    Agent,
    #[serde(rename = "IMAGE")]
    Image,
    #[serde(rename = "EPHEMERAL")]
    Ephemeral,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestAgentAssignmentExecutorDetailsAgentType {
    #[default]
    #[serde(rename = "LOCAL")]
    Local,
    #[serde(rename = "REMOTE")]
    Remote,
    #[serde(rename = "ELASTIC")]
    Elastic,
    #[serde(rename = "EPHEMERAL")]
    Ephemeral,
}
pub type FindAllResponse = Vec<RestTrustedKey>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestTrustedKey {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Expansion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<bool>,
    #[serde(rename = "subExpansions", skip_serializing_if = "Option::is_none")]
    pub sub_expansions: Option<Vec<Box<Expansion>>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EphemeralAgentsConfigurationDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "pathToConfig", skip_serializing_if = "Option::is_none")]
    pub path_to_config: Option<String>,
    #[serde(rename = "podsCleanup", skip_serializing_if = "Option::is_none")]
    pub pods_cleanup: Option<PodsCleanup>,
    #[serde(
        rename = "waitForEphemeralAgentDuration",
        skip_serializing_if = "Option::is_none"
    )]
    pub wait_for_ephemeral_agent_duration: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PodsCleanup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delay: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DirectoryInformationResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<DirectoryInformationResult>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DirectoryInformationResult {
    #[serde(rename = "artifactPlanRoots", skip_serializing_if = "Option::is_none")]
    pub artifact_plan_roots: Option<Vec<String>>,
    #[serde(rename = "buildLogJobRoots", skip_serializing_if = "Option::is_none")]
    pub build_log_job_roots: Option<serde_json::Value>,
    #[serde(rename = "isBranchBuild", skip_serializing_if = "Option::is_none")]
    pub is_branch_build: Option<bool>,
    #[serde(rename = "planName", skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[serde(rename = "storageTag", skip_serializing_if = "Option::is_none")]
    pub storage_tag: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DashboardProjectWithEnvironmentStatus {
    #[serde(rename = "deploymentProject", skip_serializing_if = "Option::is_none")]
    pub deployment_project: Option<RestDeploymentProject>,
    #[serde(rename = "environmentStatuses", skip_serializing_if = "Option::is_none")]
    pub environment_statuses: Option<Vec<RestEnvironmentDashboardStatus>>,
    #[serde(rename = "futureVersion", skip_serializing_if = "Option::is_none")]
    pub future_version: Option<RestDeploymentVersion>,
    #[serde(rename = "relatedVersion", skip_serializing_if = "Option::is_none")]
    pub related_version: Option<RestDeploymentVersion>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestEnvironmentDashboardStatus {
    #[serde(rename = "deploymentResult", skip_serializing_if = "Option::is_none")]
    pub deployment_result: Option<RestDeploymentResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<RestEnvironment>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestDeploymentResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<BuildAgent>,
    #[serde(rename = "agentId", skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<i64>,
    #[serde(rename = "agentType", skip_serializing_if = "Option::is_none")]
    pub agent_type: Option<RestDeploymentResultAgentType>,
    #[serde(rename = "customData", skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<serde_json::Value>,
    #[serde(rename = "deploymentState", skip_serializing_if = "Option::is_none")]
    pub deployment_state: Option<RestDeploymentResultDeploymentState>,
    #[serde(rename = "deploymentVersion", skip_serializing_if = "Option::is_none")]
    pub deployment_version: Option<DeploymentVersion>,
    #[serde(rename = "deploymentVersionName", skip_serializing_if = "Option::is_none")]
    pub deployment_version_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Environment>,
    #[serde(rename = "environmentId", skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<i64>,
    #[serde(rename = "executedDate", skip_serializing_if = "Option::is_none")]
    pub executed_date: Option<String>,
    #[serde(rename = "finishedDate", skip_serializing_if = "Option::is_none")]
    pub finished_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<DeploymentResultKey>,
    #[serde(rename = "lifeCycleState", skip_serializing_if = "Option::is_none")]
    pub life_cycle_state: Option<RestDeploymentResultLifeCycleState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<RestOperations>,
    #[serde(rename = "queuedDate", skip_serializing_if = "Option::is_none")]
    pub queued_date: Option<String>,
    #[serde(rename = "reasonSummary", skip_serializing_if = "Option::is_none")]
    pub reason_summary: Option<String>,
    #[serde(rename = "startedDate", skip_serializing_if = "Option::is_none")]
    pub started_date: Option<String>,
    #[serde(rename = "triggerReason", skip_serializing_if = "Option::is_none")]
    pub trigger_reason: Option<TriggerReason>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestDeploymentResultLifeCycleState {
    #[default]
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Queued")]
    Queued,
    #[serde(rename = "InProgress")]
    InProgress,
    #[serde(rename = "Finished")]
    Finished,
    #[serde(rename = "NotBuilt")]
    NotBuilt,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestDeploymentResultDeploymentState {
    #[default]
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Successful")]
    Successful,
    #[serde(rename = "Failed")]
    Failed,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestDeploymentResultAgentType {
    #[default]
    #[serde(rename = "LOCAL")]
    Local,
    #[serde(rename = "REMOTE")]
    Remote,
    #[serde(rename = "ELASTIC")]
    Elastic,
    #[serde(rename = "EPHEMERAL")]
    Ephemeral,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Environment {
    #[serde(rename = "configurationState", skip_serializing_if = "Option::is_none")]
    pub configuration_state: Option<EnvironmentConfigurationState>,
    #[serde(rename = "deploymentProjectId", skip_serializing_if = "Option::is_none")]
    pub deployment_project_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<DeploymentKey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Operations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(
        rename = "releaseApprovalPrerequisite",
        skip_serializing_if = "Option::is_none"
    )]
    pub release_approval_prerequisite: Option<EnvironmentReleaseApprovalPrerequisite>,
    #[serde(rename = "requirementSet", skip_serializing_if = "Option::is_none")]
    pub requirement_set: Option<ImmutableRequirementSet>,
    #[serde(rename = "requirementSetSupplier", skip_serializing_if = "Option::is_none")]
    pub requirement_set_supplier: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspended: Option<bool>,
    #[serde(rename = "taskDefinitions", skip_serializing_if = "Option::is_none")]
    pub task_definitions: Option<Vec<TaskDefinition>>,
    #[serde(rename = "taskDefinitionsSupplier", skip_serializing_if = "Option::is_none")]
    pub task_definitions_supplier: Option<serde_json::Value>,
    #[serde(rename = "triggerDefinitions", skip_serializing_if = "Option::is_none")]
    pub trigger_definitions: Option<Vec<TriggerDefinition>>,
    #[serde(
        rename = "triggerDefinitionsSupplier",
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_definitions_supplier: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImmutableRequirementSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<Vec<ImmutableRequirement>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImmutableRequirement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "matchValue", skip_serializing_if = "Option::is_none")]
    pub match_value: Option<String>,
    #[serde(rename = "ownerId", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<i64>,
    #[serde(rename = "pluginModuleKey", skip_serializing_if = "Option::is_none")]
    pub plugin_module_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly: Option<bool>,
    #[serde(rename = "regexMatch", skip_serializing_if = "Option::is_none")]
    pub regex_match: Option<bool>,
    #[serde(rename = "typeOfMatch", skip_serializing_if = "Option::is_none")]
    pub type_of_match: Option<ImmutableRequirementTypeOfMatch>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ImmutableRequirementTypeOfMatch {
    #[default]
    #[serde(rename = "EXISTS")]
    Exists,
    #[serde(rename = "EQUALS")]
    Equals,
    #[serde(rename = "MATCHES")]
    Matches,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum EnvironmentReleaseApprovalPrerequisite {
    #[default]
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "NOT_BROKEN")]
    NotBroken,
    #[serde(rename = "APPROVED")]
    Approved,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum EnvironmentConfigurationState {
    #[default]
    #[serde(rename = "CREATED")]
    Created,
    #[serde(rename = "DETAILED")]
    Detailed,
    #[serde(rename = "TASKED")]
    Tasked,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeploymentVersion {
    #[serde(rename = "ageZeroPoint", skip_serializing_if = "Option::is_none")]
    pub age_zero_point: Option<String>,
    #[serde(rename = "creationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "creatorDisplayName", skip_serializing_if = "Option::is_none")]
    pub creator_display_name: Option<String>,
    #[serde(rename = "creatorUserName", skip_serializing_if = "Option::is_none")]
    pub creator_user_name: Option<String>,
    #[serde(rename = "deploymentProjectId", skip_serializing_if = "Option::is_none")]
    pub deployment_project_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DeploymentVersionItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Operations>,
    #[serde(rename = "planBranchName", skip_serializing_if = "Option::is_none")]
    pub plan_branch_name: Option<String>,
    #[serde(rename = "variableContext", skip_serializing_if = "Option::is_none")]
    pub variable_context: Option<Vec<VariableSubstitutionContext>>,
    #[serde(rename = "versionStatus", skip_serializing_if = "Option::is_none")]
    pub version_status: Option<DeploymentVersionStatus>,
    #[serde(rename = "versionStatuses", skip_serializing_if = "Option::is_none")]
    pub version_statuses: Option<Vec<DeploymentVersionStatus>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeploymentResultKey {
    #[serde(rename = "deploymentResultId", skip_serializing_if = "Option::is_none")]
    pub deployment_result_id: Option<i64>,
    #[serde(rename = "entityKey", skip_serializing_if = "Option::is_none")]
    pub entity_key: Option<Key>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "resultNumber", skip_serializing_if = "Option::is_none")]
    pub result_number: Option<i32>,
    #[serde(rename = "resultNumberLong", skip_serializing_if = "Option::is_none")]
    pub result_number_long: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestDeploymentVersion {
    #[serde(rename = "ageZeroPoint", skip_serializing_if = "Option::is_none")]
    pub age_zero_point: Option<String>,
    #[serde(rename = "creationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "creatorDisplayName", skip_serializing_if = "Option::is_none")]
    pub creator_display_name: Option<String>,
    #[serde(rename = "creatorUserName", skip_serializing_if = "Option::is_none")]
    pub creator_user_name: Option<String>,
    #[serde(rename = "deploymentProjectId", skip_serializing_if = "Option::is_none")]
    pub deployment_project_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DeploymentVersionItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<Operations>,
    #[serde(rename = "planBranchName", skip_serializing_if = "Option::is_none")]
    pub plan_branch_name: Option<String>,
    #[serde(rename = "variableContext", skip_serializing_if = "Option::is_none")]
    pub variable_context: Option<Vec<VariableSubstitutionContext>>,
    #[serde(rename = "versionStatus", skip_serializing_if = "Option::is_none")]
    pub version_status: Option<RestDeploymentVersionStatus>,
    #[serde(rename = "versionStatuses", skip_serializing_if = "Option::is_none")]
    pub version_statuses: Option<Vec<DeploymentVersionStatus>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VariableSubstitutionContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "variableType", skip_serializing_if = "Option::is_none")]
    pub variable_type: Option<VariableSubstitutionContextVariableType>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum VariableSubstitutionContextVariableType {
    #[default]
    #[serde(rename = "GLOBAL")]
    Global,
    #[serde(rename = "PLAN")]
    Plan,
    #[serde(rename = "JOB")]
    Job,
    #[serde(rename = "MANUAL")]
    Manual,
    #[serde(rename = "CUSTOM")]
    Custom,
    #[serde(rename = "SYSTEM")]
    System,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "ENVIRONMENT")]
    Environment,
    #[serde(rename = "VERSION")]
    Version,
    #[serde(rename = "RESULT")]
    Result,
    #[serde(rename = "PROJECT")]
    Project,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestDeploymentVersionStatus {
    #[serde(rename = "creationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "sanitizedUserName", skip_serializing_if = "Option::is_none")]
    pub sanitized_user_name: Option<String>,
    #[serde(rename = "userName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "versionState", skip_serializing_if = "Option::is_none")]
    pub version_state: Option<RestDeploymentVersionStatusVersionState>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestDeploymentVersionStatusVersionState {
    #[default]
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Approved")]
    Approved,
    #[serde(rename = "Broken")]
    Broken,
    #[serde(rename = "Incomplete")]
    Incomplete,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Operations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
    #[serde(rename = "allowedToCreateVersion", skip_serializing_if = "Option::is_none")]
    pub allowed_to_create_version: Option<bool>,
    #[serde(rename = "allowedToExecute", skip_serializing_if = "Option::is_none")]
    pub allowed_to_execute: Option<bool>,
    #[serde(
        rename = "allowedToSetVersionStatus",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_to_set_version_status: Option<bool>,
    #[serde(rename = "canClone", skip_serializing_if = "Option::is_none")]
    pub can_clone: Option<bool>,
    #[serde(rename = "canDelete", skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<bool>,
    #[serde(rename = "canEdit", skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<bool>,
    #[serde(rename = "canExecute", skip_serializing_if = "Option::is_none")]
    pub can_execute: Option<bool>,
    #[serde(rename = "canView", skip_serializing_if = "Option::is_none")]
    pub can_view: Option<bool>,
    #[serde(rename = "canViewConfiguration", skip_serializing_if = "Option::is_none")]
    pub can_view_configuration: Option<bool>,
    #[serde(rename = "cantExecuteReason", skip_serializing_if = "Option::is_none")]
    pub cant_execute_reason: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeploymentVersionStatus {
    #[serde(rename = "creationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "sanitizedUserName", skip_serializing_if = "Option::is_none")]
    pub sanitized_user_name: Option<String>,
    #[serde(rename = "userName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "versionState", skip_serializing_if = "Option::is_none")]
    pub version_state: Option<DeploymentVersionStatusVersionState>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum DeploymentVersionStatusVersionState {
    #[default]
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Approved")]
    Approved,
    #[serde(rename = "Broken")]
    Broken,
    #[serde(rename = "Incomplete")]
    Incomplete,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeploymentVersionItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<DeploymentVersionItemType>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum DeploymentVersionItemType {
    #[default]
    #[serde(rename = "BAM_ARTIFACT")]
    BamArtifact,
    #[serde(rename = "SOURCE")]
    Source,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestDeploymentProject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environments: Option<Vec<RestEnvironment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<DeploymentKey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oid: Option<BambooEntityOid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<RestOperations>,
    #[serde(rename = "planKey", skip_serializing_if = "Option::is_none")]
    pub plan_key: Option<PlanKey>,
    #[serde(rename = "repositorySpecsManaged", skip_serializing_if = "Option::is_none")]
    pub repository_specs_managed: Option<bool>,
    #[serde(rename = "vcsBambooSpecsSource", skip_serializing_if = "Option::is_none")]
    pub vcs_bamboo_specs_source: Option<ImmutableVcsBambooSpecsSource>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestEnvironment {
    #[serde(rename = "configurationState", skip_serializing_if = "Option::is_none")]
    pub configuration_state: Option<RestEnvironmentConfigurationState>,
    #[serde(rename = "deploymentProjectId", skip_serializing_if = "Option::is_none")]
    pub deployment_project_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<RestKey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operations: Option<RestOperations>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(
        rename = "releaseApprovalPrerequisite",
        skip_serializing_if = "Option::is_none"
    )]
    pub release_approval_prerequisite: Option<
        RestEnvironmentReleaseApprovalPrerequisite,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspended: Option<bool>,
    #[serde(rename = "taskDefinitions", skip_serializing_if = "Option::is_none")]
    pub task_definitions: Option<Vec<TaskDefinition>>,
    #[serde(rename = "triggerDefinitions", skip_serializing_if = "Option::is_none")]
    pub trigger_definitions: Option<Vec<TriggerDefinition>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestOperations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
    #[serde(rename = "allowedToCreateVersion", skip_serializing_if = "Option::is_none")]
    pub allowed_to_create_version: Option<bool>,
    #[serde(rename = "allowedToExecute", skip_serializing_if = "Option::is_none")]
    pub allowed_to_execute: Option<bool>,
    #[serde(
        rename = "allowedToSetVersionStatus",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowed_to_set_version_status: Option<bool>,
    #[serde(rename = "canClone", skip_serializing_if = "Option::is_none")]
    pub can_clone: Option<bool>,
    #[serde(rename = "canDelete", skip_serializing_if = "Option::is_none")]
    pub can_delete: Option<bool>,
    #[serde(rename = "canEdit", skip_serializing_if = "Option::is_none")]
    pub can_edit: Option<bool>,
    #[serde(rename = "canExecute", skip_serializing_if = "Option::is_none")]
    pub can_execute: Option<bool>,
    #[serde(rename = "canView", skip_serializing_if = "Option::is_none")]
    pub can_view: Option<bool>,
    #[serde(rename = "canViewConfiguration", skip_serializing_if = "Option::is_none")]
    pub can_view_configuration: Option<bool>,
    #[serde(rename = "cantExecuteReason", skip_serializing_if = "Option::is_none")]
    pub cant_execute_reason: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestKey {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestEnvironmentReleaseApprovalPrerequisite {
    #[default]
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "NOT_BROKEN")]
    NotBroken,
    #[serde(rename = "APPROVED")]
    Approved,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RestEnvironmentConfigurationState {
    #[default]
    #[serde(rename = "CREATED")]
    Created,
    #[serde(rename = "DETAILED")]
    Detailed,
    #[serde(rename = "TASKED")]
    Tasked,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DeploymentKey {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BuildAgent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "agentStatus", skip_serializing_if = "Option::is_none")]
    pub agent_status: Option<AgentStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub busy: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<PipelineDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "requestedToBeStopped", skip_serializing_if = "Option::is_none")]
    pub requested_to_be_stopped: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<BuildAgentType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unresponsive: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PipelineDefinition {
    #[serde(rename = "creationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "ephemeralAgentDedication",
        skip_serializing_if = "Option::is_none"
    )]
    pub ephemeral_agent_dedication: Option<ResultKey>,
    #[serde(
        rename = "ephemeralAgentDedicationUntyped",
        skip_serializing_if = "Option::is_none"
    )]
    pub ephemeral_agent_dedication_untyped: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "lastModificationDate", skip_serializing_if = "Option::is_none")]
    pub last_modification_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<PipelineDefinitionType>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResultKey {
    #[serde(rename = "entityKey", skip_serializing_if = "Option::is_none")]
    pub entity_key: Option<Key>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "resultNumber", skip_serializing_if = "Option::is_none")]
    pub result_number: Option<i32>,
    #[serde(rename = "resultNumberLong", skip_serializing_if = "Option::is_none")]
    pub result_number_long: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum PipelineDefinitionType {
    #[default]
    #[serde(rename = "LOCAL")]
    Local,
    #[serde(rename = "REMOTE")]
    Remote,
    #[serde(rename = "ELASTIC")]
    Elastic,
    #[serde(rename = "EPHEMERAL")]
    Ephemeral,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum BuildAgentType {
    #[default]
    #[serde(rename = "LOCAL")]
    Local,
    #[serde(rename = "REMOTE")]
    Remote,
    #[serde(rename = "ELASTIC")]
    Elastic,
    #[serde(rename = "EPHEMERAL")]
    Ephemeral,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgentStatus {
    #[serde(rename = "allowDelete", skip_serializing_if = "Option::is_none")]
    pub allow_delete: Option<bool>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "orderIndex", skip_serializing_if = "Option::is_none")]
    pub order_index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ArtifactLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<MutableArtifact>,
    #[serde(rename = "buildResultsSummary", skip_serializing_if = "Option::is_none")]
    pub build_results_summary: Option<Box<ResultsSummary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "linkType", skip_serializing_if = "Option::is_none")]
    pub link_type: Option<String>,
    #[serde(rename = "producerJobResult", skip_serializing_if = "Option::is_none")]
    pub producer_job_result: Option<Box<BuildResultsSummary>>,
    #[serde(rename = "sharedArtifact", skip_serializing_if = "Option::is_none")]
    pub shared_artifact: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<Box<ConsumedSubscription>>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MutableArtifact {
    #[serde(rename = "archiverType", skip_serializing_if = "Option::is_none")]
    pub archiver_type: Option<MutableArtifactArchiverType>,
    #[serde(rename = "globallyStored", skip_serializing_if = "Option::is_none")]
    pub globally_stored: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(rename = "linkType", skip_serializing_if = "Option::is_none")]
    pub link_type: Option<String>,
    #[serde(rename = "planResultKey", skip_serializing_if = "Option::is_none")]
    pub plan_result_key: Option<PlanResultKey>,
    #[serde(rename = "sharedArtifact", skip_serializing_if = "Option::is_none")]
    pub shared_artifact: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum MutableArtifactArchiverType {
    #[default]
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "ZIP")]
    Zip,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BuildResultsSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "artifactLinksThatExist", skip_serializing_if = "Option::is_none")]
    pub artifact_links_that_exist: Option<Vec<Box<ArtifactLink>>>,
    #[serde(rename = "buildAgentId", skip_serializing_if = "Option::is_none")]
    pub build_agent_id: Option<i64>,
    #[serde(rename = "buildAgentType", skip_serializing_if = "Option::is_none")]
    pub build_agent_type: Option<BuildResultsSummaryBuildAgentType>,
    #[serde(rename = "buildCancelledDate", skip_serializing_if = "Option::is_none")]
    pub build_cancelled_date: Option<String>,
    #[serde(rename = "buildCompletedDate", skip_serializing_if = "Option::is_none")]
    pub build_completed_date: Option<String>,
    #[serde(rename = "buildDate", skip_serializing_if = "Option::is_none")]
    pub build_date: Option<String>,
    #[serde(rename = "buildKey", skip_serializing_if = "Option::is_none")]
    pub build_key: Option<String>,
    #[serde(rename = "buildNumber", skip_serializing_if = "Option::is_none")]
    pub build_number: Option<i32>,
    #[serde(rename = "buildResultKey", skip_serializing_if = "Option::is_none")]
    pub build_result_key: Option<String>,
    #[serde(rename = "buildState", skip_serializing_if = "Option::is_none")]
    pub build_state: Option<BuildResultsSummaryBuildState>,
    #[serde(rename = "buildTime", skip_serializing_if = "Option::is_none")]
    pub build_time: Option<String>,
    #[serde(rename = "chainResultsSummary", skip_serializing_if = "Option::is_none")]
    pub chain_results_summary: Option<Box<ChainResultsSummary>>,
    #[serde(rename = "changesListSummary", skip_serializing_if = "Option::is_none")]
    pub changes_list_summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commits: Option<Vec<Box<Commit>>>,
    #[serde(rename = "customBuild", skip_serializing_if = "Option::is_none")]
    pub custom_build: Option<bool>,
    #[serde(rename = "customBuildData", skip_serializing_if = "Option::is_none")]
    pub custom_build_data: Option<serde_json::Value>,
    #[serde(rename = "deltaState", skip_serializing_if = "Option::is_none")]
    pub delta_state: Option<BuildResultsSummaryDeltaState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(rename = "durationDescription", skip_serializing_if = "Option::is_none")]
    pub duration_description: Option<String>,
    #[serde(rename = "extraBuildResultsData", skip_serializing_if = "Option::is_none")]
    pub extra_build_results_data: Option<ExtraBuildResultsData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<bool>,
    #[serde(rename = "filteredTestResults", skip_serializing_if = "Option::is_none")]
    pub filtered_test_results: Option<Box<FilteredTestResultsTestClassResult>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalized: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished: Option<bool>,
    #[serde(rename = "fixingJiraIssues", skip_serializing_if = "Option::is_none")]
    pub fixing_jira_issues: Option<Vec<Box<LinkedJiraIssue>>>,
    #[serde(rename = "formatVersion", skip_serializing_if = "Option::is_none")]
    pub format_version: Option<i32>,
    #[serde(rename = "fullPlanName", skip_serializing_if = "Option::is_none")]
    pub full_plan_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "immutableChain", skip_serializing_if = "Option::is_none")]
    pub immutable_chain: Option<Box<ImmutableChain>>,
    #[serde(rename = "immutablePlan", skip_serializing_if = "Option::is_none")]
    pub immutable_plan: Option<Box<ImmutablePlan>>,
    #[serde(rename = "inProgress", skip_serializing_if = "Option::is_none")]
    pub in_progress: Option<bool>,
    #[serde(rename = "jiraIssueKeys", skip_serializing_if = "Option::is_none")]
    pub jira_issue_keys: Option<Vec<String>>,
    #[serde(rename = "jiraIssues", skip_serializing_if = "Option::is_none")]
    pub jira_issues: Option<Vec<Box<LinkedJiraIssue>>>,
    #[serde(rename = "labelNames", skip_serializing_if = "Option::is_none")]
    pub label_names: Option<Vec<String>>,
    #[serde(rename = "lifeCycleState", skip_serializing_if = "Option::is_none")]
    pub life_cycle_state: Option<BuildResultsSummaryLifeCycleState>,
    #[serde(rename = "logSize", skip_serializing_if = "Option::is_none")]
    pub log_size: Option<i64>,
    #[serde(
        rename = "manuallyOverriddenVariables",
        skip_serializing_if = "Option::is_none"
    )]
    pub manually_overridden_variables: Option<Vec<Box<VariableSubstitution>>>,
    #[serde(rename = "markedForDeletion", skip_serializing_if = "Option::is_none")]
    pub marked_for_deletion: Option<bool>,
    #[serde(rename = "notBuilt", skip_serializing_if = "Option::is_none")]
    pub not_built: Option<bool>,
    #[serde(rename = "notRunYet", skip_serializing_if = "Option::is_none")]
    pub not_run_yet: Option<bool>,
    #[serde(rename = "onceOff", skip_serializing_if = "Option::is_none")]
    pub once_off: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,
    #[serde(rename = "planIfExists", skip_serializing_if = "Option::is_none")]
    pub plan_if_exists: Option<Box<ImmutablePlan>>,
    #[serde(rename = "planKey", skip_serializing_if = "Option::is_none")]
    pub plan_key: Option<PlanKey>,
    #[serde(rename = "planName", skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[serde(rename = "planResultKey", skip_serializing_if = "Option::is_none")]
    pub plan_result_key: Option<PlanResultKey>,
    #[serde(rename = "processingDuration", skip_serializing_if = "Option::is_none")]
    pub processing_duration: Option<i64>,
    #[serde(
        rename = "processingDurationDescription",
        skip_serializing_if = "Option::is_none"
    )]
    pub processing_duration_description: Option<String>,
    #[serde(rename = "queueDuration", skip_serializing_if = "Option::is_none")]
    pub queue_duration: Option<i64>,
    #[serde(rename = "queueTime", skip_serializing_if = "Option::is_none")]
    pub queue_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued: Option<bool>,
    #[serde(rename = "reasonSummary", skip_serializing_if = "Option::is_none")]
    pub reason_summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rebuild: Option<bool>,
    #[serde(rename = "relatedJiraIssues", skip_serializing_if = "Option::is_none")]
    pub related_jira_issues: Option<Vec<Box<LinkedJiraIssue>>>,
    #[serde(rename = "relativeBuildDate", skip_serializing_if = "Option::is_none")]
    pub relative_build_date: Option<String>,
    #[serde(
        rename = "relativeBuildStartedDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_build_started_date: Option<String>,
    #[serde(rename = "relativeQueueDate", skip_serializing_if = "Option::is_none")]
    pub relative_queue_date: Option<String>,
    #[serde(rename = "repositoryChangesets", skip_serializing_if = "Option::is_none")]
    pub repository_changesets: Option<Vec<Box<RepositoryChangeset>>>,
    #[serde(rename = "restartCount", skip_serializing_if = "Option::is_none")]
    pub restart_count: Option<i32>,
    #[serde(rename = "resultVariables", skip_serializing_if = "Option::is_none")]
    pub result_variables: Option<serde_json::Value>,
    #[serde(rename = "shortReasonSummary", skip_serializing_if = "Option::is_none")]
    pub short_reason_summary: Option<String>,
    #[serde(rename = "statDate", skip_serializing_if = "Option::is_none")]
    pub stat_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<Box<ConsumedSubscription>>>,
    #[serde(rename = "substitutedVariables", skip_serializing_if = "Option::is_none")]
    pub substituted_variables: Option<Vec<Box<VariableSubstitution>>>,
    #[serde(
        rename = "substitutedVariablesEncrypted",
        skip_serializing_if = "Option::is_none"
    )]
    pub substituted_variables_encrypted: Option<Vec<Box<VariableSubstitution>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<bool>,
    #[serde(rename = "testResultsSummary", skip_serializing_if = "Option::is_none")]
    pub test_results_summary: Option<TestResultsSummary>,
    #[serde(rename = "testSummary", skip_serializing_if = "Option::is_none")]
    pub test_summary: Option<String>,
    #[serde(rename = "timeToFix", skip_serializing_if = "Option::is_none")]
    pub time_to_fix: Option<i64>,
    #[serde(rename = "triggerReason", skip_serializing_if = "Option::is_none")]
    pub trigger_reason: Option<TriggerReason>,
    #[serde(rename = "uniqueAuthors", skip_serializing_if = "Option::is_none")]
    pub unique_authors: Option<Vec<Box<Author>>>,
    #[serde(
        rename = "variableContextBaselineId",
        skip_serializing_if = "Option::is_none"
    )]
    pub variable_context_baseline_id: Option<i64>,
    #[serde(rename = "variableContextLogs", skip_serializing_if = "Option::is_none")]
    pub variable_context_logs: Option<Vec<Box<VariableContextSnapshot>>>,
    #[serde(
        rename = "variableContextLogsEncrypted",
        skip_serializing_if = "Option::is_none"
    )]
    pub variable_context_logs_encrypted: Option<Vec<Box<VariableContextSnapshot>>>,
    #[serde(rename = "vcsUpdateDuration", skip_serializing_if = "Option::is_none")]
    pub vcs_update_duration: Option<i64>,
    #[serde(rename = "vcsUpdateTime", skip_serializing_if = "Option::is_none")]
    pub vcs_update_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waiting: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FilteredTestResultsTestClassResult {
    #[serde(rename = "allFailedTestList", skip_serializing_if = "Option::is_none")]
    pub all_failed_test_list: Option<Vec<Box<TestCaseResult>>>,
    #[serde(rename = "allFailedTests", skip_serializing_if = "Option::is_none")]
    pub all_failed_tests: Option<MultimapTestClassResultTestCaseResult>,
    #[serde(rename = "existingFailedTests", skip_serializing_if = "Option::is_none")]
    pub existing_failed_tests: Option<MultimapTestClassResultTestCaseResult>,
    #[serde(rename = "fixedTests", skip_serializing_if = "Option::is_none")]
    pub fixed_tests: Option<MultimapTestClassResultTestCaseResult>,
    #[serde(rename = "newFailedTests", skip_serializing_if = "Option::is_none")]
    pub new_failed_tests: Option<MultimapTestClassResultTestCaseResult>,
    #[serde(rename = "quarantinedTests", skip_serializing_if = "Option::is_none")]
    pub quarantined_tests: Option<MultimapTestClassResultTestCaseResult>,
    #[serde(rename = "skippedTestList", skip_serializing_if = "Option::is_none")]
    pub skipped_test_list: Option<Vec<Box<TestCaseResult>>>,
    #[serde(rename = "skippedTests", skip_serializing_if = "Option::is_none")]
    pub skipped_tests: Option<MultimapTestClassResultTestCaseResult>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TestCaseResult {
    #[serde(rename = "deltaState", skip_serializing_if = "Option::is_none")]
    pub delta_state: Option<TestCaseResultDeltaState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Box<TestCaseResultError>>>,
    #[serde(rename = "failingSince", skip_serializing_if = "Option::is_none")]
    pub failing_since: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "methodName", skip_serializing_if = "Option::is_none")]
    pub method_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "prettyDuration", skip_serializing_if = "Option::is_none")]
    pub pretty_duration: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quarantined: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<TestCaseResultState>,
    #[serde(rename = "testCase", skip_serializing_if = "Option::is_none")]
    pub test_case: Option<Box<TestCase>>,
    #[serde(rename = "testClassResult", skip_serializing_if = "Option::is_none")]
    pub test_class_result: Option<Box<TestClassResult>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TestClassResult {
    #[serde(rename = "buildResultsSummary", skip_serializing_if = "Option::is_none")]
    pub build_results_summary: Option<Box<BuildResultsSummary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(rename = "failedTestCount", skip_serializing_if = "Option::is_none")]
    pub failed_test_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "shortName", skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    #[serde(rename = "skippedTestCount", skip_serializing_if = "Option::is_none")]
    pub skipped_test_count: Option<i32>,
    #[serde(rename = "successfulTestCount", skip_serializing_if = "Option::is_none")]
    pub successful_test_count: Option<i32>,
    #[serde(rename = "testCaseResults", skip_serializing_if = "Option::is_none")]
    pub test_case_results: Option<Vec<Box<TestCaseResult>>>,
    #[serde(rename = "testCaseResultsSet", skip_serializing_if = "Option::is_none")]
    pub test_case_results_set: Option<Vec<Box<TestCaseResult>>>,
    #[serde(rename = "testClass", skip_serializing_if = "Option::is_none")]
    pub test_class: Option<Box<TestClass>>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum TestCaseResultState {
    #[default]
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "SKIPPED")]
    Skipped,
    #[serde(rename = "FAILED")]
    Failed,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TestCaseResultError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "testCaseResult", skip_serializing_if = "Option::is_none")]
    pub test_case_result: Option<Box<TestCaseResult>>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum TestCaseResultDeltaState {
    #[default]
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "PASSING")]
    Passing,
    #[serde(rename = "FAILING")]
    Failing,
    #[serde(rename = "BROKEN")]
    Broken,
    #[serde(rename = "FIXED")]
    Fixed,
    #[serde(rename = "SKIPPED")]
    Skipped,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TestCase {
    #[serde(rename = "averageDuration", skip_serializing_if = "Option::is_none")]
    pub average_duration: Option<i64>,
    #[serde(
        rename = "averageDurationInSeconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub average_duration_in_seconds: Option<i64>,
    #[serde(rename = "firstRanBuildNumber", skip_serializing_if = "Option::is_none")]
    pub first_ran_build_number: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "lastRanBuildNumber", skip_serializing_if = "Option::is_none")]
    pub last_ran_build_number: Option<i32>,
    #[serde(rename = "lastRecordedBuildNumber", skip_serializing_if = "Option::is_none")]
    pub last_recorded_build_number: Option<i32>,
    #[serde(rename = "linkedJiraIssueKey", skip_serializing_if = "Option::is_none")]
    pub linked_jira_issue_key: Option<String>,
    #[serde(rename = "methodName", skip_serializing_if = "Option::is_none")]
    pub method_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "numberOfFailedRuns", skip_serializing_if = "Option::is_none")]
    pub number_of_failed_runs: Option<i32>,
    #[serde(rename = "numberOfSkippedRuns", skip_serializing_if = "Option::is_none")]
    pub number_of_skipped_runs: Option<i32>,
    #[serde(rename = "numberOfSuccessRuns", skip_serializing_if = "Option::is_none")]
    pub number_of_success_runs: Option<i32>,
    #[serde(rename = "quarantineStatistics", skip_serializing_if = "Option::is_none")]
    pub quarantine_statistics: Option<QuarantineStatistics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quarantined: Option<bool>,
    #[serde(rename = "successPercentage", skip_serializing_if = "Option::is_none")]
    pub success_percentage: Option<f64>,
    #[serde(rename = "testClass", skip_serializing_if = "Option::is_none")]
    pub test_class: Option<Box<TestClass>>,
    #[serde(rename = "totalTestRuns", skip_serializing_if = "Option::is_none")]
    pub total_test_runs: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TestClass {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "masterJobId", skip_serializing_if = "Option::is_none")]
    pub master_job_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<Box<Plan>>,
    #[serde(rename = "shortName", skip_serializing_if = "Option::is_none")]
    pub short_name: Option<String>,
    #[serde(rename = "testCaseCollection", skip_serializing_if = "Option::is_none")]
    pub test_case_collection: Option<Vec<Box<TestCase>>>,
    #[serde(rename = "testCases", skip_serializing_if = "Option::is_none")]
    pub test_cases: Option<Vec<Box<TestCase>>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QuarantineStatistics {
    #[serde(rename = "quarantineDate", skip_serializing_if = "Option::is_none")]
    pub quarantine_date: Option<String>,
    #[serde(rename = "quarantineExpiryDate", skip_serializing_if = "Option::is_none")]
    pub quarantine_expiry_date: Option<String>,
    #[serde(rename = "quarantiningUsername", skip_serializing_if = "Option::is_none")]
    pub quarantining_username: Option<String>,
}
pub type MultimapTestClassResultTestCaseResult = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ExtraBuildResultsData {
    #[serde(rename = "buildErrors", skip_serializing_if = "Option::is_none")]
    pub build_errors: Option<Vec<String>>,
    #[serde(rename = "buildReturnCode", skip_serializing_if = "Option::is_none")]
    pub build_return_code: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChainResultsSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "artifactLinksThatExist", skip_serializing_if = "Option::is_none")]
    pub artifact_links_that_exist: Option<Vec<Box<ArtifactLink>>>,
    #[serde(rename = "buildAgentId", skip_serializing_if = "Option::is_none")]
    pub build_agent_id: Option<i64>,
    #[serde(rename = "buildAgentType", skip_serializing_if = "Option::is_none")]
    pub build_agent_type: Option<ChainResultsSummaryBuildAgentType>,
    #[serde(rename = "buildCancelledDate", skip_serializing_if = "Option::is_none")]
    pub build_cancelled_date: Option<String>,
    #[serde(rename = "buildCompletedDate", skip_serializing_if = "Option::is_none")]
    pub build_completed_date: Option<String>,
    #[serde(rename = "buildDate", skip_serializing_if = "Option::is_none")]
    pub build_date: Option<String>,
    #[serde(rename = "buildKey", skip_serializing_if = "Option::is_none")]
    pub build_key: Option<String>,
    #[serde(rename = "buildNumber", skip_serializing_if = "Option::is_none")]
    pub build_number: Option<i32>,
    #[serde(rename = "buildResultKey", skip_serializing_if = "Option::is_none")]
    pub build_result_key: Option<String>,
    #[serde(rename = "buildState", skip_serializing_if = "Option::is_none")]
    pub build_state: Option<ChainResultsSummaryBuildState>,
    #[serde(rename = "buildTime", skip_serializing_if = "Option::is_none")]
    pub build_time: Option<String>,
    #[serde(rename = "changesListSummary", skip_serializing_if = "Option::is_none")]
    pub changes_list_summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commits: Option<Vec<Box<Commit>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub continuable: Option<bool>,
    #[serde(rename = "customBuild", skip_serializing_if = "Option::is_none")]
    pub custom_build: Option<bool>,
    #[serde(rename = "customBuildData", skip_serializing_if = "Option::is_none")]
    pub custom_build_data: Option<serde_json::Value>,
    #[serde(rename = "deltaState", skip_serializing_if = "Option::is_none")]
    pub delta_state: Option<ChainResultsSummaryDeltaState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(rename = "durationDescription", skip_serializing_if = "Option::is_none")]
    pub duration_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<bool>,
    #[serde(rename = "failedJobResults", skip_serializing_if = "Option::is_none")]
    pub failed_job_results: Option<Vec<Box<BuildResultsSummary>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalized: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished: Option<bool>,
    #[serde(rename = "fixedInResult", skip_serializing_if = "Option::is_none")]
    pub fixed_in_result: Option<i32>,
    #[serde(rename = "fixingJiraIssues", skip_serializing_if = "Option::is_none")]
    pub fixing_jira_issues: Option<Vec<Box<LinkedJiraIssue>>>,
    #[serde(rename = "formatVersion", skip_serializing_if = "Option::is_none")]
    pub format_version: Option<i32>,
    #[serde(rename = "fullPlanName", skip_serializing_if = "Option::is_none")]
    pub full_plan_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "immutableChain", skip_serializing_if = "Option::is_none")]
    pub immutable_chain: Option<Box<ImmutableChain>>,
    #[serde(rename = "immutablePlan", skip_serializing_if = "Option::is_none")]
    pub immutable_plan: Option<Box<ImmutableChain>>,
    #[serde(rename = "inProgress", skip_serializing_if = "Option::is_none")]
    pub in_progress: Option<bool>,
    #[serde(rename = "jiraIssueKeys", skip_serializing_if = "Option::is_none")]
    pub jira_issue_keys: Option<Vec<String>>,
    #[serde(rename = "jiraIssues", skip_serializing_if = "Option::is_none")]
    pub jira_issues: Option<Vec<Box<LinkedJiraIssue>>>,
    #[serde(rename = "labelNames", skip_serializing_if = "Option::is_none")]
    pub label_names: Option<Vec<String>>,
    #[serde(rename = "lifeCycleState", skip_serializing_if = "Option::is_none")]
    pub life_cycle_state: Option<ChainResultsSummaryLifeCycleState>,
    #[serde(rename = "logSize", skip_serializing_if = "Option::is_none")]
    pub log_size: Option<i64>,
    #[serde(
        rename = "manuallyOverriddenVariables",
        skip_serializing_if = "Option::is_none"
    )]
    pub manually_overridden_variables: Option<Vec<Box<VariableSubstitution>>>,
    #[serde(rename = "markedForDeletion", skip_serializing_if = "Option::is_none")]
    pub marked_for_deletion: Option<bool>,
    #[serde(rename = "mergeResult", skip_serializing_if = "Option::is_none")]
    pub merge_result: Option<MergeResultSummary>,
    #[serde(rename = "notBuilt", skip_serializing_if = "Option::is_none")]
    pub not_built: Option<bool>,
    #[serde(rename = "notRunYet", skip_serializing_if = "Option::is_none")]
    pub not_run_yet: Option<bool>,
    #[serde(rename = "onceOff", skip_serializing_if = "Option::is_none")]
    pub once_off: Option<bool>,
    #[serde(
        rename = "orderedJobResultSummaries",
        skip_serializing_if = "Option::is_none"
    )]
    pub ordered_job_result_summaries: Option<Vec<Box<ResultsSummary>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,
    #[serde(rename = "planIfExists", skip_serializing_if = "Option::is_none")]
    pub plan_if_exists: Option<Box<ImmutableChain>>,
    #[serde(rename = "planKey", skip_serializing_if = "Option::is_none")]
    pub plan_key: Option<PlanKey>,
    #[serde(rename = "planName", skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[serde(rename = "planResultKey", skip_serializing_if = "Option::is_none")]
    pub plan_result_key: Option<PlanResultKey>,
    #[serde(rename = "processingDuration", skip_serializing_if = "Option::is_none")]
    pub processing_duration: Option<i64>,
    #[serde(
        rename = "processingDurationDescription",
        skip_serializing_if = "Option::is_none"
    )]
    pub processing_duration_description: Option<String>,
    #[serde(rename = "queueTime", skip_serializing_if = "Option::is_none")]
    pub queue_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued: Option<bool>,
    #[serde(rename = "reasonSummary", skip_serializing_if = "Option::is_none")]
    pub reason_summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rebuild: Option<bool>,
    #[serde(rename = "relatedJiraIssues", skip_serializing_if = "Option::is_none")]
    pub related_jira_issues: Option<Vec<Box<LinkedJiraIssue>>>,
    #[serde(rename = "relativeBuildDate", skip_serializing_if = "Option::is_none")]
    pub relative_build_date: Option<String>,
    #[serde(
        rename = "relativeBuildStartedDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_build_started_date: Option<String>,
    #[serde(rename = "relativeQueueDate", skip_serializing_if = "Option::is_none")]
    pub relative_queue_date: Option<String>,
    #[serde(rename = "repositoryChangesets", skip_serializing_if = "Option::is_none")]
    pub repository_changesets: Option<Vec<Box<RepositoryChangeset>>>,
    #[serde(rename = "restartCount", skip_serializing_if = "Option::is_none")]
    pub restart_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restartable: Option<bool>,
    #[serde(rename = "shortReasonSummary", skip_serializing_if = "Option::is_none")]
    pub short_reason_summary: Option<String>,
    #[serde(rename = "specsResult", skip_serializing_if = "Option::is_none")]
    pub specs_result: Option<bool>,
    #[serde(rename = "stageResults", skip_serializing_if = "Option::is_none")]
    pub stage_results: Option<Vec<Box<ChainStageResult>>>,
    #[serde(rename = "statDate", skip_serializing_if = "Option::is_none")]
    pub stat_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<Box<ConsumedSubscription>>>,
    #[serde(rename = "substitutedVariables", skip_serializing_if = "Option::is_none")]
    pub substituted_variables: Option<Vec<Box<VariableSubstitution>>>,
    #[serde(
        rename = "substitutedVariablesEncrypted",
        skip_serializing_if = "Option::is_none"
    )]
    pub substituted_variables_encrypted: Option<Vec<Box<VariableSubstitution>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<bool>,
    #[serde(rename = "testResultsSummary", skip_serializing_if = "Option::is_none")]
    pub test_results_summary: Option<TestResultsSummary>,
    #[serde(rename = "testSummary", skip_serializing_if = "Option::is_none")]
    pub test_summary: Option<String>,
    #[serde(rename = "timeToFix", skip_serializing_if = "Option::is_none")]
    pub time_to_fix: Option<i64>,
    #[serde(rename = "totalJobCount", skip_serializing_if = "Option::is_none")]
    pub total_job_count: Option<i32>,
    #[serde(rename = "triggerReason", skip_serializing_if = "Option::is_none")]
    pub trigger_reason: Option<TriggerReason>,
    #[serde(rename = "uniqueAuthors", skip_serializing_if = "Option::is_none")]
    pub unique_authors: Option<Vec<Box<Author>>>,
    #[serde(
        rename = "variableContextBaselineId",
        skip_serializing_if = "Option::is_none"
    )]
    pub variable_context_baseline_id: Option<i64>,
    #[serde(rename = "variableContextLogs", skip_serializing_if = "Option::is_none")]
    pub variable_context_logs: Option<Vec<Box<VariableContextSnapshot>>>,
    #[serde(
        rename = "variableContextLogsEncrypted",
        skip_serializing_if = "Option::is_none"
    )]
    pub variable_context_logs_encrypted: Option<Vec<Box<VariableContextSnapshot>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waiting: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MergeResultSummary {
    #[serde(rename = "branchName", skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    #[serde(rename = "branchTargetVcsKey", skip_serializing_if = "Option::is_none")]
    pub branch_target_vcs_key: Option<String>,
    #[serde(rename = "emptyMerge", skip_serializing_if = "Option::is_none")]
    pub empty_merge: Option<bool>,
    #[serde(rename = "failureReason", skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "integrationBranchVcsKey", skip_serializing_if = "Option::is_none")]
    pub integration_branch_vcs_key: Option<String>,
    #[serde(
        rename = "integrationRepositoryBranchName",
        skip_serializing_if = "Option::is_none"
    )]
    pub integration_repository_branch_name: Option<String>,
    #[serde(rename = "integrationRepositoryId", skip_serializing_if = "Option::is_none")]
    pub integration_repository_id: Option<i64>,
    #[serde(rename = "integrationStrategy", skip_serializing_if = "Option::is_none")]
    pub integration_strategy: Option<MergeResultSummaryIntegrationStrategy>,
    #[serde(rename = "mergeResultVcsKey", skip_serializing_if = "Option::is_none")]
    pub merge_result_vcs_key: Option<String>,
    #[serde(rename = "mergeState", skip_serializing_if = "Option::is_none")]
    pub merge_state: Option<MergeResultSummaryMergeState>,
    #[serde(rename = "pushState", skip_serializing_if = "Option::is_none")]
    pub push_state: Option<MergeResultSummaryPushState>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum MergeResultSummaryPushState {
    #[default]
    #[serde(rename = "NOT_ATTEMPTED")]
    NotAttempted,
    #[serde(rename = "TO_BE_ATTEMPTED")]
    ToBeAttempted,
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "NOT_REQUIRED")]
    NotRequired,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum MergeResultSummaryMergeState {
    #[default]
    #[serde(rename = "NOT_ATTEMPTED")]
    NotAttempted,
    #[serde(rename = "TO_BE_ATTEMPTED")]
    ToBeAttempted,
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "NOT_REQUIRED")]
    NotRequired,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum MergeResultSummaryIntegrationStrategy {
    #[default]
    #[serde(rename = "BRANCH_UPDATER")]
    BranchUpdater,
    #[serde(rename = "GATE_KEEPER")]
    GateKeeper,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChainStageResult {
    #[serde(rename = "allJobsExist", skip_serializing_if = "Option::is_none")]
    pub all_jobs_exist: Option<bool>,
    #[serde(
        rename = "allNotSuccessfulJobsExist",
        skip_serializing_if = "Option::is_none"
    )]
    pub all_not_successful_jobs_exist: Option<bool>,
    #[serde(rename = "buildResults", skip_serializing_if = "Option::is_none")]
    pub build_results: Option<Vec<Box<BuildResultsSummary>>>,
    #[serde(rename = "chainResult", skip_serializing_if = "Option::is_none")]
    pub chain_result: Option<Box<ChainResultsSummary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<bool>,
    #[serde(rename = "failedBuildResults", skip_serializing_if = "Option::is_none")]
    pub failed_build_results: Option<Vec<Box<BuildResultsSummary>>>,
    #[serde(rename = "final", skip_serializing_if = "Option::is_none")]
    pub final_: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "lifeCycleState", skip_serializing_if = "Option::is_none")]
    pub life_cycle_state: Option<ChainStageResultLifeCycleState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual: Option<bool>,
    #[serde(rename = "manualVariables", skip_serializing_if = "Option::is_none")]
    pub manual_variables: Option<Vec<Box<StageVariableContext>>>,
    #[serde(
        rename = "manualVariablesEncrypted",
        skip_serializing_if = "Option::is_none"
    )]
    pub manual_variables_encrypted: Option<Vec<Box<StageVariableContext>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "notBuilt", skip_serializing_if = "Option::is_none")]
    pub not_built: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,
    #[serde(rename = "processingDuration", skip_serializing_if = "Option::is_none")]
    pub processing_duration: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restartable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runnable: Option<bool>,
    #[serde(rename = "sortedBuildResults", skip_serializing_if = "Option::is_none")]
    pub sorted_build_results: Option<Vec<Box<BuildResultsSummary>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<ChainStageResultState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<bool>,
    #[serde(rename = "successfulBuildResults", skip_serializing_if = "Option::is_none")]
    pub successful_build_results: Option<Vec<Box<BuildResultsSummary>>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StageVariableContext {
    #[serde(rename = "chainStageResult", skip_serializing_if = "Option::is_none")]
    pub chain_stage_result: Option<Box<ChainStageResult>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "variableType", skip_serializing_if = "Option::is_none")]
    pub variable_type: Option<StageVariableContextVariableType>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum StageVariableContextVariableType {
    #[default]
    #[serde(rename = "GLOBAL")]
    Global,
    #[serde(rename = "PLAN")]
    Plan,
    #[serde(rename = "JOB")]
    Job,
    #[serde(rename = "MANUAL")]
    Manual,
    #[serde(rename = "CUSTOM")]
    Custom,
    #[serde(rename = "SYSTEM")]
    System,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "ENVIRONMENT")]
    Environment,
    #[serde(rename = "VERSION")]
    Version,
    #[serde(rename = "RESULT")]
    Result,
    #[serde(rename = "PROJECT")]
    Project,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ChainStageResultState {
    #[default]
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Successful")]
    Successful,
    #[serde(rename = "Failed")]
    Failed,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ChainStageResultLifeCycleState {
    #[default]
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Queued")]
    Queued,
    #[serde(rename = "InProgress")]
    InProgress,
    #[serde(rename = "Finished")]
    Finished,
    #[serde(rename = "NotBuilt")]
    NotBuilt,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ChainResultsSummaryLifeCycleState {
    #[default]
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Queued")]
    Queued,
    #[serde(rename = "InProgress")]
    InProgress,
    #[serde(rename = "Finished")]
    Finished,
    #[serde(rename = "NotBuilt")]
    NotBuilt,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ChainResultsSummaryDeltaState {
    #[default]
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "PASSING")]
    Passing,
    #[serde(rename = "FAILING")]
    Failing,
    #[serde(rename = "BROKEN")]
    Broken,
    #[serde(rename = "FIXED")]
    Fixed,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ChainResultsSummaryBuildState {
    #[default]
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Successful")]
    Successful,
    #[serde(rename = "Failed")]
    Failed,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ChainResultsSummaryBuildAgentType {
    #[default]
    #[serde(rename = "LOCAL")]
    Local,
    #[serde(rename = "REMOTE")]
    Remote,
    #[serde(rename = "ELASTIC")]
    Elastic,
    #[serde(rename = "EPHEMERAL")]
    Ephemeral,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum BuildResultsSummaryLifeCycleState {
    #[default]
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Queued")]
    Queued,
    #[serde(rename = "InProgress")]
    InProgress,
    #[serde(rename = "Finished")]
    Finished,
    #[serde(rename = "NotBuilt")]
    NotBuilt,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum BuildResultsSummaryDeltaState {
    #[default]
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "PASSING")]
    Passing,
    #[serde(rename = "FAILING")]
    Failing,
    #[serde(rename = "BROKEN")]
    Broken,
    #[serde(rename = "FIXED")]
    Fixed,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum BuildResultsSummaryBuildState {
    #[default]
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Successful")]
    Successful,
    #[serde(rename = "Failed")]
    Failed,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum BuildResultsSummaryBuildAgentType {
    #[default]
    #[serde(rename = "LOCAL")]
    Local,
    #[serde(rename = "REMOTE")]
    Remote,
    #[serde(rename = "ELASTIC")]
    Elastic,
    #[serde(rename = "EPHEMERAL")]
    Ephemeral,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Author {
    #[serde(
        rename = "allTriggeredBuildResults",
        skip_serializing_if = "Option::is_none"
    )]
    pub all_triggered_build_results: Option<Vec<Box<ResultsSummary>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakages: Option<Vec<Box<ResultsSummary>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "failedBuilds", skip_serializing_if = "Option::is_none")]
    pub failed_builds: Option<Vec<Box<ResultsSummary>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixes: Option<Vec<Box<ResultsSummary>>>,
    #[serde(rename = "fullName", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(rename = "linkedUserName", skip_serializing_if = "Option::is_none")]
    pub linked_user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "numberOfBreakages", skip_serializing_if = "Option::is_none")]
    pub number_of_breakages: Option<i32>,
    #[serde(rename = "numberOfFailedBuilds", skip_serializing_if = "Option::is_none")]
    pub number_of_failed_builds: Option<i32>,
    #[serde(rename = "numberOfFixes", skip_serializing_if = "Option::is_none")]
    pub number_of_fixes: Option<i32>,
    #[serde(
        rename = "numberOfSuccessfulBuilds",
        skip_serializing_if = "Option::is_none"
    )]
    pub number_of_successful_builds: Option<i32>,
    #[serde(rename = "numberOfTriggeredBuilds", skip_serializing_if = "Option::is_none")]
    pub number_of_triggered_builds: Option<i32>,
    #[serde(rename = "successfulBuilds", skip_serializing_if = "Option::is_none")]
    pub successful_builds: Option<Vec<Box<ResultsSummary>>>,
    #[serde(rename = "triggeredBuildResults", skip_serializing_if = "Option::is_none")]
    pub triggered_build_results: Option<Vec<Box<ResultsSummary>>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResultsSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "artifactLinks", skip_serializing_if = "Option::is_none")]
    pub artifact_links: Option<Vec<Box<ArtifactLink>>>,
    #[serde(rename = "artifactLinksThatExist", skip_serializing_if = "Option::is_none")]
    pub artifact_links_that_exist: Option<Vec<Box<ArtifactLink>>>,
    #[serde(rename = "buildAgentId", skip_serializing_if = "Option::is_none")]
    pub build_agent_id: Option<i64>,
    #[serde(rename = "buildAgentType", skip_serializing_if = "Option::is_none")]
    pub build_agent_type: Option<ResultsSummaryBuildAgentType>,
    #[serde(rename = "buildCancelledDate", skip_serializing_if = "Option::is_none")]
    pub build_cancelled_date: Option<String>,
    #[serde(rename = "buildCompletedDate", skip_serializing_if = "Option::is_none")]
    pub build_completed_date: Option<String>,
    #[serde(rename = "buildDate", skip_serializing_if = "Option::is_none")]
    pub build_date: Option<String>,
    #[serde(rename = "buildKey", skip_serializing_if = "Option::is_none")]
    pub build_key: Option<String>,
    #[serde(rename = "buildNumber", skip_serializing_if = "Option::is_none")]
    pub build_number: Option<i32>,
    #[serde(rename = "buildResultKey", skip_serializing_if = "Option::is_none")]
    pub build_result_key: Option<String>,
    #[serde(rename = "buildState", skip_serializing_if = "Option::is_none")]
    pub build_state: Option<ResultsSummaryBuildState>,
    #[serde(rename = "buildTime", skip_serializing_if = "Option::is_none")]
    pub build_time: Option<String>,
    #[serde(rename = "changesListSummary", skip_serializing_if = "Option::is_none")]
    pub changes_list_summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commits: Option<Vec<Box<Commit>>>,
    #[serde(rename = "customBuild", skip_serializing_if = "Option::is_none")]
    pub custom_build: Option<bool>,
    #[serde(rename = "customBuildData", skip_serializing_if = "Option::is_none")]
    pub custom_build_data: Option<serde_json::Value>,
    #[serde(rename = "deltaState", skip_serializing_if = "Option::is_none")]
    pub delta_state: Option<ResultsSummaryDeltaState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(rename = "durationDescription", skip_serializing_if = "Option::is_none")]
    pub duration_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalized: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished: Option<bool>,
    #[serde(rename = "fixingJiraIssues", skip_serializing_if = "Option::is_none")]
    pub fixing_jira_issues: Option<Vec<Box<LinkedJiraIssue>>>,
    #[serde(rename = "formatVersion", skip_serializing_if = "Option::is_none")]
    pub format_version: Option<i32>,
    #[serde(rename = "fullPlanName", skip_serializing_if = "Option::is_none")]
    pub full_plan_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "immutableChain", skip_serializing_if = "Option::is_none")]
    pub immutable_chain: Option<Box<ImmutableChain>>,
    #[serde(rename = "immutablePlan", skip_serializing_if = "Option::is_none")]
    pub immutable_plan: Option<Box<ImmutablePlan>>,
    #[serde(rename = "inProgress", skip_serializing_if = "Option::is_none")]
    pub in_progress: Option<bool>,
    #[serde(rename = "jiraIssueKeys", skip_serializing_if = "Option::is_none")]
    pub jira_issue_keys: Option<Vec<String>>,
    #[serde(rename = "jiraIssues", skip_serializing_if = "Option::is_none")]
    pub jira_issues: Option<Vec<Box<LinkedJiraIssue>>>,
    #[serde(rename = "labelNames", skip_serializing_if = "Option::is_none")]
    pub label_names: Option<Vec<String>>,
    #[serde(rename = "lifeCycleState", skip_serializing_if = "Option::is_none")]
    pub life_cycle_state: Option<ResultsSummaryLifeCycleState>,
    #[serde(rename = "logSize", skip_serializing_if = "Option::is_none")]
    pub log_size: Option<i64>,
    #[serde(
        rename = "manuallyOverriddenVariables",
        skip_serializing_if = "Option::is_none"
    )]
    pub manually_overridden_variables: Option<Vec<Box<VariableSubstitution>>>,
    #[serde(rename = "markedForDeletion", skip_serializing_if = "Option::is_none")]
    pub marked_for_deletion: Option<bool>,
    #[serde(rename = "notBuilt", skip_serializing_if = "Option::is_none")]
    pub not_built: Option<bool>,
    #[serde(rename = "notRunYet", skip_serializing_if = "Option::is_none")]
    pub not_run_yet: Option<bool>,
    #[serde(rename = "onceOff", skip_serializing_if = "Option::is_none")]
    pub once_off: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,
    #[serde(rename = "planIfExists", skip_serializing_if = "Option::is_none")]
    pub plan_if_exists: Option<Box<ImmutablePlan>>,
    #[serde(rename = "planKey", skip_serializing_if = "Option::is_none")]
    pub plan_key: Option<PlanKey>,
    #[serde(rename = "planName", skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[serde(rename = "planResultKey", skip_serializing_if = "Option::is_none")]
    pub plan_result_key: Option<PlanResultKey>,
    #[serde(rename = "processingDuration", skip_serializing_if = "Option::is_none")]
    pub processing_duration: Option<i64>,
    #[serde(
        rename = "processingDurationDescription",
        skip_serializing_if = "Option::is_none"
    )]
    pub processing_duration_description: Option<String>,
    #[serde(rename = "queueTime", skip_serializing_if = "Option::is_none")]
    pub queue_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued: Option<bool>,
    #[serde(rename = "reasonSummary", skip_serializing_if = "Option::is_none")]
    pub reason_summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rebuild: Option<bool>,
    #[serde(rename = "relatedJiraIssues", skip_serializing_if = "Option::is_none")]
    pub related_jira_issues: Option<Vec<Box<LinkedJiraIssue>>>,
    #[serde(rename = "relativeBuildDate", skip_serializing_if = "Option::is_none")]
    pub relative_build_date: Option<String>,
    #[serde(
        rename = "relativeBuildStartedDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_build_started_date: Option<String>,
    #[serde(rename = "relativeQueueDate", skip_serializing_if = "Option::is_none")]
    pub relative_queue_date: Option<String>,
    #[serde(rename = "repositoryChangesets", skip_serializing_if = "Option::is_none")]
    pub repository_changesets: Option<Vec<Box<RepositoryChangeset>>>,
    #[serde(rename = "restartCount", skip_serializing_if = "Option::is_none")]
    pub restart_count: Option<i32>,
    #[serde(rename = "shortReasonSummary", skip_serializing_if = "Option::is_none")]
    pub short_reason_summary: Option<String>,
    #[serde(rename = "statDate", skip_serializing_if = "Option::is_none")]
    pub stat_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<Box<ConsumedSubscription>>>,
    #[serde(rename = "substitutedVariables", skip_serializing_if = "Option::is_none")]
    pub substituted_variables: Option<Vec<Box<VariableSubstitution>>>,
    #[serde(
        rename = "substitutedVariablesEncrypted",
        skip_serializing_if = "Option::is_none"
    )]
    pub substituted_variables_encrypted: Option<Vec<Box<VariableSubstitution>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<bool>,
    #[serde(rename = "testResultsSummary", skip_serializing_if = "Option::is_none")]
    pub test_results_summary: Option<TestResultsSummary>,
    #[serde(rename = "testSummary", skip_serializing_if = "Option::is_none")]
    pub test_summary: Option<String>,
    #[serde(rename = "timeToFix", skip_serializing_if = "Option::is_none")]
    pub time_to_fix: Option<i64>,
    #[serde(rename = "triggerReason", skip_serializing_if = "Option::is_none")]
    pub trigger_reason: Option<TriggerReason>,
    #[serde(rename = "uniqueAuthors", skip_serializing_if = "Option::is_none")]
    pub unique_authors: Option<Vec<Box<Author>>>,
    #[serde(
        rename = "variableContextBaselineId",
        skip_serializing_if = "Option::is_none"
    )]
    pub variable_context_baseline_id: Option<i64>,
    #[serde(rename = "variableContextLogs", skip_serializing_if = "Option::is_none")]
    pub variable_context_logs: Option<Vec<Box<VariableContextSnapshot>>>,
    #[serde(
        rename = "variableContextLogsEncrypted",
        skip_serializing_if = "Option::is_none"
    )]
    pub variable_context_logs_encrypted: Option<Vec<Box<VariableContextSnapshot>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waiting: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ResultsSummaryLifeCycleState {
    #[default]
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Queued")]
    Queued,
    #[serde(rename = "InProgress")]
    InProgress,
    #[serde(rename = "Finished")]
    Finished,
    #[serde(rename = "NotBuilt")]
    NotBuilt,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ResultsSummaryDeltaState {
    #[default]
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "PASSING")]
    Passing,
    #[serde(rename = "FAILING")]
    Failing,
    #[serde(rename = "BROKEN")]
    Broken,
    #[serde(rename = "FIXED")]
    Fixed,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ResultsSummaryBuildState {
    #[default]
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Successful")]
    Successful,
    #[serde(rename = "Failed")]
    Failed,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ResultsSummaryBuildAgentType {
    #[default]
    #[serde(rename = "LOCAL")]
    Local,
    #[serde(rename = "REMOTE")]
    Remote,
    #[serde(rename = "ELASTIC")]
    Elastic,
    #[serde(rename = "EPHEMERAL")]
    Ephemeral,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImmutableChain {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "allJobs", skip_serializing_if = "Option::is_none")]
    pub all_jobs: Option<Vec<Box<ImmutableJob>>>,
    #[serde(rename = "allStages", skip_serializing_if = "Option::is_none")]
    pub all_stages: Option<Vec<Box<ImmutableChainStage>>>,
    #[serde(rename = "averageBuildDuration", skip_serializing_if = "Option::is_none")]
    pub average_build_duration: Option<i64>,
    #[serde(rename = "buildDefinition", skip_serializing_if = "Option::is_none")]
    pub build_definition: Option<BuildDefinition>,
    #[serde(rename = "buildKey", skip_serializing_if = "Option::is_none")]
    pub build_key: Option<String>,
    #[serde(rename = "buildLogger", skip_serializing_if = "Option::is_none")]
    pub build_logger: Option<BuildLogger>,
    #[serde(rename = "buildName", skip_serializing_if = "Option::is_none")]
    pub build_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub busy: Option<bool>,
    #[serde(rename = "creationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "currentStatus", skip_serializing_if = "Option::is_none")]
    pub current_status: Option<String>,
    #[serde(rename = "databaseId", skip_serializing_if = "Option::is_none")]
    pub database_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "effectiveVariables", skip_serializing_if = "Option::is_none")]
    pub effective_variables: Option<Vec<Box<VariableDefinition>>>,
    #[serde(rename = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<ImmutableChainEntityType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executing: Option<bool>,
    #[serde(rename = "firstBuildNumber", skip_serializing_if = "Option::is_none")]
    pub first_build_number: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "jobCount", skip_serializing_if = "Option::is_none")]
    pub job_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "labelNames", skip_serializing_if = "Option::is_none")]
    pub label_names: Option<Vec<String>>,
    #[serde(rename = "lastBuildNumber", skip_serializing_if = "Option::is_none")]
    pub last_build_number: Option<i32>,
    #[serde(rename = "lastResultKey", skip_serializing_if = "Option::is_none")]
    pub last_result_key: Option<PlanResultKey>,
    #[serde(rename = "latestResultsSummary", skip_serializing_if = "Option::is_none")]
    pub latest_results_summary: Option<Box<ImmutableResultsSummary>>,
    #[serde(rename = "markedForDeletion", skip_serializing_if = "Option::is_none")]
    pub marked_for_deletion: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master: Option<Box<ImmutableChain>>,
    #[serde(rename = "masterId", skip_serializing_if = "Option::is_none")]
    pub master_id: Option<i64>,
    #[serde(rename = "masterIdIfExists", skip_serializing_if = "Option::is_none")]
    pub master_id_if_exists: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "notificationSet", skip_serializing_if = "Option::is_none")]
    pub notification_set: Option<Box<NotificationSet>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oid: Option<BambooEntityOid>,
    #[serde(rename = "planKey", skip_serializing_if = "Option::is_none")]
    pub plan_key: Option<PlanKey>,
    #[serde(
        rename = "planRepositoryDefinitions",
        skip_serializing_if = "Option::is_none"
    )]
    pub plan_repository_definitions: Option<Vec<PlanRepositoryDefinition>>,
    #[serde(rename = "planType", skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<ImmutableChainPlanType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Box<Project>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stages: Option<Vec<Box<ImmutableChainStage>>>,
    #[serde(rename = "storageTag", skip_serializing_if = "Option::is_none")]
    pub storage_tag: Option<ChainStorageTag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspended: Option<bool>,
    #[serde(rename = "suspendedFromBuilding", skip_serializing_if = "Option::is_none")]
    pub suspended_from_building: Option<bool>,
    #[serde(rename = "triggerDefinitions", skip_serializing_if = "Option::is_none")]
    pub trigger_definitions: Option<Vec<TriggerDefinition>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<Box<VariableDefinition>>>,
    #[serde(rename = "vcsBambooSpecsSource", skip_serializing_if = "Option::is_none")]
    pub vcs_bamboo_specs_source: Option<ImmutableVcsBambooSpecsSource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlanRepositoryDefinition {
    #[serde(
        rename = "bambooSpecsDetectionOptions",
        skip_serializing_if = "Option::is_none"
    )]
    pub bamboo_specs_detection_options: Option<VcsBambooSpecsDetectionOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<VcsBranchDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<PlanRepositoryDefinitionEntityType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "legacyRepository", skip_serializing_if = "Option::is_none")]
    pub legacy_repository: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked: Option<bool>,
    #[serde(rename = "markedForDeletion", skip_serializing_if = "Option::is_none")]
    pub marked_for_deletion: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oid: Option<BambooEntityOid>,
    #[serde(rename = "parentId", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<i64>,
    #[serde(rename = "parentOid", skip_serializing_if = "Option::is_none")]
    pub parent_oid: Option<BambooEntityOid>,
    #[serde(rename = "pluginKey", skip_serializing_if = "Option::is_none")]
    pub plugin_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
    #[serde(rename = "rootVcsLinked", skip_serializing_if = "Option::is_none")]
    pub root_vcs_linked: Option<bool>,
    #[serde(rename = "rootVcsProject", skip_serializing_if = "Option::is_none")]
    pub root_vcs_project: Option<bool>,
    #[serde(rename = "rootVcsRepositoryId", skip_serializing_if = "Option::is_none")]
    pub root_vcs_repository_id: Option<i64>,
    #[serde(rename = "rootVcsShared", skip_serializing_if = "Option::is_none")]
    pub root_vcs_shared: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<bool>,
    #[serde(
        rename = "vcsBranchDetectionOptions",
        skip_serializing_if = "Option::is_none"
    )]
    pub vcs_branch_detection_options: Option<VcsBranchDetectionOptions>,
    #[serde(
        rename = "vcsChangeDetectionOptions",
        skip_serializing_if = "Option::is_none"
    )]
    pub vcs_change_detection_options: Option<VcsChangeDetectionOptions>,
    #[serde(rename = "vcsLocation", skip_serializing_if = "Option::is_none")]
    pub vcs_location: Option<VcsLocationDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(rename = "viewerConfiguration", skip_serializing_if = "Option::is_none")]
    pub viewer_configuration: Option<VcsRepositoryViewerDefinition>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VcsRepositoryViewerDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<serde_json::Value>,
    #[serde(rename = "legacyConfigurationXml", skip_serializing_if = "Option::is_none")]
    pub legacy_configuration_xml: Option<String>,
    #[serde(rename = "legacyViewer", skip_serializing_if = "Option::is_none")]
    pub legacy_viewer: Option<bool>,
    #[serde(rename = "pluginKey", skip_serializing_if = "Option::is_none")]
    pub plugin_key: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VcsLocationDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<serde_json::Value>,
    #[serde(rename = "legacyConfigurationXml", skip_serializing_if = "Option::is_none")]
    pub legacy_configuration_xml: Option<String>,
    #[serde(rename = "legacyRepository", skip_serializing_if = "Option::is_none")]
    pub legacy_repository: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VcsChangeDetectionOptions {
    #[serde(
        rename = "changesetFilterPatternRegex",
        skip_serializing_if = "Option::is_none"
    )]
    pub changeset_filter_pattern_regex: Option<String>,
    #[serde(rename = "commitIsolationEnabled", skip_serializing_if = "Option::is_none")]
    pub commit_isolation_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<serde_json::Value>,
    #[serde(rename = "filterFilePatternOption", skip_serializing_if = "Option::is_none")]
    pub filter_file_pattern_option: Option<String>,
    #[serde(rename = "filterFilePatternRegex", skip_serializing_if = "Option::is_none")]
    pub filter_file_pattern_regex: Option<String>,
    #[serde(rename = "maxRetries", skip_serializing_if = "Option::is_none")]
    pub max_retries: Option<i32>,
    #[serde(rename = "quietPeriod", skip_serializing_if = "Option::is_none")]
    pub quiet_period: Option<i32>,
    #[serde(rename = "quietPeriodEnabled", skip_serializing_if = "Option::is_none")]
    pub quiet_period_enabled: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VcsBranchDetectionOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VcsBranchDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<serde_json::Value>,
    #[serde(rename = "vcsBranch", skip_serializing_if = "Option::is_none")]
    pub vcs_branch: Option<VcsBranch>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VcsBranch {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VcsBambooSpecsDetectionOptions {
    #[serde(rename = "bambooSpecsDetection", skip_serializing_if = "Option::is_none")]
    pub bamboo_specs_detection: Option<bool>,
    #[serde(
        rename = "bambooSpecsDetectionEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub bamboo_specs_detection_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<serde_json::Value>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum PlanRepositoryDefinitionEntityType {
    #[default]
    #[serde(rename = "CHAIN")]
    Chain,
    #[serde(rename = "STAGE")]
    Stage,
    #[serde(rename = "JOB")]
    Job,
    #[serde(rename = "REPOSITORY")]
    Repository,
    #[serde(rename = "CHAIN_BRANCH")]
    ChainBranch,
    #[serde(rename = "JOB_BRANCH")]
    JobBranch,
    #[serde(rename = "TASK")]
    Task,
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "ARTIFACT_DEFINITION")]
    ArtifactDefinition,
    #[serde(rename = "DEPLOYMENT_PROJECT")]
    DeploymentProject,
    #[serde(rename = "SHARED_CREDENTIAL")]
    SharedCredential,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotificationSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "notificationRules", skip_serializing_if = "Option::is_none")]
    pub notification_rules: Option<Vec<Box<NotificationRule>>>,
    #[serde(rename = "notificationSetType", skip_serializing_if = "Option::is_none")]
    pub notification_set_type: Option<NotificationSetNotificationSetType>,
    #[serde(rename = "sortedNotificationRules", skip_serializing_if = "Option::is_none")]
    pub sorted_notification_rules: Option<Vec<Box<NotificationRule>>>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum NotificationSetNotificationSetType {
    #[default]
    #[serde(rename = "PLAN")]
    Plan,
    #[serde(rename = "SYSTEM")]
    System,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotificationRule {
    #[serde(rename = "conditionData", skip_serializing_if = "Option::is_none")]
    pub condition_data: Option<String>,
    #[serde(rename = "conditionKey", skip_serializing_if = "Option::is_none")]
    pub condition_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "notificationManager", skip_serializing_if = "Option::is_none")]
    pub notification_manager: Option<NotificationManager>,
    #[serde(rename = "notificationSet", skip_serializing_if = "Option::is_none")]
    pub notification_set: Option<Box<NotificationSet>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(rename = "recipientType", skip_serializing_if = "Option::is_none")]
    pub recipient_type: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotificationManager {
    #[serde(
        rename = "allNotificationRecipients",
        skip_serializing_if = "Option::is_none"
    )]
    pub all_notification_recipients: Option<Vec<NotificationRecipient>>,
    #[serde(rename = "allNotificationTypes", skip_serializing_if = "Option::is_none")]
    pub all_notification_types: Option<Vec<NotificationType>>,
    #[serde(rename = "chainNotificationTypes", skip_serializing_if = "Option::is_none")]
    pub chain_notification_types: Option<Vec<NotificationType>>,
    #[serde(
        rename = "deploymentNotificationTypes",
        skip_serializing_if = "Option::is_none"
    )]
    pub deployment_notification_types: Option<Vec<NotificationType>>,
    #[serde(rename = "planNotificationTypes", skip_serializing_if = "Option::is_none")]
    pub plan_notification_types: Option<Vec<NotificationType>>,
    #[serde(rename = "systemNotificationTypes", skip_serializing_if = "Option::is_none")]
    pub system_notification_types: Option<Vec<NotificationType>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotificationType {
    #[serde(rename = "configurationData", skip_serializing_if = "Option::is_none")]
    pub configuration_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "editHtml", skip_serializing_if = "Option::is_none")]
    pub edit_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "moduleDescriptor", skip_serializing_if = "Option::is_none")]
    pub module_descriptor: Option<WeightedDescriptor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<NotificationTypeScope>,
    #[serde(rename = "viewHtml", skip_serializing_if = "Option::is_none")]
    pub view_html: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum NotificationTypeScope {
    #[default]
    #[serde(rename = "PLAN")]
    Plan,
    #[serde(rename = "CHAIN")]
    Chain,
    #[serde(rename = "SYSTEM")]
    System,
    #[serde(rename = "DEPLOYMENT")]
    Deployment,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NotificationRecipient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "editHtml", skip_serializing_if = "Option::is_none")]
    pub edit_html: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "moduleDescriptor", skip_serializing_if = "Option::is_none")]
    pub module_descriptor: Option<WeightedDescriptor>,
    #[serde(rename = "recipientConfig", skip_serializing_if = "Option::is_none")]
    pub recipient_config: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transports: Option<Vec<NotificationTransport>>,
    #[serde(rename = "viewHtml", skip_serializing_if = "Option::is_none")]
    pub view_html: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WeightedDescriptor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}
pub type NotificationTransport = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImmutableVcsBambooSpecsSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "sourceLocation", skip_serializing_if = "Option::is_none")]
    pub source_location: Option<String>,
    #[serde(
        rename = "vcsLocationBambooSpecsState",
        skip_serializing_if = "Option::is_none"
    )]
    pub vcs_location_bamboo_specs_state: Option<ImmutableVcsLocationBambooSpecsState>,
    #[serde(rename = "yamlConfiguration", skip_serializing_if = "Option::is_none")]
    pub yaml_configuration: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImmutableVcsLocationBambooSpecsState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "logFilename", skip_serializing_if = "Option::is_none")]
    pub log_filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    #[serde(rename = "specImportState", skip_serializing_if = "Option::is_none")]
    pub spec_import_state: Option<ImmutableVcsLocationBambooSpecsStateSpecImportState>,
    #[serde(rename = "specsExecutionDate", skip_serializing_if = "Option::is_none")]
    pub specs_execution_date: Option<String>,
    #[serde(rename = "specsNotFound", skip_serializing_if = "Option::is_none")]
    pub specs_not_found: Option<bool>,
    #[serde(rename = "vcsLocationId", skip_serializing_if = "Option::is_none")]
    pub vcs_location_id: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ImmutableVcsLocationBambooSpecsStateSpecImportState {
    #[default]
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "ERROR")]
    Error,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImmutableChainStage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<Box<ImmutableChain>>,
    #[serde(rename = "databaseId", skip_serializing_if = "Option::is_none")]
    pub database_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<ImmutableChainStageEntityType>,
    #[serde(rename = "final", skip_serializing_if = "Option::is_none")]
    pub final_: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<Box<ImmutableJob>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual: Option<bool>,
    #[serde(rename = "markedForDeletion", skip_serializing_if = "Option::is_none")]
    pub marked_for_deletion: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master: Option<Box<ImmutableChainStage>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oid: Option<BambooEntityOid>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImmutableJob {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "artifactDefinitions", skip_serializing_if = "Option::is_none")]
    pub artifact_definitions: Option<Vec<Box<ImmutableArtifactDefinition>>>,
    #[serde(rename = "artifactSubscriptions", skip_serializing_if = "Option::is_none")]
    pub artifact_subscriptions: Option<Vec<Box<ImmutableArtifactSubscription>>>,
    #[serde(rename = "averageBuildDuration", skip_serializing_if = "Option::is_none")]
    pub average_build_duration: Option<i64>,
    #[serde(rename = "buildDefinition", skip_serializing_if = "Option::is_none")]
    pub build_definition: Option<BuildDefinition>,
    #[serde(rename = "buildKey", skip_serializing_if = "Option::is_none")]
    pub build_key: Option<String>,
    #[serde(rename = "buildLogger", skip_serializing_if = "Option::is_none")]
    pub build_logger: Option<BuildLogger>,
    #[serde(rename = "buildName", skip_serializing_if = "Option::is_none")]
    pub build_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub busy: Option<bool>,
    #[serde(rename = "currentStatus", skip_serializing_if = "Option::is_none")]
    pub current_status: Option<String>,
    #[serde(rename = "databaseId", skip_serializing_if = "Option::is_none")]
    pub database_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub divergent: Option<bool>,
    #[serde(rename = "effectiveRequirementSet", skip_serializing_if = "Option::is_none")]
    pub effective_requirement_set: Option<Box<RequirementSet>>,
    #[serde(rename = "effectiveVariables", skip_serializing_if = "Option::is_none")]
    pub effective_variables: Option<Vec<Box<VariableDefinition>>>,
    #[serde(rename = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<ImmutableJobEntityType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executing: Option<bool>,
    #[serde(rename = "firstBuildNumber", skip_serializing_if = "Option::is_none")]
    pub first_build_number: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "labelNames", skip_serializing_if = "Option::is_none")]
    pub label_names: Option<Vec<String>>,
    #[serde(rename = "lastBuildNumber", skip_serializing_if = "Option::is_none")]
    pub last_build_number: Option<i32>,
    #[serde(rename = "latestResultsSummary", skip_serializing_if = "Option::is_none")]
    pub latest_results_summary: Option<Box<ImmutableResultsSummary>>,
    #[serde(rename = "markedForDeletion", skip_serializing_if = "Option::is_none")]
    pub marked_for_deletion: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master: Option<Box<ImmutableJob>>,
    #[serde(rename = "masterId", skip_serializing_if = "Option::is_none")]
    pub master_id: Option<i64>,
    #[serde(rename = "masterIdIfExists", skip_serializing_if = "Option::is_none")]
    pub master_id_if_exists: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oid: Option<BambooEntityOid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<ImmutableChain>>,
    #[serde(rename = "planKey", skip_serializing_if = "Option::is_none")]
    pub plan_key: Option<PlanKey>,
    #[serde(rename = "planType", skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<ImmutableJobPlanType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Box<Project>>,
    #[serde(rename = "requirementSet", skip_serializing_if = "Option::is_none")]
    pub requirement_set: Option<Box<RequirementSet>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<Box<ImmutableChainStage>>,
    #[serde(rename = "suspendedFromBuilding", skip_serializing_if = "Option::is_none")]
    pub suspended_from_building: Option<bool>,
    #[serde(rename = "taskDefinitions", skip_serializing_if = "Option::is_none")]
    pub task_definitions: Option<Vec<TaskDefinition>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<Box<VariableDefinition>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RequirementSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<Vec<Box<Requirement>>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Requirement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "matchValue", skip_serializing_if = "Option::is_none")]
    pub match_value: Option<String>,
    #[serde(rename = "ownerId", skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<i64>,
    #[serde(rename = "pluginModuleKey", skip_serializing_if = "Option::is_none")]
    pub plugin_module_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly: Option<bool>,
    #[serde(rename = "regexMatch", skip_serializing_if = "Option::is_none")]
    pub regex_match: Option<bool>,
    #[serde(rename = "requirementSet", skip_serializing_if = "Option::is_none")]
    pub requirement_set: Option<Box<RequirementSet>>,
    #[serde(rename = "systemRequirement", skip_serializing_if = "Option::is_none")]
    pub system_requirement: Option<bool>,
    #[serde(rename = "typeOfMatch", skip_serializing_if = "Option::is_none")]
    pub type_of_match: Option<RequirementTypeOfMatch>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RequirementTypeOfMatch {
    #[default]
    #[serde(rename = "EXISTS")]
    Exists,
    #[serde(rename = "EQUALS")]
    Equals,
    #[serde(rename = "MATCHES")]
    Matches,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImmutableResultsSummary {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "artifactLinks", skip_serializing_if = "Option::is_none")]
    pub artifact_links: Option<Vec<Box<ArtifactLink>>>,
    #[serde(rename = "artifactLinksThatExist", skip_serializing_if = "Option::is_none")]
    pub artifact_links_that_exist: Option<Vec<Box<ArtifactLink>>>,
    #[serde(rename = "buildAgentId", skip_serializing_if = "Option::is_none")]
    pub build_agent_id: Option<i64>,
    #[serde(rename = "buildAgentType", skip_serializing_if = "Option::is_none")]
    pub build_agent_type: Option<ImmutableResultsSummaryBuildAgentType>,
    #[serde(rename = "buildCancelledDate", skip_serializing_if = "Option::is_none")]
    pub build_cancelled_date: Option<String>,
    #[serde(rename = "buildCompletedDate", skip_serializing_if = "Option::is_none")]
    pub build_completed_date: Option<String>,
    #[serde(rename = "buildDate", skip_serializing_if = "Option::is_none")]
    pub build_date: Option<String>,
    #[serde(rename = "buildNumber", skip_serializing_if = "Option::is_none")]
    pub build_number: Option<i32>,
    #[serde(rename = "buildState", skip_serializing_if = "Option::is_none")]
    pub build_state: Option<ImmutableResultsSummaryBuildState>,
    #[serde(rename = "buildTime", skip_serializing_if = "Option::is_none")]
    pub build_time: Option<String>,
    #[serde(rename = "changesListSummary", skip_serializing_if = "Option::is_none")]
    pub changes_list_summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commits: Option<Vec<Box<Commit>>>,
    #[serde(rename = "customBuild", skip_serializing_if = "Option::is_none")]
    pub custom_build: Option<bool>,
    #[serde(rename = "customBuildData", skip_serializing_if = "Option::is_none")]
    pub custom_build_data: Option<serde_json::Value>,
    #[serde(rename = "deltaState", skip_serializing_if = "Option::is_none")]
    pub delta_state: Option<ImmutableResultsSummaryDeltaState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    #[serde(rename = "durationDescription", skip_serializing_if = "Option::is_none")]
    pub duration_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalized: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished: Option<bool>,
    #[serde(rename = "fixingJiraIssues", skip_serializing_if = "Option::is_none")]
    pub fixing_jira_issues: Option<Vec<Box<LinkedJiraIssue>>>,
    #[serde(rename = "fullPlanName", skip_serializing_if = "Option::is_none")]
    pub full_plan_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "immutableChain", skip_serializing_if = "Option::is_none")]
    pub immutable_chain: Option<Box<ImmutableChain>>,
    #[serde(rename = "immutablePlan", skip_serializing_if = "Option::is_none")]
    pub immutable_plan: Option<Box<ImmutablePlan>>,
    #[serde(rename = "inProgress", skip_serializing_if = "Option::is_none")]
    pub in_progress: Option<bool>,
    #[serde(rename = "jiraIssueKeys", skip_serializing_if = "Option::is_none")]
    pub jira_issue_keys: Option<Vec<String>>,
    #[serde(rename = "jiraIssues", skip_serializing_if = "Option::is_none")]
    pub jira_issues: Option<Vec<Box<LinkedJiraIssue>>>,
    #[serde(rename = "labelNames", skip_serializing_if = "Option::is_none")]
    pub label_names: Option<Vec<String>>,
    #[serde(rename = "lifeCycleState", skip_serializing_if = "Option::is_none")]
    pub life_cycle_state: Option<ImmutableResultsSummaryLifeCycleState>,
    #[serde(rename = "logSize", skip_serializing_if = "Option::is_none")]
    pub log_size: Option<i64>,
    #[serde(
        rename = "manuallyOverriddenVariables",
        skip_serializing_if = "Option::is_none"
    )]
    pub manually_overridden_variables: Option<Vec<Box<VariableSubstitution>>>,
    #[serde(rename = "notBuilt", skip_serializing_if = "Option::is_none")]
    pub not_built: Option<bool>,
    #[serde(rename = "notRunYet", skip_serializing_if = "Option::is_none")]
    pub not_run_yet: Option<bool>,
    #[serde(rename = "onceOff", skip_serializing_if = "Option::is_none")]
    pub once_off: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,
    #[serde(rename = "planIfExists", skip_serializing_if = "Option::is_none")]
    pub plan_if_exists: Option<Box<ImmutablePlan>>,
    #[serde(rename = "planKey", skip_serializing_if = "Option::is_none")]
    pub plan_key: Option<PlanKey>,
    #[serde(rename = "planName", skip_serializing_if = "Option::is_none")]
    pub plan_name: Option<String>,
    #[serde(rename = "planResultKey", skip_serializing_if = "Option::is_none")]
    pub plan_result_key: Option<PlanResultKey>,
    #[serde(rename = "processingDuration", skip_serializing_if = "Option::is_none")]
    pub processing_duration: Option<i64>,
    #[serde(
        rename = "processingDurationDescription",
        skip_serializing_if = "Option::is_none"
    )]
    pub processing_duration_description: Option<String>,
    #[serde(rename = "queueTime", skip_serializing_if = "Option::is_none")]
    pub queue_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queued: Option<bool>,
    #[serde(rename = "reasonSummary", skip_serializing_if = "Option::is_none")]
    pub reason_summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rebuild: Option<bool>,
    #[serde(rename = "relatedJiraIssues", skip_serializing_if = "Option::is_none")]
    pub related_jira_issues: Option<Vec<Box<LinkedJiraIssue>>>,
    #[serde(rename = "relativeBuildDate", skip_serializing_if = "Option::is_none")]
    pub relative_build_date: Option<String>,
    #[serde(
        rename = "relativeBuildStartedDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub relative_build_started_date: Option<String>,
    #[serde(rename = "relativeQueueDate", skip_serializing_if = "Option::is_none")]
    pub relative_queue_date: Option<String>,
    #[serde(rename = "repositoryChangesets", skip_serializing_if = "Option::is_none")]
    pub repository_changesets: Option<Vec<Box<RepositoryChangeset>>>,
    #[serde(rename = "restartCount", skip_serializing_if = "Option::is_none")]
    pub restart_count: Option<i32>,
    #[serde(rename = "shortReasonSummary", skip_serializing_if = "Option::is_none")]
    pub short_reason_summary: Option<String>,
    #[serde(rename = "statDate", skip_serializing_if = "Option::is_none")]
    pub stat_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<Box<ConsumedSubscription>>>,
    #[serde(rename = "substitutedVariables", skip_serializing_if = "Option::is_none")]
    pub substituted_variables: Option<Vec<Box<VariableSubstitution>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successful: Option<bool>,
    #[serde(rename = "testResultsSummary", skip_serializing_if = "Option::is_none")]
    pub test_results_summary: Option<TestResultsSummary>,
    #[serde(rename = "testSummary", skip_serializing_if = "Option::is_none")]
    pub test_summary: Option<String>,
    #[serde(rename = "timeToFix", skip_serializing_if = "Option::is_none")]
    pub time_to_fix: Option<i64>,
    #[serde(rename = "triggerReason", skip_serializing_if = "Option::is_none")]
    pub trigger_reason: Option<TriggerReason>,
    #[serde(rename = "uniqueAuthors", skip_serializing_if = "Option::is_none")]
    pub unique_authors: Option<Vec<Box<Author>>>,
    #[serde(rename = "variableContextLogs", skip_serializing_if = "Option::is_none")]
    pub variable_context_logs: Option<Vec<Box<VariableContextSnapshot>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waiting: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VariableSubstitution {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "resultSummary", skip_serializing_if = "Option::is_none")]
    pub result_summary: Option<Box<ResultsSummary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "variableType", skip_serializing_if = "Option::is_none")]
    pub variable_type: Option<VariableSubstitutionVariableType>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum VariableSubstitutionVariableType {
    #[default]
    #[serde(rename = "GLOBAL")]
    Global,
    #[serde(rename = "PLAN")]
    Plan,
    #[serde(rename = "JOB")]
    Job,
    #[serde(rename = "MANUAL")]
    Manual,
    #[serde(rename = "CUSTOM")]
    Custom,
    #[serde(rename = "SYSTEM")]
    System,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "ENVIRONMENT")]
    Environment,
    #[serde(rename = "VERSION")]
    Version,
    #[serde(rename = "RESULT")]
    Result,
    #[serde(rename = "PROJECT")]
    Project,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VariableContextSnapshot {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "resultSummary", skip_serializing_if = "Option::is_none")]
    pub result_summary: Option<Box<ResultsSummary>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "variableType", skip_serializing_if = "Option::is_none")]
    pub variable_type: Option<VariableContextSnapshotVariableType>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum VariableContextSnapshotVariableType {
    #[default]
    #[serde(rename = "GLOBAL")]
    Global,
    #[serde(rename = "PLAN")]
    Plan,
    #[serde(rename = "JOB")]
    Job,
    #[serde(rename = "MANUAL")]
    Manual,
    #[serde(rename = "CUSTOM")]
    Custom,
    #[serde(rename = "SYSTEM")]
    System,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "ENVIRONMENT")]
    Environment,
    #[serde(rename = "VERSION")]
    Version,
    #[serde(rename = "RESULT")]
    Result,
    #[serde(rename = "PROJECT")]
    Project,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TriggerReason {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nameForSentence", skip_serializing_if = "Option::is_none")]
    pub name_for_sentence: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TestResultsSummary {
    #[serde(rename = "existingFailedTestCount", skip_serializing_if = "Option::is_none")]
    pub existing_failed_test_count: Option<i32>,
    #[serde(rename = "failedTestCaseCount", skip_serializing_if = "Option::is_none")]
    pub failed_test_case_count: Option<i32>,
    #[serde(rename = "fixedTestCaseCount", skip_serializing_if = "Option::is_none")]
    pub fixed_test_case_count: Option<i32>,
    #[serde(rename = "ignoredTestCaseCount", skip_serializing_if = "Option::is_none")]
    pub ignored_test_case_count: Option<i32>,
    #[serde(rename = "newFailedTestCaseCount", skip_serializing_if = "Option::is_none")]
    pub new_failed_test_case_count: Option<i32>,
    #[serde(
        rename = "quarantinedTestCaseCount",
        skip_serializing_if = "Option::is_none"
    )]
    pub quarantined_test_case_count: Option<i32>,
    #[serde(rename = "skippedTestCaseCount", skip_serializing_if = "Option::is_none")]
    pub skipped_test_case_count: Option<i32>,
    #[serde(rename = "successfulTestCaseCount", skip_serializing_if = "Option::is_none")]
    pub successful_test_case_count: Option<i32>,
    #[serde(rename = "testSummaryDescription", skip_serializing_if = "Option::is_none")]
    pub test_summary_description: Option<String>,
    #[serde(rename = "totalTestCaseCount", skip_serializing_if = "Option::is_none")]
    pub total_test_case_count: Option<i32>,
    #[serde(rename = "totalTestDuration", skip_serializing_if = "Option::is_none")]
    pub total_test_duration: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlanResultKey {
    #[serde(rename = "buildNumber", skip_serializing_if = "Option::is_none")]
    pub build_number: Option<i32>,
    #[serde(rename = "entityKey", skip_serializing_if = "Option::is_none")]
    pub entity_key: Option<Key>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "planKey", skip_serializing_if = "Option::is_none")]
    pub plan_key: Option<PlanKey>,
    #[serde(rename = "resultNumber", skip_serializing_if = "Option::is_none")]
    pub result_number: Option<i32>,
    #[serde(rename = "resultNumberLong", skip_serializing_if = "Option::is_none")]
    pub result_number_long: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Key {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LinkedJiraIssue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "issueKey", skip_serializing_if = "Option::is_none")]
    pub issue_key: Option<String>,
    #[serde(rename = "issueType", skip_serializing_if = "Option::is_none")]
    pub issue_type: Option<LinkedJiraIssueIssueType>,
    #[serde(rename = "jiraIssueDetails", skip_serializing_if = "Option::is_none")]
    pub jira_issue_details: Option<JiraIssueDetails>,
    #[serde(rename = "resultsSummary", skip_serializing_if = "Option::is_none")]
    pub results_summary: Option<Box<ResultsSummary>>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum LinkedJiraIssueIssueType {
    #[default]
    #[serde(rename = "BUILD_RELATES")]
    BuildRelates,
    #[serde(rename = "BUILD_FIXES")]
    BuildFixes,
    #[serde(rename = "BUILD_CAUSES")]
    BuildCauses,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JiraIssueDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<JiraAssignee>,
    #[serde(rename = "displayUrl", skip_serializing_if = "Option::is_none")]
    pub display_url: Option<String>,
    #[serde(rename = "fixVersions", skip_serializing_if = "Option::is_none")]
    pub fix_versions: Option<Vec<String>>,
    #[serde(rename = "issueKey", skip_serializing_if = "Option::is_none")]
    pub issue_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<JiraStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<JiraType>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JiraType {
    #[serde(rename = "typeDescription", skip_serializing_if = "Option::is_none")]
    pub type_description: Option<String>,
    #[serde(rename = "typeIconUrl", skip_serializing_if = "Option::is_none")]
    pub type_icon_url: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JiraStatus {
    #[serde(rename = "statusDescription", skip_serializing_if = "Option::is_none")]
    pub status_description: Option<String>,
    #[serde(rename = "statusIconUrl", skip_serializing_if = "Option::is_none")]
    pub status_icon_url: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JiraAssignee {
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ImmutableResultsSummaryLifeCycleState {
    #[default]
    #[serde(rename = "Pending")]
    Pending,
    #[serde(rename = "Queued")]
    Queued,
    #[serde(rename = "InProgress")]
    InProgress,
    #[serde(rename = "Finished")]
    Finished,
    #[serde(rename = "NotBuilt")]
    NotBuilt,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ImmutableResultsSummaryDeltaState {
    #[default]
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "PASSING")]
    Passing,
    #[serde(rename = "FAILING")]
    Failing,
    #[serde(rename = "BROKEN")]
    Broken,
    #[serde(rename = "FIXED")]
    Fixed,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ImmutableResultsSummaryBuildState {
    #[default]
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Successful")]
    Successful,
    #[serde(rename = "Failed")]
    Failed,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ImmutableResultsSummaryBuildAgentType {
    #[default]
    #[serde(rename = "LOCAL")]
    Local,
    #[serde(rename = "REMOTE")]
    Remote,
    #[serde(rename = "ELASTIC")]
    Elastic,
    #[serde(rename = "EPHEMERAL")]
    Ephemeral,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImmutablePlan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "averageBuildDuration", skip_serializing_if = "Option::is_none")]
    pub average_build_duration: Option<i64>,
    #[serde(rename = "buildDefinition", skip_serializing_if = "Option::is_none")]
    pub build_definition: Option<BuildDefinition>,
    #[serde(rename = "buildKey", skip_serializing_if = "Option::is_none")]
    pub build_key: Option<String>,
    #[serde(rename = "buildLogger", skip_serializing_if = "Option::is_none")]
    pub build_logger: Option<BuildLogger>,
    #[serde(rename = "buildName", skip_serializing_if = "Option::is_none")]
    pub build_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub busy: Option<bool>,
    #[serde(rename = "currentStatus", skip_serializing_if = "Option::is_none")]
    pub current_status: Option<String>,
    #[serde(rename = "databaseId", skip_serializing_if = "Option::is_none")]
    pub database_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "effectiveVariables", skip_serializing_if = "Option::is_none")]
    pub effective_variables: Option<Vec<Box<VariableDefinition>>>,
    #[serde(rename = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<ImmutablePlanEntityType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executing: Option<bool>,
    #[serde(rename = "firstBuildNumber", skip_serializing_if = "Option::is_none")]
    pub first_build_number: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "labelNames", skip_serializing_if = "Option::is_none")]
    pub label_names: Option<Vec<String>>,
    #[serde(rename = "lastBuildNumber", skip_serializing_if = "Option::is_none")]
    pub last_build_number: Option<i32>,
    #[serde(rename = "latestResultsSummary", skip_serializing_if = "Option::is_none")]
    pub latest_results_summary: Option<Box<ImmutableResultsSummary>>,
    #[serde(rename = "markedForDeletion", skip_serializing_if = "Option::is_none")]
    pub marked_for_deletion: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master: Option<Box<ImmutablePlan>>,
    #[serde(rename = "masterId", skip_serializing_if = "Option::is_none")]
    pub master_id: Option<i64>,
    #[serde(rename = "masterIdIfExists", skip_serializing_if = "Option::is_none")]
    pub master_id_if_exists: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oid: Option<BambooEntityOid>,
    #[serde(rename = "planKey", skip_serializing_if = "Option::is_none")]
    pub plan_key: Option<PlanKey>,
    #[serde(rename = "planType", skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<ImmutablePlanPlanType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Box<Project>>,
    #[serde(rename = "suspendedFromBuilding", skip_serializing_if = "Option::is_none")]
    pub suspended_from_building: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<Box<VariableDefinition>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<ProjectEntityType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labellings: Option<Vec<Box<Labelling>>>,
    #[serde(rename = "markedForDeletion", skip_serializing_if = "Option::is_none")]
    pub marked_for_deletion: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oid: Option<BambooEntityOid>,
    #[serde(rename = "relatedLabellings", skip_serializing_if = "Option::is_none")]
    pub related_labellings: Option<Vec<Box<Labelling>>>,
    #[serde(rename = "vcsBambooSpecsSource", skip_serializing_if = "Option::is_none")]
    pub vcs_bamboo_specs_source: Option<VcsBambooSpecsSource>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VcsBambooSpecsSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "sourceLocation", skip_serializing_if = "Option::is_none")]
    pub source_location: Option<String>,
    #[serde(
        rename = "vcsLocationBambooSpecsState",
        skip_serializing_if = "Option::is_none"
    )]
    pub vcs_location_bamboo_specs_state: Option<VcsLocationBambooSpecsState>,
    #[serde(rename = "yamlConfiguration", skip_serializing_if = "Option::is_none")]
    pub yaml_configuration: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VcsLocationBambooSpecsState {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "logFilename", skip_serializing_if = "Option::is_none")]
    pub log_filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    #[serde(rename = "specImportState", skip_serializing_if = "Option::is_none")]
    pub spec_import_state: Option<VcsLocationBambooSpecsStateSpecImportState>,
    #[serde(rename = "specsExecutionDate", skip_serializing_if = "Option::is_none")]
    pub specs_execution_date: Option<String>,
    #[serde(rename = "specsNotFound", skip_serializing_if = "Option::is_none")]
    pub specs_not_found: Option<bool>,
    #[serde(rename = "vcsLocationId", skip_serializing_if = "Option::is_none")]
    pub vcs_location_id: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum VcsLocationBambooSpecsStateSpecImportState {
    #[default]
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "ERROR")]
    Error,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ProjectEntityType {
    #[default]
    #[serde(rename = "CHAIN")]
    Chain,
    #[serde(rename = "STAGE")]
    Stage,
    #[serde(rename = "JOB")]
    Job,
    #[serde(rename = "REPOSITORY")]
    Repository,
    #[serde(rename = "CHAIN_BRANCH")]
    ChainBranch,
    #[serde(rename = "JOB_BRANCH")]
    JobBranch,
    #[serde(rename = "TASK")]
    Task,
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "ARTIFACT_DEFINITION")]
    ArtifactDefinition,
    #[serde(rename = "DEPLOYMENT_PROJECT")]
    DeploymentProject,
    #[serde(rename = "SHARED_CREDENTIAL")]
    SharedCredential,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Labelling {
    #[serde(rename = "buildResultsSummary", skip_serializing_if = "Option::is_none")]
    pub build_results_summary: Option<Box<ResultsSummary>>,
    #[serde(rename = "creationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    #[serde(rename = "lastModificationDate", skip_serializing_if = "Option::is_none")]
    pub last_modification_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<Box<Plan>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Box<Project>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<Box<Plan>>,
    #[serde(rename = "userName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Plan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "averageBuildDuration", skip_serializing_if = "Option::is_none")]
    pub average_build_duration: Option<i64>,
    #[serde(rename = "buildDefinition", skip_serializing_if = "Option::is_none")]
    pub build_definition: Option<BuildDefinition>,
    #[serde(rename = "buildDefinitionXml", skip_serializing_if = "Option::is_none")]
    pub build_definition_xml: Option<Box<BuildDefinitionForBuild>>,
    #[serde(rename = "buildKey", skip_serializing_if = "Option::is_none")]
    pub build_key: Option<String>,
    #[serde(rename = "buildLogger", skip_serializing_if = "Option::is_none")]
    pub build_logger: Option<BuildLogger>,
    #[serde(rename = "buildName", skip_serializing_if = "Option::is_none")]
    pub build_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub busy: Option<bool>,
    #[serde(rename = "currentStatus", skip_serializing_if = "Option::is_none")]
    pub current_status: Option<String>,
    #[serde(rename = "databaseId", skip_serializing_if = "Option::is_none")]
    pub database_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "effectiveVariables", skip_serializing_if = "Option::is_none")]
    pub effective_variables: Option<Vec<Box<VariableDefinition>>>,
    #[serde(rename = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<PlanEntityType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executing: Option<bool>,
    #[serde(rename = "firstBuildNumber", skip_serializing_if = "Option::is_none")]
    pub first_build_number: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "labelNames", skip_serializing_if = "Option::is_none")]
    pub label_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labellings: Option<Vec<Box<Labelling>>>,
    #[serde(rename = "lastBuildNumber", skip_serializing_if = "Option::is_none")]
    pub last_build_number: Option<i32>,
    #[serde(rename = "latestResultsSummary", skip_serializing_if = "Option::is_none")]
    pub latest_results_summary: Option<Box<ResultsSummary>>,
    #[serde(rename = "markedForDeletion", skip_serializing_if = "Option::is_none")]
    pub marked_for_deletion: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master: Option<Box<ImmutablePlan>>,
    #[serde(rename = "masterId", skip_serializing_if = "Option::is_none")]
    pub master_id: Option<i64>,
    #[serde(rename = "masterIdIfExists", skip_serializing_if = "Option::is_none")]
    pub master_id_if_exists: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oid: Option<BambooEntityOid>,
    #[serde(rename = "planKey", skip_serializing_if = "Option::is_none")]
    pub plan_key: Option<PlanKey>,
    #[serde(rename = "planType", skip_serializing_if = "Option::is_none")]
    pub plan_type: Option<PlanPlanType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<Box<Project>>,
    #[serde(rename = "suspendedFromBuilding", skip_serializing_if = "Option::is_none")]
    pub suspended_from_building: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<Box<VariableDefinition>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VariableDefinition {
    #[serde(rename = "deploymentVersionId", skip_serializing_if = "Option::is_none")]
    pub deployment_version_id: Option<i64>,
    #[serde(rename = "environmentId", skip_serializing_if = "Option::is_none")]
    pub environment_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<Box<Plan>>,
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<Box<Plan>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "variableType", skip_serializing_if = "Option::is_none")]
    pub variable_type: Option<VariableDefinitionVariableType>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum VariableDefinitionVariableType {
    #[default]
    #[serde(rename = "GLOBAL")]
    Global,
    #[serde(rename = "PLAN")]
    Plan,
    #[serde(rename = "JOB")]
    Job,
    #[serde(rename = "MANUAL")]
    Manual,
    #[serde(rename = "CUSTOM")]
    Custom,
    #[serde(rename = "SYSTEM")]
    System,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "ENVIRONMENT")]
    Environment,
    #[serde(rename = "VERSION")]
    Version,
    #[serde(rename = "RESULT")]
    Result,
    #[serde(rename = "PROJECT")]
    Project,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum PlanPlanType {
    #[default]
    #[serde(rename = "CHAIN")]
    Chain,
    #[serde(rename = "JOB")]
    Job,
    #[serde(rename = "CHAIN_BRANCH")]
    ChainBranch,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum PlanEntityType {
    #[default]
    #[serde(rename = "CHAIN")]
    Chain,
    #[serde(rename = "STAGE")]
    Stage,
    #[serde(rename = "JOB")]
    Job,
    #[serde(rename = "REPOSITORY")]
    Repository,
    #[serde(rename = "CHAIN_BRANCH")]
    ChainBranch,
    #[serde(rename = "JOB_BRANCH")]
    JobBranch,
    #[serde(rename = "TASK")]
    Task,
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "ARTIFACT_DEFINITION")]
    ArtifactDefinition,
    #[serde(rename = "DEPLOYMENT_PROJECT")]
    DeploymentProject,
    #[serde(rename = "SHARED_CREDENTIAL")]
    SharedCredential,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BuildDefinitionForBuild {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<Box<Plan>>,
    #[serde(rename = "xmlData", skip_serializing_if = "Option::is_none")]
    pub xml_data: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Label {
    #[serde(rename = "creationDate", skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "lastModificationDate", skip_serializing_if = "Option::is_none")]
    pub last_modification_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ImmutablePlanPlanType {
    #[default]
    #[serde(rename = "CHAIN")]
    Chain,
    #[serde(rename = "JOB")]
    Job,
    #[serde(rename = "CHAIN_BRANCH")]
    ChainBranch,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ImmutablePlanEntityType {
    #[default]
    #[serde(rename = "CHAIN")]
    Chain,
    #[serde(rename = "STAGE")]
    Stage,
    #[serde(rename = "JOB")]
    Job,
    #[serde(rename = "REPOSITORY")]
    Repository,
    #[serde(rename = "CHAIN_BRANCH")]
    ChainBranch,
    #[serde(rename = "JOB_BRANCH")]
    JobBranch,
    #[serde(rename = "TASK")]
    Task,
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "ARTIFACT_DEFINITION")]
    ArtifactDefinition,
    #[serde(rename = "DEPLOYMENT_PROJECT")]
    DeploymentProject,
    #[serde(rename = "SHARED_CREDENTIAL")]
    SharedCredential,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ImmutableJobPlanType {
    #[default]
    #[serde(rename = "CHAIN")]
    Chain,
    #[serde(rename = "JOB")]
    Job,
    #[serde(rename = "CHAIN_BRANCH")]
    ChainBranch,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ImmutableJobEntityType {
    #[default]
    #[serde(rename = "CHAIN")]
    Chain,
    #[serde(rename = "STAGE")]
    Stage,
    #[serde(rename = "JOB")]
    Job,
    #[serde(rename = "REPOSITORY")]
    Repository,
    #[serde(rename = "CHAIN_BRANCH")]
    ChainBranch,
    #[serde(rename = "JOB_BRANCH")]
    JobBranch,
    #[serde(rename = "TASK")]
    Task,
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "ARTIFACT_DEFINITION")]
    ArtifactDefinition,
    #[serde(rename = "DEPLOYMENT_PROJECT")]
    DeploymentProject,
    #[serde(rename = "SHARED_CREDENTIAL")]
    SharedCredential,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImmutableArtifactDefinition {
    #[serde(rename = "copyPattern", skip_serializing_if = "Option::is_none")]
    pub copy_pattern: Option<String>,
    #[serde(rename = "copyPatterns", skip_serializing_if = "Option::is_none")]
    pub copy_patterns: Option<Vec<String>>,
    #[serde(rename = "exclusionPatterns", skip_serializing_if = "Option::is_none")]
    pub exclusion_patterns: Option<Vec<String>>,
    #[serde(rename = "httpCompressionOn", skip_serializing_if = "Option::is_none")]
    pub http_compression_on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "producerJob", skip_serializing_if = "Option::is_none")]
    pub producer_job: Option<Box<ImmutableJob>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(rename = "sharedArtifact", skip_serializing_if = "Option::is_none")]
    pub shared_artifact: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<Box<ImmutableArtifactSubscription>>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ImmutableArtifactSubscription {
    #[serde(rename = "artifactDefinition", skip_serializing_if = "Option::is_none")]
    pub artifact_definition: Option<Box<ImmutableArtifactDefinition>>,
    #[serde(rename = "consumerJob", skip_serializing_if = "Option::is_none")]
    pub consumer_job: Option<Box<ImmutableJob>>,
    #[serde(rename = "destinationDirectory", skip_serializing_if = "Option::is_none")]
    pub destination_directory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ImmutableChainStageEntityType {
    #[default]
    #[serde(rename = "CHAIN")]
    Chain,
    #[serde(rename = "STAGE")]
    Stage,
    #[serde(rename = "JOB")]
    Job,
    #[serde(rename = "REPOSITORY")]
    Repository,
    #[serde(rename = "CHAIN_BRANCH")]
    ChainBranch,
    #[serde(rename = "JOB_BRANCH")]
    JobBranch,
    #[serde(rename = "TASK")]
    Task,
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "ARTIFACT_DEFINITION")]
    ArtifactDefinition,
    #[serde(rename = "DEPLOYMENT_PROJECT")]
    DeploymentProject,
    #[serde(rename = "SHARED_CREDENTIAL")]
    SharedCredential,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ImmutableChainPlanType {
    #[default]
    #[serde(rename = "CHAIN")]
    Chain,
    #[serde(rename = "JOB")]
    Job,
    #[serde(rename = "CHAIN_BRANCH")]
    ChainBranch,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum ImmutableChainEntityType {
    #[default]
    #[serde(rename = "CHAIN")]
    Chain,
    #[serde(rename = "STAGE")]
    Stage,
    #[serde(rename = "JOB")]
    Job,
    #[serde(rename = "REPOSITORY")]
    Repository,
    #[serde(rename = "CHAIN_BRANCH")]
    ChainBranch,
    #[serde(rename = "JOB_BRANCH")]
    JobBranch,
    #[serde(rename = "TASK")]
    Task,
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "ARTIFACT_DEFINITION")]
    ArtifactDefinition,
    #[serde(rename = "DEPLOYMENT_PROJECT")]
    DeploymentProject,
    #[serde(rename = "SHARED_CREDENTIAL")]
    SharedCredential,
}
pub type ChainStorageTag = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BuildLogger {
    #[serde(rename = "interceptorStack", skip_serializing_if = "Option::is_none")]
    pub interceptor_stack: Option<LogInterceptorStack>,
    #[serde(rename = "logEntryCount", skip_serializing_if = "Option::is_none")]
    pub log_entry_count: Option<i32>,
    #[serde(rename = "mutatorStack", skip_serializing_if = "Option::is_none")]
    pub mutator_stack: Option<LogMutatorStack>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent: Option<bool>,
    #[serde(rename = "timeOfLastLog", skip_serializing_if = "Option::is_none")]
    pub time_of_last_log: Option<i64>,
}
pub type LogMutatorStack = serde_json::Value;
pub type LogInterceptorStack = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BuildDefinition {
    #[serde(
        rename = "branchIntegrationConfiguration",
        skip_serializing_if = "Option::is_none"
    )]
    pub branch_integration_configuration: Option<BranchIntegrationConfiguration>,
    #[serde(
        rename = "branchMonitoringConfiguration",
        skip_serializing_if = "Option::is_none"
    )]
    pub branch_monitoring_configuration: Option<BranchMonitoringConfiguration>,
    #[serde(
        rename = "branchSpecificConfiguration",
        skip_serializing_if = "Option::is_none"
    )]
    pub branch_specific_configuration: Option<BranchSpecificConfiguration>,
    #[serde(rename = "cleanWorkingDirectory", skip_serializing_if = "Option::is_none")]
    pub clean_working_directory: Option<bool>,
    #[serde(rename = "configObjects", skip_serializing_if = "Option::is_none")]
    pub config_objects: Option<serde_json::Value>,
    #[serde(rename = "customConfiguration", skip_serializing_if = "Option::is_none")]
    pub custom_configuration: Option<serde_json::Value>,
    #[serde(
        rename = "dockerPipelineConfiguration",
        skip_serializing_if = "Option::is_none"
    )]
    pub docker_pipeline_configuration: Option<DockerPipelineConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged: Option<bool>,
    #[serde(
        rename = "repositoryIdDefiningWorkingDir",
        skip_serializing_if = "Option::is_none"
    )]
    pub repository_id_defining_working_dir: Option<i64>,
    #[serde(rename = "taskDefinitions", skip_serializing_if = "Option::is_none")]
    pub task_definitions: Option<Vec<TaskDefinition>>,
    #[serde(rename = "triggerDefinitions", skip_serializing_if = "Option::is_none")]
    pub trigger_definitions: Option<Vec<TriggerDefinition>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<TaskConditionConfig>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<TaskDefinitionEntityType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalising: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oid: Option<BambooEntityOid>,
    #[serde(rename = "pluginKey", skip_serializing_if = "Option::is_none")]
    pub plugin_key: Option<String>,
    #[serde(rename = "rootDirectorySelector", skip_serializing_if = "Option::is_none")]
    pub root_directory_selector: Option<TaskRootDirectorySelector>,
    #[serde(rename = "userDescription", skip_serializing_if = "Option::is_none")]
    pub user_description: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskRootDirectorySelector {
    #[serde(
        rename = "repositoryDefiningWorkingDirectory",
        skip_serializing_if = "Option::is_none"
    )]
    pub repository_defining_working_directory: Option<i64>,
    #[serde(rename = "taskRootDirectoryType", skip_serializing_if = "Option::is_none")]
    pub task_root_directory_type: Option<TaskRootDirectorySelectorTaskRootDirectoryType>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum TaskRootDirectorySelectorTaskRootDirectoryType {
    #[default]
    #[serde(rename = "INHERITED")]
    Inherited,
    #[serde(rename = "DEFAULT")]
    Default,
    #[serde(rename = "REPOSITORY")]
    Repository,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum TaskDefinitionEntityType {
    #[default]
    #[serde(rename = "CHAIN")]
    Chain,
    #[serde(rename = "STAGE")]
    Stage,
    #[serde(rename = "JOB")]
    Job,
    #[serde(rename = "REPOSITORY")]
    Repository,
    #[serde(rename = "CHAIN_BRANCH")]
    ChainBranch,
    #[serde(rename = "JOB_BRANCH")]
    JobBranch,
    #[serde(rename = "TASK")]
    Task,
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "ARTIFACT_DEFINITION")]
    ArtifactDefinition,
    #[serde(rename = "DEPLOYMENT_PROJECT")]
    DeploymentProject,
    #[serde(rename = "SHARED_CREDENTIAL")]
    SharedCredential,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TaskConditionConfig {
    #[serde(rename = "conditionPluginKey", skip_serializing_if = "Option::is_none")]
    pub condition_plugin_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DockerPipelineConfiguration {
    #[serde(rename = "additionalArgs", skip_serializing_if = "Option::is_none")]
    pub additional_args: Option<Vec<String>>,
    #[serde(rename = "dataVolumes", skip_serializing_if = "Option::is_none")]
    pub data_volumes: Option<Vec<DataVolume>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataVolume {
    #[serde(rename = "containerDirectory", skip_serializing_if = "Option::is_none")]
    pub container_directory: Option<String>,
    #[serde(rename = "hostDirectory", skip_serializing_if = "Option::is_none")]
    pub host_directory: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BranchSpecificConfiguration {
    #[serde(rename = "awaitingSpecsExecution", skip_serializing_if = "Option::is_none")]
    pub awaiting_specs_execution: Option<bool>,
    #[serde(rename = "branchCleanupDisabled", skip_serializing_if = "Option::is_none")]
    pub branch_cleanup_disabled: Option<bool>,
    #[serde(rename = "ignoreSpecs", skip_serializing_if = "Option::is_none")]
    pub ignore_specs: Option<bool>,
    #[serde(rename = "notificationStrategy", skip_serializing_if = "Option::is_none")]
    pub notification_strategy: Option<BranchSpecificConfigurationNotificationStrategy>,
    #[serde(rename = "planBranchWorkflow", skip_serializing_if = "Option::is_none")]
    pub plan_branch_workflow: Option<BranchSpecificConfigurationPlanBranchWorkflow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum BranchSpecificConfigurationPlanBranchWorkflow {
    #[default]
    #[serde(rename = "BRANCH_WORKFLOW")]
    BranchWorkflow,
    #[serde(rename = "MANUAL_WORKFLOW")]
    ManualWorkflow,
    #[serde(rename = "PULL_REQUEST_WORKFLOW")]
    PullRequestWorkflow,
    #[serde(rename = "FORK_ENABLED_PULL_REQUEST_WORKFLOW")]
    ForkEnabledPullRequestWorkflow,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum BranchSpecificConfigurationNotificationStrategy {
    #[default]
    #[serde(rename = "NOTIFY_COMMITTERS")]
    NotifyCommitters,
    #[serde(rename = "INHERIT")]
    Inherit,
    #[serde(rename = "NONE")]
    None,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BranchMonitoringConfiguration {
    #[serde(rename = "branchTriggeringOption", skip_serializing_if = "Option::is_none")]
    pub branch_triggering_option: Option<
        BranchMonitoringConfigurationBranchTriggeringOption,
    >,
    #[serde(
        rename = "defaultBranchIntegrationConfiguration",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_branch_integration_configuration: Option<BranchIntegrationConfiguration>,
    #[serde(
        rename = "defaultBranchNotificationStrategy",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_branch_notification_strategy: Option<
        BranchMonitoringConfigurationDefaultBranchNotificationStrategy,
    >,
    #[serde(rename = "defaultTrigger", skip_serializing_if = "Option::is_none")]
    pub default_trigger: Option<TriggerDefinition>,
    #[serde(
        rename = "inactiveBranchCleanUpEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub inactive_branch_clean_up_enabled: Option<bool>,
    #[serde(
        rename = "inactiveBranchCleanUpPeriodInDays",
        skip_serializing_if = "Option::is_none"
    )]
    pub inactive_branch_clean_up_period_in_days: Option<i32>,
    #[serde(rename = "matchingPattern", skip_serializing_if = "Option::is_none")]
    pub matching_pattern: Option<String>,
    #[serde(
        rename = "planBranchCreationEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub plan_branch_creation_enabled: Option<bool>,
    #[serde(rename = "planBranchWorkflow", skip_serializing_if = "Option::is_none")]
    pub plan_branch_workflow: Option<BranchMonitoringConfigurationPlanBranchWorkflow>,
    #[serde(
        rename = "remoteJiraBranchLinkingEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_jira_branch_linking_enabled: Option<bool>,
    #[serde(
        rename = "removedBranchCleanUpEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub removed_branch_clean_up_enabled: Option<bool>,
    #[serde(
        rename = "removedBranchCleanUpPeriodInDays",
        skip_serializing_if = "Option::is_none"
    )]
    pub removed_branch_clean_up_period_in_days: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TriggerDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "pluginKey", skip_serializing_if = "Option::is_none")]
    pub plugin_key: Option<String>,
    #[serde(
        rename = "triggerConditionsConfiguration",
        skip_serializing_if = "Option::is_none"
    )]
    pub trigger_conditions_configuration: Option<serde_json::Value>,
    #[serde(rename = "triggeringRepositories", skip_serializing_if = "Option::is_none")]
    pub triggering_repositories: Option<Vec<i64>>,
    #[serde(rename = "userDescription", skip_serializing_if = "Option::is_none")]
    pub user_description: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum BranchMonitoringConfigurationPlanBranchWorkflow {
    #[default]
    #[serde(rename = "BRANCH_WORKFLOW")]
    BranchWorkflow,
    #[serde(rename = "MANUAL_WORKFLOW")]
    ManualWorkflow,
    #[serde(rename = "PULL_REQUEST_WORKFLOW")]
    PullRequestWorkflow,
    #[serde(rename = "FORK_ENABLED_PULL_REQUEST_WORKFLOW")]
    ForkEnabledPullRequestWorkflow,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum BranchMonitoringConfigurationDefaultBranchNotificationStrategy {
    #[default]
    #[serde(rename = "NOTIFY_COMMITTERS")]
    NotifyCommitters,
    #[serde(rename = "INHERIT")]
    Inherit,
    #[serde(rename = "NONE")]
    None,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum BranchMonitoringConfigurationBranchTriggeringOption {
    #[default]
    #[serde(rename = "INHERITED")]
    Inherited,
    #[serde(rename = "MANUAL_ONLY")]
    ManualOnly,
    #[serde(rename = "CUSTOM")]
    Custom,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BranchIntegrationConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(
        rename = "integrationPlanBranchKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub integration_plan_branch_key: Option<PlanKey>,
    #[serde(rename = "integrationPoint", skip_serializing_if = "Option::is_none")]
    pub integration_point: Option<BranchIntegrationPoint>,
    #[serde(rename = "pushEnabled", skip_serializing_if = "Option::is_none")]
    pub push_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<BranchIntegrationConfigurationStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BranchIntegrationPoint {
    #[serde(
        rename = "integrationPlanBranchKey",
        skip_serializing_if = "Option::is_none"
    )]
    pub integration_plan_branch_key: Option<PlanKey>,
    #[serde(rename = "integrationVcsReference", skip_serializing_if = "Option::is_none")]
    pub integration_vcs_reference: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PlanKey {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "partialKey", skip_serializing_if = "Option::is_none")]
    pub partial_key: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum BranchIntegrationConfigurationStrategy {
    #[default]
    #[serde(rename = "BRANCH_UPDATER")]
    BranchUpdater,
    #[serde(rename = "GATE_KEEPER")]
    GateKeeper,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConsumedSubscription {
    #[serde(rename = "artifactLink", skip_serializing_if = "Option::is_none")]
    pub artifact_link: Option<Box<ArtifactLink>>,
    #[serde(rename = "consumerResultSummary", skip_serializing_if = "Option::is_none")]
    pub consumer_result_summary: Option<Box<ResultsSummary>>,
    #[serde(rename = "destinationDirectory", skip_serializing_if = "Option::is_none")]
    pub destination_directory: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Commit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<Box<Author>>,
    #[serde(rename = "authorContext", skip_serializing_if = "Option::is_none")]
    pub author_context: Option<AuthorContext>,
    #[serde(rename = "changeSetId", skip_serializing_if = "Option::is_none")]
    pub change_set_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<CommitFile>>,
    #[serde(rename = "foreignCommit", skip_serializing_if = "Option::is_none")]
    pub foreign_commit: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "repositoryChangeset", skip_serializing_if = "Option::is_none")]
    pub repository_changeset: Option<Box<RepositoryChangeset>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepositoryChangeset {
    #[serde(rename = "buildTrigger", skip_serializing_if = "Option::is_none")]
    pub build_trigger: Option<bool>,
    #[serde(rename = "changesetId", skip_serializing_if = "Option::is_none")]
    pub changeset_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commits: Option<Vec<Box<Commit>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,
    #[serde(rename = "repositoryData", skip_serializing_if = "Option::is_none")]
    pub repository_data: Option<Box<RepositoryDataEntity>>,
    #[serde(rename = "resultsSummary", skip_serializing_if = "Option::is_none")]
    pub results_summary: Option<Box<ResultsSummary>>,
    #[serde(rename = "skippedCommitsCount", skip_serializing_if = "Option::is_none")]
    pub skipped_commits_count: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepositoryDataEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<RepositoryDataEntityEntityType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "markedForDeletion", skip_serializing_if = "Option::is_none")]
    pub marked_for_deletion: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oid: Option<BambooEntityOid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<RepositoryDataEntity>>,
    #[serde(rename = "pluginKey", skip_serializing_if = "Option::is_none")]
    pub plugin_key: Option<String>,
    #[serde(rename = "projectId", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(rename = "xmlData", skip_serializing_if = "Option::is_none")]
    pub xml_data: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum RepositoryDataEntityEntityType {
    #[default]
    #[serde(rename = "CHAIN")]
    Chain,
    #[serde(rename = "STAGE")]
    Stage,
    #[serde(rename = "JOB")]
    Job,
    #[serde(rename = "REPOSITORY")]
    Repository,
    #[serde(rename = "CHAIN_BRANCH")]
    ChainBranch,
    #[serde(rename = "JOB_BRANCH")]
    JobBranch,
    #[serde(rename = "TASK")]
    Task,
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "ARTIFACT_DEFINITION")]
    ArtifactDefinition,
    #[serde(rename = "DEPLOYMENT_PROJECT")]
    DeploymentProject,
    #[serde(rename = "SHARED_CREDENTIAL")]
    SharedCredential,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BambooEntityOid {
    #[serde(rename = "entityOid", skip_serializing_if = "Option::is_none")]
    pub entity_oid: Option<i64>,
    #[serde(rename = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<BambooEntityOidEntityType>,
    #[serde(rename = "serverKey", skip_serializing_if = "Option::is_none")]
    pub server_key: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum BambooEntityOidEntityType {
    #[default]
    #[serde(rename = "CHAIN")]
    Chain,
    #[serde(rename = "STAGE")]
    Stage,
    #[serde(rename = "JOB")]
    Job,
    #[serde(rename = "REPOSITORY")]
    Repository,
    #[serde(rename = "CHAIN_BRANCH")]
    ChainBranch,
    #[serde(rename = "JOB_BRANCH")]
    JobBranch,
    #[serde(rename = "TASK")]
    Task,
    #[serde(rename = "PROJECT")]
    Project,
    #[serde(rename = "ARTIFACT_DEFINITION")]
    ArtifactDefinition,
    #[serde(rename = "DEPLOYMENT_PROJECT")]
    DeploymentProject,
    #[serde(rename = "SHARED_CREDENTIAL")]
    SharedCredential,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CommitFile {
    #[serde(rename = "changesetId", skip_serializing_if = "Option::is_none")]
    pub changeset_id: Option<String>,
    #[serde(rename = "cleanName", skip_serializing_if = "Option::is_none")]
    pub clean_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    #[serde(rename = "revisionKnown", skip_serializing_if = "Option::is_none")]
    pub revision_known: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthorContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "linkedUserName", skip_serializing_if = "Option::is_none")]
    pub linked_user_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
pub type AddAgentAssignmentForEnvironmentRequest = String;
pub type AddAgentAssignmentForJobRequest = String;
pub type AddPermissionsForGroup1Request = Vec<String>;
pub type AddPermissionsForGroup2Request = Vec<String>;
pub type AddPermissionsForGroup3Request = Vec<String>;
pub type AddPermissionsForGroup4Request = Vec<String>;
pub type AddPermissionsForGroup5Request = Vec<String>;
pub type AddPermissionsForGroup6Request = Vec<String>;
pub type AddPermissionsForGroupRequest = Vec<String>;
pub type AddPermissionsForRole1Request = Vec<String>;
pub type AddPermissionsForRole2Request = Vec<String>;
pub type AddPermissionsForRole3Request = Vec<String>;
pub type AddPermissionsForRole4Request = Vec<String>;
pub type AddPermissionsForRole5Request = Vec<String>;
pub type AddPermissionsForRole6Request = Vec<String>;
pub type AddPermissionsForRoleRequest = Vec<String>;
pub type AddPermissionsForUser1Request = Vec<String>;
pub type AddPermissionsForUser2Request = Vec<String>;
pub type AddPermissionsForUser3Request = Vec<String>;
pub type AddPermissionsForUser4Request = Vec<String>;
pub type AddPermissionsForUser5Request = Vec<String>;
pub type AddPermissionsForUser6Request = Vec<String>;
pub type AddPermissionsForUserRequest = Vec<String>;
pub type AddUsersToGroupRequest = Vec<String>;
pub type AssignGroupsRequest = Vec<String>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BuildMonitoringLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateAccessTokenRequest {
    #[serde(rename = "daysUntilExpiry", skip_serializing_if = "Option::is_none")]
    pub days_until_expiry: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CreateCommentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}
pub type CreateUserRepositoryAliasRequest = Vec<String>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnvironmentIds {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<i64>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FilePart {
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "formField", skip_serializing_if = "Option::is_none")]
    pub form_field: Option<bool>,
    #[serde(rename = "inputStream", skip_serializing_if = "Option::is_none")]
    pub input_stream: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetEphemeralAgentPodRawLogsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prev: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<String>>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}
pub type ImmutableListCommit = serde_json::Value;
pub type ImmutableListVariableSubstitutionContext = serde_json::Value;
pub type ImmutableMapStringString = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ManualEncryptionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ManualEncryptionResponse {
    #[serde(rename = "encryptedText", skip_serializing_if = "Option::is_none")]
    pub encrypted_text: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct NextBuildNumber {
    #[serde(rename = "nextBuildNumber", skip_serializing_if = "Option::is_none")]
    pub next_build_number: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReindexBean {
    #[serde(rename = "reindexInProgress", skip_serializing_if = "Option::is_none")]
    pub reindex_in_progress: Option<bool>,
    #[serde(rename = "reindexPending", skip_serializing_if = "Option::is_none")]
    pub reindex_pending: Option<bool>,
}
pub type RemovePermissionsForGroup1Request = Vec<String>;
pub type RemovePermissionsForGroup2Request = Vec<String>;
pub type RemovePermissionsForGroup3Request = Vec<String>;
pub type RemovePermissionsForGroup4Request = Vec<String>;
pub type RemovePermissionsForGroup5Request = Vec<String>;
pub type RemovePermissionsForGroup6Request = Vec<String>;
pub type RemovePermissionsForGroupRequest = Vec<String>;
pub type RemovePermissionsForRole1Request = Vec<String>;
pub type RemovePermissionsForRole2Request = Vec<String>;
pub type RemovePermissionsForRole3Request = Vec<String>;
pub type RemovePermissionsForRole4Request = Vec<String>;
pub type RemovePermissionsForRole5Request = Vec<String>;
pub type RemovePermissionsForRole6Request = Vec<String>;
pub type RemovePermissionsForRoleRequest = Vec<String>;
pub type RemovePermissionsForUser1Request = Vec<String>;
pub type RemovePermissionsForUser2Request = Vec<String>;
pub type RemovePermissionsForUser3Request = Vec<String>;
pub type RemovePermissionsForUser4Request = Vec<String>;
pub type RemovePermissionsForUser5Request = Vec<String>;
pub type RemovePermissionsForUser6Request = Vec<String>;
pub type RemovePermissionsForUserRequest = Vec<String>;
pub type RemoveUsersFromGroupRequest = Vec<String>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestArtifactHandler {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<serde_json::Value>,
    #[serde(
        rename = "nonsharedArtifactsEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub nonshared_artifacts_enabled: Option<bool>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "sharedArtifactsEnabled", skip_serializing_if = "Option::is_none")]
    pub shared_artifacts_enabled: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestAuditLogConfiguration {
    #[serde(rename = "auditLoggingEnabled", skip_serializing_if = "Option::is_none")]
    pub audit_logging_enabled: Option<bool>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
pub type RestBranches = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestBuildConcurrency {
    #[serde(rename = "buildConcurrencyEnabled", skip_serializing_if = "Option::is_none")]
    pub build_concurrency_enabled: Option<bool>,
    #[serde(
        rename = "defaultConcurrentBuildsNumber",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_concurrent_builds_number: Option<i32>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestBuildMonitoring {
    #[serde(rename = "buildMonitoringEnabled", skip_serializing_if = "Option::is_none")]
    pub build_monitoring_enabled: Option<bool>,
    #[serde(
        rename = "buildQueueMinutesTimeoutDefault",
        skip_serializing_if = "Option::is_none"
    )]
    pub build_queue_minutes_timeout_default: Option<i32>,
    #[serde(
        rename = "buildTimeMultiplierDefault",
        skip_serializing_if = "Option::is_none"
    )]
    pub build_time_multiplier_default: Option<f64>,
    #[serde(rename = "enableLogLineCountLimit", skip_serializing_if = "Option::is_none")]
    pub enable_log_line_count_limit: Option<bool>,
    #[serde(
        rename = "enableLogLineLengthLimit",
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_log_line_length_limit: Option<bool>,
    #[serde(rename = "enableLogSizeLimit", skip_serializing_if = "Option::is_none")]
    pub enable_log_size_limit: Option<bool>,
    #[serde(rename = "forceStopHangingBuilds", skip_serializing_if = "Option::is_none")]
    pub force_stop_hanging_builds: Option<bool>,
    #[serde(rename = "liveLogsAreActive", skip_serializing_if = "Option::is_none")]
    pub live_logs_are_active: Option<bool>,
    #[serde(
        rename = "logQuietMinutesTimeDefault",
        skip_serializing_if = "Option::is_none"
    )]
    pub log_quiet_minutes_time_default: Option<i32>,
    #[serde(rename = "maxLogLineCount", skip_serializing_if = "Option::is_none")]
    pub max_log_line_count: Option<i32>,
    #[serde(rename = "maxLogLineLength", skip_serializing_if = "Option::is_none")]
    pub max_log_line_length: Option<i32>,
    #[serde(rename = "maxLogSizeKilobytes", skip_serializing_if = "Option::is_none")]
    pub max_log_size_kilobytes: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestChart {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(rename = "imageMap", skip_serializing_if = "Option::is_none")]
    pub image_map: Option<String>,
    #[serde(rename = "imageMapName", skip_serializing_if = "Option::is_none")]
    pub image_map_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestCombinedExpiryConfiguration {
    #[serde(rename = "buildsToKeep", skip_serializing_if = "Option::is_none")]
    pub builds_to_keep: Option<i32>,
    #[serde(rename = "cronExpression", skip_serializing_if = "Option::is_none")]
    pub cron_expression: Option<String>,
    #[serde(rename = "deploymentsToKeep", skip_serializing_if = "Option::is_none")]
    pub deployments_to_keep: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "expireArtifacts", skip_serializing_if = "Option::is_none")]
    pub expire_artifacts: Option<bool>,
    #[serde(rename = "expireLogs", skip_serializing_if = "Option::is_none")]
    pub expire_logs: Option<bool>,
    #[serde(rename = "expireResults", skip_serializing_if = "Option::is_none")]
    pub expire_results: Option<bool>,
    #[serde(rename = "labelsToExclude", skip_serializing_if = "Option::is_none")]
    pub labels_to_exclude: Option<String>,
    #[serde(rename = "maximumBuildsToKeep", skip_serializing_if = "Option::is_none")]
    pub maximum_builds_to_keep: Option<i32>,
    #[serde(rename = "maximumIgnoredLogSize", skip_serializing_if = "Option::is_none")]
    pub maximum_ignored_log_size: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestCreateVersionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "nextVersionName", skip_serializing_if = "Option::is_none")]
    pub next_version_name: Option<String>,
    #[serde(rename = "planResultKey", skip_serializing_if = "Option::is_none")]
    pub plan_result_key: Option<String>,
}
pub type RestDeploymentVersionList = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestElasticInstanceLog {
    #[serde(rename = "instanceId", skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestEnableContainer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestErrorCollection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    #[serde(rename = "fieldErrors", skip_serializing_if = "Option::is_none")]
    pub field_errors: Option<serde_json::Value>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestFavicon {
    #[serde(rename = "faviconType", skip_serializing_if = "Option::is_none")]
    pub favicon_type: Option<String>,
    #[serde(rename = "faviconUrl", skip_serializing_if = "Option::is_none")]
    pub favicon_url: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestGeneralConfiguration {
    #[serde(rename = "baseUrl", skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    #[serde(
        rename = "branchDetectionIntervalSeconds",
        skip_serializing_if = "Option::is_none"
    )]
    pub branch_detection_interval_seconds: Option<i32>,
    #[serde(rename = "brokerClientUrl", skip_serializing_if = "Option::is_none")]
    pub broker_client_url: Option<String>,
    #[serde(rename = "brokerUrl", skip_serializing_if = "Option::is_none")]
    pub broker_url: Option<String>,
    #[serde(
        rename = "dashboardDefaultPageSize",
        skip_serializing_if = "Option::is_none"
    )]
    pub dashboard_default_page_size: Option<i32>,
    #[serde(rename = "enableGravatarSupport", skip_serializing_if = "Option::is_none")]
    pub enable_gravatar_support: Option<bool>,
    #[serde(rename = "enableGzipCompression", skip_serializing_if = "Option::is_none")]
    pub enable_gzip_compression: Option<bool>,
    #[serde(rename = "enableRssPolling", skip_serializing_if = "Option::is_none")]
    pub enable_rss_polling: Option<bool>,
    #[serde(rename = "gravatarServerUrl", skip_serializing_if = "Option::is_none")]
    pub gravatar_server_url: Option<String>,
    #[serde(rename = "instanceName", skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    #[serde(
        rename = "quietPeriodGloballyDisabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub quiet_period_globally_disabled: Option<bool>,
    #[serde(
        rename = "rssPollingCronExpression",
        skip_serializing_if = "Option::is_none"
    )]
    pub rss_polling_cron_expression: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestIMServerConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "requireTLSSSLConnection", skip_serializing_if = "Option::is_none")]
    pub require_tlssslconnection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestIdContainer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestMailConfiguration {
    #[serde(rename = "emailSetting", skip_serializing_if = "Option::is_none")]
    pub email_setting: Option<String>,
    #[serde(rename = "fromAddress", skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
    #[serde(rename = "jndiLocation", skip_serializing_if = "Option::is_none")]
    pub jndi_location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "precedenceBulkHeaderExcluded",
        skip_serializing_if = "Option::is_none"
    )]
    pub precedence_bulk_header_excluded: Option<bool>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "smtpPassword", skip_serializing_if = "Option::is_none")]
    pub smtp_password: Option<String>,
    #[serde(rename = "smtpPort", skip_serializing_if = "Option::is_none")]
    pub smtp_port: Option<String>,
    #[serde(rename = "smtpServer", skip_serializing_if = "Option::is_none")]
    pub smtp_server: Option<String>,
    #[serde(rename = "smtpUsername", skip_serializing_if = "Option::is_none")]
    pub smtp_username: Option<String>,
    #[serde(rename = "subjectPrefix", skip_serializing_if = "Option::is_none")]
    pub subject_prefix: Option<String>,
    #[serde(rename = "tlsEnabled", skip_serializing_if = "Option::is_none")]
    pub tls_enabled: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestNamingPreview {
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "nextVersionName", skip_serializing_if = "Option::is_none")]
    pub next_version_name: Option<String>,
    #[serde(rename = "subsequentVersionName", skip_serializing_if = "Option::is_none")]
    pub subsequent_version_name: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestNewUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "fullName", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "passwordConfirm", skip_serializing_if = "Option::is_none")]
    pub password_confirm: Option<String>,
    #[serde(rename = "sanitizedName", skip_serializing_if = "Option::is_none")]
    pub sanitized_name: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
pub type RestPlanSpec = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestQuarantineConfig {
    #[serde(rename = "quarantineTestsEnabled", skip_serializing_if = "Option::is_none")]
    pub quarantine_tests_enabled: Option<bool>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestRemoteAgentCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestRemoteAgentConfiguration {
    #[serde(rename = "remoteAgentsSupported", skip_serializing_if = "Option::is_none")]
    pub remote_agents_supported: Option<bool>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestRepositoryConnectionResult {
    #[serde(rename = "connectionErrors", skip_serializing_if = "Option::is_none")]
    pub connection_errors: Option<Vec<String>>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestRepositoryMinimal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestStorageConfiguration {
    #[serde(rename = "hardLimit", skip_serializing_if = "Option::is_none")]
    pub hard_limit: Option<i32>,
    #[serde(rename = "softLimit", skip_serializing_if = "Option::is_none")]
    pub soft_limit: Option<i32>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestUserDarkFeature {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "userName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestUserPasswordUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "passwordConfirm", skip_serializing_if = "Option::is_none")]
    pub password_confirm: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestUserRenameRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "oldName", skip_serializing_if = "Option::is_none")]
    pub old_name: Option<String>,
}
pub type RestVcsBranches = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RestVerificationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "rawValue", skip_serializing_if = "Option::is_none")]
    pub raw_value: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RunExpiryResponse {
    #[serde(rename = "statusUrl", skip_serializing_if = "Option::is_none")]
    pub status_url: Option<String>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SecuritySettingsLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
}
pub type SetVisibleFiltersRequest = Vec<i64>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimpleRestArtifactHandler {
    #[serde(
        rename = "nonsharedArtifactsEnabled",
        skip_serializing_if = "Option::is_none"
    )]
    pub nonshared_artifacts_enabled: Option<bool>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_: Option<String>,
    #[serde(rename = "sharedArtifactsEnabled", skip_serializing_if = "Option::is_none")]
    pub shared_artifacts_enabled: Option<bool>,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Status {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "statusCode", skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
}
pub type TestConnection1Request = serde_json::Value;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TestConnectionResultDto {
    #[serde(rename = "clientVersion", skip_serializing_if = "Option::is_none")]
    pub client_version: Option<String>,
    #[serde(rename = "errorMessage", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "serverVersion", skip_serializing_if = "Option::is_none")]
    pub server_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}
pub type UnassignGroupsRequest = Vec<String>;
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserBean {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "fullName", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
pub type VersionName = serde_json::Value;
