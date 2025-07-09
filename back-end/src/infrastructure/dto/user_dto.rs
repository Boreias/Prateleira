use uuid::Uuid;
use chrono::NaiveDate;
use serde::Serialize;


#[derive(Serialize)]
pub struct UserDTO {
    pub id: Uuid,
    pub name: String,
    pub nickname: String,
    pub email: String,
    pub birth_date: NaiveDate,
    pub avatar: String
}