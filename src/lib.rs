use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use std::net::SocketAddr;
use log::info;

pub async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

pub async fn get_root() -> &'static str {
    "Welcome to by blogs."
}

pub async fn run() {
    // Create the router and define the routes
    let app = Router::new()
        .route("/", get(get_root)) // Correctly register the root route
        .route("/health_check", get(health_check)); // Register the health check route
                                                    // .route("/", get_root());
                                                    // run our app with hyper, listening globally on port 3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    // Create the listener
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("Server listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
