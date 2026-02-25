use sqlx::FromRow;
use uuid::Uuid;


#[derive(Debug, FromRow)]
pub struct BookGenderRow {
    pub id: Uuid,
    pub book_id: Uuid,
    pub gender_id: Uuid,
}