use axum::response::Html;
use axum::routing::post;
use axum::Form;
use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use tracing::info;

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


#[derive(Deserialize, Serialize, Debug)]
struct UserForm {
    name: String,
    email: String,
}


async fn handle_form(Form(user_form): Form<UserForm>) -> Html<String> {
    Html(format!(
        r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>Submission Result</title>
                <style>
                    body {{
                        font-family: Arial, sans-serif;
                        max-width: 500px;
                        margin: 50px auto;
                        padding: 20px;
                    }}
                    .info {{
                        background-color: #f8f9fa;
                        padding: 20px;
                        border-radius: 4px;
                        margin-top: 20px;
                    }}
                    .back-link {{
                        display: inline-block;
                        margin-top: 20px;
                        color: #007bff;
                        text-decoration: none;
                    }}
                    .back-link:hover {{
                        text-decoration: underline;
                    }}
                </style>
            </head>
            <body>
                <h1>Submission Received</h1>
                <div class="info">
                    <p><strong>Name:</strong> {}</p>
                    <p><strong>Email:</strong> {}</p>
                </div>
                <a href="/" class="back-link">← Back to Form</a>
            </body>
        </html>
        "#,
        user_form.name, user_form.email
    ))
}

async fn show_form() -> Html<String> {
    Html(
        r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>User Information Form</title>
                <style>
                    body {
                        font-family: Arial, sans-serif;
                        max-width: 500px;
                        margin: 50px auto;
                        padding: 20px;
                    }
                    form {
                        display: flex;
                        flex-direction: column;
                        gap: 10px;
                    }
                    input {
                        padding: 8px;
                        border: 1px solid #ddd;
                        border-radius: 4px;
                    }
                    button {
                        padding: 10px;
                        background-color: #007bff;
                        color: white;
                        border: none;
                        border-radius: 4px;
                        cursor: pointer;
                    }
                    button:hover {
                        background-color: #0056b3;
                    }
                </style>
            </head>
            <body>
                <h1>Enter Your Information</h1>
                <form action="/submit" method="post">
                    <input type="text" name="name" placeholder="Your Name" required>
                    <input type="email" name="email" placeholder="Your Email" required>
                    <button type="submit">Submit</button>
                </form>
            </body>
        </html>
    "#
        .to_string(),
    )
}


pub async fn run(address: SocketAddr) {
    // Create the router and define the routes
    let app = Router::new()
        .route("/", get(show_form))
        .route("/submit", post(handle_form))
        .route("/health_check", get(health_check));
    
    // Create the listener
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .unwrap_or_else(|_| {
            panic!(
                "Failed to bind to address: {}:{}",
                address.ip(),
                address.port()
            )
        });
    info!(
        "Starting server at http://{}:{}",
        address.ip(),
        address.port()
    );
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}
