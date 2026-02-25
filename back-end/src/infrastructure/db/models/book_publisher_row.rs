use sqlx::FromRow;
use uuid::Uuid;


#[derive(Debug, FromRow)]
pub struct BookPublisherRow {
    pub id: Uuid,
    pub book_id: Uuid,
    pub publisher_id: Uuid,
}