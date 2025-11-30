use axum::{Json, debug_handler};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Vehicle {
    manufacter: String,
    model: String,
    year: u32,
    id: Option<String>,
}

#[debug_handler]
pub async fn vehicle_get() -> Json<Vehicle> {
    println!("Caller retrieved a vehicle from Axum");
    Json::from(Vehicle {
        manufacter: "Fiat".to_string(),
        model: "Panda".to_string(),
        year: 2020,
        id: Some(Uuid::new_v4().to_string()),
    })
}
pub async fn vehicle_post(Json(mut v): Json<Vehicle>) -> Json<Vehicle> {
    println!(
        "Manufacter is {}, model is {} and year is {}",
        v.manufacter, v.model, v.year
    );
    v.id = Some(Uuid::new_v4().to_string());

    Json::from(v)
}
