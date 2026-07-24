use sqlx::FromRow;
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, FromRow)]
pub struct UserRefreshTokenRow {
    pub id: Uuid,
    pub token: String,
    pub expire_at: NaiveDateTime
}