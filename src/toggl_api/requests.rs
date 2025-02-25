use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request for the `GET /api/v9/me` does not require a specific body (we pass query parameters).
/// So we typically won't define a separate request struct for that unless you need it.

// -------------------------
// POST /api/v9/workspaces/{workspace_id}/time_entries
// -------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTimeEntryRequest {
    /// Whether the time entry is marked as billable, optional, default false
    #[serde(default)]
    pub billable: Option<bool>,

    /// Must be provided when creating a time entry and should identify the service/application used
    pub created_with: String,

    /// Time entry description, optional
    #[serde(default)]
    pub description: Option<String>,

    /// Time entry duration. For running entries should be negative (e.g. -1)
    #[serde(default = "default_negative_duration")]
    pub duration: i64,

    /// Deprecated: can typically be ignored
    #[serde(default)]
    pub duronly: Option<bool>,

    /// Optional event metadata
    #[serde(default)]
    pub event_metadata: Option<EventMetadata>,

    /// Project ID, legacy field
    #[serde(default)]
    pub pid: Option<i64>,

    /// Project ID, optional
    #[serde(default)]
    pub project_id: Option<i64>,

    /// List of user IDs to share this time entry with
    #[serde(default)]
    pub shared_with_user_ids: Option<Vec<i64>>,

    /// Start time in UTC, required. Format: 2006-01-02T15:04:05Z
    pub start: String,

    /// If provided, the date part will take precedence over the date part of `start`
    #[serde(default)]
    pub start_date: Option<String>,

    /// Stop time in UTC; can be omitted if it's still running (duration = -1)
    #[serde(default)]
    pub stop: Option<String>,

    /// Can be "add" or "delete". Used when updating an existing time entry
    #[serde(default)]
    pub tag_action: Option<String>,

    /// IDs of tags to add/remove
    #[serde(default)]
    pub tag_ids: Option<Vec<i64>>,

    /// Names of tags to add/remove
    #[serde(default)]
    pub tags: Option<Vec<String>>,

    /// Task ID
    #[serde(default)]
    pub task_id: Option<i64>,

    /// Task ID, legacy field
    #[serde(default)]
    pub tid: Option<i64>,

    /// Time Entry creator ID, if omitted will use the requester user ID
    #[serde(default)]
    pub user_id: Option<i64>,

    /// Workspace ID (required)
    pub workspace_id: i64,
}

/// Additional metadata for time entry creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    #[serde(default)]
    pub origin_feature: Option<String>,
    #[serde(default)]
    pub visible_goals_count: Option<i64>,
}

fn default_negative_duration() -> i64 {
    -1
}

// -------------------------
// POST /api/v9/workspaces/{workspace_id}/clients
// -------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateClientRequest {
    pub name: String,
    #[serde(default)]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTagRequest {
    pub name: String,
}
// -------------------------
// GET /api/v9/workspaces/{workspace_id}/clients
// (no body needed, possibly some query params?)
// -------------------------

// If you need to filter by name or status, you can define a separate Query struct if you like.

// -------------------------
// GET /api/v9/workspaces/{workspace_id}/projects
// POST /api/v9/workspaces/{workspace_id}/projects
// -------------------------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateProjectRequest {
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub auto_estimates: Option<bool>,
    #[serde(default)]
    pub billable: Option<bool>,
    #[serde(default)]
    pub cid: Option<i64>, // legacy
    #[serde(default)]
    pub client_id: Option<i64>,
    #[serde(default)]
    pub client_name: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub estimated_hours: Option<i64>,
    #[serde(default)]
    pub fixed_fee: Option<f64>,
    #[serde(default)]
    pub is_private: Option<bool>,
    #[serde(default)]
    pub is_shared: Option<bool>,
    pub name: String,
    #[serde(default)]
    pub rate: Option<f64>,
    #[serde(default)]
    pub rate_change_mode: Option<String>,
    #[serde(default)]
    pub recurring: Option<bool>,
    #[serde(default)]
    pub recurring_parameters: Option<RecurringParameters>,
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub template: Option<bool>,
    #[serde(default)]
    pub template_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringParameters {
    #[serde(default)]
    pub custom_period: Option<i64>,
    #[serde(default)]
    pub period: Option<String>,
    #[serde(default)]
    pub project_start_date: Option<String>,
}

// -------------------------
// GET /api/v9/workspaces/{workspace_id}/projects/{project_id}/tasks
// POST /api/v9/workspaces/{workspace_id}/projects/{project_id}/tasks
// -------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    #[serde(default)]
    pub active: Option<bool>,
    #[serde(default)]
    pub estimated_seconds: Option<i64>,
    pub name: String,
    #[serde(default)]
    pub user_id: Option<i64>,
}
