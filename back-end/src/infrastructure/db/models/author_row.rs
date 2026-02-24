use sqlx::FromRow;
use uuid::Uuid;


#[derive(Debug, FromRow)]
pub struct AuthorRow {
    pub id: Uuid,
    pub name: String,
    pub avatar: String,
}