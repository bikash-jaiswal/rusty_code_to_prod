use reqwest::Client;
use zero2prod;

async fn spawn_app() -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    // Pass the SocketAddr to your run function
    let server = zero2prod::run(listener.local_addr().unwrap());
    
    // Spawn the server in the background
    tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn test_health_check() {
    let address = spawn_app().await;
    let client = Client::new();

    let response = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("Failed to send request");

    println!("Response: {:?}", response);
    assert!(response.status().is_success());
    assert_eq!(Some(61), response.content_length());
}