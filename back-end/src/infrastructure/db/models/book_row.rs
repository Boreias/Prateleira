use sqlx::FromRow;
use uuid::Uuid;


#[derive(Debug, FromRow)]
pub struct BookRow {
    pub id: Uuid,
    pub title: String,
    pub subtitle: Option<String>,
    pub publisher_id: Uuid,
    pub series_collection: Option<i32>,
    pub volume: Option<i32>,
    pub edition: Option<i32>,
    pub publication_year: Option<i32>,
    pub pages: Option<i32>,
    pub language: Option<String>,
    pub isbn: String,
    pub synopsis: Option<String>
}
