use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Represents a key-value item in the user's sync database profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileItem {
    #[serde(rename = "_id")]
    pub id: String,
    /// The stored value, which can be any valid JSON type.
    pub val: serde_json::Value,
}

/// Represents the user's central server-side profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    /// The user's unique ID (can exceed 53 bits, so it's often a string).
    #[serde(default)]
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    pub email: String,
    #[serde(default)]
    pub parent_email: Option<String>,
    pub email_confirmed: bool,
    pub billing_period: String,
    #[serde(default)]
    pub paid_through: Option<SystemTime>,
    pub ios_sub: bool,
    pub marvin_points: i64,
    pub next_multiplier: i64,
    pub reward_points_earned: f64,
    pub reward_points_spent: f64,
    pub reward_points_earned_today: f64,
    pub reward_points_spent_today: f64,
    #[serde(default)]
    pub reward_points_last_date: Option<String>,
    pub tomatoes: i64,
    pub tomatoes_today: i64,
    pub tomato_time: i64,
    pub tomato_time_today: i64,
    #[serde(default)]
    pub tomato_date: Option<String>,
    pub default_snooze: i64,
    pub default_auto_snooze: bool,
    pub default_offset: i64,
    #[serde(default)]
    pub tracking: Option<String>,
    #[serde(default)]
    pub tracking_since: Option<i64>,
    #[serde(default)]
    pub current_version: Option<String>,
    #[serde(default)]
    pub signup_app_version: Option<String>,
}
