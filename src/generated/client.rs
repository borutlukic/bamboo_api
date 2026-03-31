//! Generated HTTP client for regular API requests
//!
//! This file contains the HTTP client implementation for GET, POST, etc.
//! Do not edit manually - regenerate using the appropriate script.
#![allow(clippy::format_in_format_args)]
#![allow(clippy::let_unit_value)]
use super::types::*;
use thiserror::Error;
/// HTTP client errors that can occur during API requests
#[derive(Error, Debug)]
pub enum HttpError {
    /// Network or connection error (from reqwest)
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),
    /// Middleware error (from reqwest-middleware)
    #[error("Middleware error: {0}")]
    Middleware(#[from] reqwest_middleware::Error),
    /// Request serialization error
    #[error("Failed to serialize request: {0}")]
    Serialization(String),
    /// Response deserialization error
    #[error("Failed to deserialize response: {0}")]
    Deserialization(String),
    /// HTTP error response (4xx, 5xx)
    #[error("HTTP error {status}: {message}")]
    Http { status: u16, message: String, body: Option<String> },
    /// Authentication error
    #[error("Authentication error: {0}")]
    Auth(String),
    /// Request timeout
    #[error("Request timeout")]
    Timeout,
    /// Invalid configuration
    #[error("Configuration error: {0}")]
    Config(String),
    /// Generic error
    #[error("{0}")]
    Other(String),
}
impl HttpError {
    /// Create an HTTP error from a status code and message
    pub fn from_status(
        status: u16,
        message: impl Into<String>,
        body: Option<String>,
    ) -> Self {
        Self::Http {
            status,
            message: message.into(),
            body,
        }
    }
    /// Create a serialization error
    pub fn serialization_error(error: impl std::fmt::Display) -> Self {
        Self::Serialization(error.to_string())
    }
    /// Create a deserialization error
    pub fn deserialization_error(error: impl std::fmt::Display) -> Self {
        Self::Deserialization(error.to_string())
    }
    /// Check if this is a client error (4xx)
    pub fn is_client_error(&self) -> bool {
        matches!(self, Self::Http { status, .. } if * status >= 400 && * status < 500)
    }
    /// Check if this is a server error (5xx)
    pub fn is_server_error(&self) -> bool {
        matches!(self, Self::Http { status, .. } if * status >= 500 && * status < 600)
    }
    /// Check if this error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            Self::Network(_) => true,
            Self::Middleware(_) => true,
            Self::Timeout => true,
            Self::Http { status, .. } => matches!(status, 429 | 500 | 502 | 503 | 504),
            _ => false,
        }
    }
}
/// Result type for HTTP operations
pub type HttpResult<T> = Result<T, HttpError>;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use std::collections::BTreeMap;
/// HTTP client for making API requests
#[derive(Clone)]
pub struct HttpClient {
    base_url: String,
    api_key: Option<String>,
    http_client: ClientWithMiddleware,
    custom_headers: BTreeMap<String, String>,
}
impl HttpClient {
    /// Create a new HTTP client with default configuration
    pub fn new() -> Self {
        Self::with_config(true)
    }
    /// Create a new HTTP client with custom configuration
    pub fn with_config(enable_tracing: bool) -> Self {
        let reqwest_client = {
                    let mut headers = reqwest::header::HeaderMap::new();
                    headers.insert(
                        reqwest::header::ACCEPT,
                        reqwest::header::HeaderValue::from_static("application/json"),
                    );
                    reqwest::Client::builder()
                        .default_headers(headers)
                        .build()
                        .expect("failed to build HTTP client")
                };
        let mut client_builder = ClientBuilder::new(reqwest_client);
        if enable_tracing {
            use reqwest_tracing::TracingMiddleware;
            client_builder = client_builder.with(TracingMiddleware::default());
        }
        let http_client = client_builder.build();
        Self {
            base_url: String::new(),
            api_key: None,
            http_client,
            custom_headers: BTreeMap::new(),
        }
    }
    /// Set the base URL for all requests
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        self.base_url = base_url.into();
        self
    }
    /// Set the API key for authentication
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }
    /// Add a custom header to all requests
    pub fn with_header(
        mut self,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.custom_headers.insert(name.into(), value.into());
        self
    }
    /// Add multiple custom headers
    pub fn with_headers(mut self, headers: BTreeMap<String, String>) -> Self {
        self.custom_headers.extend(headers);
        self
    }
}
impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}
impl HttpClient {
    ///PUT /api/latest/quickFilter/{id}/activate
    pub async fn activate_filter(&self, id: i64) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/quickFilter/{}/activate", id)
        );
        let mut req = self.http_client.put(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/agent/assignment
    pub async fn add_agent_assignment(
        &self,
        executor_type: Option<impl AsRef<str>>,
        executor_id: Option<i64>,
        entity_id: Option<i64>,
        assignment_type: Option<impl AsRef<str>>,
    ) -> HttpResult<RestDedicatedAgent> {
        let url = format!("{}{}", self.base_url, "/api/latest/agent/assignment");
        let mut req = self.http_client.post(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = executor_type {
                query_params.push(("executorType", v.as_ref().to_string()));
            }
            if let Some(v) = executor_id {
                query_params.push(("executorId", v.to_string()));
            }
            if let Some(v) = entity_id {
                query_params.push(("entityId", v.to_string()));
            }
            if let Some(v) = assignment_type {
                query_params.push(("assignmentType", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/deploy/environment/{environmentId}/agent-assignment
    pub async fn add_agent_assignment_for_environment(
        &self,
        environment_id: impl AsRef<str>,
        request: AddAgentAssignmentForEnvironmentRequest,
    ) -> HttpResult<RestAgentAssignmentExecutorDetails> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/deploy/environment/{}/agent-assignment", environment_id
            .as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/config/job/{jobKey}/agent-assignment
    pub async fn add_agent_assignment_for_job(
        &self,
        job_key: impl AsRef<str>,
        request: AddAgentAssignmentForJobRequest,
    ) -> HttpResult<RestAgentAssignmentExecutorDetails> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/config/job/{}/agent-assignment",
            job_key.as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/deploy/project/{deploymentProjectId}/repository
    pub async fn add_assigned_repository(
        &self,
        deployment_project_id: impl AsRef<str>,
        request: RestIdContainer,
    ) -> HttpResult<RestRepositoryMinimal> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/deploy/project/{}/repository",
            deployment_project_id.as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/project/{projectKey}/repository
    pub async fn add_assigned_repository_1(
        &self,
        project_key: impl AsRef<str>,
        request: RestIdContainer,
    ) -> HttpResult<RestRepositoryMinimal> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/project/{}/repository",
            project_key.as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/result/{projectKey}-{buildKey}-{buildNumber}/comment
    pub async fn add_build_comment(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        build_number: impl AsRef<str>,
        request: CreateCommentRequest,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/result/{}-{}-{}/comment",
            project_key.as_ref(), build_key.as_ref(), build_number.as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/result/{projectKey}-{buildKey}-{buildNumber}/label
    pub async fn add_build_label(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        build_number: impl AsRef<str>,
        request: RestBuildLabel,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/result/{}-{}-{}/label",
            project_key.as_ref(), build_key.as_ref(), build_number.as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/ephemeral/templateConfiguration/{configurationId}/capability
    pub async fn add_capability(
        &self,
        configuration_id: i64,
        request: RestCapability,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/ephemeral/templateConfiguration/{}/capability",
            configuration_id)
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/deploy/project
    pub async fn add_deployment_project(
        &self,
        request: RestCreateDeploymentProjectRequest,
    ) -> HttpResult<RestDeploymentProject> {
        let url = format!("{}{}", self.base_url, "/api/latest/deploy/project");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/permissions/deployment/{id}/groups/{name}
    pub async fn add_permissions_for_group(
        &self,
        name: impl AsRef<str>,
        id: impl AsRef<str>,
        request: AddPermissionsForGroupRequest,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/deployment/{}/groups/{}", name.as_ref(), id
            .as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/permissions/environment/{id}/groups/{name}
    pub async fn add_permissions_for_group_1(
        &self,
        name: impl AsRef<str>,
        id: impl AsRef<str>,
        request: AddPermissionsForGroup1Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/environment/{}/groups/{}", name.as_ref(), id
            .as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/permissions/global/groups/{name}
    pub async fn add_permissions_for_group_2(
        &self,
        name: impl AsRef<str>,
        ignore: Option<impl AsRef<str>>,
        request: AddPermissionsForGroup2Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/permissions/global/groups/{}",
            name.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = ignore {
                query_params.push(("ignore", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/permissions/plan/{key}/groups/{name}
    pub async fn add_permissions_for_group_3(
        &self,
        name: impl AsRef<str>,
        key: impl AsRef<str>,
        request: AddPermissionsForGroup3Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/permissions/plan/{}/groups/{}",
            name.as_ref(), key.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/permissions/project/{key}/groups/{name}
    pub async fn add_permissions_for_group_4(
        &self,
        name: impl AsRef<str>,
        key: impl AsRef<str>,
        request: AddPermissionsForGroup4Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/project/{}/groups/{}", name.as_ref(), key
            .as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/permissions/projectplan/{key}/groups/{name}
    pub async fn add_permissions_for_group_5(
        &self,
        name: impl AsRef<str>,
        key: impl AsRef<str>,
        request: AddPermissionsForGroup5Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/projectplan/{}/groups/{}", name.as_ref(),
            key.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/permissions/repository/{id}/groups/{name}
    pub async fn add_permissions_for_group_6(
        &self,
        name: impl AsRef<str>,
        id: impl AsRef<str>,
        request: AddPermissionsForGroup6Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/repository/{}/groups/{}", name.as_ref(), id
            .as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/permissions/deployment/{id}/roles/{name}
    pub async fn add_permissions_for_role(
        &self,
        name: impl AsRef<str>,
        id: impl AsRef<str>,
        request: AddPermissionsForRoleRequest,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/deployment/{}/roles/{}", name.as_ref(), id
            .as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/permissions/environment/{id}/roles/{name}
    pub async fn add_permissions_for_role_1(
        &self,
        name: impl AsRef<str>,
        id: impl AsRef<str>,
        request: AddPermissionsForRole1Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/environment/{}/roles/{}", name.as_ref(), id
            .as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/permissions/global/roles/{name}
    pub async fn add_permissions_for_role_2(
        &self,
        name: impl AsRef<str>,
        ignore: Option<impl AsRef<str>>,
        request: AddPermissionsForRole2Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/permissions/global/roles/{}",
            name.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = ignore {
                query_params.push(("ignore", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/permissions/plan/{key}/roles/{name}
    pub async fn add_permissions_for_role_3(
        &self,
        name: impl AsRef<str>,
        key: impl AsRef<str>,
        request: AddPermissionsForRole3Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/permissions/plan/{}/roles/{}",
            name.as_ref(), key.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/permissions/project/{key}/roles/{name}
    pub async fn add_permissions_for_role_4(
        &self,
        name: impl AsRef<str>,
        key: impl AsRef<str>,
        request: AddPermissionsForRole4Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/permissions/project/{}/roles/{}",
            name.as_ref(), key.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/permissions/projectplan/{key}/roles/{name}
    pub async fn add_permissions_for_role_5(
        &self,
        name: impl AsRef<str>,
        key: impl AsRef<str>,
        request: AddPermissionsForRole5Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/projectplan/{}/roles/{}", name.as_ref(), key
            .as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/permissions/repository/{id}/roles/{name}
    pub async fn add_permissions_for_role_6(
        &self,
        name: impl AsRef<str>,
        id: impl AsRef<str>,
        request: AddPermissionsForRole6Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/repository/{}/roles/{}", name.as_ref(), id
            .as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/permissions/deployment/{id}/users/{name}
    pub async fn add_permissions_for_user(
        &self,
        name: impl AsRef<str>,
        id: impl AsRef<str>,
        request: AddPermissionsForUserRequest,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/deployment/{}/users/{}", name.as_ref(), id
            .as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/permissions/environment/{id}/users/{name}
    pub async fn add_permissions_for_user_1(
        &self,
        name: impl AsRef<str>,
        id: impl AsRef<str>,
        request: AddPermissionsForUser1Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/environment/{}/users/{}", name.as_ref(), id
            .as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/permissions/global/users/{name}
    pub async fn add_permissions_for_user_2(
        &self,
        name: impl AsRef<str>,
        ignore: Option<impl AsRef<str>>,
        request: AddPermissionsForUser2Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/permissions/global/users/{}",
            name.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = ignore {
                query_params.push(("ignore", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/permissions/plan/{key}/users/{name}
    pub async fn add_permissions_for_user_3(
        &self,
        name: impl AsRef<str>,
        key: impl AsRef<str>,
        request: AddPermissionsForUser3Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/permissions/plan/{}/users/{}",
            name.as_ref(), key.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/permissions/project/{key}/users/{name}
    pub async fn add_permissions_for_user_4(
        &self,
        name: impl AsRef<str>,
        key: impl AsRef<str>,
        request: AddPermissionsForUser4Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/permissions/project/{}/users/{}",
            name.as_ref(), key.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/permissions/projectplan/{key}/users/{name}
    pub async fn add_permissions_for_user_5(
        &self,
        name: impl AsRef<str>,
        key: impl AsRef<str>,
        request: AddPermissionsForUser5Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/projectplan/{}/users/{}", name.as_ref(), key
            .as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/permissions/repository/{id}/users/{name}
    pub async fn add_permissions_for_user_6(
        &self,
        name: impl AsRef<str>,
        id: impl AsRef<str>,
        request: AddPermissionsForUser6Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/repository/{}/users/{}", name.as_ref(), id
            .as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/plan/{projectKey}-{buildKey}/label
    pub async fn add_plan_label(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        request: RestPlanLabel,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/plan/{}-{}/label", project_key
            .as_ref(), build_key.as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/plan/{projectKey}-{buildKey}/variables
    pub async fn add_plan_variable(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        request: RestVariable,
    ) -> HttpResult<RestVariable> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/plan/{}-{}/variables",
            project_key.as_ref(), build_key.as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/deploy/environment/{environmentId}/requirement
    pub async fn add_requirement_for_environment(
        &self,
        environment_id: impl AsRef<str>,
        request: RestRequirement,
    ) -> HttpResult<RestRequirement> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/deploy/environment/{}/requirement", environment_id
            .as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /responsibility/latest/brokenBuild/{planResultKeyOrPlanKey}/{name}
    pub async fn add_responsible(
        &self,
        name: impl AsRef<str>,
        plan_result_key_or_plan_key: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/responsibility/latest/brokenBuild/{}/{}",
            name.as_ref(), plan_result_key_or_plan_key.as_ref())
        );
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/admin/groups/{name}/add-users
    pub async fn add_users_to_group(
        &self,
        name: impl AsRef<str>,
        request: AddUsersToGroupRequest,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/admin/groups/{}/add-users", name
            .as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/admin/users/{name}/groups
    pub async fn assign_groups(
        &self,
        name: impl AsRef<str>,
        request: AssignGroupsRequest,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/admin/users/{}/groups", name
            .as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/agent/authentication/{agentUuid}
    pub async fn authenticate_agent(
        &self,
        agent_uuid: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/agent/authentication/{}",
            agent_uuid.as_ref())
        );
        let mut req = self.http_client.put(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/buildNumber/{projectKey}-{buildKey}/bump
    pub async fn bump_build_number(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        request: NextBuildNumber,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/buildNumber/{}-{}/bump",
            project_key.as_ref(), build_key.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/admin/users/credentials
    pub async fn change_user_password(
        &self,
        request: RestUserPasswordUpdate,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/latest/admin/users/credentials");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/queue/{projectKey}-{buildKey}-{buildNumber}
    pub async fn continue_build(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        build_number: i64,
        execute_all_stages: Option<bool>,
        stage: Option<impl AsRef<str>>,
    ) -> HttpResult<RestQueuedBuild> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/queue/{}-{}-{}", project_key
            .as_ref(), build_key.as_ref(), build_number)
        );
        let mut req = self.http_client.put(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = execute_all_stages {
                query_params.push(("executeAllStages", v.to_string()));
            }
            if let Some(v) = stage {
                query_params.push(("stage", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /admin/latest/security/trustedKey
    pub async fn create(&self, request: RestTrustedKey) -> HttpResult<RestTrustedKey> {
        let url = format!("{}{}", self.base_url, "/admin/latest/security/trustedKey");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/access-token
    pub async fn create_access_token(
        &self,
        request: CreateAccessTokenRequest,
    ) -> HttpResult<RestAccessToken> {
        let url = format!("{}{}", self.base_url, "/api/latest/access-token");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/agent/{agentId}/capability
    pub async fn create_agent_capability(
        &self,
        agent_id: i64,
        request: RestCapability,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/agent/{}/capability", agent_id)
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/deploy/environment/{environmentId}/variable
    pub async fn create_environment_variable(
        &self,
        environment_id: impl AsRef<str>,
        request: RestVariable,
    ) -> HttpResult<RestVariableDefinitionContext> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/deploy/environment/{}/variable",
            environment_id.as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /admin/latest/globalVariables
    pub async fn create_global_variable(
        &self,
        request: RestGlobalVariable,
    ) -> HttpResult<RestGlobalVariable> {
        let url = format!("{}{}", self.base_url, "/admin/latest/globalVariables");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /admin/latest/security/groups
    pub async fn create_group(&self, request: RestGroup) -> HttpResult<RestGroup> {
        let url = format!("{}{}", self.base_url, "/admin/latest/security/groups");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/admin/groups
    pub async fn create_group_1(&self, request: RestGroup) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/latest/admin/groups");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/elasticConfiguration
    pub async fn create_image_configuration(
        &self,
        request: RestElasticImageConfig,
    ) -> HttpResult<RestElasticImageConfig> {
        let url = format!("{}{}", self.base_url, "/api/latest/elasticConfiguration");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/plan/{projectKey}-{buildKey}/branch/{branchName}
    pub async fn create_new_branch(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        branch_name: impl AsRef<str>,
        cleanup_enabled: Option<impl AsRef<str>>,
        vcs_branch: Option<impl AsRef<str>>,
        enabled: Option<impl AsRef<str>>,
    ) -> HttpResult<RestPlanBranch> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/plan/{}-{}/branch/{}",
            project_key.as_ref(), build_key.as_ref(), branch_name.as_ref())
        );
        let mut req = self.http_client.put(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = cleanup_enabled {
                query_params.push(("cleanupEnabled", v.as_ref().to_string()));
            }
            if let Some(v) = vcs_branch {
                query_params.push(("vcsBranch", v.as_ref().to_string()));
            }
            if let Some(v) = enabled {
                query_params.push(("enabled", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/project/{projectKey}/variable
    pub async fn create_or_update_variable(
        &self,
        project_key: impl AsRef<str>,
        request: RestVariable,
    ) -> HttpResult<RestVariableDefinitionContext> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/project/{}/variable", project_key
            .as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/project
    pub async fn create_project(
        &self,
        request: RestProjectCreate,
    ) -> HttpResult<RestProjectCreate> {
        let url = format!("{}{}", self.base_url, "/api/latest/project");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/quickFilter
    pub async fn create_quick_filter(
        &self,
        request: RestQuickFilter,
    ) -> HttpResult<RestQuickFilter> {
        let url = format!("{}{}", self.base_url, "/api/latest/quickFilter");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/ephemeral/templateConfiguration
    pub async fn create_template_configuration(
        &self,
        request: RestEphemeralAgentTemplate,
    ) -> HttpResult<RestEphemeralAgentTemplate> {
        let url = format!(
            "{}{}", self.base_url, "/api/latest/ephemeral/templateConfiguration"
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/admin/users
    pub async fn create_user(&self, request: RestNewUser) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/latest/admin/users");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/admin/users/{name}/alias
    pub async fn create_user_repository_alias(
        &self,
        name: impl AsRef<str>,
        request: CreateUserRepositoryAliasRequest,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/admin/users/{}/alias", name
            .as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/deploy/project/{deploymentProjectId}/version
    pub async fn create_version(
        &self,
        deployment_project_id: impl AsRef<str>,
        request: RestCreateVersionRequest,
    ) -> HttpResult<RestDeploymentVersion> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/deploy/project/{}/version",
            deployment_project_id.as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/quickFilter/deactivate
    pub async fn deactivate_all_filters(&self) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/latest/quickFilter/deactivate");
        let mut req = self.http_client.put(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/quickFilter/{id}/deactivate
    pub async fn deactivate_filter(&self, id: i64) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/quickFilter/{}/deactivate", id)
        );
        let mut req = self.http_client.put(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /admin/latest/security/trustedKey/{id}
    pub async fn delete(&self, id: impl AsRef<str>) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/admin/latest/security/trustedKey/{}", id
            .as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/agent/{agentId}
    pub async fn delete_agent(&self, agent_id: i64) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/agent/{}", agent_id)
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/agent/{agentId}/capability/{capabilityKey}
    pub async fn delete_agent_capability(
        &self,
        agent_id: i64,
        capability_key: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/agent/{}/capability/{}",
            agent_id, capability_key.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/agent/{agentId}/capability
    pub async fn delete_all_agent_capabilities(&self, agent_id: i64) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/agent/{}/capability", agent_id)
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/avatar/user/avatar.png
    pub async fn delete_avatar(&self) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/latest/avatar/user/avatar.png");
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/ephemeral/templateConfiguration/{configurationId}/capability/{name}
    pub async fn delete_capability(
        &self,
        name: impl AsRef<str>,
        configuration_id: i64,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/ephemeral/templateConfiguration/{}/capability/{}", name
            .as_ref(), configuration_id)
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/elasticConfiguration/{configurationId}
    pub async fn delete_configuration(&self, configuration_id: i64) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/elasticConfiguration/{}",
            configuration_id)
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/deploy/project/{deploymentProjectId}
    pub async fn delete_deployment_project(
        &self,
        deployment_project_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/deploy/project/{}",
            deployment_project_id.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/deploy/environment/{environmentId}/variable/{variableName}
    pub async fn delete_environment_variable(
        &self,
        variable_name: impl AsRef<str>,
        environment_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/deploy/environment/{}/variable/{}", variable_name
            .as_ref(), environment_id.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /admin/latest/globalVariables/{variableId}
    pub async fn delete_global_variable(
        &self,
        variable_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/admin/latest/globalVariables/{}",
            variable_id.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/admin/groups/{name}
    pub async fn delete_group(&self, name: impl AsRef<str>) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/admin/groups/{}", name.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /admin/latest/config/imServer
    pub async fn delete_im_server_configuration(&self) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/admin/latest/config/imServer");
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /admin/latest/config/mailServer
    pub async fn delete_mail_configuration(&self) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/admin/latest/config/mailServer");
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/plan/{projectKey}-{buildKey}
    pub async fn delete_plan(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/plan/{}-{}", project_key
            .as_ref(), build_key.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/plan/{projectKey}-{buildKey}/variables/{variableName}
    pub async fn delete_plan_variable(
        &self,
        project_key: impl AsRef<str>,
        variable_name: impl AsRef<str>,
        build_key: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/plan/{}-{}/variables/{}",
            project_key.as_ref(), variable_name.as_ref(), build_key.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/project/{projectKey}
    pub async fn delete_project(&self, project_key: impl AsRef<str>) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/project/{}", project_key
            .as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/project/{projectKey}/sharedCredentials/{sharedCredentialId}
    pub async fn delete_project_shared_credentials(
        &self,
        project_key: impl AsRef<str>,
        shared_credential_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/project/{}/sharedCredentials/{}",
            project_key.as_ref(), shared_credential_id.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/project/{projectKey}/variable/{variableName}
    pub async fn delete_project_variable(
        &self,
        project_key: impl AsRef<str>,
        variable_name: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/project/{}/variable/{}",
            project_key.as_ref(), variable_name.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/quickFilter/{id}
    pub async fn delete_quick_filter(&self, id: i64) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/quickFilter/{}", id)
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/deploy/project/{deploymentProjectId}/repository/{repositoryId}
    pub async fn delete_repository_mapping(
        &self,
        repository_id: impl AsRef<str>,
        deployment_project_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/deploy/project/{}/repository/{}",
            repository_id.as_ref(), deployment_project_id.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/project/{projectKey}/repository/{repositoryId}
    pub async fn delete_repository_mapping_1(
        &self,
        project_key: impl AsRef<str>,
        repository_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/project/{}/repository/{}",
            project_key.as_ref(), repository_id.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/ephemeral/templateConfiguration/{configurationId}
    pub async fn delete_template_configuration(
        &self,
        configuration_id: i64,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/ephemeral/templateConfiguration/{}", configuration_id)
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/admin/users/{name}
    pub async fn delete_user(&self, name: impl AsRef<str>) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/admin/users/{}", name.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/agent/{agentId}/disable
    pub async fn disable_agent(&self, agent_id: i64) -> HttpResult<RestAgent> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/agent/{}/disable", agent_id)
        );
        let mut req = self.http_client.put(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/plan/{projectKey}-{buildKey}/enable
    pub async fn disable_plan(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/plan/{}-{}/enable", project_key
            .as_ref(), build_key.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/deploy/project/{deploymentProjectId}
    pub async fn edit_deployment_project(
        &self,
        deployment_project_id: impl AsRef<str>,
        request: RestUpdateDeploymentProjectRequest,
    ) -> HttpResult<RestDeploymentProject> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/deploy/project/{}",
            deployment_project_id.as_ref())
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/plan/{projectKey}-{buildKey}/variables/{variableName}
    pub async fn edit_plan_variable(
        &self,
        project_key: impl AsRef<str>,
        variable_name: impl AsRef<str>,
        build_key: impl AsRef<str>,
        request: RestVariable,
    ) -> HttpResult<RestVariable> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/plan/{}-{}/variables/{}",
            project_key.as_ref(), variable_name.as_ref(), build_key.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/agent/{agentId}/enable
    pub async fn enable_agent(&self, agent_id: i64) -> HttpResult<RestAgent> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/agent/{}/enable", agent_id)
        );
        let mut req = self.http_client.put(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/repository/{repositoryId}/enableAllProjectsAccess
    pub async fn enable_all_projects_access(
        &self,
        repository_id: i64,
        request: RestEnableContainer,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/repository/{}/enableAllProjectsAccess", repository_id)
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/project/{projectKey}/repository/{repositoryId}/enableAllRepositoriesAccess
    pub async fn enable_all_repositories_access(
        &self,
        project_key: impl AsRef<str>,
        repository_id: i64,
        request: RestEnableContainer,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/project/{}/repository/{}/enableAllRepositoriesAccess",
            project_key.as_ref(), repository_id)
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/repository/{repositoryId}/enableAllRepositoriesAccess
    pub async fn enable_all_repositories_access_1(
        &self,
        repository_id: i64,
        request: RestEnableContainer,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/repository/{}/enableAllRepositoriesAccess",
            repository_id)
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/repository/{repositoryId}/enableCi
    pub async fn enable_ci(
        &self,
        repository_id: i64,
        request: RestEnableContainer,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/repository/{}/enableCi",
            repository_id)
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/plan/{projectKey}-{buildKey}/enable
    pub async fn enable_plan(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/plan/{}-{}/enable", project_key
            .as_ref(), build_key.as_ref())
        );
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/repository/{repositoryId}/enableProjectCreation
    pub async fn enable_project_creation(
        &self,
        repository_id: i64,
        request: RestEnableContainer,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/repository/{}/enableProjectCreation", repository_id)
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/plan/{projectKey}-{buildKey}/branch/enableSpecsForBranches
    pub async fn enable_specs_for_branches(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/plan/{}-{}/branch/enableSpecsForBranches", project_key
            .as_ref(), build_key.as_ref())
        );
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/encrypt
    pub async fn encrypt(
        &self,
        request: ManualEncryptionRequest,
    ) -> HttpResult<ManualEncryptionResponse> {
        let url = format!("{}{}", self.base_url, "/api/latest/encrypt");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/project/{deploymentProjectId}/specs
    pub async fn export_deployment_spec(
        &self,
        deployment_project_id: impl AsRef<str>,
        package: Option<impl AsRef<str>>,
        format: Option<impl AsRef<str>>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/deploy/project/{}/specs",
            deployment_project_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = package {
                query_params.push(("package", v.as_ref().to_string()));
            }
            if let Some(v) = format {
                query_params.push(("format", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/plan/{projectKey}-{buildKey}/specs
    pub async fn export_plan_spec(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        package: Option<impl AsRef<str>>,
        format: Option<impl AsRef<str>>,
    ) -> HttpResult<RestPlanSpec> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/plan/{}-{}/specs", project_key
            .as_ref(), build_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = package {
                query_params.push(("package", v.as_ref().to_string()));
            }
            if let Some(v) = format {
                query_params.push(("format", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/project/{projectKey}/specs
    pub async fn export_project_specs(
        &self,
        project_key: impl AsRef<str>,
        package: Option<impl AsRef<str>>,
        format: Option<impl AsRef<str>>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/project/{}/specs", project_key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = package {
                query_params.push(("package", v.as_ref().to_string()));
            }
            if let Some(v) = format {
                query_params.push(("format", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/security/trustedKey
    pub async fn find_all(&self) -> HttpResult<FindAllResponse> {
        let url = format!("{}{}", self.base_url, "/admin/latest/security/trustedKey");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/environment/{environmentId}/agent-assignment
    pub async fn find_assigned_agents_by_environment(
        &self,
        environment_id: impl AsRef<str>,
    ) -> HttpResult<FindAssignedAgentsByEnvironmentResponse> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/deploy/environment/{}/agent-assignment", environment_id
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/config/job/{jobKey}/agent-assignment
    pub async fn find_assigned_agents_by_job(
        &self,
        job_key: impl AsRef<str>,
    ) -> HttpResult<FindAssignedAgentsByJobResponse> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/config/job/{}/agent-assignment",
            job_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/admin/users/{name}/assigned-groups
    pub async fn find_assigned_groups(
        &self,
        name: impl AsRef<str>,
        filter: Option<impl AsRef<str>>,
        limit: Option<i64>,
        start: Option<i64>,
    ) -> HttpResult<FindAssignedGroupsResponse> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/admin/users/{}/assigned-groups",
            name.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = filter {
                query_params.push(("filter", v.as_ref().to_string()));
            }
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/admin/expiry/custom/plan
    pub async fn find_plans_with_custom_expiry_settings(
        &self,
        limit: Option<i64>,
        start: Option<i64>,
    ) -> HttpResult<FindPlansWithCustomExpirySettingsResponse> {
        let url = format!("{}{}", self.base_url, "/api/latest/admin/expiry/custom/plan");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/environment/{environmentId}/possible-agent-assignment
    pub async fn find_possible_agents_for_environment(
        &self,
        environment_id: impl AsRef<str>,
        max_result: Option<i64>,
        search_term: Option<impl AsRef<str>>,
        start_index: Option<i64>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/deploy/environment/{}/possible-agent-assignment",
            environment_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_result {
                query_params.push(("max-result", v.to_string()));
            }
            if let Some(v) = search_term {
                query_params.push(("searchTerm", v.as_ref().to_string()));
            }
            if let Some(v) = start_index {
                query_params.push(("start-index", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/config/job/{jobKey}/agent-assignment/possible-agent-assignment
    pub async fn find_possible_agents_for_job(
        &self,
        job_key: impl AsRef<str>,
        max_result: Option<i64>,
        search_term: Option<impl AsRef<str>>,
        start_index: Option<i64>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/config/job/{}/agent-assignment/possible-agent-assignment",
            job_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_result {
                query_params.push(("max-result", v.to_string()));
            }
            if let Some(v) = search_term {
                query_params.push(("searchTerm", v.as_ref().to_string()));
            }
            if let Some(v) = start_index {
                query_params.push(("start-index", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/repository
    pub async fn find_repository(
        &self,
        search_term: Option<impl AsRef<str>>,
    ) -> HttpResult<RestRepositoryList> {
        let url = format!("{}{}", self.base_url, "/api/latest/repository");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = search_term {
                query_params.push(("searchTerm", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/admin/users/{name}/unassigned-groups
    pub async fn find_unassigned_groups(
        &self,
        name: impl AsRef<str>,
        filter: Option<impl AsRef<str>>,
        limit: Option<i64>,
        start: Option<i64>,
    ) -> HttpResult<FindUnassignedGroupsResponse> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/admin/users/{}/unassigned-groups", name.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = filter {
                query_params.push(("filter", v.as_ref().to_string()));
            }
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/admin/users/{name}/unassigned-aliases
    pub async fn find_unassigned_user_repository_aliases(
        &self,
        name: impl AsRef<str>,
        filter: Option<impl AsRef<str>>,
        limit: Option<i64>,
        start: Option<i64>,
    ) -> HttpResult<FindUnassignedUserRepositoryAliasesResponse> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/admin/users/{}/unassigned-aliases", name.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = filter {
                query_params.push(("filter", v.as_ref().to_string()));
            }
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/repository/{repositoryId}/usage
    pub async fn find_usage(
        &self,
        repository_id: i64,
        max_plans: Option<i64>,
        max_environments: Option<i64>,
    ) -> HttpResult<RestRepositoryUsageModel> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/repository/{}/usage",
            repository_id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_plans {
                query_params.push(("max-plans", v.to_string()));
            }
            if let Some(v) = max_environments {
                query_params.push(("max-environments", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/admin/groups/{name}/more-members
    pub async fn find_users_in_group(
        &self,
        name: impl AsRef<str>,
        filter: Option<impl AsRef<str>>,
        limit: Option<i64>,
        start: Option<i64>,
    ) -> HttpResult<FindUsersInGroupResponse> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/admin/groups/{}/more-members",
            name.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = filter {
                query_params.push(("filter", v.as_ref().to_string()));
            }
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/admin/groups/{name}/more-non-members
    pub async fn find_users_not_in_group(
        &self,
        name: impl AsRef<str>,
        filter: Option<impl AsRef<str>>,
        limit: Option<i64>,
        start: Option<i64>,
    ) -> HttpResult<FindUsersNotInGroupResponse> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/admin/groups/{}/more-non-members", name.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = filter {
                query_params.push(("filter", v.as_ref().to_string()));
            }
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/darkFeatures/{key}
    pub async fn get(&self, key: impl AsRef<str>) -> HttpResult<RestDarkFeature> {
        let url = format!(
            "{}{}", self.base_url, format!("/admin/latest/darkFeatures/{}", key.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/quickFilter/active
    pub async fn get_active_filters(&self) -> HttpResult<GetActiveFiltersResponse> {
        let url = format!("{}{}", self.base_url, "/api/latest/quickFilter/active");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/agent/assignment
    pub async fn get_agent_assignments(
        &self,
        executor_type: Option<impl AsRef<str>>,
        executor_id: Option<i64>,
    ) -> HttpResult<GetAgentAssignmentsResponse> {
        let url = format!("{}{}", self.base_url, "/api/latest/agent/assignment");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = executor_type {
                query_params.push(("executorType", v.as_ref().to_string()));
            }
            if let Some(v) = executor_id {
                query_params.push(("executorId", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/agent/authentication
    pub async fn get_agent_authentications(
        &self,
        pending: Option<bool>,
    ) -> HttpResult<GetAgentAuthenticationsResponse> {
        let url = format!("{}{}", self.base_url, "/api/latest/agent/authentication");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = pending {
                query_params.push(("pending", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/config/agents/{agentId}
    pub async fn get_agent_by_id(
        &self,
        agent_id: impl AsRef<str>,
    ) -> HttpResult<RestAgent> {
        let url = format!(
            "{}{}", self.base_url, format!("/admin/latest/config/agents/{}", agent_id
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/agent/{agentId}/capability
    pub async fn get_agent_capabilities(
        &self,
        agent_id: i64,
        include_shared: Option<bool>,
    ) -> HttpResult<GetAgentCapabilitiesResponse> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/agent/{}/capability", agent_id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = include_shared {
                query_params.push(("includeShared", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/agent/{agentId}
    pub async fn get_agent_information(
        &self,
        agent_id: i64,
        max_result: Option<i64>,
        expand: Option<impl AsRef<str>>,
        include_shared: Option<bool>,
        start_index: Option<i64>,
    ) -> HttpResult<RestAgentInformation> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/agent/{}", agent_id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_result {
                query_params.push(("max-result", v.to_string()));
            }
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if let Some(v) = include_shared {
                query_params.push(("includeShared", v.to_string()));
            }
            if let Some(v) = start_index {
                query_params.push(("start-index", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/artifactHandlers/agentLocal
    pub async fn get_agent_local_artifact_handler(
        &self,
    ) -> HttpResult<RestArtifactHandler> {
        let url = format!(
            "{}{}", self.base_url, "/admin/latest/artifactHandlers/agentLocal"
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/environment/{environmentId}/requirement/summary
    pub async fn get_agent_matches_for_environment(
        &self,
        environment_id: impl AsRef<str>,
    ) -> HttpResult<RestAgentSummary> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/deploy/environment/{}/requirement/summary",
            environment_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/agent/{agentId}/status
    pub async fn get_agent_status(&self, agent_id: i64) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/agent/{}/status", agent_id)
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/config/agents
    pub async fn get_agents(&self) -> HttpResult<GetAgentsResponse> {
        let url = format!("{}{}", self.base_url, "/admin/latest/config/agents");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/agent
    pub async fn get_agents_1(
        &self,
        online: Option<bool>,
    ) -> HttpResult<GetAgents1Response> {
        let url = format!("{}{}", self.base_url, "/api/latest/agent");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = online {
                query_params.push(("online", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/darkFeatures
    pub async fn get_all(&self) -> HttpResult<GetAllResponse> {
        let url = format!("{}{}", self.base_url, "/admin/latest/darkFeatures");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/capability/groupedListing
    pub async fn get_all_capabilities_on_server(
        &self,
        max_result: Option<i64>,
        search_term: Option<impl AsRef<str>>,
        last_group: Option<impl AsRef<str>>,
        start_index: Option<i64>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, "/api/latest/capability/groupedListing"
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_result {
                query_params.push(("max-result", v.to_string()));
            }
            if let Some(v) = search_term {
                query_params.push(("searchTerm", v.as_ref().to_string()));
            }
            if let Some(v) = last_group {
                query_params.push(("lastGroup", v.as_ref().to_string()));
            }
            if let Some(v) = start_index {
                query_params.push(("start-index", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/dashboard
    pub async fn get_all_deployment_projects(
        &self,
    ) -> HttpResult<GetAllDeploymentProjectsResponse> {
        let url = format!("{}{}", self.base_url, "/api/latest/deploy/dashboard");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/project/all
    pub async fn get_all_deployment_projects_1(
        &self,
    ) -> HttpResult<GetAllDeploymentProjects1Response> {
        let url = format!("{}{}", self.base_url, "/api/latest/deploy/project/all");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/environment/{environmentId}/variables
    pub async fn get_all_environment_variables(
        &self,
        environment_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/deploy/environment/{}/variables",
            environment_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/plan
    pub async fn get_all_plan_list(
        &self,
        expand: Option<impl AsRef<str>>,
    ) -> HttpResult<RestPlans> {
        let url = format!("{}{}", self.base_url, "/api/latest/plan");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/
    pub async fn get_all_services(&self) -> HttpResult<RestResources> {
        let url = format!("{}{}", self.base_url, "/api/latest/");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/elasticConfiguration
    pub async fn get_all_1(&self) -> HttpResult<GetAll1Response> {
        let url = format!("{}{}", self.base_url, "/api/latest/elasticConfiguration");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/config/auditLog
    pub async fn get_audit_log_configuration(
        &self,
    ) -> HttpResult<RestAuditLogConfiguration> {
        let url = format!("{}{}", self.base_url, "/admin/latest/config/auditLog");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/deployment/{id}/available-groups
    pub async fn get_available_groups(
        &self,
        id: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
    ) -> HttpResult<GetAvailableGroupsResponse> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/deployment/{}/available-groups", id
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/environment/{id}/available-groups
    pub async fn get_available_groups_1(
        &self,
        id: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
    ) -> HttpResult<GetAvailableGroups1Response> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/environment/{}/available-groups", id
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/global/available-groups
    pub async fn get_available_groups_2(
        &self,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
        ignore: Option<impl AsRef<str>>,
    ) -> HttpResult<GetAvailableGroups2Response> {
        let url = format!(
            "{}{}", self.base_url, "/api/latest/permissions/global/available-groups"
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if let Some(v) = ignore {
                query_params.push(("ignore", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/plan/{key}/available-groups
    pub async fn get_available_groups_3(
        &self,
        key: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
    ) -> HttpResult<GetAvailableGroups3Response> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/plan/{}/available-groups", key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/project/{key}/available-groups
    pub async fn get_available_groups_4(
        &self,
        key: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
    ) -> HttpResult<GetAvailableGroups4Response> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/project/{}/available-groups", key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/projectplan/{key}/available-groups
    pub async fn get_available_groups_5(
        &self,
        key: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
    ) -> HttpResult<GetAvailableGroups5Response> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/projectplan/{}/available-groups", key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/repository/{id}/available-groups
    pub async fn get_available_groups_6(
        &self,
        id: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
    ) -> HttpResult<GetAvailableGroups6Response> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/repository/{}/available-groups", id
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/chart/reports
    pub async fn get_available_reports(
        &self,
        max_result: Option<i64>,
        expand: Option<impl AsRef<str>>,
        start_index: Option<i64>,
    ) -> HttpResult<RestReports> {
        let url = format!("{}{}", self.base_url, "/api/latest/chart/reports");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_result {
                query_params.push(("max-result", v.to_string()));
            }
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if let Some(v) = start_index {
                query_params.push(("start-index", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/deployment/{id}/available-users
    pub async fn get_available_users(
        &self,
        id: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
    ) -> HttpResult<GetAvailableUsersResponse> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/deployment/{}/available-users", id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/environment/{id}/available-users
    pub async fn get_available_users_1(
        &self,
        id: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
    ) -> HttpResult<GetAvailableUsers1Response> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/environment/{}/available-users", id
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/global/available-users
    pub async fn get_available_users_2(
        &self,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
        ignore: Option<impl AsRef<str>>,
    ) -> HttpResult<GetAvailableUsers2Response> {
        let url = format!(
            "{}{}", self.base_url, "/api/latest/permissions/global/available-users"
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if let Some(v) = ignore {
                query_params.push(("ignore", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/plan/{key}/available-users
    pub async fn get_available_users_3(
        &self,
        key: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
    ) -> HttpResult<GetAvailableUsers3Response> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/plan/{}/available-users", key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/project/{key}/available-users
    pub async fn get_available_users_4(
        &self,
        key: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
    ) -> HttpResult<GetAvailableUsers4Response> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/project/{}/available-users", key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/projectplan/{key}/available-users
    pub async fn get_available_users_5(
        &self,
        key: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
    ) -> HttpResult<GetAvailableUsers5Response> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/projectplan/{}/available-users", key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/repository/{id}/available-users
    pub async fn get_available_users_6(
        &self,
        id: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
    ) -> HttpResult<GetAvailableUsers6Response> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/repository/{}/available-users", id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/artifactHandlers/bambooRemote
    pub async fn get_bamboo_remote_artifact_handler(
        &self,
    ) -> HttpResult<SimpleRestArtifactHandler> {
        let url = format!(
            "{}{}", self.base_url, "/admin/latest/artifactHandlers/bambooRemote"
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/result/{projectKey}-{buildKey}/branch/{branchName}
    pub async fn get_branch_history(
        &self,
        build_key: impl AsRef<str>,
        branch_name: impl AsRef<str>,
        project_key: impl AsRef<str>,
        include_all_states: Option<bool>,
        continuable: Option<bool>,
        issue_key: Option<impl AsRef<str>>,
        max_results: Option<i64>,
        start_index: Option<i64>,
        label: Option<impl AsRef<str>>,
        buildstate: Option<impl AsRef<str>>,
        favourite: Option<impl AsRef<str>>,
        expand: Option<impl AsRef<str>>,
        life_cycle_state: Option<impl AsRef<str>>,
    ) -> HttpResult<RestResults> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/result/{}-{}/branch/{}",
            build_key.as_ref(), branch_name.as_ref(), project_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = include_all_states {
                query_params.push(("includeAllStates", v.to_string()));
            }
            if let Some(v) = continuable {
                query_params.push(("continuable", v.to_string()));
            }
            if let Some(v) = issue_key {
                query_params.push(("issueKey", v.as_ref().to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("max-results", v.to_string()));
            }
            if let Some(v) = start_index {
                query_params.push(("start-index", v.to_string()));
            }
            if let Some(v) = label {
                query_params.push(("label", v.as_ref().to_string()));
            }
            if let Some(v) = buildstate {
                query_params.push(("buildstate", v.as_ref().to_string()));
            }
            if let Some(v) = favourite {
                query_params.push(("favourite", v.as_ref().to_string()));
            }
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if let Some(v) = life_cycle_state {
                query_params.push(("lifeCycleState", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /responsibility/latest/brokenBuild/byUser/{name}
    pub async fn get_broken_builds_for_user(
        &self,
        name: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
        show_all_responsible: Option<impl AsRef<str>>,
    ) -> HttpResult<GetBrokenBuildsForUserResponse> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/responsibility/latest/brokenBuild/byUser/{}", name.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = show_all_responsible {
                query_params.push(("showAllResponsible", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/result/{projectKey}-{buildKey}-{buildNumber}
    pub async fn get_build(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        build_number: impl AsRef<str>,
        expand: Option<impl AsRef<str>>,
        build_positions: Option<impl AsRef<str>>,
    ) -> HttpResult<BuildResult> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/result/{}-{}-{}", project_key
            .as_ref(), build_key.as_ref(), build_number.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if let Some(v) = build_positions {
                query_params.push(("buildPositions", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/result/{projectKey}-{buildKey}/{buildNumber}
    pub async fn get_build_alias(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        build_number: impl AsRef<str>,
        expand: Option<impl AsRef<str>>,
    ) -> HttpResult<BuildResult> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/result/{}-{}/{}", project_key
            .as_ref(), build_key.as_ref(), build_number.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/result/{projectKey}-{buildKey}-{buildNumber}/comment
    pub async fn get_build_comments(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        build_number: impl AsRef<str>,
    ) -> HttpResult<RestComments> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/result/{}-{}-{}/comment",
            project_key.as_ref(), build_key.as_ref(), build_number.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/config/build/concurrency
    pub async fn get_build_concurrency(&self) -> HttpResult<RestBuildConcurrency> {
        let url = format!(
            "{}{}", self.base_url, "/admin/latest/config/build/concurrency"
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/result/{projectKey}-{buildKey}
    pub async fn get_build_history(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        include_all_states: Option<bool>,
        continuable: Option<bool>,
        expand: Option<impl AsRef<str>>,
        issue_key: Option<impl AsRef<str>>,
        max_results: Option<i64>,
        start_index: Option<i64>,
        label: Option<impl AsRef<str>>,
        buildstate: Option<impl AsRef<str>>,
        favourite: Option<impl AsRef<str>>,
        life_cycle_state: Option<impl AsRef<str>>,
    ) -> HttpResult<RestResults> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/result/{}-{}", project_key
            .as_ref(), build_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = include_all_states {
                query_params.push(("includeAllStates", v.to_string()));
            }
            if let Some(v) = continuable {
                query_params.push(("continuable", v.to_string()));
            }
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if let Some(v) = issue_key {
                query_params.push(("issueKey", v.as_ref().to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("max-results", v.to_string()));
            }
            if let Some(v) = start_index {
                query_params.push(("start-index", v.to_string()));
            }
            if let Some(v) = label {
                query_params.push(("label", v.as_ref().to_string()));
            }
            if let Some(v) = buildstate {
                query_params.push(("buildstate", v.as_ref().to_string()));
            }
            if let Some(v) = favourite {
                query_params.push(("favourite", v.as_ref().to_string()));
            }
            if let Some(v) = life_cycle_state {
                query_params.push(("lifeCycleState", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/result/{projectKey}-{buildKey}-{buildNumber}/label
    pub async fn get_build_labels(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        build_number: impl AsRef<str>,
    ) -> HttpResult<RestBuildLabels> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/result/{}-{}-{}/label",
            project_key.as_ref(), build_key.as_ref(), build_number.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/config/build/monitoring
    pub async fn get_build_monitoring(&self) -> HttpResult<RestBuildMonitoring> {
        let url = format!(
            "{}{}", self.base_url, "/admin/latest/config/build/monitoring"
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/queue/deployment
    pub async fn get_build_queue(
        &self,
        expand: Option<impl AsRef<str>>,
    ) -> HttpResult<RestQueuedDeployments> {
        let url = format!("{}{}", self.base_url, "/api/latest/queue/deployment");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/queue
    pub async fn get_build_queue_1(
        &self,
        max_result: Option<i64>,
        start_index: Option<i64>,
    ) -> HttpResult<RestQueuedBuilds> {
        let url = format!("{}{}", self.base_url, "/api/latest/queue");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_result {
                query_params.push(("max-result", v.to_string()));
            }
            if let Some(v) = start_index {
                query_params.push(("start-index", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/result/byChangeset/{csid}
    pub async fn get_build_results_for_changeset(
        &self,
        csid: impl AsRef<str>,
    ) -> HttpResult<RestResults> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/result/byChangeset/{}", csid
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/result/byCheckoutChangeset/{csid}
    pub async fn get_build_results_for_checkout_changeset(
        &self,
        csid: impl AsRef<str>,
    ) -> HttpResult<RestResults> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/result/byCheckoutChangeset/{}",
            csid.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/ephemeral/templateConfiguration/{configurationId}/capability
    pub async fn get_capabilities(
        &self,
        configuration_id: i64,
        limit: Option<i64>,
        start: Option<i64>,
    ) -> HttpResult<GetCapabilitiesResponse> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/ephemeral/templateConfiguration/{}/capability",
            configuration_id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/clone/{projectKey}-{buildKey}:{toProjectKey}-{toBuildKey}
    pub async fn get_clone(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        to_build_key: impl AsRef<str>,
        to_project_key: impl AsRef<str>,
    ) -> HttpResult<RestPlan> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/clone/{}-{}:{}-{}", project_key
            .as_ref(), build_key.as_ref(), to_build_key.as_ref(), to_project_key
            .as_ref())
        );
        let mut req = self.http_client.put(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/ephemeral/config
    pub async fn get_configuration(
        &self,
    ) -> HttpResult<EphemeralAgentsConfigurationDTO> {
        let url = format!("{}{}", self.base_url, "/admin/latest/ephemeral/config");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/expiry/configuration
    pub async fn get_configuration_1(
        &self,
    ) -> HttpResult<RestCombinedExpiryConfiguration> {
        let url = format!("{}{}", self.base_url, "/admin/latest/expiry/configuration");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/elasticConfiguration/{configurationId}
    pub async fn get_configuration_2(
        &self,
        configuration_id: i64,
    ) -> HttpResult<RestElasticImageConfig> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/elasticConfiguration/{}",
            configuration_id)
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/currentUser
    pub async fn get_current_user(&self) -> HttpResult<UserBean> {
        let url = format!("{}{}", self.base_url, "/api/latest/currentUser");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/projectVersioning/{deploymentProjectId}/namingPreview
    pub async fn get_deployment_naming_preview(
        &self,
        deployment_project_id: impl AsRef<str>,
        next_version_name: impl AsRef<str>,
        incrementable_variables: Option<impl AsRef<str>>,
        increment_numbers: Option<impl AsRef<str>>,
    ) -> HttpResult<RestNamingPreview> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/deploy/projectVersioning/{}/namingPreview",
            deployment_project_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            query_params
                .push(("nextVersionName", next_version_name.as_ref().to_string()));
            if let Some(v) = incrementable_variables {
                query_params.push(("incrementableVariables", v.as_ref().to_string()));
            }
            if let Some(v) = increment_numbers {
                query_params.push(("incrementNumbers", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/dashboard/{projectId}
    pub async fn get_deployment_project(
        &self,
        project_id: impl AsRef<str>,
    ) -> HttpResult<GetDeploymentProjectResponse> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/deploy/dashboard/{}", project_id
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/projectVersioning/{deploymentProjectId}/variables
    pub async fn get_deployment_project_variables(
        &self,
        deployment_project_id: impl AsRef<str>,
    ) -> HttpResult<VersionVariables> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/deploy/projectVersioning/{}/variables",
            deployment_project_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/project/{deploymentProjectId}/versions
    pub async fn get_deployment_project_versions(
        &self,
        deployment_project_id: impl AsRef<str>,
        branch_key: Option<impl AsRef<str>>,
    ) -> HttpResult<RestDeploymentVersionList> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/deploy/project/{}/versions",
            deployment_project_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = branch_key {
                query_params.push(("branchKey", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/project/{deploymentProjectId}
    pub async fn get_deployment_project_1(
        &self,
        deployment_project_id: impl AsRef<str>,
    ) -> HttpResult<RestDeploymentProject> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/deploy/project/{}",
            deployment_project_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/dashboard/paginate
    pub async fn get_deployment_projects(
        &self,
        filter: Option<impl AsRef<str>>,
        limit: Option<i64>,
        start: Option<i64>,
    ) -> HttpResult<GetDeploymentProjectsResponse> {
        let url = format!(
            "{}{}", self.base_url, "/api/latest/deploy/dashboard/paginate"
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = filter {
                query_params.push(("filter", v.as_ref().to_string()));
            }
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/project/forPlan
    pub async fn get_deployment_projects_for_plan(
        &self,
        plan_key: impl AsRef<str>,
    ) -> HttpResult<GetDeploymentProjectsForPlanResponse> {
        let url = format!("{}{}", self.base_url, "/api/latest/deploy/project/forPlan");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            query_params.push(("planKey", plan_key.as_ref().to_string()));
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/result/{deploymentResultId}
    pub async fn get_deployment_result(
        &self,
        deployment_result_id: impl AsRef<str>,
        include_logs: Option<impl AsRef<str>>,
    ) -> HttpResult<RestDeploymentResult> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/deploy/result/{}",
            deployment_result_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = include_logs {
                query_params.push(("includeLogs", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/environment/{environmentId}/results
    pub async fn get_deployment_result_list(
        &self,
        environment_id: impl AsRef<str>,
        expand: Option<impl AsRef<str>>,
    ) -> HttpResult<RestDeploymentResultList> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/deploy/environment/{}/results",
            environment_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/environment/{environmentId}/requirement/detailedSummary
    pub async fn get_detailed_agent_matches_for_environment(
        &self,
        environment_id: impl AsRef<str>,
    ) -> HttpResult<RestAgentSummary> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/deploy/environment/{}/requirement/detailedSummary",
            environment_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/job/{jobKey}/docker
    pub async fn get_docker_pipeline_configuration(
        &self,
        job_key: impl AsRef<str>,
    ) -> HttpResult<RestDockerPipelineConfiguration> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/job/{}/docker", job_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/environment/{environmentId}/docker
    pub async fn get_docker_pipelines_configuration(
        &self,
        environment_id: impl AsRef<str>,
    ) -> HttpResult<RestDockerPipelineConfiguration> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/deploy/environment/{}/docker",
            environment_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/environment/{environmentId}
    pub async fn get_environment(
        &self,
        environment_id: impl AsRef<str>,
    ) -> HttpResult<RestEnvironmentWithConfigCounts> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/deploy/environment/{}",
            environment_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/deploy/dashboard/status
    pub async fn get_environment_statutes(
        &self,
        request: EnvironmentIds,
    ) -> HttpResult<GetEnvironmentStatutesResponse> {
        let url = format!("{}{}", self.base_url, "/api/latest/deploy/dashboard/status");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/environment/{environmentId}/variable/{variableName}
    pub async fn get_environment_variable(
        &self,
        variable_name: impl AsRef<str>,
        environment_id: impl AsRef<str>,
    ) -> HttpResult<RestVariable> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/deploy/environment/{}/variable/{}", variable_name
            .as_ref(), environment_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/environment/agent/{agentId}
    pub async fn get_environments_executable_by_agent(
        &self,
        agent_id: impl AsRef<str>,
        filter: Option<impl AsRef<str>>,
        limit: Option<i64>,
        start: Option<i64>,
    ) -> HttpResult<GetEnvironmentsExecutableByAgentResponse> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/deploy/environment/agent/{}",
            agent_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = filter {
                query_params.push(("filter", v.as_ref().to_string()));
            }
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/environment/elasticImageConfiguration/{configurationId}
    pub async fn get_environments_executable_by_elastic_configuration(
        &self,
        configuration_id: impl AsRef<str>,
        filter: Option<impl AsRef<str>>,
        limit: Option<i64>,
        start: Option<i64>,
    ) -> HttpResult<GetEnvironmentsExecutableByElasticConfigurationResponse> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/deploy/environment/elasticImageConfiguration/{}",
            configuration_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = filter {
                query_params.push(("filter", v.as_ref().to_string()));
            }
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/environment/ephemeral/{templateId}
    pub async fn get_environments_executable_by_ephemeral_agent_template(
        &self,
        template_id: impl AsRef<str>,
        filter: Option<impl AsRef<str>>,
        limit: Option<i64>,
        start: Option<i64>,
    ) -> HttpResult<GetEnvironmentsExecutableByEphemeralAgentTemplateResponse> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/deploy/environment/ephemeral/{}",
            template_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = filter {
                query_params.push(("filter", v.as_ref().to_string()));
            }
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/ephemeral/pod/{pod}/logs
    pub async fn get_ephemeral_agent_pod_logs(
        &self,
        pod: impl AsRef<str>,
        container_name: Option<impl AsRef<str>>,
        limit: Option<impl AsRef<str>>,
        after_timestamp: Option<impl AsRef<str>>,
    ) -> HttpResult<GetEphemeralAgentPodLogsResponse> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/ephemeral/pod/{}/logs", pod
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = container_name {
                query_params.push(("containerName", v.as_ref().to_string()));
            }
            if let Some(v) = limit {
                query_params.push(("limit", v.as_ref().to_string()));
            }
            if let Some(v) = after_timestamp {
                query_params.push(("afterTimestamp", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/ephemeral/pod/{pod}/logs/raw
    pub async fn get_ephemeral_agent_pod_raw_logs(
        &self,
        pod: impl AsRef<str>,
        container_name: Option<impl AsRef<str>>,
    ) -> HttpResult<GetEphemeralAgentPodRawLogsResponse> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/ephemeral/pod/{}/logs/raw", pod
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = container_name {
                query_params.push(("containerName", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/websudo-session
    pub async fn get_expiry(&self) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/latest/websudo-session");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/plan/favicon/{planKey}
    pub async fn get_favicon_for_plan(
        &self,
        plan_key: impl AsRef<str>,
    ) -> HttpResult<RestFavicon> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/plan/favicon/{}", plan_key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/darkFeatures/{key}/user/{userName}
    pub async fn get_for_user(
        &self,
        user_name: impl AsRef<str>,
        key: impl AsRef<str>,
    ) -> HttpResult<RestUserDarkFeature> {
        let url = format!(
            "{}{}", self.base_url, format!("/admin/latest/darkFeatures/{}/user/{}",
            user_name.as_ref(), key.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/config/general
    pub async fn get_general_configuration(
        &self,
    ) -> HttpResult<RestGeneralConfiguration> {
        let url = format!("{}{}", self.base_url, "/admin/latest/config/general");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/globalVariables/{variableId}
    pub async fn get_global_variable(
        &self,
        variable_id: impl AsRef<str>,
    ) -> HttpResult<RestGlobalVariable> {
        let url = format!(
            "{}{}", self.base_url, format!("/admin/latest/globalVariables/{}",
            variable_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/globalVariables
    pub async fn get_global_variables(&self) -> HttpResult<RestGlobalVariables> {
        let url = format!("{}{}", self.base_url, "/admin/latest/globalVariables");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/security/groups
    pub async fn get_group(
        &self,
        name: impl AsRef<str>,
    ) -> HttpResult<GetGroupResponse> {
        let url = format!("{}{}", self.base_url, "/admin/latest/security/groups");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            query_params.push(("name", name.as_ref().to_string()));
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/admin/groups
    pub async fn get_groups(
        &self,
        filter: Option<impl AsRef<str>>,
        limit: Option<i64>,
        start: Option<i64>,
    ) -> HttpResult<GetGroupsResponse> {
        let url = format!("{}{}", self.base_url, "/api/latest/admin/groups");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = filter {
                query_params.push(("filter", v.as_ref().to_string()));
            }
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/config/imServer
    pub async fn get_im_server_config(&self) -> HttpResult<RestIMServerConfiguration> {
        let url = format!("{}{}", self.base_url, "/admin/latest/config/imServer");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/chart
    pub async fn get_image_url(
        &self,
        build_keys: Option<impl AsRef<str>>,
        report_key: Option<impl AsRef<str>>,
    ) -> HttpResult<RestChart> {
        let url = format!("{}{}", self.base_url, "/api/latest/chart");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = build_keys {
                query_params.push(("buildKeys", v.as_ref().to_string()));
            }
            if let Some(v) = report_key {
                query_params.push(("reportKey", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/info
    pub async fn get_info(&self) -> HttpResult<RestInfo> {
        let url = format!("{}{}", self.base_url, "/api/latest/info");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/elasticInstances/instance/{instanceId}/logs
    pub async fn get_instance_log(
        &self,
        instance_id: impl AsRef<str>,
    ) -> HttpResult<RestElasticInstanceLog> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/elasticInstances/instance/{}/logs", instance_id
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/plan/{projectKey}-{buildKey}/issue/{issueKey}
    pub async fn get_issue_details(
        &self,
        project_key: impl AsRef<str>,
        issue_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
    ) -> HttpResult<RestJiraIssue> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/plan/{}-{}/issue/{}", project_key
            .as_ref(), issue_key.as_ref(), build_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/issue-status/{issueKey}
    pub async fn get_jira_issue_status_for_project(
        &self,
        issue_key: impl AsRef<str>,
    ) -> HttpResult<RestJiraIssueRelatedDeploymentProjects> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/deploy/issue-status/{}",
            issue_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/issue-status/{issueKey}/{deploymentProjectId}
    pub async fn get_jira_issue_status_for_project_1(
        &self,
        issue_key: impl AsRef<str>,
        deployment_project_id: impl AsRef<str>,
    ) -> HttpResult<RestDeploymentProjectStatusForJiraIssue> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/deploy/issue-status/{}/{}",
            issue_key.as_ref(), deployment_project_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/scheduler/jobs
    pub async fn get_jobs(&self) -> HttpResult<GetJobsResponse> {
        let url = format!("{}{}", self.base_url, "/admin/latest/scheduler/jobs");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/result
    pub async fn get_latest_build_results(
        &self,
        include_all_states: Option<bool>,
        continuable: Option<bool>,
        expand: Option<impl AsRef<str>>,
        issue_key: Option<impl AsRef<str>>,
        max_results: Option<i64>,
        start_index: Option<i64>,
        label: Option<impl AsRef<str>>,
        buildstate: Option<impl AsRef<str>>,
        favourite: Option<impl AsRef<str>>,
        life_cycle_state: Option<impl AsRef<str>>,
    ) -> HttpResult<RestResults> {
        let url = format!("{}{}", self.base_url, "/api/latest/result");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = include_all_states {
                query_params.push(("includeAllStates", v.to_string()));
            }
            if let Some(v) = continuable {
                query_params.push(("continuable", v.to_string()));
            }
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if let Some(v) = issue_key {
                query_params.push(("issueKey", v.as_ref().to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("max-results", v.to_string()));
            }
            if let Some(v) = start_index {
                query_params.push(("start-index", v.to_string()));
            }
            if let Some(v) = label {
                query_params.push(("label", v.as_ref().to_string()));
            }
            if let Some(v) = buildstate {
                query_params.push(("buildstate", v.as_ref().to_string()));
            }
            if let Some(v) = favourite {
                query_params.push(("favourite", v.as_ref().to_string()));
            }
            if let Some(v) = life_cycle_state {
                query_params.push(("lifeCycleState", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/result/{projectKey}
    pub async fn get_latest_build_results_for_project(
        &self,
        project_key: impl AsRef<str>,
        include_all_states: Option<bool>,
        continuable: Option<bool>,
        expand: Option<impl AsRef<str>>,
        issue_key: Option<impl AsRef<str>>,
        max_results: Option<i64>,
        start_index: Option<i64>,
        label: Option<impl AsRef<str>>,
        buildstate: Option<impl AsRef<str>>,
        favourite: Option<impl AsRef<str>>,
        life_cycle_state: Option<impl AsRef<str>>,
    ) -> HttpResult<RestResults> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/result/{}", project_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = include_all_states {
                query_params.push(("includeAllStates", v.to_string()));
            }
            if let Some(v) = continuable {
                query_params.push(("continuable", v.to_string()));
            }
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if let Some(v) = issue_key {
                query_params.push(("issueKey", v.as_ref().to_string()));
            }
            if let Some(v) = max_results {
                query_params.push(("max-results", v.to_string()));
            }
            if let Some(v) = start_index {
                query_params.push(("start-index", v.to_string()));
            }
            if let Some(v) = label {
                query_params.push(("label", v.as_ref().to_string()));
            }
            if let Some(v) = buildstate {
                query_params.push(("buildstate", v.as_ref().to_string()));
            }
            if let Some(v) = favourite {
                query_params.push(("favourite", v.as_ref().to_string()));
            }
            if let Some(v) = life_cycle_state {
                query_params.push(("lifeCycleState", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/version/{deploymentVersionId}/status
    pub async fn get_latest_version_statuses(
        &self,
        deployment_version_id: impl AsRef<str>,
    ) -> HttpResult<RestDeploymentVersionStatuses> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/deploy/version/{}/status",
            deployment_version_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/config/mailServer
    pub async fn get_mail_configuration(&self) -> HttpResult<RestMailConfiguration> {
        let url = format!("{}{}", self.base_url, "/admin/latest/config/mailServer");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /responsibility/latest/brokenBuild/myBrokenBuilds
    pub async fn get_my_broken_builds(
        &self,
        limit: Option<i64>,
        start: Option<i64>,
    ) -> HttpResult<GetMyBrokenBuildsResponse> {
        let url = format!(
            "{}{}", self.base_url, "/responsibility/latest/brokenBuild/myBrokenBuilds"
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/buildNumber/{projectKey}-{buildKey}
    pub async fn get_next_build_number(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
    ) -> HttpResult<NextBuildNumber> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/buildNumber/{}-{}", project_key
            .as_ref(), build_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/projectVersioning/{deploymentProjectId}/nextVersion
    pub async fn get_next_deployment_versions(
        &self,
        deployment_project_id: impl AsRef<str>,
        result_key: Option<impl AsRef<str>>,
    ) -> HttpResult<RestNamingPreview> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/deploy/projectVersioning/{}/nextVersion",
            deployment_project_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = result_key {
                query_params.push(("resultKey", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/server/nodes
    pub async fn get_nodes_status(&self) -> HttpResult<RestServerNodesInfo> {
        let url = format!("{}{}", self.base_url, "/api/latest/server/nodes");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/dashboard/paginate/{projectId}
    pub async fn get_paginate_deployment_project(
        &self,
        project_id: impl AsRef<str>,
        filter: Option<impl AsRef<str>>,
        limit: Option<i64>,
        start: Option<impl AsRef<str>>,
    ) -> HttpResult<GetPaginateDeploymentProjectResponse> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/deploy/dashboard/paginate/{}",
            project_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = filter {
                query_params.push(("filter", v.as_ref().to_string()));
            }
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/project/{projectKey}/repositories
    pub async fn get_paginated_project_repositories(
        &self,
        project_key: impl AsRef<str>,
        filter: Option<impl AsRef<str>>,
        limit: Option<i64>,
        start: Option<i64>,
    ) -> HttpResult<GetPaginatedProjectRepositoriesResponse> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/project/{}/repositories",
            project_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = filter {
                query_params.push(("filter", v.as_ref().to_string()));
            }
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/project/{projectKey}/sharedCredentials
    pub async fn get_paginated_project_shared_credentials(
        &self,
        project_key: impl AsRef<str>,
        filter: Option<impl AsRef<str>>,
        limit: Option<i64>,
        start: Option<i64>,
    ) -> HttpResult<GetPaginatedProjectSharedCredentialsResponse> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/project/{}/sharedCredentials",
            project_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = filter {
                query_params.push(("filter", v.as_ref().to_string()));
            }
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/plan/{projectKey}-{buildKey}
    pub async fn get_plan(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        expand: Option<impl AsRef<str>>,
    ) -> HttpResult<RestPlan> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/plan/{}-{}", project_key
            .as_ref(), build_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/plan/{projectKey}/{buildKey}
    pub async fn get_plan_alias(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        expand: Option<impl AsRef<str>>,
    ) -> HttpResult<RestPlan> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/plan/{}/{}", project_key
            .as_ref(), build_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/plan/{projectKey}-{buildKey}/artifact
    pub async fn get_plan_artifact_definition(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        max_result: Option<i64>,
        start_index: Option<i64>,
    ) -> HttpResult<RestArtifactDefinitions> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/plan/{}-{}/artifact", project_key
            .as_ref(), build_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_result {
                query_params.push(("max-result", v.to_string()));
            }
            if let Some(v) = start_index {
                query_params.push(("start-index", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/plan/{projectKey}-{buildKey}/branch/{branchName}
    pub async fn get_plan_branch(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        branch_name: impl AsRef<str>,
    ) -> HttpResult<RestPlanBranch> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/plan/{}-{}/branch/{}",
            project_key.as_ref(), build_key.as_ref(), branch_name.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/plan/{projectKey}-{buildKey}/branch
    pub async fn get_plan_branches(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        enabled_only: Option<impl AsRef<str>>,
        clover_enabled: Option<impl AsRef<str>>,
        my: Option<impl AsRef<str>>,
        favourite: Option<impl AsRef<str>>,
    ) -> HttpResult<RestBranches> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/plan/{}-{}/branch", project_key
            .as_ref(), build_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = enabled_only {
                query_params.push(("enabledOnly", v.as_ref().to_string()));
            }
            if let Some(v) = clover_enabled {
                query_params.push(("cloverEnabled", v.as_ref().to_string()));
            }
            if let Some(v) = my {
                query_params.push(("my", v.as_ref().to_string()));
            }
            if let Some(v) = favourite {
                query_params.push(("favourite", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/planDirectoryInfo/{planKey}
    pub async fn get_plan_directory(
        &self,
        plan_key: impl AsRef<str>,
    ) -> HttpResult<DirectoryInformationResponse> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/planDirectoryInfo/{}", plan_key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/plan/{projectKey}-{buildKey}/label
    pub async fn get_plan_labels(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
    ) -> HttpResult<RestBuildLabels> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/plan/{}-{}/label", project_key
            .as_ref(), build_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/chart/planSummary
    pub async fn get_plan_summary(
        &self,
        build_keys: Option<impl AsRef<str>>,
    ) -> HttpResult<RestChart> {
        let url = format!("{}{}", self.base_url, "/api/latest/chart/planSummary");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = build_keys {
                query_params.push(("buildKeys", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/plan/{projectKey}-{buildKey}/variables/{variableName}
    pub async fn get_plan_variable(
        &self,
        project_key: impl AsRef<str>,
        variable_name: impl AsRef<str>,
        build_key: impl AsRef<str>,
    ) -> HttpResult<RestVariable> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/plan/{}-{}/variables/{}",
            project_key.as_ref(), variable_name.as_ref(), build_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/plan/{projectKey}-{buildKey}/variables
    pub async fn get_plan_variables(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/plan/{}-{}/variables",
            project_key.as_ref(), build_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/preview/possibleResults
    pub async fn get_possible_results(
        &self,
        deployment_project_id: Option<impl AsRef<str>>,
        plan_key: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, "/api/latest/deploy/preview/possibleResults"
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = deployment_project_id {
                query_params.push(("deploymentProjectId", v.as_ref().to_string()));
            }
            query_params.push(("planKey", plan_key.as_ref().to_string()));
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/project/{projectKey}
    pub async fn get_project(
        &self,
        project_key: impl AsRef<str>,
        expand: Option<impl AsRef<str>>,
        show_empty: Option<bool>,
    ) -> HttpResult<RestProject> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/project/{}", project_key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if let Some(v) = show_empty {
                query_params.push(("showEmpty", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/project/{projectKey}/variable/{variableName}
    pub async fn get_project_variable(
        &self,
        project_key: impl AsRef<str>,
        variable_name: impl AsRef<str>,
    ) -> HttpResult<RestVariable> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/project/{}/variable/{}",
            project_key.as_ref(), variable_name.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/project/{projectKey}/variables
    pub async fn get_project_variables(
        &self,
        project_key: impl AsRef<str>,
    ) -> HttpResult<GetProjectVariablesResponse> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/project/{}/variables",
            project_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/project
    pub async fn get_projects(
        &self,
        expand: Option<impl AsRef<str>>,
        show_empty: Option<bool>,
    ) -> HttpResult<RestProjects> {
        let url = format!("{}{}", self.base_url, "/api/latest/project");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = expand {
                query_params.push(("expand", v.as_ref().to_string()));
            }
            if let Some(v) = show_empty {
                query_params.push(("showEmpty", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/config/quarantine
    pub async fn get_quarantine_settings(&self) -> HttpResult<RestQuarantineConfig> {
        let url = format!("{}{}", self.base_url, "/admin/latest/config/quarantine");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/quickFilter/{id}
    pub async fn get_quick_filter(&self, id: i64) -> HttpResult<RestQuickFilter> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/quickFilter/{}", id)
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/quickFilter
    pub async fn get_quick_filters(
        &self,
        configured: Option<bool>,
    ) -> HttpResult<GetQuickFiltersResponse> {
        let url = format!("{}{}", self.base_url, "/api/latest/quickFilter");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = configured {
                query_params.push(("configured", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/reindex
    pub async fn get_reindex_info(&self) -> HttpResult<ReindexBean> {
        let url = format!("{}{}", self.base_url, "/api/latest/reindex");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/config/remoteAgentSupport
    pub async fn get_remote_agent_configuration(
        &self,
    ) -> HttpResult<RestRemoteAgentConfiguration> {
        let url = format!(
            "{}{}", self.base_url, "/admin/latest/config/remoteAgentSupport"
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/agent/remote
    pub async fn get_remote_agents(
        &self,
        online: Option<bool>,
    ) -> HttpResult<GetRemoteAgentsResponse> {
        let url = format!("{}{}", self.base_url, "/api/latest/agent/remote");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = online {
                query_params.push(("online", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/environment/{environmentId}/requirement/{requirementId}
    pub async fn get_requirement_for_environment(
        &self,
        environment_id: impl AsRef<str>,
        requirement_id: impl AsRef<str>,
    ) -> HttpResult<RestRequirement> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/deploy/environment/{}/requirement/{}", environment_id
            .as_ref(), requirement_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/environment/{environmentId}/requirement
    pub async fn get_requirements_for_environment(
        &self,
        environment_id: impl AsRef<str>,
    ) -> HttpResult<GetRequirementsForEnvironmentResponse> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/deploy/environment/{}/requirement", environment_id
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /responsibility/latest/brokenBuild/{planResultKeyOrPlanKey}
    pub async fn get_responsible_for_plan_result(
        &self,
        plan_result_key_or_plan_key: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/responsibility/latest/brokenBuild/{}",
            plan_result_key_or_plan_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/elastic/config
    pub async fn get_rest_elastic_configuration(
        &self,
    ) -> HttpResult<RestElasticConfiguration> {
        let url = format!("{}{}", self.base_url, "/admin/latest/elastic/config");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/result/status/{projectKey}-{buildKey}-{buildNumber}
    pub async fn get_result_status(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        build_number: impl AsRef<str>,
    ) -> HttpResult<RestResultStatus> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/result/status/{}-{}-{}",
            project_key.as_ref(), build_key.as_ref(), build_number.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/repository/{repositoryId}/rssrepository
    pub async fn get_rss_repositories_allowed_to_access_repository(
        &self,
        repository_id: i64,
    ) -> HttpResult<GetRssRepositoriesAllowedToAccessRepositoryResponse> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/repository/{}/rssrepository",
            repository_id)
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/artifactHandlers/s3
    pub async fn get_s3_artifact_handler(&self) -> HttpResult<RestArtifactHandler> {
        let url = format!("{}{}", self.base_url, "/admin/latest/artifactHandlers/s3");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/security/settings
    pub async fn get_security_settings(&self) -> HttpResult<RestSecuritySettings> {
        let url = format!("{}{}", self.base_url, "/admin/latest/security/settings");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/artifactHandlers/sftp
    pub async fn get_sftp_artifact_handler(
        &self,
    ) -> HttpResult<SimpleRestArtifactHandler> {
        let url = format!("{}{}", self.base_url, "/admin/latest/artifactHandlers/sftp");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/repository/{repositoryId}/scan/status
    pub async fn get_specs_detection_status(
        &self,
        repository_id: i64,
        max_result: Option<i64>,
        branch: Option<impl AsRef<str>>,
    ) -> HttpResult<RestVcsLocationSpecsStatus> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/repository/{}/scan/status",
            repository_id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_result {
                query_params.push(("max-result", v.to_string()));
            }
            if let Some(v) = branch {
                query_params.push(("branch", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/expiry/status
    pub async fn get_status(&self) -> HttpResult<RestCombinedExpiryStatus> {
        let url = format!("{}{}", self.base_url, "/admin/latest/expiry/status");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/server
    pub async fn get_status_1(&self) -> HttpResult<RestServerStatusInfo> {
        let url = format!("{}{}", self.base_url, "/api/latest/server");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/status
    pub async fn get_status_2(&self) -> HttpResult<RestAnonymousServerStatusInfo> {
        let url = format!("{}{}", self.base_url, "/api/latest/status");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /admin/latest/storageSettings
    pub async fn get_storage_configuration(
        &self,
    ) -> HttpResult<RestStorageConfiguration> {
        let url = format!("{}{}", self.base_url, "/admin/latest/storageSettings");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/ephemeral/templateConfiguration/{configurationId}
    pub async fn get_template_configuration(
        &self,
        configuration_id: i64,
    ) -> HttpResult<RestEphemeralAgentTemplate> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/ephemeral/templateConfiguration/{}", configuration_id)
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/ephemeral/templateConfiguration
    pub async fn get_template_configurations_page(
        &self,
        filter: Option<impl AsRef<str>>,
        limit: Option<i64>,
        start: Option<i64>,
    ) -> HttpResult<GetTemplateConfigurationsPageResponse> {
        let url = format!(
            "{}{}", self.base_url, "/api/latest/ephemeral/templateConfiguration"
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = filter {
                query_params.push(("filter", v.as_ref().to_string()));
            }
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/admin/users/{name}/access-token
    pub async fn get_user_access_tokens(
        &self,
        name: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
    ) -> HttpResult<GetUserAccessTokensResponse> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/admin/users/{}/access-token",
            name.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/admin/users/{name}/alias
    pub async fn get_user_repository_aliases(
        &self,
        name: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
    ) -> HttpResult<GetUserRepositoryAliasesResponse> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/admin/users/{}/alias", name
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/access-token
    pub async fn get_user_tokens(
        &self,
        limit: Option<i64>,
        start: Option<i64>,
    ) -> HttpResult<GetUserTokensResponse> {
        let url = format!("{}{}", self.base_url, "/api/latest/access-token");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/admin/users
    pub async fn get_users(
        &self,
        filter: Option<impl AsRef<str>>,
        limit: Option<i64>,
        start: Option<i64>,
    ) -> HttpResult<GetUsersResponse> {
        let url = format!("{}{}", self.base_url, "/api/latest/admin/users");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = filter {
                query_params.push(("filter", v.as_ref().to_string()));
            }
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/projectVersioning/{deploymentProjectId}/parseVariables
    pub async fn get_variables_from_name(
        &self,
        deployment_project_id: impl AsRef<str>,
        next_version_name: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/deploy/projectVersioning/{}/parseVariables",
            deployment_project_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            query_params
                .push(("nextVersionName", next_version_name.as_ref().to_string()));
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/plan/{projectKey}-{buildKey}/vcsBranches
    pub async fn get_vcs_branches(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        max_result: Option<i64>,
        search_term: Option<impl AsRef<str>>,
        start_index: Option<i64>,
    ) -> HttpResult<RestVcsBranches> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/plan/{}-{}/vcsBranches",
            project_key.as_ref(), build_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_result {
                query_params.push(("max-result", v.to_string()));
            }
            if let Some(v) = search_term {
                query_params.push(("searchTerm", v.as_ref().to_string()));
            }
            if let Some(v) = start_index {
                query_params.push(("start-index", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/version/{deploymentVersionId}/build-result
    pub async fn get_version_and_plan_result(
        &self,
        deployment_version_id: impl AsRef<str>,
    ) -> HttpResult<RestDeploymentVersionAndPlanResult> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/deploy/version/{}/build-result",
            deployment_version_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/preview/versionName
    pub async fn get_version_name(
        &self,
        result_key: Option<impl AsRef<str>>,
        deployment_project_id: i64,
    ) -> HttpResult<VersionName> {
        let url = format!(
            "{}{}", self.base_url, "/api/latest/deploy/preview/versionName"
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = result_key {
                query_params.push(("resultKey", v.as_ref().to_string()));
            }
            query_params
                .push(("deploymentProjectId", deployment_project_id.to_string()));
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/preview/version
    pub async fn get_version_preview(
        &self,
        previous_version_id: Option<impl AsRef<str>>,
        version_id: Option<impl AsRef<str>>,
        deployment_project_id: Option<impl AsRef<str>>,
        version_name: Option<impl AsRef<str>>,
    ) -> HttpResult<VersionPreview> {
        let url = format!("{}{}", self.base_url, "/api/latest/deploy/preview/version");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = previous_version_id {
                query_params.push(("previousVersionId", v.as_ref().to_string()));
            }
            if let Some(v) = version_id {
                query_params.push(("versionId", v.as_ref().to_string()));
            }
            if let Some(v) = deployment_project_id {
                query_params.push(("deploymentProjectId", v.as_ref().to_string()));
            }
            if let Some(v) = version_name {
                query_params.push(("versionName", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/preview/result
    pub async fn get_version_preview_1(
        &self,
        previous_version_id: Option<impl AsRef<str>>,
        deployment_project_id: Option<impl AsRef<str>>,
        plan_key: Option<impl AsRef<str>>,
        result_key: Option<impl AsRef<str>>,
        build_number: Option<impl AsRef<str>>,
    ) -> HttpResult<VersionPreview> {
        let url = format!("{}{}", self.base_url, "/api/latest/deploy/preview/result");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = previous_version_id {
                query_params.push(("previousVersionId", v.as_ref().to_string()));
            }
            if let Some(v) = deployment_project_id {
                query_params.push(("deploymentProjectId", v.as_ref().to_string()));
            }
            if let Some(v) = plan_key {
                query_params.push(("planKey", v.as_ref().to_string()));
            }
            if let Some(v) = result_key {
                query_params.push(("resultKey", v.as_ref().to_string()));
            }
            if let Some(v) = build_number {
                query_params.push(("buildNumber", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/quickFilter/visible
    pub async fn get_visible_filters(&self) -> HttpResult<GetVisibleFiltersResponse> {
        let url = format!("{}{}", self.base_url, "/api/latest/quickFilter/visible");
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/repository/{repositoryId}/rssrepository
    pub async fn grant_rss_repository_access(
        &self,
        repository_id: i64,
        request: RestIdContainer,
    ) -> HttpResult<RestRepositoryMinimal> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/repository/{}/rssrepository",
            repository_id)
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /admin/latest/session/{name}
    pub async fn invalidate_user_sessions(
        &self,
        name: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/admin/latest/session/{}", name.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/project/{deploymentProjectId}/repository
    pub async fn list_assigned_repositories(
        &self,
        deployment_project_id: impl AsRef<str>,
    ) -> HttpResult<ListAssignedRepositoriesResponse> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/deploy/project/{}/repository",
            deployment_project_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/project/{projectKey}/repository
    pub async fn list_assigned_repositories_1(
        &self,
        project_key: impl AsRef<str>,
    ) -> HttpResult<ListAssignedRepositories1Response> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/project/{}/repository",
            project_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/deployment/{id}/groups
    pub async fn list_group_permissions(
        &self,
        id: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
    ) -> HttpResult<ListGroupPermissionsResponse> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/deployment/{}/groups", id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/environment/{id}/groups
    pub async fn list_group_permissions_1(
        &self,
        id: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
    ) -> HttpResult<ListGroupPermissions1Response> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/environment/{}/groups", id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/global/groups
    pub async fn list_group_permissions_2(
        &self,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
        ignore: Option<impl AsRef<str>>,
    ) -> HttpResult<ListGroupPermissions2Response> {
        let url = format!(
            "{}{}", self.base_url, "/api/latest/permissions/global/groups"
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if let Some(v) = ignore {
                query_params.push(("ignore", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/plan/{key}/groups
    pub async fn list_group_permissions_3(
        &self,
        key: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
    ) -> HttpResult<ListGroupPermissions3Response> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/permissions/plan/{}/groups", key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/project/{key}/groups
    pub async fn list_group_permissions_4(
        &self,
        key: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
    ) -> HttpResult<ListGroupPermissions4Response> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/permissions/project/{}/groups",
            key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/projectplan/{key}/groups
    pub async fn list_group_permissions_5(
        &self,
        key: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
    ) -> HttpResult<ListGroupPermissions5Response> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/projectplan/{}/groups", key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/repository/{id}/groups
    pub async fn list_group_permissions_6(
        &self,
        id: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
    ) -> HttpResult<ListGroupPermissions6Response> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/repository/{}/groups", id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/deployment/{id}/roles
    pub async fn list_role_permissions(
        &self,
        id: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
    ) -> HttpResult<ListRolePermissionsResponse> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/permissions/deployment/{}/roles",
            id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/environment/{id}/roles
    pub async fn list_role_permissions_1(
        &self,
        id: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
    ) -> HttpResult<ListRolePermissions1Response> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/environment/{}/roles", id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/global/roles
    pub async fn list_role_permissions_2(
        &self,
        limit: Option<i64>,
        start: Option<i64>,
        ignore: Option<impl AsRef<str>>,
    ) -> HttpResult<ListRolePermissions2Response> {
        let url = format!("{}{}", self.base_url, "/api/latest/permissions/global/roles");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = ignore {
                query_params.push(("ignore", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/plan/{key}/roles
    pub async fn list_role_permissions_3(
        &self,
        key: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
    ) -> HttpResult<ListRolePermissions3Response> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/permissions/plan/{}/roles", key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/project/{key}/roles
    pub async fn list_role_permissions_4(
        &self,
        key: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
    ) -> HttpResult<ListRolePermissions4Response> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/permissions/project/{}/roles",
            key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/projectplan/{key}/roles
    pub async fn list_role_permissions_5(
        &self,
        key: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
    ) -> HttpResult<ListRolePermissions5Response> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/projectplan/{}/roles", key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/repository/{id}/roles
    pub async fn list_role_permissions_6(
        &self,
        id: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
    ) -> HttpResult<ListRolePermissions6Response> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/permissions/repository/{}/roles",
            id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/deployment/{id}/users
    pub async fn list_user_permissions(
        &self,
        id: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
    ) -> HttpResult<ListUserPermissionsResponse> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/permissions/deployment/{}/users",
            id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/environment/{id}/users
    pub async fn list_user_permissions_1(
        &self,
        id: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
    ) -> HttpResult<ListUserPermissions1Response> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/environment/{}/users", id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/global/users
    pub async fn list_user_permissions_2(
        &self,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
        ignore: Option<impl AsRef<str>>,
    ) -> HttpResult<ListUserPermissions2Response> {
        let url = format!("{}{}", self.base_url, "/api/latest/permissions/global/users");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if let Some(v) = ignore {
                query_params.push(("ignore", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/plan/{key}/users
    pub async fn list_user_permissions_3(
        &self,
        key: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
    ) -> HttpResult<ListUserPermissions3Response> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/permissions/plan/{}/users", key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/project/{key}/users
    pub async fn list_user_permissions_4(
        &self,
        key: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
    ) -> HttpResult<ListUserPermissions4Response> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/permissions/project/{}/users",
            key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/projectplan/{key}/users
    pub async fn list_user_permissions_5(
        &self,
        key: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
    ) -> HttpResult<ListUserPermissions5Response> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/projectplan/{}/users", key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/permissions/repository/{id}/users
    pub async fn list_user_permissions_6(
        &self,
        id: impl AsRef<str>,
        limit: Option<i64>,
        start: Option<i64>,
        name: Option<impl AsRef<str>>,
    ) -> HttpResult<ListUserPermissions6Response> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/permissions/repository/{}/users",
            id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = limit {
                query_params.push(("limit", v.to_string()));
            }
            if let Some(v) = start {
                query_params.push(("start", v.to_string()));
            }
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/plan/{projectKey}-{buildKey}/favourite
    pub async fn mark_plan_favourite(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/plan/{}-{}/favourite",
            project_key.as_ref(), build_key.as_ref())
        );
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/deploy/environment/{environmentId}/move/{position}/{relativeEnvironmentId}
    pub async fn move_environment(
        &self,
        environment_id: impl AsRef<str>,
        relative_environment_id: impl AsRef<str>,
        position: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/deploy/environment/{}/move/{}/{}", environment_id
            .as_ref(), relative_environment_id.as_ref(), position.as_ref())
        );
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/server/pause
    pub async fn pause(&self) -> HttpResult<RestServerStatusInfo> {
        let url = format!("{}{}", self.base_url, "/api/latest/server/pause");
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/server/prepareForRestart
    pub async fn prepare_for_restart(&self) -> HttpResult<RestServerStatusInfo> {
        let url = format!("{}{}", self.base_url, "/api/latest/server/prepareForRestart");
        let mut req = self.http_client.put(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /admin/latest/darkFeatures/{key}
    pub async fn put(
        &self,
        key: impl AsRef<str>,
        request: RestDarkFeature,
    ) -> HttpResult<RestDarkFeature> {
        let url = format!(
            "{}{}", self.base_url, format!("/admin/latest/darkFeatures/{}", key.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /admin/latest/darkFeatures/{key}/user/{userName}
    pub async fn put_1(
        &self,
        user_name: impl AsRef<str>,
        key: impl AsRef<str>,
        request: RestDarkFeature,
    ) -> HttpResult<RestUserDarkFeature> {
        let url = format!(
            "{}{}", self.base_url, format!("/admin/latest/darkFeatures/{}/user/{}",
            user_name.as_ref(), key.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/plan/{projectKey}-{buildKey}/test/{testId}/quarantine
    pub async fn quarantine_test(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        test_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/plan/{}-{}/test/{}/quarantine",
            project_key.as_ref(), build_key.as_ref(), test_id.as_ref())
        );
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/websudo-session
    pub async fn refresh_web_sudo_session(&self) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/latest/websudo-session");
        let mut req = self.http_client.put(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/reindex
    pub async fn reindex(&self) -> HttpResult<ReindexBean> {
        let url = format!("{}{}", self.base_url, "/api/latest/reindex");
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/deploy/environment/{environmentId}/agent-assignment/{executorKey}
    pub async fn remove_agent_assignment_from_environment(
        &self,
        environment_id: impl AsRef<str>,
        executor_key: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/deploy/environment/{}/agent-assignment/{}",
            environment_id.as_ref(), executor_key.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/config/job/{jobKey}/agent-assignment/{executorKey}
    pub async fn remove_agent_assignment_from_job(
        &self,
        executor_key: impl AsRef<str>,
        job_key: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/config/job/{}/agent-assignment/{}", executor_key
            .as_ref(), job_key.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/agent/assignment
    pub async fn remove_assignment(
        &self,
        executor_type: Option<impl AsRef<str>>,
        executor_id: Option<i64>,
        entity_id: Option<i64>,
        assignment_type: Option<impl AsRef<str>>,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/latest/agent/assignment");
        let mut req = self.http_client.delete(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = executor_type {
                query_params.push(("executorType", v.as_ref().to_string()));
            }
            if let Some(v) = executor_id {
                query_params.push(("executorId", v.to_string()));
            }
            if let Some(v) = entity_id {
                query_params.push(("entityId", v.to_string()));
            }
            if let Some(v) = assignment_type {
                query_params.push(("assignmentType", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/result/{projectKey}-{buildKey}-{buildNumber}/comment/{commentId}
    pub async fn remove_build_comment(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        comment_id: impl AsRef<str>,
        build_number: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/result/{}-{}-{}/comment/{}",
            project_key.as_ref(), build_key.as_ref(), comment_id.as_ref(), build_number
            .as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/result/{projectKey}-{buildKey}-{buildNumber}/label/{labelName}
    pub async fn remove_build_label(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        label_name: impl AsRef<str>,
        build_number: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/result/{}-{}-{}/label/{}",
            project_key.as_ref(), build_key.as_ref(), label_name.as_ref(), build_number
            .as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/queue/deployment/{deploymentResultId}
    pub async fn remove_deployment_from_queue(
        &self,
        deployment_result_id: i64,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/queue/deployment/{}",
            deployment_result_id)
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/permissions/deployment/{id}/groups/{name}
    pub async fn remove_permissions_for_group(
        &self,
        name: impl AsRef<str>,
        id: impl AsRef<str>,
        request: RemovePermissionsForGroupRequest,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/deployment/{}/groups/{}", name.as_ref(), id
            .as_ref())
        );
        let mut req = self
            .http_client
            .delete(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/permissions/environment/{id}/groups/{name}
    pub async fn remove_permissions_for_group_1(
        &self,
        name: impl AsRef<str>,
        id: impl AsRef<str>,
        request: RemovePermissionsForGroup1Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/environment/{}/groups/{}", name.as_ref(), id
            .as_ref())
        );
        let mut req = self
            .http_client
            .delete(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/permissions/global/groups/{name}
    pub async fn remove_permissions_for_group_2(
        &self,
        name: impl AsRef<str>,
        ignore: Option<impl AsRef<str>>,
        request: RemovePermissionsForGroup2Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/permissions/global/groups/{}",
            name.as_ref())
        );
        let mut req = self
            .http_client
            .delete(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = ignore {
                query_params.push(("ignore", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/permissions/plan/{key}/groups/{name}
    pub async fn remove_permissions_for_group_3(
        &self,
        name: impl AsRef<str>,
        key: impl AsRef<str>,
        request: RemovePermissionsForGroup3Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/permissions/plan/{}/groups/{}",
            name.as_ref(), key.as_ref())
        );
        let mut req = self
            .http_client
            .delete(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/permissions/project/{key}/groups/{name}
    pub async fn remove_permissions_for_group_4(
        &self,
        name: impl AsRef<str>,
        key: impl AsRef<str>,
        request: RemovePermissionsForGroup4Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/project/{}/groups/{}", name.as_ref(), key
            .as_ref())
        );
        let mut req = self
            .http_client
            .delete(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/permissions/projectplan/{key}/groups/{name}
    pub async fn remove_permissions_for_group_5(
        &self,
        name: impl AsRef<str>,
        key: impl AsRef<str>,
        request: RemovePermissionsForGroup5Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/projectplan/{}/groups/{}", name.as_ref(),
            key.as_ref())
        );
        let mut req = self
            .http_client
            .delete(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/permissions/repository/{id}/groups/{name}
    pub async fn remove_permissions_for_group_6(
        &self,
        name: impl AsRef<str>,
        id: impl AsRef<str>,
        request: RemovePermissionsForGroup6Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/repository/{}/groups/{}", name.as_ref(), id
            .as_ref())
        );
        let mut req = self
            .http_client
            .delete(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/permissions/deployment/{id}/roles/{name}
    pub async fn remove_permissions_for_role(
        &self,
        name: impl AsRef<str>,
        id: impl AsRef<str>,
        request: RemovePermissionsForRoleRequest,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/deployment/{}/roles/{}", name.as_ref(), id
            .as_ref())
        );
        let mut req = self
            .http_client
            .delete(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/permissions/environment/{id}/roles/{name}
    pub async fn remove_permissions_for_role_1(
        &self,
        name: impl AsRef<str>,
        id: impl AsRef<str>,
        request: RemovePermissionsForRole1Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/environment/{}/roles/{}", name.as_ref(), id
            .as_ref())
        );
        let mut req = self
            .http_client
            .delete(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/permissions/global/roles/{name}
    pub async fn remove_permissions_for_role_2(
        &self,
        name: impl AsRef<str>,
        ignore: Option<impl AsRef<str>>,
        request: RemovePermissionsForRole2Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/permissions/global/roles/{}",
            name.as_ref())
        );
        let mut req = self
            .http_client
            .delete(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = ignore {
                query_params.push(("ignore", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/permissions/plan/{key}/roles/{name}
    pub async fn remove_permissions_for_role_3(
        &self,
        name: impl AsRef<str>,
        key: impl AsRef<str>,
        request: RemovePermissionsForRole3Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/permissions/plan/{}/roles/{}",
            name.as_ref(), key.as_ref())
        );
        let mut req = self
            .http_client
            .delete(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/permissions/project/{key}/roles/{name}
    pub async fn remove_permissions_for_role_4(
        &self,
        name: impl AsRef<str>,
        key: impl AsRef<str>,
        request: RemovePermissionsForRole4Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/permissions/project/{}/roles/{}",
            name.as_ref(), key.as_ref())
        );
        let mut req = self
            .http_client
            .delete(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/permissions/projectplan/{key}/roles/{name}
    pub async fn remove_permissions_for_role_5(
        &self,
        name: impl AsRef<str>,
        key: impl AsRef<str>,
        request: RemovePermissionsForRole5Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/projectplan/{}/roles/{}", name.as_ref(), key
            .as_ref())
        );
        let mut req = self
            .http_client
            .delete(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/permissions/repository/{id}/roles/{name}
    pub async fn remove_permissions_for_role_6(
        &self,
        name: impl AsRef<str>,
        id: impl AsRef<str>,
        request: RemovePermissionsForRole6Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/repository/{}/roles/{}", name.as_ref(), id
            .as_ref())
        );
        let mut req = self
            .http_client
            .delete(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/permissions/deployment/{id}/users/{name}
    pub async fn remove_permissions_for_user(
        &self,
        name: impl AsRef<str>,
        id: impl AsRef<str>,
        request: RemovePermissionsForUserRequest,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/deployment/{}/users/{}", name.as_ref(), id
            .as_ref())
        );
        let mut req = self
            .http_client
            .delete(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/permissions/environment/{id}/users/{name}
    pub async fn remove_permissions_for_user_1(
        &self,
        name: impl AsRef<str>,
        id: impl AsRef<str>,
        request: RemovePermissionsForUser1Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/environment/{}/users/{}", name.as_ref(), id
            .as_ref())
        );
        let mut req = self
            .http_client
            .delete(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/permissions/global/users/{name}
    pub async fn remove_permissions_for_user_2(
        &self,
        name: impl AsRef<str>,
        ignore: Option<impl AsRef<str>>,
        request: RemovePermissionsForUser2Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/permissions/global/users/{}",
            name.as_ref())
        );
        let mut req = self
            .http_client
            .delete(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = ignore {
                query_params.push(("ignore", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/permissions/plan/{key}/users/{name}
    pub async fn remove_permissions_for_user_3(
        &self,
        name: impl AsRef<str>,
        key: impl AsRef<str>,
        request: RemovePermissionsForUser3Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/permissions/plan/{}/users/{}",
            name.as_ref(), key.as_ref())
        );
        let mut req = self
            .http_client
            .delete(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/permissions/project/{key}/users/{name}
    pub async fn remove_permissions_for_user_4(
        &self,
        name: impl AsRef<str>,
        key: impl AsRef<str>,
        request: RemovePermissionsForUser4Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/permissions/project/{}/users/{}",
            name.as_ref(), key.as_ref())
        );
        let mut req = self
            .http_client
            .delete(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/permissions/projectplan/{key}/users/{name}
    pub async fn remove_permissions_for_user_5(
        &self,
        name: impl AsRef<str>,
        key: impl AsRef<str>,
        request: RemovePermissionsForUser5Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/projectplan/{}/users/{}", name.as_ref(), key
            .as_ref())
        );
        let mut req = self
            .http_client
            .delete(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/permissions/repository/{id}/users/{name}
    pub async fn remove_permissions_for_user_6(
        &self,
        name: impl AsRef<str>,
        id: impl AsRef<str>,
        request: RemovePermissionsForUser6Request,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/permissions/repository/{}/users/{}", name.as_ref(), id
            .as_ref())
        );
        let mut req = self
            .http_client
            .delete(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/admin/expiry/custom/plan/{planKey}
    pub async fn remove_plan_custom_expiry_settings(
        &self,
        plan_key: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/admin/expiry/custom/plan/{}",
            plan_key.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/plan/{projectKey}-{buildKey}/label/{labelName}
    pub async fn remove_plan_label(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        label_name: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/plan/{}-{}/label/{}", project_key
            .as_ref(), build_key.as_ref(), label_name.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/deploy/environment/{environmentId}/requirement/{requirementId}
    pub async fn remove_requirement_from_environment(
        &self,
        environment_id: impl AsRef<str>,
        requirement_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/deploy/environment/{}/requirement/{}", environment_id
            .as_ref(), requirement_id.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /responsibility/latest/brokenBuild/{planResultKeyOrPlanKey}/{name}
    pub async fn remove_responsible(
        &self,
        name: impl AsRef<str>,
        plan_result_key_or_plan_key: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/responsibility/latest/brokenBuild/{}/{}",
            name.as_ref(), plan_result_key_or_plan_key.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/admin/groups/{name}/remove-users
    pub async fn remove_users_from_group(
        &self,
        name: impl AsRef<str>,
        request: RemoveUsersFromGroupRequest,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/admin/groups/{}/remove-users",
            name.as_ref())
        );
        let mut req = self
            .http_client
            .delete(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/websudo-session
    pub async fn remove_web_sudo_from_session(&self) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/latest/websudo-session");
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/admin/users/rename
    pub async fn rename_user(&self, request: RestUserRenameRequest) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/latest/admin/users/rename");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /admin/latest/user
    pub async fn rename_user_post(
        &self,
        external_rename: Option<impl AsRef<str>>,
        request: RestUserRenameRequest,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/admin/latest/user");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = external_rename {
                query_params.push(("externalRename", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /admin/latest/user/{newUserName}
    pub async fn rename_user_put(
        &self,
        new_user_name: impl AsRef<str>,
        external_rename: Option<impl AsRef<str>>,
        request: RestUserRenameRequest,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/admin/latest/user/{}", new_user_name
            .as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = external_rename {
                query_params.push(("externalRename", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/server/resume
    pub async fn resume(&self) -> HttpResult<RestServerStatusInfo> {
        let url = format!("{}{}", self.base_url, "/api/latest/server/resume");
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/avatar/user/{userName}/avatar.png
    pub async fn retrieve_avatar(
        &self,
        user_name: impl AsRef<str>,
        s: Option<i64>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/avatar/user/{}/avatar.png",
            user_name.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = s {
                query_params.push(("s", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/repository/{targetRepositoryId}/rssrepository/{repositoryId}
    pub async fn revoke_permission_to_use_repository_by_rss_repo(
        &self,
        repository_id: i64,
        target_repository_id: i64,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/repository/{}/rssrepository/{}",
            repository_id, target_repository_id)
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/access-token/{tokenId}
    pub async fn revoke_token(&self, token_id: impl AsRef<str>) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/access-token/{}", token_id
            .as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/admin/users/{name}/access-token/{tokenId}
    pub async fn revoke_user_token(
        &self,
        token_id: impl AsRef<str>,
        name: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/admin/users/{}/access-token/{}",
            token_id.as_ref(), name.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /admin/latest/expiry/run
    pub async fn run(&self) -> HttpResult<RunExpiryResponse> {
        let url = format!("{}{}", self.base_url, "/admin/latest/expiry/run");
        let mut req = self.http_client.put(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /admin/latest/ephemeral/config
    pub async fn save_configuration(
        &self,
        request: EphemeralAgentsConfigurationDTO,
    ) -> HttpResult<EphemeralAgentsConfigurationDTO> {
        let url = format!("{}{}", self.base_url, "/admin/latest/ephemeral/config");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/deploy/environment/{environmentId}/docker
    pub async fn save_docker_pipelines_configuration(
        &self,
        environment_id: impl AsRef<str>,
        request: RestDockerPipelineConfiguration,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/deploy/environment/{}/docker",
            environment_id.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /admin/latest/storageSettings
    pub async fn save_storage_configuration(
        &self,
        request: RestStorageConfiguration,
    ) -> HttpResult<RestStorageConfiguration> {
        let url = format!("{}{}", self.base_url, "/admin/latest/storageSettings");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /triggers/latest/remote/changeDetection
    pub async fn schedule_change_detection(
        &self,
        skip_branches: Option<bool>,
        plan_key: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, "/triggers/latest/remote/changeDetection"
        );
        let mut req = self.http_client.post(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = skip_branches {
                query_params.push(("skipBranches", v.to_string()));
            }
            query_params.push(("planKey", plan_key.as_ref().to_string()));
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/quicksearch
    pub async fn search(
        &self,
        search_term: Option<impl AsRef<str>>,
        search_entity: Option<impl AsRef<str>>,
    ) -> HttpResult<JsonSearchResultsList> {
        let url = format!("{}{}", self.base_url, "/api/latest/quicksearch");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = search_term {
                query_params.push(("searchTerm", v.as_ref().to_string()));
            }
            if let Some(v) = search_entity {
                query_params.push(("searchEntity", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/search/authors
    pub async fn search_authors(
        &self,
        max_result: Option<i64>,
        unlinked_only: Option<bool>,
        search_term: impl AsRef<str>,
        start_index: Option<i64>,
    ) -> HttpResult<SearchResultsList> {
        let url = format!("{}{}", self.base_url, "/api/latest/search/authors");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_result {
                query_params.push(("max-result", v.to_string()));
            }
            if let Some(v) = unlinked_only {
                query_params.push(("unlinkedOnly", v.to_string()));
            }
            query_params.push(("searchTerm", search_term.as_ref().to_string()));
            if let Some(v) = start_index {
                query_params.push(("start-index", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/deploy/project/{deploymentProjectId}/repository/search
    pub async fn search_available_repositories(
        &self,
        deployment_project_id: impl AsRef<str>,
        max_result: Option<i64>,
        search_term: Option<impl AsRef<str>>,
        start_index: Option<i64>,
    ) -> HttpResult<RestRepositoryList> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/deploy/project/{}/repository/search",
            deployment_project_id.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_result {
                query_params.push(("max-result", v.to_string()));
            }
            if let Some(v) = search_term {
                query_params.push(("searchTerm", v.as_ref().to_string()));
            }
            if let Some(v) = start_index {
                query_params.push(("start-index", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/project/{projectKey}/repository/search
    pub async fn search_available_repositories_1(
        &self,
        project_key: impl AsRef<str>,
        search_term: Option<impl AsRef<str>>,
    ) -> HttpResult<RestRepositoryList> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/project/{}/repository/search",
            project_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = search_term {
                query_params.push(("searchTerm", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/repository/{repositoryId}/rssrepository/search
    pub async fn search_available_repositories_2(
        &self,
        repository_id: i64,
        search_term: Option<impl AsRef<str>>,
    ) -> HttpResult<RestRepositoryList> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/repository/{}/rssrepository/search", repository_id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = search_term {
                query_params.push(("searchTerm", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/search/branches
    pub async fn search_branches(
        &self,
        max_result: Option<i64>,
        search_term: Option<impl AsRef<str>>,
        include_master_branch: Option<bool>,
        start_index: Option<i64>,
        branch_name_only: Option<bool>,
        fuzzy: Option<bool>,
        master_plan_key: impl AsRef<str>,
        released_in_deployment: Option<i64>,
    ) -> HttpResult<SearchResultsList> {
        let url = format!("{}{}", self.base_url, "/api/latest/search/branches");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_result {
                query_params.push(("max-result", v.to_string()));
            }
            if let Some(v) = search_term {
                query_params.push(("searchTerm", v.as_ref().to_string()));
            }
            if let Some(v) = include_master_branch {
                query_params.push(("includeMasterBranch", v.to_string()));
            }
            if let Some(v) = start_index {
                query_params.push(("start-index", v.to_string()));
            }
            if let Some(v) = branch_name_only {
                query_params.push(("branchNameOnly", v.to_string()));
            }
            if let Some(v) = fuzzy {
                query_params.push(("fuzzy", v.to_string()));
            }
            query_params.push(("masterPlanKey", master_plan_key.as_ref().to_string()));
            if let Some(v) = released_in_deployment {
                query_params.push(("releasedInDeployment", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/search/deployments
    pub async fn search_deployments(
        &self,
        max_result: Option<i64>,
        search_term: Option<impl AsRef<str>>,
        start_index: Option<i64>,
        permission: Option<impl AsRef<str>>,
    ) -> HttpResult<SearchResultsList> {
        let url = format!("{}{}", self.base_url, "/api/latest/search/deployments");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_result {
                query_params.push(("max-result", v.to_string()));
            }
            if let Some(v) = search_term {
                query_params.push(("searchTerm", v.as_ref().to_string()));
            }
            if let Some(v) = start_index {
                query_params.push(("start-index", v.to_string()));
            }
            if let Some(v) = permission {
                query_params.push(("permission", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/agent/assignment/search
    pub async fn search_entity_for_agent(
        &self,
        max_result: Option<i64>,
        executor_type: Option<impl AsRef<str>>,
        search_term: Option<impl AsRef<str>>,
        executor_id: Option<i64>,
        entity_type: Option<impl AsRef<str>>,
        start_index: Option<i64>,
        assignment_type: Option<impl AsRef<str>>,
    ) -> HttpResult<SearchResultsList> {
        let url = format!("{}{}", self.base_url, "/api/latest/agent/assignment/search");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_result {
                query_params.push(("max-result", v.to_string()));
            }
            if let Some(v) = executor_type {
                query_params.push(("executorType", v.as_ref().to_string()));
            }
            if let Some(v) = search_term {
                query_params.push(("searchTerm", v.as_ref().to_string()));
            }
            if let Some(v) = executor_id {
                query_params.push(("executorId", v.to_string()));
            }
            if let Some(v) = entity_type {
                query_params.push(("entityType", v.as_ref().to_string()));
            }
            if let Some(v) = start_index {
                query_params.push(("start-index", v.to_string()));
            }
            if let Some(v) = assignment_type {
                query_params.push(("assignmentType", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/dependency/search/{projectKey}-{buildKey}/child
    pub async fn search_for_available_plan_child_dependencies(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        max_result: Option<i64>,
        search_term: impl AsRef<str>,
        start_index: Option<i64>,
    ) -> HttpResult<RestDependencies> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/dependency/search/{}-{}/child",
            project_key.as_ref(), build_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_result {
                query_params.push(("max-result", v.to_string()));
            }
            query_params.push(("searchTerm", search_term.as_ref().to_string()));
            if let Some(v) = start_index {
                query_params.push(("start-index", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/dependency/search/{projectKey}-{buildKey}/parent
    pub async fn search_for_available_plan_parent_dependencies(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        max_result: Option<i64>,
        search_term: impl AsRef<str>,
        start_index: Option<i64>,
    ) -> HttpResult<RestDependencies> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/dependency/search/{}-{}/parent",
            project_key.as_ref(), build_key.as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_result {
                query_params.push(("max-result", v.to_string()));
            }
            query_params.push(("searchTerm", search_term.as_ref().to_string()));
            if let Some(v) = start_index {
                query_params.push(("start-index", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/search/jobs/{planKey}
    pub async fn search_jobs(
        &self,
        plan_key: impl AsRef<str>,
        max_result: Option<i64>,
        search_term: Option<impl AsRef<str>>,
        start_index: Option<i64>,
    ) -> HttpResult<SearchResultsList> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/search/jobs/{}", plan_key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_result {
                query_params.push(("max-result", v.to_string()));
            }
            if let Some(v) = search_term {
                query_params.push(("searchTerm", v.as_ref().to_string()));
            }
            if let Some(v) = start_index {
                query_params.push(("start-index", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/search/plans
    pub async fn search_plans(
        &self,
        max_result: Option<i64>,
        search_term: Option<impl AsRef<str>>,
        start_index: Option<i64>,
        permission: Option<impl AsRef<str>>,
        type_: Option<impl AsRef<str>>,
        fuzzy: Option<bool>,
    ) -> HttpResult<SearchResultsList> {
        let url = format!("{}{}", self.base_url, "/api/latest/search/plans");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_result {
                query_params.push(("max-result", v.to_string()));
            }
            if let Some(v) = search_term {
                query_params.push(("searchTerm", v.as_ref().to_string()));
            }
            if let Some(v) = start_index {
                query_params.push(("start-index", v.to_string()));
            }
            if let Some(v) = permission {
                query_params.push(("permission", v.as_ref().to_string()));
            }
            if let Some(v) = type_ {
                query_params.push(("type", v.as_ref().to_string()));
            }
            if let Some(v) = fuzzy {
                query_params.push(("fuzzy", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/search/projects
    pub async fn search_projects(
        &self,
        max_result: Option<i64>,
        search_term: Option<impl AsRef<str>>,
        start_index: Option<i64>,
        permission: Option<impl AsRef<str>>,
    ) -> HttpResult<SearchResultsList> {
        let url = format!("{}{}", self.base_url, "/api/latest/search/projects");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_result {
                query_params.push(("max-result", v.to_string()));
            }
            if let Some(v) = search_term {
                query_params.push(("searchTerm", v.as_ref().to_string()));
            }
            if let Some(v) = start_index {
                query_params.push(("start-index", v.to_string()));
            }
            if let Some(v) = permission {
                query_params.push(("permission", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/repository/{repositoryId}/rssBranches
    pub async fn search_specs_branches(
        &self,
        repository_id: i64,
        search_term: Option<impl AsRef<str>>,
    ) -> HttpResult<RestBranchList> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/repository/{}/rssBranches",
            repository_id)
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = search_term {
                query_params.push(("searchTerm", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/search/stages/{planKey}
    pub async fn search_stages(
        &self,
        plan_key: impl AsRef<str>,
        max_result: Option<i64>,
        search_term: Option<impl AsRef<str>>,
        start_index: Option<i64>,
        stage_id: Option<impl AsRef<str>>,
    ) -> HttpResult<SearchResultsList> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/search/stages/{}", plan_key
            .as_ref())
        );
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_result {
                query_params.push(("max-result", v.to_string()));
            }
            if let Some(v) = search_term {
                query_params.push(("searchTerm", v.as_ref().to_string()));
            }
            if let Some(v) = start_index {
                query_params.push(("start-index", v.to_string()));
            }
            if let Some(v) = stage_id {
                query_params.push(("stageId", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/search/users
    pub async fn search_users(
        &self,
        max_result: Option<i64>,
        search_term: impl AsRef<str>,
        start_index: Option<i64>,
    ) -> HttpResult<SearchResultsList> {
        let url = format!("{}{}", self.base_url, "/api/latest/search/users");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_result {
                query_params.push(("max-result", v.to_string()));
            }
            query_params.push(("searchTerm", search_term.as_ref().to_string()));
            if let Some(v) = start_index {
                query_params.push(("start-index", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///GET /api/latest/search/versions
    pub async fn search_versions(
        &self,
        max_result: Option<i64>,
        branch_key: Option<impl AsRef<str>>,
        search_term: Option<impl AsRef<str>>,
        start_index: Option<i64>,
        deployment_project_id: i64,
        chronological_order: Option<bool>,
    ) -> HttpResult<SearchResultsList> {
        let url = format!("{}{}", self.base_url, "/api/latest/search/versions");
        let mut req = self.http_client.get(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = max_result {
                query_params.push(("max-result", v.to_string()));
            }
            if let Some(v) = branch_key {
                query_params.push(("branchKey", v.as_ref().to_string()));
            }
            if let Some(v) = search_term {
                query_params.push(("searchTerm", v.as_ref().to_string()));
            }
            if let Some(v) = start_index {
                query_params.push(("start-index", v.to_string()));
            }
            query_params
                .push(("deploymentProjectId", deployment_project_id.to_string()));
            if let Some(v) = chronological_order {
                query_params.push(("chronologicalOrder", v.to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /admin/latest/expiry/configuration
    pub async fn set_configuration(
        &self,
        request: RestCombinedExpiryConfiguration,
    ) -> HttpResult<RestCombinedExpiryConfiguration> {
        let url = format!("{}{}", self.base_url, "/admin/latest/expiry/configuration");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/job/{jobKey}/docker
    pub async fn set_docker_pipeline_configuration(
        &self,
        job_key: impl AsRef<str>,
        request: RestDockerPipelineConfiguration,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/job/{}/docker", job_key.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/quickFilter/visible
    pub async fn set_visible_filters(
        &self,
        request: SetVisibleFiltersRequest,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/latest/quickFilter/visible");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/queue/{projectKey}-{buildKey}
    pub async fn start_build_1(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        execute_all_stages: Option<bool>,
        custom_revision: Option<impl AsRef<str>>,
        stage: Option<impl AsRef<str>>,
    ) -> HttpResult<RestQueuedBuild> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/queue/{}-{}", project_key
            .as_ref(), build_key.as_ref())
        );
        let mut req = self.http_client.post(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = execute_all_stages {
                query_params.push(("executeAllStages", v.to_string()));
            }
            if let Some(v) = custom_revision {
                query_params.push(("customRevision", v.as_ref().to_string()));
            }
            if let Some(v) = stage {
                query_params.push(("stage", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/queue/deployment
    pub async fn start_deployment(
        &self,
        version_id: impl AsRef<str>,
        environment_id: impl AsRef<str>,
        verbose_logging: Option<impl AsRef<str>>,
    ) -> HttpResult<RestQueuedDeployment> {
        let url = format!("{}{}", self.base_url, "/api/latest/queue/deployment");
        let mut req = self.http_client.post(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            query_params.push(("versionId", version_id.as_ref().to_string()));
            query_params.push(("environmentId", environment_id.as_ref().to_string()));
            if let Some(v) = verbose_logging {
                query_params.push(("verboseLogging", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/queue/{projectKey}-{buildKey}-{buildNumber}
    pub async fn stop_build(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        build_number: i64,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/queue/{}-{}-{}", project_key
            .as_ref(), build_key.as_ref(), build_number)
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /admin/latest/ephemeral/config/test-connection
    pub async fn test_connection(
        &self,
        request: EphemeralAgentsConfigurationDTO,
    ) -> HttpResult<TestConnectionResultDto> {
        let url = format!(
            "{}{}", self.base_url, "/admin/latest/ephemeral/config/test-connection"
        );
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/repository/testConnection
    pub async fn test_connection_1(
        &self,
        request: TestConnection1Request,
    ) -> HttpResult<RestRepositoryConnectionResult> {
        let url = format!(
            "{}{}", self.base_url, "/api/latest/repository/testConnection"
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /admin/latest/scheduler/jobs/trigger
    pub async fn trigger_job(
        &self,
        request: RestScheduledJob,
    ) -> HttpResult<RestQuarantineConfig> {
        let url = format!("{}{}", self.base_url, "/admin/latest/scheduler/jobs/trigger");
        let mut req = self
            .http_client
            .post(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/repository/{repositoryId}/scanNow
    pub async fn trigger_specs_scanning(
        &self,
        repository_id: i64,
        branch: Option<impl AsRef<str>>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/repository/{}/scanNow",
            repository_id)
        );
        let mut req = self.http_client.post(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = branch {
                query_params.push(("branch", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/repository/scan
    pub async fn trigger_specs_scanning_1(
        &self,
        name: Option<impl AsRef<str>>,
        repository_id: Option<i64>,
        id: Option<i64>,
        repository_name: Option<impl AsRef<str>>,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/latest/repository/scan");
        let mut req = self.http_client.post(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            if let Some(v) = name {
                query_params.push(("name", v.as_ref().to_string()));
            }
            if let Some(v) = repository_id {
                query_params.push(("repositoryId", v.to_string()));
            }
            if let Some(v) = id {
                query_params.push(("id", v.to_string()));
            }
            if let Some(v) = repository_name {
                query_params.push(("repositoryName", v.as_ref().to_string()));
            }
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/admin/users/{name}/groups
    pub async fn unassign_groups(
        &self,
        name: impl AsRef<str>,
        request: UnassignGroupsRequest,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/admin/users/{}/groups", name
            .as_ref())
        );
        let mut req = self
            .http_client
            .delete(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/plan/{projectKey}-{buildKey}/test/{testId}/unleash
    pub async fn unleash_test(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
        test_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/plan/{}-{}/test/{}/unleash",
            project_key.as_ref(), build_key.as_ref(), test_id.as_ref())
        );
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/admin/users/{name}/alias
    pub async fn unlink_user_repository_alias(
        &self,
        name: impl AsRef<str>,
        request: RestUserAlias,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/admin/users/{}/alias", name
            .as_ref())
        );
        let mut req = self
            .http_client
            .delete(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///DELETE /api/latest/plan/{projectKey}-{buildKey}/favourite
    pub async fn unmark_plan_favourite(
        &self,
        project_key: impl AsRef<str>,
        build_key: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/plan/{}-{}/favourite",
            project_key.as_ref(), build_key.as_ref())
        );
        let mut req = self.http_client.delete(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/agent/{agentId}/capability/{capabilityKey}
    pub async fn update_agent_capability(
        &self,
        agent_id: i64,
        capability_key: impl AsRef<str>,
        request: RestRemoteAgentCapability,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/agent/{}/capability/{}",
            agent_id, capability_key.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /admin/latest/artifactHandlers/agentLocal
    pub async fn update_agent_local_handler(
        &self,
        request: RestArtifactHandler,
    ) -> HttpResult<RestArtifactHandler> {
        let url = format!(
            "{}{}", self.base_url, "/admin/latest/artifactHandlers/agentLocal"
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/elasticConfiguration/image-id/{imageId}
    pub async fn update_all_image_ids(
        &self,
        image_id: impl AsRef<str>,
        new_image_id: impl AsRef<str>,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/elasticConfiguration/image-id/{}", image_id.as_ref())
        );
        let mut req = self.http_client.put(url);
        {
            let mut query_params: Vec<(&str, String)> = Vec::new();
            query_params.push(("newImageId", new_image_id.as_ref().to_string()));
            if !query_params.is_empty() {
                req = req.query(&query_params);
            }
        }
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /admin/latest/config/auditLog
    pub async fn update_audit_log_configuration(
        &self,
        request: RestAuditLogConfiguration,
    ) -> HttpResult<RestAuditLogConfiguration> {
        let url = format!("{}{}", self.base_url, "/admin/latest/config/auditLog");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /admin/latest/artifactHandlers/bambooRemote
    pub async fn update_bamboo_remote_handler(
        &self,
        request: SimpleRestArtifactHandler,
    ) -> HttpResult<RestArtifactHandler> {
        let url = format!(
            "{}{}", self.base_url, "/admin/latest/artifactHandlers/bambooRemote"
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/ephemeral/templateConfiguration/{configurationId}/capability
    pub async fn update_capability(
        &self,
        configuration_id: i64,
        request: RestCapability,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/ephemeral/templateConfiguration/{}/capability",
            configuration_id)
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/elasticConfiguration/{configurationId}
    pub async fn update_configuration(
        &self,
        configuration_id: i64,
        request: RestElasticImageConfig,
    ) -> HttpResult<RestElasticImageConfig> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/elasticConfiguration/{}",
            configuration_id)
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/deploy/environment/{environmentId}/prerequisites
    pub async fn update_environment_prerequisites(
        &self,
        environment_id: impl AsRef<str>,
        request: RestEnvironmentPrerequisites,
    ) -> HttpResult<()> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/deploy/environment/{}/prerequisites", environment_id
            .as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/deploy/environment/{environmentId}/variable/{variableName}
    pub async fn update_environment_variable(
        &self,
        variable_name: impl AsRef<str>,
        environment_id: impl AsRef<str>,
        request: RestVariable,
    ) -> HttpResult<RestVariableDefinitionContext> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/deploy/environment/{}/variable/{}", variable_name
            .as_ref(), environment_id.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /admin/latest/config/general
    pub async fn update_general_configuration(
        &self,
        request: RestGeneralConfiguration,
    ) -> HttpResult<RestBuildConcurrency> {
        let url = format!("{}{}", self.base_url, "/admin/latest/config/general");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /admin/latest/globalVariables/{variableId}
    pub async fn update_global_variable(
        &self,
        variable_id: impl AsRef<str>,
        request: RestGlobalVariable,
    ) -> HttpResult<RestGlobalVariable> {
        let url = format!(
            "{}{}", self.base_url, format!("/admin/latest/globalVariables/{}",
            variable_id.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /admin/latest/config/imServer
    pub async fn update_im_server_config(
        &self,
        request: RestIMServerConfiguration,
    ) -> HttpResult<RestIMServerConfiguration> {
        let url = format!("{}{}", self.base_url, "/admin/latest/config/imServer");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /admin/latest/config/mailServer
    pub async fn update_mail_configuration(
        &self,
        request: RestMailConfiguration,
    ) -> HttpResult<RestMailConfiguration> {
        let url = format!("{}{}", self.base_url, "/admin/latest/config/mailServer");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /admin/latest/config/quarantine
    pub async fn update_quarantine_settings(
        &self,
        request: RestQuarantineConfig,
    ) -> HttpResult<RestQuarantineConfig> {
        let url = format!("{}{}", self.base_url, "/admin/latest/config/quarantine");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/quickFilter/{id}
    pub async fn update_quick_filter(
        &self,
        id: i64,
        request: RestQuickFilter,
    ) -> HttpResult<RestQuickFilter> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/quickFilter/{}", id)
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /admin/latest/config/remoteAgentSupport
    pub async fn update_remote_agent_configuration(
        &self,
        request: RestRemoteAgentConfiguration,
    ) -> HttpResult<RestRemoteAgentConfiguration> {
        let url = format!(
            "{}{}", self.base_url, "/admin/latest/config/remoteAgentSupport"
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/deploy/environment/{environmentId}/requirement/{requirementId}
    pub async fn update_requirement_for_environment(
        &self,
        environment_id: impl AsRef<str>,
        requirement_id: impl AsRef<str>,
        request: RestRequirement,
    ) -> HttpResult<RestRequirement> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/deploy/environment/{}/requirement/{}", environment_id
            .as_ref(), requirement_id.as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /admin/latest/config/agents/{agentId}
    pub async fn update_rest_agent(
        &self,
        agent_id: impl AsRef<str>,
        request: RestAgent,
    ) -> HttpResult<RestAgent> {
        let url = format!(
            "{}{}", self.base_url, format!("/admin/latest/config/agents/{}", agent_id
            .as_ref())
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /admin/latest/config/build/concurrency
    pub async fn update_rest_build_concurrency(
        &self,
        request: RestBuildConcurrency,
    ) -> HttpResult<RestBuildConcurrency> {
        let url = format!(
            "{}{}", self.base_url, "/admin/latest/config/build/concurrency"
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /admin/latest/config/build/monitoring
    pub async fn update_rest_build_monitoring(
        &self,
        request: RestBuildMonitoring,
    ) -> HttpResult<BuildMonitoringLink> {
        let url = format!(
            "{}{}", self.base_url, "/admin/latest/config/build/monitoring"
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /admin/latest/elastic/config
    pub async fn update_rest_elastic_configuration(
        &self,
        request: RestElasticConfiguration,
    ) -> HttpResult<RestElasticConfiguration> {
        let url = format!("{}{}", self.base_url, "/admin/latest/elastic/config");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /admin/latest/security/settings
    pub async fn update_rest_security_settings(
        &self,
        request: RestSecuritySettings,
    ) -> HttpResult<SecuritySettingsLink> {
        let url = format!("{}{}", self.base_url, "/admin/latest/security/settings");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /admin/latest/artifactHandlers/s3
    pub async fn update_s3_handler(
        &self,
        request: RestArtifactHandler,
    ) -> HttpResult<RestArtifactHandler> {
        let url = format!("{}{}", self.base_url, "/admin/latest/artifactHandlers/s3");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /admin/latest/artifactHandlers/sftp
    pub async fn update_sftp_handler(
        &self,
        request: SimpleRestArtifactHandler,
    ) -> HttpResult<RestArtifactHandler> {
        let url = format!("{}{}", self.base_url, "/admin/latest/artifactHandlers/sftp");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/ephemeral/templateConfiguration/{configurationId}
    pub async fn update_template_configuration(
        &self,
        configuration_id: i64,
        request: RestEphemeralAgentTemplate,
    ) -> HttpResult<RestEphemeralAgentTemplate> {
        let url = format!(
            "{}{}", self.base_url,
            format!("/api/latest/ephemeral/templateConfiguration/{}", configuration_id)
        );
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///POST /api/latest/deploy/version/{deploymentVersionId}/status/{newStatus}
    pub async fn update_version_status(
        &self,
        deployment_version_id: impl AsRef<str>,
        new_status: impl AsRef<str>,
    ) -> HttpResult<RestDeploymentVersionStatuses> {
        let url = format!(
            "{}{}", self.base_url, format!("/api/latest/deploy/version/{}/status/{}",
            deployment_version_id.as_ref(), new_status.as_ref())
        );
        let mut req = self.http_client.post(url);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.json().await.map_err(HttpError::deserialization_error)?;
            Ok(body)
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /api/latest/avatar/user/avatar.png
    pub async fn upload_avatar(&self, form: reqwest::multipart::Form) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/api/latest/avatar/user/avatar.png");
        let mut req = self.http_client.put(url).multipart(form);
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
    ///PUT /admin/latest/globalVariables/verify
    pub async fn verify_global_variable_value(
        &self,
        request: RestVerificationRequest,
    ) -> HttpResult<()> {
        let url = format!("{}{}", self.base_url, "/admin/latest/globalVariables/verify");
        let mut req = self
            .http_client
            .put(url)
            .body(serde_json::to_vec(&request).map_err(HttpError::serialization_error)?)
            .header("content-type", "application/json");
        if let Some(api_key) = &self.api_key {
            req = req.bearer_auth(api_key);
        }
        for (name, value) in &self.custom_headers {
            req = req.header(name, value);
        }
        let response = req.send().await?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            let status_code = status.as_u16();
            let message = status.canonical_reason().unwrap_or("Unknown error");
            let body = response.text().await.ok();
            Err(HttpError::from_status(status_code, message, body))
        }
    }
}
