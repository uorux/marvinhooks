use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::Response,
    Json, Router,
    routing::post,
};
use serde::Deserialize;
use serde_json::Value;
use tokio::time::{sleep, Sleep};
use std::{env, sync::Arc, time::Duration};

use crate::{cache::cache, models::tasks::{ProjectOrCategory, Task}, api::client::MarvinClient};

/// Main router for Marvin webhooks.
pub fn router() -> Router {
    Router::new()
        // Protected endpoints:
        .route("/start-tracking", post(start_tracking))
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


    // TODO: remove localhost override
    let marvin_client = MarvinClient::new(Some(marvin_api_token), Some(marvin_full_access_token)).with_base_url("http://localhost:12082/api");


    while parent_id != "root" && parent_id != "unassigned" {
        // Get project from cache
        let parent = cache::cache_get(Arc::clone(&*cache::MARVIN_PROJECT_CACHE), &parent_id);
        let parent = match parent {
            Some(parent) => parent,
            None => {
                match marvin_client.read_doc(&parent_id).await {
                    Ok(doc) => {
                        let title = match doc.extra.get("title") {
                            Some(Value::String(title)) => title.clone(),
                            _ => {
                                return Err(StatusCode::SERVICE_UNAVAILABLE)
                            }
                        };
                        let parent = match doc.extra.get("parentId") {
                            Some(Value::String(parent)) => parent.clone(),
                            _ => {
                                return Err(StatusCode::SERVICE_UNAVAILABLE)
                            }
                        };
                        (title, parent)
                    },
                    Err(err) => {
                        println!("{}", err);
                        return Err(StatusCode::SERVICE_UNAVAILABLE)
                    },
                }
            }
        };
        cache::cache_put(Arc::clone(&*cache::MARVIN_PROJECT_CACHE), parent_id, parent.clone());
        parents.push(parent.0);
        parent_id = parent.1;
        // Make the marvin rate limiter happy by waiting 1 second
        sleep(Duration::from_secs(1)).await;
    }

    for parent in parents {
        println!("{:?}", parent);
    }

    // Grab full parent tree (from cache)
    // Select items we want
    // Construct toggl tree
    // Start tracking on toggl

    Ok("Webhook processed successfully".to_string())
}

/// Primary endpoint that routes based on `webhook_type`.
async fn stop_tracking(Json(payload): Json<Task>) -> Result<String, StatusCode> {
    println!("Webhook Called");

    Ok("Webhook processed successfully".to_string())
}


/// A second example endpoint that doesn’t do any type-based routing.
async fn other_webhook(Json(payload): Json<Value>) -> Result<String, StatusCode> {
    Ok("Other webhook processed".to_string())
}
