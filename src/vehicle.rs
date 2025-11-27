use axum::{Json, debug_handler};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Vehicle {
    manufacter: String,
    model: String,
    year: u32,
    id: String,
}

#[debug_handler]
pub async fn vehicle_get() -> Json<Vehicle> {
    println!("Caller retrieved a vehicle from Axum");
    Json::from(Vehicle {
        manufacter: "Fiat".to_string(),
        model: "Panda".to_string(),
        year: 2020,
        id: Uuid::new_v4().to_string(),
    })
}
pub async fn vehicle_post() {}
