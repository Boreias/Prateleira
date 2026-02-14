use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct GenderRow {
    pub id: i32,
    pub name: String,
}