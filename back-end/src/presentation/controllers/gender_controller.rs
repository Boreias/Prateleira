use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::application::services::gender_service::GenderService;


#[derive(Deserialize)]
pub struct RegisterGenderRequest {
    name: String,
}


#[derive(Serialize)]
pub struct RegisterGenderResponse {
    pub message: String,
}


pub async fn register_gender(
    Json(RegisterGenderRequest { name }): Json<RegisterGender>,
    gender_service: State<GenderService>,
) -> Result<Json<RegisterGenderResponse>, StatusCode> {
    match gender_service.register_gender(name).await {
        Ok(_) => Ok(Json(RegisterGenderResponse { message: "Gender registered successfully".to_string()})),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}