use axum::{http::StatusCode, response::IntoResponse, Json, routing::get, Router};
use std::net::SocketAddr;
use tracing::info;
use std::collections::HashMap;
use chrono::prelude::*;

pub async fn health_check() -> impl IntoResponse {
    info!("loading http://{}", "http://127.0.0.1:3000/health_check");
    // Creating JSON response with status and timestamp
    let mut response: HashMap<&str, String> = HashMap::new();
    response.insert("status", "ok".to_string());
    response.insert("timestamp", Utc::now().to_utc().to_rfc2822());
    (StatusCode::OK, Json(response))
}

pub async fn get_root() -> &'static str {
    "Welcome to by blogs."
}

pub async fn run(address: SocketAddr) {
    // Create the router and define the routes
    let app = Router::new()
        .route("/", get(get_root)) // Correctly register the root route
        .route("/health_check", get(health_check)); // Register the health check route
                                                    
    // Create the listener
    let listener = tokio::net::TcpListener::bind(address).await.expect(&format!("Failed to bind to address: {}:{}", address.ip(), address.port()));
    info!("Starting server at http://{}:{}", address.ip(), address.port());
    axum::serve(listener, app).await.expect("Server failed to start");
}
