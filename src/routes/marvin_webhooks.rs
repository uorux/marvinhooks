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
use std::env;

use crate::handlers::marvin::{handle_marvin_webhook_type, handle_other_webhook};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct WebhookRequest {
    #[serde(rename = "type")]
    webhook_type: String,
    #[serde(flatten)]
    data: Value,
}

/// Main router for Marvin webhooks.
pub fn router() -> Router {
    Router::new()
        // Protected endpoints:
        .route("/marvin-webhook", post(marvin_webhook))
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
async fn marvin_webhook(Json(payload): Json<WebhookRequest>) -> Result<String, StatusCode> {
    handle_marvin_webhook_type(&payload.webhook_type, &payload.data)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok("Webhook processed successfully".to_string())
}

/// A second example endpoint that doesn’t do any type-based routing.
async fn other_webhook(Json(payload): Json<Value>) -> Result<String, StatusCode> {
    handle_other_webhook(&payload)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    Ok("Other webhook processed".to_string())
}
