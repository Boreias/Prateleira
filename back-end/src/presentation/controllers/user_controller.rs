use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::application::services::user_service::UserService;


#[derive(Deserialize)]
pub struct RegisterUserRequest {
    name: String,
    nickname: String,
    email: String,
    password: String,
    avatar: String,
}


#[derive(Serialize)]
pub struct RegisterUserResponse {
    pub message: String,
}


pub async fn register_user(
    Json(RegisterUserRequest { name, nickname, email, password, avatar }): Json<RegisterUserRequest>,
    user_service: State<UserService>,
) -> Result<Json<RegisterUserResponse>, StatusCode> {
    match user_service.register_user(name, nickname, email, password, avatar).await {
        Ok(_) => Ok(Json(RegisterUserResponse { message: "User registered successfully".to_string() })),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}