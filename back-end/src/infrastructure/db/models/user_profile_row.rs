use sqlx::FromRow;
use uuid::Uuid;
use chrono::NaiveDate;


#[derive(Debug, FromRow)]
pub struct UserProfileRow {
    pub id: Uuid,
    pub name: String,
    pub bio: Option<String>,
    pub birth_date: NaiveDate
}