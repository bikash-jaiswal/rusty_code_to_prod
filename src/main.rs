use zero2prod::run;
use std::net::SocketAddr;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    run(address).await;
    
}
