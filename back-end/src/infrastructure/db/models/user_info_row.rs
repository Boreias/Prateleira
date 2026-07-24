use sqlx::FromRow;
use uuid::Uuid;
use chrono::NaiveDate;


#[derive(Debug, FromRow)]
pub struct UserInfoRow {
    pub id: Uuid,
    pub name: String,
    pub bio: Option<String>,
    pub birth_date: NaiveDate,
    pub avatar: Option<String>,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate
}