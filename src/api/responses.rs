use serde::{Deserialize, Serialize};
use crate::models::{
    Task,
    Event,
    Habit,
};

// For many endpoints returning just "OK", we can use a `String` directly.
// But let's define some typed responses if needed.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResponse(pub String); // e.g. "OK"

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeResponse {
    pub email: String,
    // plus all the other fields from the Profile struct if you like
}

/// The GET /api/trackedItem returns a single Task/Project in JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackedItemResponse {
    #[serde(rename = "_id")]
    pub id: String,
    pub db: String,
    pub title: String,
    // possibly more fields
}

/// The GET /api/todayItems or /api/dueItems returns an array of tasks/projects
pub type TodayItemsResponse = Vec<Task>;

// The GET /api/children response is also a list of tasks or projects
pub type ChildrenResponse = Vec<Task>;

// The GET /api/categories is a list of categories (ProjectOrCategory)
pub type CategoriesResponse = Vec<crate::models::ProjectOrCategory>;

// The GET /api/labels is a list of Label
pub type LabelsResponse = Vec<crate::models::Label>;

// The GET /api/reminders returns an array of Reminders
pub type GetRemindersResponse = Vec<crate::models::Reminder>;

// The GET /api/goals returns an array of goals
pub type GoalsResponse = Vec<crate::models::Goal>;

// The GET /api/habits returns an array of habits
pub type HabitsResponse = Vec<Habit>;
