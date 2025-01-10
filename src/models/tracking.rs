use serde::{Deserialize, Serialize};

/// Represents a tracker for measuring progress or metrics over time.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tracker {
    /// The title of the tracker.
    pub title: String,
    /// Type of the tracker: "number" or "rating".
    pub tracker_type: String,
    #[serde(default)]
    pub start_value: Option<f64>,
    #[serde(default)]
    pub target_value: Option<f64>,
    #[serde(default)]
    pub is_cumulative: Option<bool>,
    #[serde(default)]
    pub units: Option<String>,
    #[serde(default)]
    pub min_rating: Option<f64>,
    #[serde(default)]
    pub max_rating: Option<f64>,
    #[serde(default)]
    pub min_label: Option<String>,
    #[serde(default)]
    pub max_label: Option<String>,
    #[serde(default)]
    pub symbol: Option<String>,
    #[serde(default)]
    pub due_date: Option<String>,
    /// The type of asking for tracker updates (e.g., "n per week", "monthly").
    #[serde(default)]
    pub ask_type: Option<String>,
    #[serde(default)]
    pub ask_on: Option<Vec<i64>>,
    #[serde(default)]
    pub ask_date: Option<i64>,
    #[serde(default)]
    pub ask_weekdays: Option<bool>,
    #[serde(default)]
    pub show_after_record: Option<bool>,
    #[serde(default)]
    pub start_time: Option<String>,
    #[serde(default)]
    pub end_time: Option<String>,
    #[serde(default)]
    pub show_as: Option<String>,
    /// Array of [time1, value1, time2, value2, ...] for tracking historical data.
    #[serde(default)]
    pub history: Vec<f64>,
}
