use axum::{
    Json, Router,
    body::Body,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::post,
};
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::{env, sync::Arc, time::Duration};
use tokio::time::{Sleep, sleep};

use crate::{
    WORKSPACE_ID,
    api::{
        client::MarvinClient,
        requests::{CreateProjectRequest, CreateTaskRequest},
    },
    cache::cache::{
        self, TOGGL_CLIENT_CACHE, TOGGL_PROJECT_CACHE, TOGGL_TASK_CACHE, cache_get, cache_put,
        log_toggl_cache_state,
    },
    models::tasks::{ProjectOrCategory, Task},
    toggl_api::{
        client::TogglClient,
        requests::{CreateClientRequest, CreateTagRequest},
    },
};

/// Removes timestamp prefixes like "11:55 am" or "6:10 pm" from the beginning of task names.
/// Examples:
///   "11:55 am blah blah blah" -> "blah blah blah"
///   "6:10 pm Week 1: ISA design" -> "Week 1: ISA design"
fn remove_timestamp_prefix(text: &str) -> String {
    // Pattern matches: digits:digits followed by optional space and am/pm, then a space
    let re = Regex::new(r"^\d{1,2}:\d{2}\s*(?:am|pm|AM|PM)\s+").unwrap();
    re.replace(text, "").to_string()
}

/// Resolved Toggl IDs from a Marvin task hierarchy.
#[derive(Debug, Clone)]
struct ResolvedTogglIds {
    client_id: Option<i64>,
    project_id: Option<i64>,
    task_id: Option<i64>,
    description: String,
    tags: Vec<i64>,
}

/// Resolves a Marvin Task to Toggl IDs by walking the parent hierarchy.
/// If `create_if_missing` is true, creates missing clients/projects/tasks in Toggl.
/// If false, returns None for IDs that don't exist.
async fn resolve_marvin_task_to_toggl(
    payload: &Task,
    marvin_client: &MarvinClient,
    toggl_client: &TogglClient,
    workspace_id: i64,
    create_if_missing: bool,
) -> Result<ResolvedTogglIds, StatusCode> {
    println!(
        "=== resolve_marvin_task_to_toggl ===\nTask: '{}'\nParent ID: '{}'\ncreate_if_missing: {}",
        payload.title, payload.parent_id, create_if_missing
    );
    log_toggl_cache_state();

    // Walk parent hierarchy to collect parent names
    let mut parent_id = payload.parent_id.clone();
    let mut parents: Vec<String> = vec![];

    while parent_id != "root" && parent_id != "unassigned" {
        let parent = cache::cache_get(Arc::clone(&*cache::MARVIN_PROJECT_CACHE), &parent_id);
        let parent = match parent {
            Some(parent) => parent,
            None => match marvin_client.read_doc(&parent_id).await {
                Ok(doc) => {
                    let title = match doc.extra.get("title") {
                        Some(Value::String(title)) => title.clone(),
                        _ => return Err(StatusCode::SERVICE_UNAVAILABLE),
                    };
                    let parent = match doc.extra.get("parentId") {
                        Some(Value::String(parent)) => parent.clone(),
                        _ => return Err(StatusCode::SERVICE_UNAVAILABLE),
                    };
                    (title, parent)
                }
                Err(err) => {
                    println!("{}", err);
                    return Err(StatusCode::SERVICE_UNAVAILABLE);
                }
            },
        };
        cache::cache_put(
            Arc::clone(&*cache::MARVIN_PROJECT_CACHE),
            parent_id,
            parent.clone(),
        );
        parents.push(remove_timestamp_prefix(&parent.0));
        parent_id = parent.1;
        sleep(Duration::from_secs(1)).await;
    }

    println!("Parent hierarchy (len={}): {:?}", parents.len(), parents);

    // Collect tags from labels
    let mut tags: Vec<i64> = vec![];
    for id in &payload.label_ids {
        let label = cache::cache_get(Arc::clone(&*cache::MARVIN_LABEL_CACHE), id);
        let label = match label {
            Some(label) => label,
            None => {
                let mut result: String = "".to_string();
                sleep(Duration::from_secs(1)).await;
                match marvin_client.get_labels().await {
                    Ok(labels) => {
                        for l in labels {
                            if *id == l.id {
                                result = l.title.to_string();
                            }
                            cache::cache_put(
                                Arc::clone(&*cache::MARVIN_LABEL_CACHE),
                                l.id,
                                l.title,
                            );
                        }
                    }
                    Err(err) => {
                        println!("Error collecting labels: {}", err);
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                }
                result
            }
        };
        if label.is_empty() {
            continue;
        }
        let tag = cache::cache_get(Arc::clone(&*cache::TOGGL_TAG_CACHE), &label);
        let tag = match tag {
            Some(tag) => tag,
            None => {
                let mut result: i64 = -1;
                sleep(Duration::from_secs(1)).await;
                match toggl_client.list_tags(workspace_id).await {
                    Ok(existing_tags) => {
                        for t in existing_tags {
                            if label == t.name {
                                result = t.id;
                            }
                            cache::cache_put(Arc::clone(&*cache::TOGGL_TAG_CACHE), t.name, t.id);
                        }
                    }
                    Err(err) => {
                        println!("Error collecting tags: {}", err);
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                }

                if result == -1 && create_if_missing {
                    let tag_request = CreateTagRequest { name: label.clone() };
                    match toggl_client.create_tag(workspace_id, &tag_request).await {
                        Ok(tag) => {
                            result = tag.id;
                            cache::cache_put(Arc::clone(&*cache::TOGGL_TAG_CACHE), tag.name, tag.id);
                        }
                        Err(err) => {
                            println!("Error adding tag: {}", err);
                            return Err(StatusCode::INTERNAL_SERVER_ERROR);
                        }
                    }
                }
                result
            }
        };
        if tag != -1 {
            tags.push(tag);
        }
    }

    // No parents - just description, no project
    if parents.is_empty() {
        return Ok(ResolvedTogglIds {
            client_id: None,
            project_id: None,
            task_id: None,
            description: remove_timestamp_prefix(payload.title.trim()),
            tags,
        });
    }

    // Determine client/project/task names based on hierarchy depth
    let (client_name, project_name, task_name): (&str, &str, Option<&str>) = match parents.len() {
        1 => (&parents[0], &parents[0], None),
        2 => (&parents[1], &parents[0], None),
        3 => (&parents[2], &parents[1], Some(&parents[0])),
        _ => (
            &parents[parents.len() - 1],
            &parents[parents.len() - 2],
            Some(&parents[0]),
        ),
    };

    let client_name = remove_timestamp_prefix(client_name.trim());
    let project_name = remove_timestamp_prefix(project_name.trim());
    let task_name = task_name.map(|t| remove_timestamp_prefix(t.trim()));
    let description = remove_timestamp_prefix(payload.title.trim());

    println!(
        "Resolved names -> client: '{}', project: '{}', task: {:?}, description: '{}'",
        client_name, project_name, task_name, description
    );

    // Resolve client ID
    let client_id = match cache_get(Arc::clone(&*TOGGL_CLIENT_CACHE), &client_name) {
        Some(id) => Some(id),
        None => {
            let clients = match toggl_client.list_clients(workspace_id, None, None).await {
                Ok(clients) => clients,
                Err(error) => {
                    println!("Error fetching clients {}", error);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
            };
            let mut found_id: Option<i64> = None;
            for c in clients {
                if client_name == c.name {
                    found_id = Some(c.id);
                }
                cache_put(Arc::clone(&*TOGGL_CLIENT_CACHE), c.name, c.id);
            }
            match found_id {
                Some(id) => Some(id),
                None if create_if_missing => {
                    let request = &CreateClientRequest {
                        name: client_name.clone(),
                        notes: None,
                    };
                    match toggl_client.create_client(workspace_id, request).await {
                        Ok(c) => Some(c.id),
                        Err(error) => {
                            println!("Error creating client {}", error);
                            return Err(StatusCode::INTERNAL_SERVER_ERROR);
                        }
                    }
                }
                None => None,
            }
        }
    };

    // Resolve project ID (requires client_id)
    let project_id = match client_id {
        Some(cid) => {
            match cache_get(Arc::clone(&*TOGGL_PROJECT_CACHE), &(cid, project_name.clone())) {
                Some(id) => Some(id),
                None => {
                    let projects = match toggl_client.list_projects(workspace_id).await {
                        Ok(projects) => projects,
                        Err(error) => {
                            println!("Error fetching projects {}", error);
                            return Err(StatusCode::INTERNAL_SERVER_ERROR);
                        }
                    };
                    let mut found_id: Option<i64> = None;
                    for p in projects {
                        if project_name == p.name && p.client_id == Some(cid) {
                            found_id = Some(p.id);
                        }
                        if let Some(pcid) = p.client_id {
                            cache_put(Arc::clone(&*TOGGL_PROJECT_CACHE), (pcid, p.name), p.id);
                        }
                    }
                    match found_id {
                        Some(id) => Some(id),
                        None if create_if_missing => {
                            let mut request: crate::toggl_api::requests::CreateProjectRequest =
                                Default::default();
                            request.active = Some(true);
                            request.auto_estimates = Some(false);
                            request.billable = Some(false);
                            request.color = Some("#ffffff".to_string());
                            request.is_private = Some(true);
                            request.name = project_name.clone();
                            request.client_id = Some(cid);
                            match toggl_client.create_project(workspace_id, &request).await {
                                Ok(p) => Some(p.id),
                                Err(error) => {
                                    println!("Error creating project {}", error);
                                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                                }
                            }
                        }
                        None => None,
                    }
                }
            }
        }
        None => None,
    };

    // Resolve task ID (requires project_id)
    let task_id = match (project_id, &task_name) {
        (Some(pid), Some(tname)) => {
            match cache_get(Arc::clone(&*TOGGL_TASK_CACHE), &(pid, tname.clone())) {
                Some(id) => Some(id),
                None => {
                    let tasks = match toggl_client.get_project_tasks(workspace_id, pid).await {
                        Ok(tasks) => tasks,
                        Err(error) => {
                            println!("Error fetching tasks {}", error);
                            return Err(StatusCode::INTERNAL_SERVER_ERROR);
                        }
                    };
                    let mut found_id: Option<i64> = None;
                    for t in tasks {
                        if *tname == t.name {
                            found_id = Some(t.id);
                        }
                        cache_put(Arc::clone(&*TOGGL_TASK_CACHE), (pid, t.name), t.id);
                    }
                    match found_id {
                        Some(id) => Some(id),
                        None if create_if_missing => {
                            let request = &crate::toggl_api::requests::CreateTaskRequest {
                                active: Some(true),
                                estimated_seconds: Some(0),
                                name: tname.clone(),
                                user_id: None,
                            };
                            match toggl_client.create_task(workspace_id, pid, request).await {
                                Ok(t) => Some(t.id),
                                Err(error) => {
                                    println!("Error creating task {}", error);
                                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                                }
                            }
                        }
                        None => None,
                    }
                }
            }
        }
        _ => None,
    };

    println!(
        "=== Resolved Toggl IDs ===\nclient_id: {:?}\nproject_id: {:?}\ntask_id: {:?}\ndescription: '{}'\ntags: {:?}",
        client_id, project_id, task_id, description, tags
    );

    Ok(ResolvedTogglIds {
        client_id,
        project_id,
        task_id,
        description,
        tags,
    })
}

/// Main router for Marvin webhooks.
pub fn router() -> Router {
    Router::new()
        // Protected endpoints:
        .route("/start-tracking", post(start_tracking))
        .route("/stop-tracking", post(stop_tracking))
        .route("/marvin-other", post(other_webhook))
        // Attach our auth layer to every route in this router.
        .layer(middleware::from_fn(require_auth))
}

/// Check if the request has a valid "Authorization" header that matches
/// the `MARVIN_WEBHOOK_TOKEN` environment variable.
async fn require_auth(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    // Load the secret token from env (once per request). If it's missing from env,
    // return 500 — or handle this differently, e.g. panic at startup so you know immediately.
    let token = match env::var("MARVIN_WEBHOOK_TOKEN") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("MARVIN_WEBHOOK_TOKEN is not set!");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Check the Authorization header
    eprintln!("Headers: {:#?}", req.headers());
    let auth_header = req.headers().get("Authorization");
    match auth_header {
        Some(header_value) if header_value == token.as_str() => {
            // Correct token => allow the request to continue
            Ok(next.run(req).await)
        }
        _ => {
            eprintln!("Unauthorized webhook attempt");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

/// Primary endpoint that routes based on `webhook_type`.
async fn start_tracking(Json(payload): Json<Task>) -> Result<String, StatusCode> {
    println!("Webhook Called");

    let marvin_api_token = match env::var("MARVIN_API_TOKEN") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("MARVIN_API_TOKEN is not set!");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let marvin_full_access_token = match env::var("MARVIN_FULL_ACCESS_TOKEN") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("MARVIN_FULL_ACCESS_TOKEN is not set!");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let toggl_api_token = match env::var("TOGGL_API_TOKEN") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("TOGGL_API_TOKEN is not set!");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let workspace_id = match WORKSPACE_ID.get() {
        Some(workspace_id) => *workspace_id,
        None => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let toggl_client = TogglClient::new(toggl_api_token, "api_token".to_string());
    let marvin_client = MarvinClient::new(Some(marvin_api_token), Some(marvin_full_access_token));

    // Resolve task to Toggl IDs, creating missing entities
    let resolved = resolve_marvin_task_to_toggl(
        &payload,
        &marvin_client,
        &toggl_client,
        workspace_id,
        true, // create_if_missing
    )
    .await?;

    println!("Tags: {:#?}", resolved.tags);

    // Stop any currently running entry
    let result = toggl_client.stop_current_time_entry(None).await;
    match result {
        Err(error) => println!("Stop current time entry error: {}", error),
        Ok(_) => (),
    }

    // Start new time entry
    match toggl_client
        .start_time_entry(
            workspace_id,
            resolved.project_id,
            resolved.task_id,
            &resolved.description,
            resolved.tags,
        )
        .await
    {
        Err(error) => {
            println!("Start time entry error: {}", error);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        Ok(_) => (),
    }

    Ok("Webhook processed successfully".to_string())
}

/// Primary endpoint that routes based on `webhook_type`.
async fn stop_tracking(Json(payload): Json<Task>) -> Result<String, StatusCode> {
    println!("Webhook Called");

    let marvin_api_token = match env::var("MARVIN_API_TOKEN") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("MARVIN_API_TOKEN is not set!");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let marvin_full_access_token = match env::var("MARVIN_FULL_ACCESS_TOKEN") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("MARVIN_FULL_ACCESS_TOKEN is not set!");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let toggl_api_token = match env::var("TOGGL_API_TOKEN") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("TOGGL_API_TOKEN is not set!");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let workspace_id = match WORKSPACE_ID.get() {
        Some(workspace_id) => *workspace_id,
        None => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let toggl_client = TogglClient::new(toggl_api_token, "api_token".to_string());
    let marvin_client = MarvinClient::new(Some(marvin_api_token), Some(marvin_full_access_token));

    // Check if there's a current time entry
    let current_entry = match toggl_client.get_current_time_entry().await {
        Ok(Some(entry)) => entry,
        Ok(None) => {
            println!("No current time entry running, nothing to stop");
            return Ok("No current time entry".to_string());
        }
        Err(error) => {
            println!("Error fetching current time entry: {}", error);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Resolve task to Toggl IDs (without creating missing entities)
    let resolved = resolve_marvin_task_to_toggl(
        &payload,
        &marvin_client,
        &toggl_client,
        workspace_id,
        false, // don't create_if_missing
    )
    .await?;

    // Compare project_id and description
    let current_description = current_entry.description.as_deref().unwrap_or("");
    let project_matches = current_entry.project_id == resolved.project_id;
    let description_matches = current_description == resolved.description;

    if !project_matches || !description_matches {
        println!(
            "Current time entry does not match task. Project match: {}, Description match: {} (current: '{}', expected: '{}')",
            project_matches, description_matches, current_description, resolved.description
        );
        return Ok("Current time entry does not match task".to_string());
    }

    println!("Time entry matches task, proceeding with stop");

    // Collect labels for productivity override
    let mut labels: Vec<String> = vec![];
    for id in &payload.label_ids {
        let label = cache::cache_get(Arc::clone(&*cache::MARVIN_LABEL_CACHE), id);
        let label = match label {
            Some(label) => label,
            None => {
                let mut result: String = "".to_string();
                sleep(Duration::from_secs(1)).await;
                match marvin_client.get_labels().await {
                    Ok(fetched_labels) => {
                        for l in fetched_labels {
                            if *id == l.id {
                                result = l.title.to_string();
                            }
                            cache::cache_put(
                                Arc::clone(&*cache::MARVIN_LABEL_CACHE),
                                l.id,
                                l.title,
                            );
                        }
                    }
                    Err(err) => {
                        println!("Error collecting labels: {}", err);
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                }
                result
            }
        };
        if !label.is_empty() {
            labels.push(label);
        }
    }

    println!("Labels: {:#?}", labels);

    let mut productivity_override = None;
    for label in labels {
        if label == "productiveOverride" {
            productivity_override = Some(true);
        }
        if label == "unproductiveOverride" {
            productivity_override = Some(false);
        }
    }

    let result = toggl_client
        .stop_current_time_entry(productivity_override)
        .await;

    match result {
        Err(error) => println!("Stop current time entry error: {}", error),
        Ok(_) => (),
    }

    Ok("Webhook processed successfully".to_string())
}

/// A second example endpoint that doesn’t do any type-based routing.
async fn other_webhook(Json(payload): Json<Value>) -> Result<String, StatusCode> {
    Ok("Other webhook processed".to_string())
}
