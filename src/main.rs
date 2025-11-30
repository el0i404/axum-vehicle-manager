use axum::routing::{get, post};

mod vehicle;

#[tokio::main]

async fn main() {
    //1. Create Axum router
    let router = axum::Router::new()
        .route("/vehicle", get(vehicle::vehicle_get))
        .route("/vehicle", post(vehicle::vehicle_post));
    //2. Define the IP and port listener (TCP)
    let address = "127.0.0.1:5580";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    //3. Axum serve to lunch to App
    axum::serve(listener, router).await.unwrap();
}
