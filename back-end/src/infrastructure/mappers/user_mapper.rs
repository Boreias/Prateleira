use crate::domain::entities::user::User;
use crate::infrastructure::db::models::user_row::UserRow;

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        User::new(
            row.id,
            row.name,
            row.nickname,
            row.email,
            row.password,
            row.salt,
            row.birth_date,
            row.registration_date,
            row.avatar
        )
    }
}