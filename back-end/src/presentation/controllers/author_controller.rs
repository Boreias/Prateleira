use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::application::services::author_service::AuthorService;
use crate::domain::entities::book::Book;


#[derive(Deserialize)]
pub struct RegisterAuthorRequest {
    name: String,
    avatar: String,
    books: Vec<Book>,
}


#[derive(Serialize)]
pub struct RegisterAuthorResponse {
    pub message: String,
}


pub async fn register_author(
    Json(RegisterAuthorRequest { name, avatar, books }): Json<RegisterAuthorRequest>,
    author_service: State<AuthorService>,
) -> Result<Json<RegisterAuthorResponse>, StatusCode> {
    match author_service.register_author(name, avatar, books).await {
        Ok(_) => Ok(Json(RegisterAuthorResponse { message: "Author registered successfully".to_string() })),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}