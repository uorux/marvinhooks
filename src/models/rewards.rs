use serde::{Deserialize, Serialize};

/// Represents a reward item with details for gamification or achievements.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reward {
    #[serde(rename = "_id")]
    pub id: String,
    #[serde(default)]
    pub is_archived: Option<bool>,
    pub rank: i64,
    #[serde(default)]
    pub group_id: Option<String>,
    pub title: String,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub is_pinned: Option<bool>,
    /// Points required to claim the reward.
    #[serde(default)]
    pub reward_points: Option<i64>,
    /// Timestamps of when the reward was earned.
    #[serde(default)]
    pub earned: Vec<i64>,
    /// Timestamps of when the reward was spent.
    #[serde(default)]
    pub spent: Vec<i64>,
}
