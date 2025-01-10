use axum::{
    Router,
    routing::get,
};
use tower_http::cors::{Any, CorsLayer};

mod routes; // bring in our `routes` module
mod handlers;
mod models;

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
    axum::serve(listener, app).await.expect("Server failed");
}
