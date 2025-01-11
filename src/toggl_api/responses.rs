use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// GET /api/v9/me response
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeResponse {
    pub id: i64,
    pub email: String,
    #[serde(default)]
    pub fullname: Option<String>,
    #[serde(default)]
    pub timezone: Option<String>,
    #[serde(default)]
    pub default_workspace_id: Option<i64>,

    // Many more fields if you like (clients, projects, tasks, etc. if with_related_data=true)
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// A generic "client" object returned by the Toggl Track API
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TogglClient {
    pub id: i64,
    pub name: String,
    #[serde(default)]
    pub notes: Option<String>,
    #[serde(default)]
    pub archived: Option<bool>,
    #[serde(default)]
    pub at: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// A wrapper for listing clients
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListClientsResponse {
    pub items: Vec<TogglClient>,
}

/// A single project object
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TogglProject {
    pub id: i64,
    pub name: String,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub client_id: Option<i64>,
    #[serde(default)]
    pub is_private: Option<bool>,

    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// A wrapper for listing projects
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListProjectsResponse {
    pub items: Vec<TogglProject>,
}

/// A single task object
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TogglTask {
    pub id: i64,
    pub name: String,
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub estimated_seconds: Option<i64>,
    #[serde(default)]
    pub project_id: Option<i64>,

    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

/// A time entry object
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeEntry {
    pub id: i64,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub duration: Option<i64>,
    pub start: String,
    #[serde(default)]
    pub stop: Option<String>,
    #[serde(default)]
    pub billable: Option<bool>,
    #[serde(default)]
    pub user_id: Option<i64>,
    #[serde(default)]
    pub project_id: Option<i64>,
    #[serde(default)]
    pub workspace_id: Option<i64>,

    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}
