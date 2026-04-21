use sqlx::FromRow;
use uuid::Uuid;


#[derive(Debug, FromRow)]
pub struct PublisherRow {
    pub id: Uuid,
    pub name: String,
    pub site: Option<String>,
    pub email: Option<String>
}