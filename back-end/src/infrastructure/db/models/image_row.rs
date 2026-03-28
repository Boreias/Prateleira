use sqlx::FromRow;
use uuid::Uuid;


#[derive(Debug, FromRow, Clone)]
pub struct ImageRow {
    pub id: Uuid,
    pub original_name: String,
    pub image_path: String
}