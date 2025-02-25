use crate::toggl_api::error::TogglError;
use crate::toggl_api::requests::*;
use crate::toggl_api::responses::*;
use crate::LEISURE_BALANCE;
use crate::LEISURE_RATE;
use chrono::DateTime;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use reqwest::{Client as HttpClient, Method, StatusCode};
use std::sync::atomic::Ordering;
use std::time::SystemTime;

/// The default base URL for Toggl Track API v9.
pub const TOGGL_BASE_URL: &str = "https://api.track.toggl.com/api/v9";

#[derive(Debug, Clone)]
pub struct TogglClient {
    http: HttpClient,
    base_url: String,
    /// Typically your Toggl username (which could be your email or API token)
    username: String,
    /// Your Toggl password or "api_token" if you're using an API token as the username
    password: String,
}

impl TogglClient {
    /// Create a new TogglClient. Provide your Toggl login (username) and password.
    /// If using an API token, pass the token as `username`, and "api_token" as `password`.
    pub fn new(username: String, password: String) -> Self {
        Self {
            http: HttpClient::new(),
            base_url: TOGGL_BASE_URL.to_string(),
            username,
            password,
        }
    }

    /// Override the base URL if needed (for testing or custom environments).
    pub fn with_base_url(mut self, url: &str) -> Self {
        self.base_url = url.to_string();
        self
    }

    //--------------------------------------------------------------------------
    // Utility methods (GET, POST, etc.)
    //--------------------------------------------------------------------------

    async fn get_json<T>(&self, endpoint: &str) -> Result<T, TogglError>
    where
        T: serde::de::DeserializeOwned,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        let req = self
            .http
            .request(Method::GET, &url)
            .basic_auth(&self.username, Some(&self.password));
        let resp = req.send().await?;
        println!("{:#?}", resp);
        if !resp.status().is_success() {
            return Err(TogglError::StatusCodeError(resp.status()));
        }
        Ok(resp.json::<T>().await?)
    }

    async fn get_json_with_query<T, Q>(&self, endpoint: &str, query: &Q) -> Result<T, TogglError>
    where
        T: serde::de::DeserializeOwned,
        Q: serde::Serialize,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        let req = self
            .http
            .request(Method::GET, &url)
            .basic_auth(&self.username, Some(&self.password))
            .query(query);
        let resp = req.send().await?;
        println!("{:#?}", resp);
        if !resp.status().is_success() {
            return Err(TogglError::StatusCodeError(resp.status()));
        }
        Ok(resp.json::<T>().await?)
    }

    async fn post_json<Rq, Rs>(&self, endpoint: &str, body: &Rq) -> Result<Rs, TogglError>
    where
        Rq: serde::Serialize,
        Rs: serde::de::DeserializeOwned,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        let req = self
            .http
            .request(Method::POST, &url)
            .basic_auth(&self.username, Some(&self.password))
            .json(body);
        println!("{:#?}", req);
        let resp = req.send().await?;
        println!("{:#?}", resp);
        if !resp.status().is_success() {
            println!("{}", resp.text().await.unwrap());
            return Err(TogglError::StatusCodeError(StatusCode::INTERNAL_SERVER_ERROR));
            //return Err(TogglError::StatusCodeError(resp.status()));
        }
        Ok(resp.json::<Rs>().await?)
    }

    /// A generic helper for PATCH requests with no request body, returning a JSON response.
    async fn patch_json_no_body<Rs>(&self, endpoint: &str) -> Result<Rs, TogglError>
    where
        Rs: serde::de::DeserializeOwned,
    {
        let url = format!("{}/{}", self.base_url, endpoint);

        let req = self
            .http
            .request(Method::PATCH, &url)
            .basic_auth(&self.username, Some(&self.password));

        let resp = req.send().await?;
        println!("{:#?}", resp);
        if !resp.status().is_success() {
            return Err(TogglError::StatusCodeError(resp.status()));
        }
        Ok(resp.json::<Rs>().await?)
    }



    //--------------------------------------------------------------------------
    // Endpoint methods
    //--------------------------------------------------------------------------

    /// GET /api/v9/me
    /// Returns details for the current user.
    /// Optionally pass `with_related_data` if you want clients/projects/time entries, etc.
    pub async fn get_me(&self, with_related_data: Option<bool>) -> Result<MeResponse, TogglError> {
        let mut endpoint = "me".to_string();
        if let Some(wrd) = with_related_data {
            // We can add a query param: ?with_related_data=true
            // Alternatively, build a query struct and call get_json_with_query
            // For simplicity, we'll do manual string concatenation:
            if wrd {
                endpoint.push_str("?with_related_data=true");
            }
        }
        self.get_json(&endpoint).await
    }

    /// Focus method: Create a time entry that is "active" (running now) in the given workspace.
    /// That means set `duration` to -1 and `start` to the current time in UTC.
    /// Optionally, you can provide project/task info. The request struct is flexible.
    ///
    /// Example usage:
    /// ```ignore
    /// let entry = client
    ///    .start_time_entry(123456, Some(99999), None, "My test entry")
    ///    .await?;
    /// ```
    pub async fn start_time_entry(
        &self,
        workspace_id: i64,
        project_id: Option<i64>,
        task_id: Option<i64>,
        description: &str,
        tags: Vec<i64>,
    ) -> Result<TimeEntry, TogglError> {
        // Prepare a "now" in UTC, properly formatted
        let now_utc = chrono::Utc::now().to_rfc3339();

        let body = CreateTimeEntryRequest {
            billable: Some(false),
            created_with: "MarvinWebhook".to_string(),
            description: Some(description.to_string()),
            duration: -1, // negative => running
            duronly: None,
            event_metadata: None,
            pid: None,
            project_id,
            shared_with_user_ids: None,
            start: now_utc,
            start_date: None,
            stop: None, // no stop => it's running
            tag_action: Some("add".to_string()),
            tag_ids: Some(tags),
            tags: None,
            task_id,
            tid: None,
            user_id: None,
            workspace_id,
        };

        // POST /workspaces/{workspace_id}/time_entries
        let endpoint = format!("workspaces/{}/time_entries", workspace_id);

        self.post_json(&endpoint, &body).await
    }

    /// Create a client in the specified workspace.
    /// POST /api/v9/workspaces/{workspace_id}/clients
    pub async fn create_client(
        &self,
        workspace_id: i64,
        req: &CreateClientRequest,
    ) -> Result<crate::toggl_api::responses::TogglClient, TogglError> {
        let endpoint = format!("workspaces/{}/clients", workspace_id);
        self.post_json(&endpoint, req).await
    }

    /// List all clients in a workspace.
    /// GET /api/v9/workspaces/{workspace_id}/clients
    pub async fn list_clients(
        &self,
        workspace_id: i64,
        status: Option<&str>,
        name_filter: Option<&str>,
    ) -> Result<Vec<crate::toggl_api::responses::TogglClient>, TogglError> {
        // Build query parameters for status and name if needed
        #[derive(Serialize)]
        struct QueryParams<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            status: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<&'a str>,
        }

        let query = QueryParams {
            status,
            name: name_filter,
        };

        let endpoint = format!("workspaces/{}/clients", workspace_id);
        self.get_json_with_query(&endpoint, &query).await
    }

    /// Create a project in the specified workspace.
    /// POST /api/v9/workspaces/{workspace_id}/projects
    pub async fn create_project(
        &self,
        workspace_id: i64,
        req: &CreateProjectRequest,
    ) -> Result<TogglProject, TogglError> {
        let endpoint = format!("workspaces/{}/projects", workspace_id);
        self.post_json(&endpoint, req).await
    }

    /// Get a list of projects in the workspace.
    /// GET /api/v9/workspaces/{workspace_id}/projects
    pub async fn list_projects(
        &self,
        workspace_id: i64,
    ) -> Result<Vec<TogglProject>, TogglError> {
        let endpoint = format!("workspaces/{}/projects", workspace_id);
        self.get_json(&endpoint).await
    }

    /// Get tasks for a given project.
    /// GET /api/v9/workspaces/{workspace_id}/projects/{project_id}/tasks
    pub async fn get_project_tasks(
        &self,
        workspace_id: i64,
        project_id: i64,
    ) -> Result<Vec<TogglTask>, TogglError> {
        let endpoint = format!(
            "workspaces/{}/projects/{}/tasks",
            workspace_id, project_id
        );
        self.get_json(&endpoint).await
    }

    /// Create a task for a given project.
    /// POST /api/v9/workspaces/{workspace_id}/projects/{project_id}/tasks
    pub async fn create_task(
        &self,
        workspace_id: i64,
        project_id: i64,
        req: &CreateTaskRequest,
    ) -> Result<TogglTask, TogglError> {
        let endpoint = format!(
            "workspaces/{}/projects/{}/tasks",
            workspace_id, project_id
        );
        self.post_json(&endpoint, req).await
    }

    pub async fn stop_time_entry(
        &self,
        workspace_id: i64,
        time_entry_id: i64,
    ) -> Result<TimeEntry, TogglError> {
        let endpoint = format!(
            "workspaces/{}/time_entries/{}/stop",
            workspace_id, time_entry_id
        );
        self.patch_json_no_body(&endpoint).await
    }

    pub async fn get_current_time_entry(&self) -> Result<Option<TimeEntry>, TogglError> {
        let endpoint = "me/time_entries/current";
        let url = format!("{}/{}", self.base_url, endpoint);

        let req = self
            .http
            .request(Method::GET, &url)
            .basic_auth(&self.username, Some(&self.password));

        let resp = req.send().await?;


        match resp.status() {
            StatusCode::OK => {
                // Attempt to deserialize the time entry
                let te = resp.json::<TimeEntry>().await?;
                Ok(Some(te))
            }
            StatusCode::NOT_FOUND => {
                // Means no current time entry is running
                Ok(None)
            }
            s if s.is_success() => {
                // Possibly a 200 with "null" => handle gracefully
                let maybe_te = resp.json::<Option<TimeEntry>>().await?;
                Ok(maybe_te)
            }
            _ => Err(TogglError::StatusCodeError(resp.status())),
        }
    }

    pub async fn stop_current_time_entry(&self) -> Result<Option<TimeEntry>, TogglError> {
        // 1) Find current time entry
        let current_te_opt = self.get_current_time_entry().await?;

        // 2) If None, nothing is running
        let current_te = match current_te_opt {
            Some(te) => te,
            None => return Ok(None), // No current entry
        };

        // Update third time count: don't update if neutral | no tags, add if productive, remove if unproductive
        // Third time rate should be an envvar so I don't need to rebuild
        match current_te.tags {
            Some(tags) => {
                let target: DateTime<Utc> = current_te.start
                    .parse()
                    .expect("Invalid datetime format");
                // Get the current time in UTC.
                let now: DateTime<Utc> = Utc::now();
                // Compute the difference as a chrono Duration.
                let diff = target.signed_duration_since(now);
                // Convert the difference to milliseconds.
                let diff_ms = diff.num_milliseconds();
                let rate = *LEISURE_RATE.lock().unwrap();
                let time_change = -1.0 * rate * diff_ms as f64;
                for tag in tags {
                    if tag == "productive" {
                        LEISURE_BALANCE.fetch_add(time_change as i64, Ordering::SeqCst);
                    } else if tag == "unproductive" {
                        LEISURE_BALANCE.fetch_sub(time_change as i64, Ordering::SeqCst);
                    }
                }
            },
            None => ()
        }


        // 3) Extract workspace
        let ws_id = match current_te.workspace_id {
            Some(id) => id,
            None => {
                // Some older field might store 'wid' (if toggl_responses::TimeEntry uses `wid`).
                // If you stored it under .wid or .workspace_id, pick whichever is correct.
                // If missing altogether, you'd need to track it externally or handle error.
                return Err(TogglError::DataError(
                    "Current time entry has no workspace_id".to_string(),
                ));
            }
        };

        // 4) Call stop_time_entry
        let stopped_te = self.stop_time_entry(ws_id, current_te.id).await?;
        Ok(Some(stopped_te))
    }
}
