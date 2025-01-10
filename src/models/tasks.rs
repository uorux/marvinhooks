use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Subtask {
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
    pub done: bool,
    pub rank: i64,
    #[serde(default)]
    pub time_estimate: Option<i64>,
    #[serde(default)]
    pub task_time: Option<String>,
    #[serde(default)]
    pub reminder_offset: Option<i64>,
    #[serde(default)]
    pub reminder_time: Option<i64>,
    #[serde(default)]
    pub snooze: Option<i64>,
    #[serde(default)]
    pub auto_snooze: Option<bool>,
    #[serde(default)]
    pub remind_at: Option<String>,
    #[serde(default)]
    pub reminder: Option<ReminderInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReminderInfo {
    pub time: String,
    pub diff: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    #[serde(rename = "_id")]
    pub id: String,
    pub created_at: i64,
    pub updated_at: i64,
    #[serde(default)]
    pub worked_on_at: Option<i64>,
    pub title: String,
    pub parent_id: String,
    #[serde(default)]
    pub due_date: Option<String>,
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
    pub day: String,
    #[serde(default)]
    pub first_scheduled: Option<String>,
    #[serde(default)]
    pub planned_week: Option<String>,
    #[serde(default)]
    pub planned_month: Option<String>,
    #[serde(default)]
    pub sprint_id: Option<String>,
    #[serde(default)]
    pub rank: Option<i64>,
    #[serde(default)]
    pub master_rank: Option<i64>,
    #[serde(default)]
    pub done: Option<bool>,
    #[serde(default)]
    pub completed_at: Option<i64>,
    #[serde(default)]
    pub duration: Option<i64>,
    #[serde(default)]
    pub times: Vec<i64>,
    #[serde(default)]
    pub first_tracked: Option<i64>,
    #[serde(default)]
    pub done_at: Option<i64>,
    #[serde(default)]
    pub is_reward: Option<bool>,
    #[serde(default)]
    pub is_starred: Option<bool>,
    #[serde(default)]
    pub is_frogged: Option<bool>,
    #[serde(default)]
    pub is_pinned: Option<bool>,
    #[serde(default)]
    pub pin_id: Option<String>,
    #[serde(default)]
    pub recurring: Option<bool>,
    #[serde(default)]
    pub recurring_task_id: Option<String>,
    #[serde(default)]
    pub echo: Option<bool>,
    #[serde(default)]
    pub echo_id: Option<String>,
    #[serde(default)]
    pub link: Option<String>,
    #[serde(default)]
    pub subtasks: HashMap<String, Subtask>,
    #[serde(default)]
    pub color_bar: Option<String>,
    #[serde(default)]
    pub label_ids: Vec<String>,
    #[serde(default)]
    pub time_estimate: Option<i64>,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub daily_section: Option<String>,
    #[serde(default)]
    pub bonus_section: Option<String>,
    #[serde(default)]
    pub custom_section: Option<String>,
    #[serde(default)]
    pub time_block_section: Option<String>,
    #[serde(default)]
    pub depends_on: HashMap<String, bool>,
    #[serde(default)]
    pub backburner: Option<bool>,
    #[serde(default)]
    pub review_date: Option<String>,
    #[serde(default)]
    pub item_snooze_time: Option<i64>,
    #[serde(default)]
    pub perma_snooze_time: Option<String>,
    #[serde(default)]
    pub cal_id: Option<String>,
    #[serde(default)]
    pub cal_url: Option<String>,
    #[serde(default)]
    pub etag: Option<String>,
    #[serde(default)]
    pub cal_data: Option<String>,
    #[serde(default)]
    pub generated_at: Option<i64>,
    #[serde(default)]
    pub echoed_at: Option<i64>,
    #[serde(default)]
    pub deleted_at: Option<i64>,
    #[serde(default)]
    pub restored_at: Option<i64>,
    #[serde(default)]
    pub onboard: Option<bool>,
    #[serde(default)]
    pub imported: Option<bool>,
    #[serde(default)]
    pub marvin_points: Option<i64>,
    #[serde(default)]
    pub mp_notes: Option<Vec<String>>,
    #[serde(default)]
    pub reward_points: Option<i64>,
    #[serde(default)]
    pub reward_id: Option<i64>,
    #[serde(default)]
    pub task_time: Option<String>,
    #[serde(default)]
    pub reminder_offset: Option<i64>,
    #[serde(default)]
    pub reminder_time: Option<i64>,
    #[serde(default)]
    pub snooze: Option<i64>,
    #[serde(default)]
    pub auto_snooze: Option<bool>,
    #[serde(default)]
    pub remind_at: Option<String>,
    #[serde(default)]
    pub reminder: Option<ReminderInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectOrCategory {
    #[serde(rename = "_id")]
    pub id: String,
    pub title: String,
    pub r#type: String,
    pub updated_at: i64,
    #[serde(default)]
    pub worked_on_at: Option<i64>,
    pub parent_id: String,
    #[serde(default)]
    pub rank: Option<i64>,
    #[serde(default)]
    pub day_rank: Option<i64>,
    #[serde(default)]
    pub day: Option<String>,
    #[serde(default)]
    pub first_scheduled: Option<String>,
    #[serde(default)]
    pub due_date: Option<String>,
    #[serde(default)]
    pub label_ids: Vec<String>,
    #[serde(default)]
    pub time_estimate: Option<i64>,
    #[serde(default)]
    pub start_date: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
    #[serde(default)]
    pub planned_week: Option<String>,
    #[serde(default)]
    pub planned_month: Option<String>,
    #[serde(default)]
    pub sprint_id: Option<String>,
    #[serde(default)]
    pub done: Option<bool>,
    #[serde(default)]
    pub done_date: Option<String>,
    #[serde(default)]
    pub priority: Option<String>,
    #[serde(default)]
    pub color: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub recurring: Option<bool>,
    #[serde(default)]
    pub recurring_task_id: Option<String>,
    #[serde(default)]
    pub echo: Option<bool>,
    #[serde(default)]
    pub is_frogged: Option<bool>,
    #[serde(default)]
    pub review_date: Option<String>,
    #[serde(default)]
    pub marvin_points: Option<i64>,
    #[serde(default)]
    pub mp_notes: Option<Vec<String>>,
}


/// RecurringTask documents
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecurringTask {
    #[serde(rename = "_id")]
    pub id: String,
    /// "task", "sequence" (deprecated), or "project"
    pub recurring_type: String,
    pub title: String,
    #[serde(default)]
    pub parent_id: Option<String>,
    /// e.g. "daily", "weekly", "monthly", "n per week", ...
    pub r#type: String,
    #[serde(default)]
    pub day: Option<i64>,
    #[serde(default)]
    pub date: Option<i64>,
    #[serde(default)]
    pub week_days: Option<Vec<i64>>,
    #[serde(default)]
    pub repeat: Option<i64>,
    #[serde(default)]
    pub repeat_start: Option<String>,
    #[serde(default)]
    pub limit_to_weekdays: Option<bool>,

    /// Subtasks in a recurring task
    #[serde(default)]
    pub subtask_list: Option<Vec<Task>>, // or Vec<Subtask> if you want a subtask-only approach

    #[serde(default)]
    pub section: Option<String>,
    #[serde(default)]
    pub time_estimate: Option<i64>,
    #[serde(default)]
    pub label_ids: Option<Vec<String>>,
    #[serde(default)]
    pub due_in: Option<i64>,
    #[serde(default)]
    pub auto_plan: Option<String>,
    #[serde(default)]
    pub echo_days: Option<i64>,
    #[serde(default)]
    pub on_count: Option<i64>,
    #[serde(default)]
    pub off_count: Option<i64>,
    #[serde(default)]
    pub custom_recurrence: Option<String>,
    #[serde(default)]
    pub end_date: Option<String>,
}

/// SavedItem documents (Templates).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedItem {
    #[serde(rename = "_id")]
    pub id: String,
    /// "task", "taskGroup", or "project"
    pub item_type: String,
    pub title: String,
    pub rank: i64,
    /// Usually length=1 if item_type=="task"
    pub tasks: Vec<Task>,
    #[serde(default)]
    pub default_parent_id: Option<String>,
}

/// Represents a Goal that habits, tasks, and projects can be attached to.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Goal {
    /// The goal's unique ID.
    #[serde(rename = "_id")]
    pub id: String,
    /// The goal's title (e.g., "Reach 10MRR").
    pub title: String,
    /// A note for the goal.
    pub note: String,
    /// If true, the goal is hidden from the daily view.
    #[serde(default)]
    pub hide_in_day_view: Option<bool>,
    /// A category or parent ID for grouping goals.
    #[serde(default)]
    pub parent_id: Option<String>,
    /// The goal's priority or importance, indicated by starring.
    #[serde(default)]
    pub is_starred: Option<i64>,
    /// Associated label IDs for the goal.
    #[serde(default)]
    pub label_ids: Vec<String>,
    /// Importance of the goal, rated 1-5 stars.
    #[serde(default)]
    pub importance: Option<i64>,
    /// Difficulty of the goal, rated 1-5 stars.
    #[serde(default)]
    pub difficulty: Option<i64>,
    /// Motivations for pursuing the goal.
    #[serde(default)]
    pub motivations: Vec<String>,
    /// Anticipated challenges for the goal.
    #[serde(default)]
    pub challenges: Vec<Challenge>,
    /// Indicates if the user has committed to the goal.
    #[serde(default)]
    pub committed: Option<bool>,
    /// Expected number of tasks to complete each week.
    #[serde(default)]
    pub expected_tasks: Option<i64>,
    /// Expected number of minutes to work on this goal each week.
    #[serde(default)]
    pub expected_duration: Option<i64>,
    /// Expected habit success level (e.g., "B-").
    #[serde(default)]
    pub expected_habits: Option<String>,
    /// True if check-ins are enabled for the goal.
    #[serde(default)]
    pub check_in: Option<bool>,
    /// Array of numbers representing check-in data.
    #[serde(default)]
    pub check_ins: Vec<i64>,
    /// The last check-in date (formatted as "YYYY-MM-DD").
    #[serde(default)]
    pub last_check_in: Option<String>,
    /// Number of weeks between check-ins.
    #[serde(default)]
    pub check_in_weeks: Option<i64>,
    /// Start date for check-ins (formatted as "YYYY-MM-DD").
    #[serde(default)]
    pub check_in_start: Option<String>,
    /// Questions to answer during a check-in.
    #[serde(default)]
    pub check_in_questions: Vec<CheckInQuestion>,
    /// The status of the goal: "backburner", "pending", "active", "done".
    pub status: String,
    /// Timestamp for when the goal became active. Falls back to `createdAt`.
    #[serde(default)]
    pub started_at: Option<i64>,
    /// The color associated with the goal.
    #[serde(default)]
    pub color: Option<String>,
    /// The due date for achieving the goal (formatted as "YYYY-MM-DD").
    #[serde(default)]
    pub due_date: Option<String>,
    /// Whether this is an "end goal" (true) or ongoing (false).
    #[serde(default)]
    pub has_end: Option<bool>,
    /// Sections associated with the goal.
    #[serde(default)]
    pub sections: Vec<GoalSection>,
    /// If true, task/project completion contributes to goal progress.
    #[serde(default)]
    pub task_progress: Option<bool>,
    /// Trackers contributing to progress calculation.
    #[serde(flatten)]
    pub tracker_progress: Option<HashMap<String, bool>>,
}

/// Represents an anticipated challenge for achieving a goal.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Challenge {
    /// Unique ID for the challenge.
    #[serde(rename = "_id")]
    pub id: String,
    /// Description of the challenge.
    pub challenge: String,
    /// The action the user will take if the challenge arises.
    pub action: String,
}

/// Represents a section within a goal, which can organize trackers, habits, and tasks/projects.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalSection {
    /// The section's unique ID.
    #[serde(rename = "_id")]
    pub id: String,
    /// The section's title.
    pub title: String,
    /// A note associated with the section.
    pub note: String,
}

/// Represents a question answered during a goal check-in.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckInQuestion {
    /// Unique ID for the question.
    #[serde(rename = "_id")]
    pub id: String,
    /// The question's title or content.
    pub title: String,
}
