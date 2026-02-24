use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct BookRow {
    pub id: Uuid,
    pub title: String,
    pub subtitle: Option<String>,
    pub series_collection: Option<i8>,
    pub volume: Option<i8>,
    pub edition: Option<i8>,
    pub publication_year: i16,
    pub pages: i32,
    pub language: String,
    pub isbn: String,
    pub synopsis: String,
    pub cover: String
}
