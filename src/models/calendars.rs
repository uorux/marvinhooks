use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a calendar configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Calendar {
    #[serde(rename = "_id")]
    pub id: String,
    pub two_way: bool,
    pub provider: String,
    pub display_name: String,
    pub url: String,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
    #[serde(default)]
    pub item_type: Option<String>,
    #[serde(default)]
    pub ctag: Option<String>,
    #[serde(default)]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub label_ids: Option<Vec<String>>,
    #[serde(default)]
    pub calendar_id: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub access_role: Option<String>,
    #[serde(default)]
    pub rank: Option<i64>,
    #[serde(default)]
    pub time_zone_fix: Option<i64>,
}

/// Represents an individual event on the calendar.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
    pub is_all_day: bool,
    #[serde(default)]
    pub parent_id: Option<String>,
    #[serde(default)]
    pub label_ids: Vec<String>,
    pub start: String,
    pub length: i64,
    #[serde(default)]
    pub cal_id: Option<String>,
    #[serde(default)]
    pub cal_url: Option<String>,
    #[serde(default)]
    pub etag: Option<String>,
    #[serde(default)]
    pub cal_data: Option<String>,
    #[serde(default)]
    pub cancel_dates: HashMap<String, bool>,
    #[serde(default)]
    pub exceptions: HashMap<String, EventException>,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub hidden: Option<bool>,
    #[serde(default)]
    pub time_zone_fix: Option<i64>,
}

/// Represents an exception for a recurring event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventException {
    #[serde(default)]
    pub etag: Option<String>,
    #[serde(default)]
    pub cal_data: Option<String>,
    #[serde(default)]
    pub start: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub length: Option<i64>,
}

/// Represents a time block, which is a scheduled time period for a task or event.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeBlock {
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
    pub date: String,
    pub time: String,
    pub duration: String,
    #[serde(default)]
    pub is_section: Option<bool>,
    #[serde(default)]
    pub cal_id: Option<String>,
    #[serde(default)]
    pub cal_url: Option<String>,
    #[serde(default)]
    pub etag: Option<String>,
    #[serde(default)]
    pub cal_data: Option<String>,
    #[serde(default)]
    pub cancel_dates: HashMap<String, bool>,
    #[serde(default)]
    pub exceptions: HashMap<String, TimeBlockException>,
    #[serde(default)]
    pub recurrence: Option<serde_json::Value>,
    #[serde(default)]
    pub note: Option<String>,
}

/// Represents an exception for a recurring time block.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeBlockException {
    #[serde(default)]
    pub etag: Option<String>,
    #[serde(default)]
    pub cal_data: Option<String>,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub time: Option<String>,
    #[serde(default)]
    pub duration: Option<String>,
}
