use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct PublisherRow {
    pub id: i32,
    pub name: String,
    pub site: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
}