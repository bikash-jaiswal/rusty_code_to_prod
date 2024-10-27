
use reqwest::Client;


#[tokio::test]
async fn test_health_check() {
    spawn_app().await;
    // Create router with health check endpoint
    let client = Client::new();

    let response = client
        .get("http://localhost:3000/health_check")
        .send()
        .await
        .expect("Failed to send request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
    // assert_eq!(Some(0), response.content_length());
    let body = response.text().await.expect("Failed to read response text");
    assert_eq!(body, "OK");
    
}


async fn spawn_app() {
    zero2prod::run().await;
}