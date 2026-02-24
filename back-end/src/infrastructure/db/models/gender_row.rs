use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct GenderRow {
    pub id: Uuid,
    pub name: String,
}