use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::models::habits::{Habit, HabitRecord}; // Import Habit and HabitRecord from habits.rs

/// Represents a webhook payload for editing an object.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookEditPayload<T> {
    /// The old (or updated) version of the object.
    #[serde(flatten)]
    pub old: T,
    /// The set of changes applied to the object.
    #[serde(default)]
    pub setter: Option<HashMap<String, serde_json::Value>>,
}

/// Represents a webhook payload for recording a habit.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookRecordHabitPayload {
    /// The old version of the habit being recorded.
    #[serde(flatten)]
    pub old: Habit,
    /// The record details for the habit.
    #[serde(default)]
    pub record: Option<HabitRecord>,
}
