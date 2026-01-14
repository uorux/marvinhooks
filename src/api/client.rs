use reqwest::{Client as HttpClient, StatusCode};
use serde_json::json;
use std::time::Duration;
use tokio::time::sleep;
use crate::api::error::ApiError;
use crate::api::requests::*;
use crate::api::responses::*;
use crate::models::{
    tasks::{Task, ProjectOrCategory},
    calendars::Event,
    habits::Habit,
    reminders::Reminder,
};

/// Maximum number of retries for rate-limited requests
const MAX_RETRIES: u32 = 5;
/// Initial backoff delay in seconds
const INITIAL_BACKOFF_SECS: u64 = 2;

/// The default base URL for Marvin's API.
pub const MARVIN_BASE_URL: &str = "https://serv.amazingmarvin.com/api";

/// The main client struct. 
/// - `api_token` is used for endpoints that only need limited access.
/// - `full_access_token` is used for endpoints that require full access.
#[derive(Debug, Clone)]
pub struct MarvinClient {
    http: HttpClient,
    base_url: String,
    api_token: Option<String>,
    full_access_token: Option<String>,
}

impl MarvinClient {
    /// Create a new MarvinClient with optional tokens. 
    /// If you only need limited access, set `full_access_token` to None.
    pub fn new(
        api_token: Option<String>,
        full_access_token: Option<String>,
    ) -> Self {
        Self {
            http: HttpClient::new(),
            base_url: MARVIN_BASE_URL.to_string(),
            api_token,
            full_access_token,
        }
    }

    /// Override the base URL if needed (e.g. for testing or local dev).
    pub fn with_base_url(mut self, url: &str) -> Self {
        self.base_url = url.to_string();
        self
    }

    //--------------------------------------------------------------------------
    // Utility
    //--------------------------------------------------------------------------

    async fn get<T>(&self, endpoint: &str, query: Option<&[(&str, &str)]>) -> Result<T, ApiError>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        let mut retries = 0;
        let mut backoff_secs = INITIAL_BACKOFF_SECS;

        loop {
            let mut req = self.http.get(&url);

            if let Some(q) = query {
                req = req.query(q);
            }

            // Use whichever token is available, typically the API token
            if let Some(ref token) = self.api_token {
                req = req.header("X-API-Token", token);
            }
            if let Some(ref token) = self.full_access_token {
                req = req.header("X-Full-Access-Token", token);
            }

            let resp = req.send().await?;
            println!("{:#?}", resp);

            if resp.status() == StatusCode::TOO_MANY_REQUESTS {
                if retries >= MAX_RETRIES {
                    println!("[MARVIN] Rate limited, max retries ({}) exceeded", MAX_RETRIES);
                    return Err(ApiError::StatusCodeError(resp.status()));
                }
                println!(
                    "[MARVIN] Rate limited (429), retry {}/{} after {}s backoff",
                    retries + 1,
                    MAX_RETRIES,
                    backoff_secs
                );
                sleep(Duration::from_secs(backoff_secs)).await;
                retries += 1;
                backoff_secs *= 2; // Exponential backoff
                continue;
            }

            if !resp.status().is_success() {
                return Err(ApiError::StatusCodeError(resp.status()));
            }
            let data = resp.json::<T>().await?;
            return Ok(data);
        }
    }

    async fn post_json<Req, Res>(&self, endpoint: &str, body: &Req) -> Result<Res, ApiError>
    where
        Req: serde::Serialize,
        Res: serde::de::DeserializeOwned,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        let mut retries = 0;
        let mut backoff_secs = INITIAL_BACKOFF_SECS;

        loop {
            let mut req = self.http.post(&url).json(body);

            if let Some(ref token) = self.api_token {
                req = req.header("X-API-Token", token);
            }
            if let Some(ref token) = self.full_access_token {
                req = req.header("X-Full-Access-Token", token);
            }

            let resp = req.send().await?;
            println!("{:#?}", resp);

            if resp.status() == StatusCode::TOO_MANY_REQUESTS {
                if retries >= MAX_RETRIES {
                    println!("[MARVIN] Rate limited, max retries ({}) exceeded", MAX_RETRIES);
                    return Err(ApiError::StatusCodeError(resp.status()));
                }
                println!(
                    "[MARVIN] Rate limited (429), retry {}/{} after {}s backoff",
                    retries + 1,
                    MAX_RETRIES,
                    backoff_secs
                );
                sleep(Duration::from_secs(backoff_secs)).await;
                retries += 1;
                backoff_secs *= 2;
                continue;
            }

            if !resp.status().is_success() {
                return Err(ApiError::StatusCodeError(resp.status()));
            }
            let data = resp.json::<Res>().await?;
            return Ok(data);
        }
    }

    // Sometimes responses are just "OK" or a raw string. We'll have a variant:
    async fn post_json_ok<Req>(&self, endpoint: &str, body: &Req) -> Result<String, ApiError>
    where
        Req: serde::Serialize,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        let mut retries = 0;
        let mut backoff_secs = INITIAL_BACKOFF_SECS;

        loop {
            let mut req = self.http.post(&url).json(body);

            if let Some(ref token) = self.api_token {
                req = req.header("X-API-Token", token);
            }
            if let Some(ref token) = self.full_access_token {
                req = req.header("X-Full-Access-Token", token);
            }

            let resp = req.send().await?;
            println!("{:#?}", resp);

            if resp.status() == StatusCode::TOO_MANY_REQUESTS {
                if retries >= MAX_RETRIES {
                    println!("[MARVIN] Rate limited, max retries ({}) exceeded", MAX_RETRIES);
                    return Err(ApiError::StatusCodeError(resp.status()));
                }
                println!(
                    "[MARVIN] Rate limited (429), retry {}/{} after {}s backoff",
                    retries + 1,
                    MAX_RETRIES,
                    backoff_secs
                );
                sleep(Duration::from_secs(backoff_secs)).await;
                retries += 1;
                backoff_secs *= 2;
                continue;
            }

            if !resp.status().is_success() {
                return Err(ApiError::StatusCodeError(resp.status()));
            }
            let text = resp.text().await?;
            return Ok(text);
        }
    }

    //--------------------------------------------------------------------------
    // Endpoints
    //--------------------------------------------------------------------------

    /// Test credentials with /api/test
    pub async fn test_credentials(&self) -> Result<String, ApiError> {
        // POST body is empty or not used:
        let body = TestRequest{};
        self.post_json_ok("test", &body).await
    }

    /// Create a task via /api/addTask
    pub async fn create_task(&self, req: &CreateTaskRequest) -> Result<Task, ApiError> {
        self.post_json("addTask", req).await
    }

    /// Mark a task done via /api/markDone
    pub async fn mark_done(&self, req: &MarkDoneRequest) -> Result<Task, ApiError> {
        self.post_json("markDone", req).await
    }

    /// Create a project via /api/addProject
    pub async fn create_project(&self, req: &CreateProjectRequest) -> Result<ProjectOrCategory, ApiError> {
        self.post_json("addProject", req).await
    }

    /// Create an event via /api/addEvent
    pub async fn create_event(&self, req: &CreateEventRequest) -> Result<Event, ApiError> {
        self.post_json("addEvent", req).await
    }

    /// Read any doc (requires full-access token) via GET /api/doc?id=xyz
    pub async fn read_doc(&self, doc_id: &str) -> Result<MarvinDoc, ApiError> {
        // Must use full-access token here. 
        // We'll assume you used new(...) with full_access_token or you won't succeed.
        let query = &[("id", doc_id)];
        self.get("doc", Some(query)).await
    }

    /// Update any doc (requires full-access token) via POST /api/doc/update
    pub async fn update_doc(&self, req: &UpdateDocRequest) -> Result<MarvinDoc, ApiError> {
        self.post_json("doc/update", req).await
    }

    /// Create any doc (requires full-access token) via POST /api/doc/create
    pub async fn create_doc(&self, req: &CreateDocRequest) -> Result<MarvinDoc, ApiError> {
        self.post_json("doc/create", req).await
    }

    /// Delete any doc (requires full-access token) via POST /api/doc/delete
    pub async fn delete_doc(&self, req: &DeleteDocRequest) -> Result<String, ApiError> {
        self.post_json_ok("doc/delete", req).await
    }

    /// Get the currently tracked task: GET /api/trackedItem
    pub async fn get_tracked_item(&self) -> Result<TrackedItemResponse, ApiError> {
        self.get("trackedItem", None).await
    }

    /// Get child tasks/projects of a category/project: GET /api/children?parentId=XYZ
    pub async fn get_children(&self, parent_id: &str) -> Result<ChildrenResponse, ApiError> {
        let query = &[("parentId", parent_id)];
        self.get("children", Some(query)).await
    }

    /// Get tasks/projects scheduled today: GET /api/todayItems
    /// You can pass a date=YYYY-MM-DD as a query param if desired.
    pub async fn get_today_items(&self, date: Option<&str>) -> Result<TodayItemsResponse, ApiError> {
        let query = date.map(|d| [("date", d)]);
        if let Some(q) = query {
            self.get("todayItems", Some(&q)).await
        } else {
            self.get("todayItems", None).await
        }
    }

    /// Get tasks/projects due by a certain date: GET /api/dueItems
    pub async fn get_due_items(&self, by: Option<&str>) -> Result<Vec<Task>, ApiError> {
        let query = by.map(|d| [("by", d)]);
        if let Some(q) = query {
            self.get("dueItems", Some(&q)).await
        } else {
            self.get("dueItems", None).await
        }
    }

    /// Get a list of today's time blocks: GET /api/todayTimeBlocks?date=YYYY-MM-DD
    pub async fn get_today_time_blocks(&self, date: Option<&str>) -> Result<Vec<crate::models::calendars::TimeBlock>, ApiError> {
        let query = date.map(|d| [("date", d)]);
        if let Some(q) = query {
            self.get("todayTimeBlocks", Some(&q)).await
        } else {
            self.get("todayTimeBlocks", None).await
        }
    }

    /// Get a list of all categories: GET /api/categories
    pub async fn get_categories(&self) -> Result<CategoriesResponse, ApiError> {
        self.get("categories", None).await
    }

    /// Get a list of all labels: GET /api/labels
    pub async fn get_labels(&self) -> Result<LabelsResponse, ApiError> {
        self.get("labels", None).await
    }

    /// Start/stop time tracking: POST /api/track
    pub async fn track(&self, req: &TrackRequest) -> Result<TrackResponse, ApiError> {
        // or alias /api/time
        self.post_json("track", req).await
    }

    /// Getting time track info for tasks: POST /api/tracks
    pub async fn get_tracks(&self, req: &TracksRequest) -> Result<Vec<TrackItem>, ApiError> {
        self.post_json("tracks", req).await
    }

    /// Claim or unclaim or spend reward points: /api/claimRewardPoints, /api/unclaimRewardPoints, /api/spendRewardPoints
    /// We can unify them in a single function or separate them. We'll demonstrate a single function:
    pub async fn claim_reward_points(&self, req: &ClaimRewardPointsRequest) -> Result<MeResponse, ApiError> {
        match req.op.as_str() {
            "CLAIM" => self.post_json("claimRewardPoints", req).await,
            "UNCLAIM" => self.post_json("unclaimRewardPoints", req).await,
            "SPEND" => self.post_json("spendRewardPoints", req).await,
            _ => Err(ApiError::DataError(format!("Invalid op: {}", req.op))),
        }
    }

    /// Reset reward points: /api/resetRewardPoints (requires full access token)
    pub async fn reset_reward_points(&self) -> Result<MeResponse, ApiError> {
        // The request body is empty or minimal
        let body = ResetRewardPointsRequest{};
        self.post_json("resetRewardPoints", &body).await
    }

    /// Get Marvin Kudos info: GET /api/kudos
    pub async fn get_kudos(&self) -> Result<serde_json::Value, ApiError> {
        // The docs show something like { "kudos": 0, "level": 1, "kudosRemaining": 350 }
        // We'll parse as Value or define a typed struct.
        self.get("kudos", None).await
    }

    /// Retrieve account info: GET /api/me
    pub async fn me(&self) -> Result<MeResponse, ApiError> {
        self.get("me", None).await
    }

    /// Get a list of reminders: GET /api/reminders (requires FULL access if you want all).
    pub async fn get_reminders(&self) -> Result<GetRemindersResponse, ApiError> {
        self.get("reminders", None).await
    }

    /// Set (create or update) reminders: POST /api/reminder/set
    pub async fn set_reminders(&self, req: &SetRemindersRequest) -> Result<String, ApiError> {
        self.post_json_ok("reminder/set", req).await
    }

    /// Delete specific reminders: POST /api/reminder/delete
    pub async fn delete_reminders(&self, req: &DeleteRemindersRequest) -> Result<String, ApiError> {
        self.post_json_ok("reminder/delete", req).await
    }

    /// Delete all reminders: POST /api/reminder/deleteAll (requires FULL access token)
    pub async fn delete_all_reminders(&self) -> Result<String, ApiError> {
        let body = DeleteAllRemindersRequest{};
        self.post_json_ok("reminder/deleteAll", &body).await
    }

    /// Get goals: GET /api/goals
    pub async fn get_goals(&self) -> Result<GoalsResponse, ApiError> {
        self.get("goals", None).await
    }

    /// Update a Habit (record, undo, rewrite): /api/updateHabit
    /// The server returns the new habit value or the updated habit object, 
    /// but let's parse it as a full Habit if the server supports that.
    pub async fn update_habit(&self, req: &UpdateHabitRequest) -> Result<Habit, ApiError> {
        self.post_json("updateHabit", req).await
    }

    /// Get a single Habit: GET /api/habit?id=xyz
    pub async fn get_habit(&self, habit_id: &str) -> Result<Habit, ApiError> {
        let query = &[("id", habit_id)];
        self.get("habit", Some(query)).await
    }

    /// List Habits: GET /api/habits
    /// If you pass raw=1, you need a full-access token, but we’ll keep it simple.
    pub async fn get_habits(&self, raw: bool) -> Result<HabitsResponse, ApiError> {
        if raw {
            let query = &[("raw", "1")];
            self.get("habits", Some(query)).await
        } else {
            self.get("habits", None).await
        }
    }
}
