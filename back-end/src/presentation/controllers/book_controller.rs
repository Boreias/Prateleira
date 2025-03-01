use axum::{Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::application::services::book_service::BookService;
use crate::domain::entities::authors::Author;
use crate::domain::entities::publisher::Publisher;
use crate::domain::entities::gender::Gender;


#[derive(Deserialize)]
pub struct RegisterBookRequest {
    title: String,
    subtitle: String,
    authors: Vec<Author>,
    publisher: Publisher,
    series_collection: i32,
    volume: i32,
    edition: i32,
    publication_year: i32,
    pages: i32,
    language: String,
    isbn: String,
    synopsis: String,
    cover: String,
    genders: Vec<Gender>
}


#[derive(Serialize)]
pub struct RegisterBookResponse {
    pub message: String,
}


pub async fn register_book(
    Json(RegisterBookRequest { title,
                                subtitle,
                                authors,
                                publisher,
                                series_collection,
                                volume,
                                edition,
                                publication_year,
                                pages,
                                language,
                                isbn,
                                synopsis,
                                cover,
                                genders }): Json<RegisterBookRequest>, book_service: State<BookService>,
) -> Result<Json<RegisterBookResponse>, StatusCode> {
    match book_service.register_book(title,
                                    subtitle,
                                    authors,
                                    publisher,
                                    series_collection,
                                    volume,
                                    edition,
                                    publication_year,
                                    pages,
                                    language,
                                    isbn,
                                    synopsis,
                                    cover,
                                    genders).await {
        Ok(_) => Ok(Json(RegisterBookResponse { message: "Book registered successfully".to_string() })),
        Err(_) => Err(StatusCode::BAD_REQUEST),
    }
}