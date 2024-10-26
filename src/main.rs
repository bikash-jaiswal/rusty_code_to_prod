use axum::{routing::get, Router};

pub async fn ping() -> &'static str {
    "Request listening at https://localhost:3000"
}

pub async fn get_root() -> &'static str {
    "Welcome to by blogs."
}

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    tracing_subscriber::fmt::init();

    // Create the router and define the routes
    let app = Router::new()
        .route("/", get(get_root)) // Correctly register the root route
        .route("/health_check", get(ping)); // Register the health check route
                                            // .route("/", get_root());
                                            // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
