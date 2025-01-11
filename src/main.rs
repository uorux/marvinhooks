use axum::{
    Router,
    routing::get,
};
use tower_http::cors::{Any, CorsLayer};
use tokio::signal;

mod api;
mod routes; // bring in our `routes` module
mod models;
mod cache;

#[tokio::main]
async fn main() {
    // Initialize logging (useful for debug)
    tracing_subscriber::fmt::init();

    // Build our application by composing routes
    let app = Router::new()
        .merge(routes::marvin_webhooks::router()) // Our Marvin webhook routes
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
