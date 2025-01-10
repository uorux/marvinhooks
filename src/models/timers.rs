use serde::{Deserialize, Serialize};

/// Represents a standard timer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timer {
    /// Time elapsed on the timer (in milliseconds).
    pub elapsed: i64,
    /// Progress of the timer as a percentage (0.0 to 1.0).
    pub progress: f64,
    /// Total duration of the timer (in milliseconds).
    pub duration: i64,
    #[serde(default)]
    pub task_id: Option<String>,
    #[serde(default)]
    pub beep_count: Option<i64>,
    /// Indicates whether the timer has completed.
    pub done: bool,
    /// An optional ID for the timer.
    #[serde(default)]
    pub timer_id: Option<i64>,
    /// Indicates if the timer requires user confirmation when it ends.
    #[serde(default)]
    pub needs_confirmation: Option<bool>,
}

/// Represents a tomato (pomodoro) timer with work and break cycles.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TomatoTimer {
    /// Time elapsed on the tomato timer (in milliseconds).
    pub elapsed: i64,
    /// Progress of the timer as a percentage (0.0 to 1.0).
    pub progress: f64,
    /// Work duration of the tomato timer (in milliseconds).
    pub work_duration: i64,
    /// Break duration of the tomato timer (in milliseconds).
    pub break_duration: i64,
    /// The current cycle of work/break.
    pub cycle: i64,
    /// Total number of cycles to repeat.
    pub repeat: i64,
    #[serde(default)]
    pub beep_count: Option<i64>,
    /// Indicates if the timer is in the work phase (true) or break phase (false).
    pub is_work: bool,
    /// Indicates whether the tomato timer has completed.
    pub done: bool,
    #[serde(default)]
    pub timer_id: Option<i64>,
    #[serde(default)]
    pub task_id: Option<String>,
    /// Indicates if the timer requires confirmation after completing a cycle.
    #[serde(default)]
    pub needs_confirmation: Option<bool>,
}
