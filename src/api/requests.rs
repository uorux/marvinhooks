use serde::{Serialize, Deserialize};

/// A request body for testing credentials (`/api/test`).
/// The server expects just the token in the header, but we
/// can define a struct if we needed to send anything else.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRequest {
    // Possibly empty or not used if the endpoint
    // only returns a simple "OK".
}

/// POST body to create a new Task.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskRequest {
    #[serde(default)]
    pub done: bool,
    #[serde(default)]
    pub day: Option<String>,
    pub title: String,

    #[serde(default)]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub label_ids: Option<Vec<String>>,
    #[serde(default)]
    pub first_scheduled: Option<String>,
    #[serde(default)]
    pub rank: Option<i64>,

    #[serde(default)]
    pub daily_section: Option<String>,
    #[serde(default)]
    pub bonus_section: Option<String>,
    #[serde(default)]
    pub custom_section: Option<String>,
    #[serde(default)]
    pub time_block_section: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub due_date: Option<String>,
    #[serde(default)]
    pub time_estimate: Option<i64>,
    #[serde(default)]
    pub is_reward: Option<bool>,
    #[serde(default)]
    pub is_starred: Option<i64>,
    #[serde(default)]
    pub is_frogged: Option<i64>,
    #[serde(default)]
    pub planned_week: Option<String>,
    #[serde(default)]
    pub planned_month: Option<String>,
    #[serde(default)]
    pub reward_points: Option<f64>,
    #[serde(default)]
    pub reward_id: Option<String>,
    #[serde(default)]
    pub backburner: Option<bool>,
    #[serde(default)]
    pub review_date: Option<String>,

    #[serde(default)]
    pub item_snooze_time: Option<i64>,
    #[serde(default)]
    pub perma_snooze_time: Option<String>,

    /// Time zone offset in minutes
    pub time_zone_offset: Option<i32>,
}

/// POST body to mark a Task done.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkDoneRequest {
    pub item_id: String,
    pub time_zone_offset: Option<i32>,
}

/// POST body for creating a Project
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectRequest {
    #[serde(default)]
    pub done: bool,
    #[serde(default)]
    pub day: Option<String>,
    pub title: String,

    #[serde(default)]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub label_ids: Option<Vec<String>>,
    #[serde(default)]
    pub first_scheduled: Option<String>,
    #[serde(default)]
    pub rank: Option<i64>,

    #[serde(default)]
    pub daily_section: Option<String>,
    #[serde(default)]
    pub bonus_section: Option<String>,
    #[serde(default)]
    pub custom_section: Option<String>,
    #[serde(default)]
    pub time_block_section: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub due_date: Option<String>,
    #[serde(default)]
    pub time_estimate: Option<i64>,
    #[serde(default)]
    pub is_reward: Option<bool>,
    #[serde(default)]
    pub priority: Option<String>,
    #[serde(default)]
    pub is_frogged: Option<i64>,
    #[serde(default)]
    pub planned_week: Option<String>,
    #[serde(default)]
    pub planned_month: Option<String>,
    #[serde(default)]
    pub reward_points: Option<f64>,
    #[serde(default)]
    pub reward_id: Option<String>,
    #[serde(default)]
    pub backburner: Option<bool>,
    #[serde(default)]
    pub review_date: Option<String>,

    #[serde(default)]
    pub item_snooze_time: Option<i64>,
    #[serde(default)]
    pub perma_snooze_time: Option<String>,

    /// Time zone offset in minutes
    pub time_zone_offset: Option<i32>,
}

/// POST body to create an Event
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateEventRequest {
    pub title: String,
    #[serde(default)]
    pub note: Option<String>,
    pub length: i64,
    pub start: String, // ISO date/time
}

/// POST body for reading a doc using full access token
/// Actually, this is a GET, so we might not need a request body,
/// but let's define it for consistency if we wanted to do a client method that looks like .get_doc("someId")
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadDocQuery {
    pub id: String,
}

/// The doc structure we read/update. We’ll keep it general, with `extra` for unknown fields.
use std::collections::HashMap;
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarvinDoc {
    #[serde(rename = "_id")]
    pub id: String,

    #[serde(rename = "_rev")]
    pub rev: Option<String>,

    #[serde(default)]
    pub db: Option<String>,

    #[serde(default)]
    pub created_at: Option<i64>,
    #[serde(default)]
    pub updated_at: Option<i64>,

    /// Any other fields stored in the doc
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// POST body for updating an existing doc
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateDocRequest {
    pub item_id: String,
    pub setters: Vec<DocSetter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocSetter {
    pub key: String,
    pub val: Value,
}

/// POST body for creating an entirely new doc
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateDocRequest {
    #[serde(rename = "_id")]
    pub id: String,
    pub db: String,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub done: Option<bool>,
    #[serde(default)]
    pub created_at: Option<i64>,
    // Flatten or store other fields as you wish
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// POST body for deleting a doc
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteDocRequest {
    pub item_id: String,
}

/// POST body for the track/time endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackRequest {
    pub task_id: String,
    pub action: String, // "START" or "STOP"
}

/// Response for track/time endpoints
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackResponse {
    pub start_id: Option<String>,
    pub start_times: Option<Vec<i64>>,
    pub stop_id: Option<String>,
    pub stop_times: Option<Vec<i64>>,
    #[serde(default)]
    pub issues: Vec<String>,
}

/// Request for /api/tracks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TracksRequest {
    pub task_ids: Vec<String>,
}

/// Response item for /api/tracks
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackItem {
    pub task_id: String,
    pub times: Vec<i64>,
}

/// Claim reward points
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimRewardPointsRequest {
    pub points: f64,
    #[serde(default)]
    pub item_id: Option<String>,
    pub date: String,
    pub op: String, // "CLAIM", "UNCLAIM", or "SPEND"
}

/// Reset reward points (requires full access token)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetRewardPointsRequest {
    // No body needed for /api/resetRewardPoints, but you could define one if desired
}

/// Setting (creating) reminders
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetRemindersRequest {
    pub reminders: Vec<crate::models::reminders::Reminder>,
}

/// Deleting one or more reminders
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteRemindersRequest {
    pub reminder_ids: Vec<String>,
}

/// Delete all reminders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAllRemindersRequest {
    // no body needed, but could define fields if needed
}

/// Update Habit (record, undo, rewrite)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateHabitRequest {
    pub habit_id: String,

    #[serde(default)]
    pub time: Option<i64>,
    #[serde(default)]
    pub value: Option<f64>,

    #[serde(default)]
    pub undo: Option<bool>,

    #[serde(default)]
    pub history: Option<Vec<f64>>,

    #[serde(default)]
    pub update_db: bool,
}
