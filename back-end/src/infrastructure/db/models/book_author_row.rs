use sqlx::FromRow;
use uuid::Uuid;


#[derive(Debug, FromRow)]
pub struct BookAuthorRow {
    pub id: Uuid,
    pub book_id: Uuid,
    pub author_id: Uuid,
}