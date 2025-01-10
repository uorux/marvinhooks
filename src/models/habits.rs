use serde::{Deserialize, Serialize};

/// Represents a habit with tracking and scheduling details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Habit {
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    /// Could be "unassigned" or a project/category ID.
    #[serde(default)]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub label_ids: Vec<String>,
    #[serde(default)]
    pub is_starred: Option<i64>,
    #[serde(default)]
    pub is_frogged: Option<i64>,
    #[serde(default)]
    pub time_estimate: Option<i64>,
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub units: Option<String>,
    /// Frequency of the habit: "day", "week", "month", "quarter", or "year".
    pub period: String,
    pub target: f64,
    #[serde(default)]
    pub is_positive: Option<bool>,
    /// Type of tracking: "boolean" or "number".
    pub record_type: String,
    #[serde(default)]
    pub show_in_day_view: Option<bool>,
    #[serde(default)]
    pub show_in_calendar: Option<bool>,
    #[serde(default)]
    pub ask_on: Option<Vec<i64>>,
    #[serde(default)]
    pub start_time: Option<String>,
    #[serde(default)]
    pub end_time: Option<String>,
    #[serde(default)]
    pub time: Option<String>,
    #[serde(default)]
    pub show_after_success: Option<bool>,
    #[serde(default)]
    pub show_after_record: Option<bool>,
    #[serde(default)]
    pub done: Option<bool>,
    /// Array of [time1, value1, time2, value2, ...] representing the tracking history.
    #[serde(default)]
    pub history: Vec<f64>,
    #[serde(default)]
    pub dismissed: Option<String>,
}

/// Represents a single record for a habit.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HabitRecord {
    pub value: f64,
    pub time: i64,
}
