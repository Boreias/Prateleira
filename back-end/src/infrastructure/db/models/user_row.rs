use sqlx::FromRow;
use uuid::Uuid;
use chrono::NaiveDate;

#[derive(Debug, FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub name: String,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub salt: String,
    pub birth_date: NaiveDate,
    pub registration_date: NaiveDate,
    pub avatar: String
}