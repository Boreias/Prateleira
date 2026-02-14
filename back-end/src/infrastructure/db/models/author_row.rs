use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct AuthorRow {
    pub id: i32,
    pub name: String,
    pub avatar: String,
}