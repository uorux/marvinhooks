use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a server-side reminder object for push notifications.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reminder {
    /// Unix timestamp (seconds) for when the reminder is set to trigger.
    pub time: i64,
    /// Minutes ahead of the task due time when the user wants the reminder (-1 = use default).
    pub offset: i64,
    /// A unique ID for the reminder, often the associated `taskId` or a random string.
    pub reminder_id: String,
    /// The type of reminder ("T", "M", "DT", "DP", "t").
    #[serde(rename = "type")]
    pub reminder_type: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub snooze: Option<i64>,
    #[serde(default)]
    pub auto_snooze: Option<bool>,
    pub can_track: bool,
}

/// Represents an inline reminder format stored in tasks or subtasks.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetterInfo {
    /// Flattened data structure for changed fields (e.g., `{ "dueDate": "2020-10-15" }`).
    #[serde(flatten)]
    pub data: HashMap<String, serde_json::Value>,
}
