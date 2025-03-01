use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::application::services::publisher_service::PublisherService;



#[derive(Deserialize)]
pub struct RegisterPublisherRequest {
    name: String,
    avatar: String,
}


#[derive(Serialize)]
pub struct RegisterPublisherResponse {
    pub message: String,
}



pub async fn register_publisher(
    Json(RegisterPublisherRequest { name, avatar }): Json<RegisterPublisherRequest>,
    publisher_service: State<PublisherService>,
) -> Result<Json<RegisterPublisherResponse>, StatusCode> {
    match publisher_service.register_publisher(name, avatar).await {
        Ok(_) => Ok(Json(RegisterPublisherResponse { message: "Publisher registered successfully".to_string()})),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}