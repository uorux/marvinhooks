use axum::{
    body::Body, http::{Request, StatusCode}, middleware::{self, Next}, response::Response, routing::{get, post}, Json, Router
};
use serde::Deserialize;
use serde_json::Value;
use tokio::time::{sleep, Sleep};
use std::{env, sync::{atomic::Ordering, Arc}, time::Duration};

use crate::{api::{client::MarvinClient, requests::{CreateProjectRequest, CreateTaskRequest}}, cache::cache::{self, cache_get, cache_put, TOGGL_CLIENT_CACHE, TOGGL_PROJECT_CACHE, TOGGL_TASK_CACHE}, models::tasks::{ProjectOrCategory, Task}, toggl_api::{client::{TogglClient, StopCondition}, requests::CreateClientRequest}, LEISURE_BALANCE, LEISURE_RATE, WORKSPACE_ID};

/// Main router for webhooks
pub fn router() -> Router {
    Router::new()
        // Protected endpoints:
        .route("/reset-balance", post(reset_balance))
        .route("/add-balance", post(add_balance))
        .route("/change-rate", post(change_rate))
        .route("/get-balance", get(get_balance))
        .route("/get-rate", get(get_rate))
        .route("/stop-current", get(stop_current))
        .layer(middleware::from_fn(require_auth))
}

/// Check if the request has a valid "Authorization" header that matches
/// the `THIRD_TIME_WEBHOOK_TOKEN` environment variable.
async fn require_auth(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    // Load the secret token from env (once per request). If it's missing from env, 
    // return 500 — or handle this differently, e.g. panic at startup so you know immediately.
    let token = match env::var("THIRD_TIME_WEBHOOK_TOKEN") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("THIRD_TIME_WEBHOOK_TOKEN is not set!");
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


// --------------------
// 2. Request payloads
// --------------------
#[derive(Deserialize)]
struct AddBalanceRequest {
    amount: i64,
}

#[derive(Deserialize)]
struct ChangeRateRequest {
    rate: f64,
}

// --------------------
// 3. Handlers / Endpoints
// --------------------

// POST /reset-balance
async fn reset_balance() -> Result<String, StatusCode> {
    LEISURE_BALANCE.store(0, Ordering::SeqCst);

    Ok(format!("Balance reset to 0"))
}

// POST /add-balance
async fn add_balance(
    Json(payload): Json<AddBalanceRequest>,
) -> Result<String, StatusCode> {
    LEISURE_BALANCE.fetch_add(payload.amount, Ordering::SeqCst);

    Ok(format!("Balance is now {}", LEISURE_BALANCE.load(Ordering::SeqCst).to_string()))
}

// POST /change-rate
async fn change_rate(
    Json(payload): Json<ChangeRateRequest>,
) -> Result<String, StatusCode> {
    let mut rate = LEISURE_RATE.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    *rate = payload.rate;

    Ok(format!("Rate is now {}", *rate))
}

// GET /get-balance
async fn get_balance() -> Result<String, StatusCode> {
    let amount = LEISURE_BALANCE.load(Ordering::SeqCst) / 1000;
    Ok(amount.to_string())
}

// GET /get-rate
async fn get_rate() -> Result<String, StatusCode> {
    let rate = LEISURE_RATE.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(rate.to_string())
}

// GET /get-rate
async fn stop_current() -> Result<String, StatusCode> {
    let toggl_api_token = match env::var("TOGGL_API_TOKEN") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("TOGGL_API_TOKEN is not set!");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let toggl_client = TogglClient::new(toggl_api_token, "api_token".to_string());

    let result = toggl_client
        .stop_current_time_entry(None, StopCondition::Always)
        .await;

    match result {
        Err(error) => println!("Stop current time entry error: {}", error),
        Ok(_) => (),
    }

    Ok("".to_string())
}
