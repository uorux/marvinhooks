use axum::{
    Json, Router,
    body::Body,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::post,
};
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
    },
    models::tasks::{ProjectOrCategory, Task},
    toggl_api::{
        client::TogglClient,
        requests::{CreateClientRequest, CreateTagRequest},
    },
};

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

    let mut parent_id = payload.parent_id;
    let mut parents: Vec<String> = vec![];

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

    let toggl_client = TogglClient::new(toggl_api_token, "api_token".to_string());

    // TODO: remove localhost override
    let marvin_client = MarvinClient::new(Some(marvin_api_token), Some(marvin_full_access_token));

    while parent_id != "root" && parent_id != "unassigned" {
        // Get project from cache
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
        parents.push(parent.0);
        parent_id = parent.1;
        // Make the marvin rate limiter happy by waiting 1 second
        sleep(Duration::from_secs(1)).await;
    }

    let workspace_id = match WORKSPACE_ID.get() {
        Some(workspace_id) => workspace_id,
        None => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let mut labels: Vec<String> = vec![];

    // Collect all labels for passthrough
    let mut tags: Vec<i64> = vec![];
    for id in payload.label_ids {
        let label = cache::cache_get(Arc::clone(&*cache::MARVIN_LABEL_CACHE), &id);
        let label = match label {
            Some(label) => label,
            None => {
                let mut result: String = "".to_string();
                // Make the marvin rate limiter happy by waiting 1 second
                sleep(Duration::from_secs(1)).await;
                match marvin_client.get_labels().await {
                    Ok(labels) => {
                        for l in labels {
                            if id == l.id {
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
        if label == "" {
            continue;
        }
        let tag = cache::cache_get(Arc::clone(&*cache::TOGGL_TAG_CACHE), &label);
        let tag = match tag {
            Some(tag) => tag,
            None => {
                let mut result: i64 = -1;
                // Make the marvin rate limiter happy by waiting 1 second
                sleep(Duration::from_secs(1)).await;
                match toggl_client.list_tags(*workspace_id).await {
                    Ok(tags) => {
                        for l in tags {
                            if label == l.name {
                                result = l.id;
                            }
                            cache::cache_put(Arc::clone(&*cache::TOGGL_TAG_CACHE), l.name, l.id);
                        }
                    }
                    Err(err) => {
                        println!("Error collecting tags: {}", err);
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                }

                if result == -1 {
                    let tag_request = CreateTagRequest {
                        name: label.clone(),
                    };
                    match toggl_client.create_tag(*workspace_id, &tag_request).await {
                        Ok(tag) => {
                            result = tag.id;
                            cache::cache_put(
                                Arc::clone(&*cache::TOGGL_TAG_CACHE),
                                tag.name,
                                tag.id,
                            );
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

    println!("Tags: {:#?}", tags);

    // Length 0: description is what is being done
    // Length 1: client [0], project [0], task is what is being done
    // Length 2: client [1], project [1], task is what is being done
    // Length 3: client [1], project [2], task is what is being done
    // Length 4: client [1], project [2], task [3], description is what is being done

    if parents.len() > 0 {
        let mut client = &parents[0];
        let mut project = &parents[0];

        let mut task = None;
        let description = &payload.title;
        if parents.len() == 1 {
            client = &parents[0];
            project = &parents[0];
        }
        if parents.len() == 2 {
            client = &parents[1];
            project = &parents[0];
        }
        if parents.len() == 3 {
            task = Some(&parents[0]);
            project = &parents[1];
            client = &parents[2];
        }
        if parents.len() > 3 {
            task = Some(&parents[0]);
            project = &parents[parents.len() - 2];
            client = &parents[parents.len() - 1];
        }

        let client = &str::trim(client).to_string();
        let project = &str::trim(project).to_string();
        let task = task.map(|x| str::trim(x).to_string());
        let description = &str::trim(description).to_string();

        let client = match cache_get(Arc::clone(&*TOGGL_CLIENT_CACHE), &client) {
            Some(client) => client,
            None => {
                // List of clients and use that; if not, create our own
                let clients = match toggl_client.list_clients(*workspace_id, None, None).await {
                    Ok(clients) => clients,
                    Err(error) => {
                        println!("Error fetching clients {}", error);
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                };
                let mut our_client: Option<i64> = None;
                for returned_client in clients {
                    if *client == returned_client.name {
                        our_client = Some(returned_client.id);
                    }
                    cache_put(
                        Arc::clone(&*TOGGL_CLIENT_CACHE),
                        returned_client.name,
                        returned_client.id,
                    );
                }
                match our_client {
                    Some(client) => client,
                    None => {
                        // Create our own
                        let request = &CreateClientRequest {
                            name: client.to_string(),
                            notes: None,
                        };
                        let result = toggl_client.create_client(*workspace_id, request).await;
                        match result {
                            Ok(client) => client.id,
                            Err(error) => {
                                println!("Error creating client {}", error);
                                return Err(StatusCode::INTERNAL_SERVER_ERROR);
                            }
                        }
                    }
                }
            }
        };

        let project = match cache_get(
            Arc::clone(&*TOGGL_PROJECT_CACHE),
            &(client, (*project).clone()),
        ) {
            Some(project) => project,
            None => {
                // List of clients and use that; if not, create our own
                let projects = match toggl_client.list_projects(*workspace_id).await {
                    Ok(clients) => clients,
                    Err(error) => {
                        println!("Error fetching projects {}", error);
                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                    }
                };
                let mut our_project: Option<i64> = None;
                for returned_project in projects {
                    if *project == returned_project.name
                        && returned_project.client_id == Some(client)
                    {
                        our_project = Some(returned_project.id);
                    }
                    match returned_project.client_id {
                        Some(client) => cache_put(
                            Arc::clone(&*TOGGL_PROJECT_CACHE),
                            (client, returned_project.name),
                            returned_project.id,
                        ),
                        None => (),
                    };
                }
                match our_project {
                    Some(project) => project,
                    None => {
                        // Create our own
                        let mut request: crate::toggl_api::requests::CreateProjectRequest =
                            Default::default();
                        request.active = Some(true);
                        request.auto_estimates = Some(false);
                        request.billable = Some(false);
                        request.color = Some("#ffffff".to_string());
                        request.is_private = Some(true);
                        request.name = (*project).clone();
                        request.client_id = Some(client);
                        let result = toggl_client.create_project(*workspace_id, &request).await;
                        match result {
                            Ok(client) => client.id,
                            Err(error) => {
                                println!("Error creating project {}", error);
                                return Err(StatusCode::INTERNAL_SERVER_ERROR);
                            }
                        }
                    }
                }
            }
        };

        let task = match task {
            Some(task) => {
                match cache_get(Arc::clone(&*TOGGL_TASK_CACHE), &(project, (task).clone())) {
                    Some(task) => Some(task),
                    None => {
                        // List of clients and use that; if not, create our own
                        let tasks =
                            match toggl_client.get_project_tasks(*workspace_id, project).await {
                                Ok(clients) => clients,
                                Err(error) => {
                                    println!("Error fetching tasks {}", error);
                                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                                }
                            };
                        let mut our_task: Option<i64> = None;
                        for returned_task in tasks {
                            if task == returned_task.name {
                                our_task = Some(returned_task.id);
                            }
                            cache_put(
                                Arc::clone(&*TOGGL_TASK_CACHE),
                                (project, returned_task.name),
                                returned_task.id,
                            );
                        }
                        match our_task {
                            Some(task) => Some(task),
                            None => {
                                // Create our own
                                let request = &crate::toggl_api::requests::CreateTaskRequest {
                                    active: Some(true),
                                    estimated_seconds: Some(0),
                                    name: (task).clone(),
                                    user_id: None,
                                };
                                let result = toggl_client
                                    .create_task(*workspace_id, project, request)
                                    .await;
                                match result {
                                    Ok(task) => Some(task.id),
                                    Err(error) => {
                                        println!("Error creating tasks {}", error);
                                        return Err(StatusCode::INTERNAL_SERVER_ERROR);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            None => None,
        };

        let result = toggl_client.stop_current_time_entry(None).await;

        match result {
            Err(error) => println!("Stop current time entry error: {}", error),
            Ok(_) => (),
        }

        match toggl_client
            .start_time_entry(*workspace_id, Some(project), task, description, tags)
            .await
        {
            Err(error) => {
                println!("Start time entry error: {}", error);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
            Ok(_) => (),
        }
    } else {
        let result = toggl_client.stop_current_time_entry(None).await;

        match result {
            Err(error) => println!("Stop current time entry error: {}", error),
            Ok(_) => (),
        }

        match toggl_client
            .start_time_entry(*workspace_id, None, None, payload.title.as_str(), tags)
            .await
        {
            Err(error) => {
                println!("start time entry error: {}", error);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
            Ok(_) => (),
        }
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

    let toggl_client = TogglClient::new(toggl_api_token, "api_token".to_string());

    let marvin_client = MarvinClient::new(Some(marvin_api_token), Some(marvin_full_access_token));

    // Collect all labels for passthrough
    let mut labels: Vec<String> = vec![];
    for id in payload.label_ids {
        let label = cache::cache_get(Arc::clone(&*cache::MARVIN_LABEL_CACHE), &id);
        let label = match label {
            Some(label) => label,
            None => {
                let mut result: String = "".to_string();
                // Make the marvin rate limiter happy by waiting 1 second
                sleep(Duration::from_secs(1)).await;
                match marvin_client.get_labels().await {
                    Ok(labels) => {
                        for l in labels {
                            if id == l.id {
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
        if label != "" {
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
