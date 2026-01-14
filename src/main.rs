use std::{env, sync::{atomic::AtomicI64, Arc, LazyLock, Mutex, OnceLock}};

use axum::{
    Router,
    routing::get,
};
use toggl_api::client::TogglClient;
use tower_http::cors::{Any, CorsLayer};
use tokio::signal;
use anyhow::anyhow;

mod api;
mod toggl_api;
mod routes; // bring in our `routes` module
mod models;
mod cache;

use cache::cache::{
    TOGGL_CLIENT_CACHE, TOGGL_PROJECT_CACHE, TOGGL_TASK_CACHE, TOGGL_TAG_CACHE,
    cache_put, log_toggl_cache_state,
};

static WORKSPACE_ID: OnceLock<i64> = OnceLock::new();
static LEISURE_BALANCE: AtomicI64 = AtomicI64::new(0);
static LEISURE_RATE: LazyLock<Mutex<f64>> = LazyLock::new(|| {Mutex::new(1.0/3.0)});

#[tokio::main]
async fn main() {
    // Initialize logging (useful for debug)
    tracing_subscriber::fmt::init();

    let toggl_api_token = match env::var("TOGGL_API_TOKEN") {
        Ok(val) => val,
        Err(_) => {
            panic!("TOGGL_API_TOKEN is not set!");
        }
    };

    let toggl_client = TogglClient::new(toggl_api_token, "api_token".to_string());

    // Fetch user data with related data to populate caches
    let result = toggl_client.get_me(Some(true)).await;
    let me_response = match result {
        Ok(result) => result,
        Err(err) => {
            panic!("Could not retrieve user data from toggl: {:#}", anyhow!(err));
        }
    };

    let workspace_id = me_response.default_workspace_id.unwrap();
    WORKSPACE_ID.set(workspace_id).unwrap();

    // Populate caches from the /me response
    println!("Initializing Toggl caches from /me response...");

    // Cache clients
    if let Some(clients) = me_response.clients {
        println!("Caching {} clients", clients.len());
        for client in clients {
            cache_put(Arc::clone(&*TOGGL_CLIENT_CACHE), client.name, client.id);
        }
    }

    // Cache projects (keyed by client_id + name)
    if let Some(projects) = me_response.projects {
        println!("Caching {} projects", projects.len());
        for project in projects {
            if let Some(client_id) = project.client_id {
                cache_put(
                    Arc::clone(&*TOGGL_PROJECT_CACHE),
                    (client_id, project.name),
                    project.id,
                );
            }
        }
    }

    // Cache tasks (keyed by project_id + name)
    if let Some(tasks) = me_response.tasks {
        println!("Caching {} tasks", tasks.len());
        for task in tasks {
            if let Some(project_id) = task.project_id {
                cache_put(
                    Arc::clone(&*TOGGL_TASK_CACHE),
                    (project_id, task.name),
                    task.id,
                );
            }
        }
    }

    // Cache tags
    if let Some(tags) = me_response.tags {
        println!("Caching {} tags", tags.len());
        for tag in tags {
            cache_put(Arc::clone(&*TOGGL_TAG_CACHE), tag.name, tag.id);
        }
    }

    log_toggl_cache_state();

    // Build our application by composing routes
    let app = Router::new()
        .merge(routes::marvin_webhooks::router()) // Our Marvin webhook routes
        .merge(routes::third_time::router()) // Our Third Time webhook routes
    // Example of an entirely different route: 
        .route("/health", get(|| async { "OK" }))
        // Add a CORS layer so Marvin’s client can POST from https://app.amazingmarvin.com
        .layer(
            CorsLayer::new()
                .allow_methods([axum::http::Method::OPTIONS, axum::http::Method::POST, axum::http::Method::GET])
                .allow_headers(Any)
                // If you only want to support the web-based app, do:
                // .allow_origin("https://app.amazingmarvin.com".parse::<HeaderValue>().unwrap())
                // Alternatively, allow all (which also supports desktop/mobile):
                .allow_origin(Any),
        );



    // Run the app with a global listener on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind listener");
    axum::serve(listener, app).with_graceful_shutdown(shutdown_signal()).await.expect("Server failed");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
