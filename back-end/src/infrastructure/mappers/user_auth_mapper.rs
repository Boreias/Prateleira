use crate::domain::entities::user_auth::UserAuth;
use crate::infrastructure::db::models::user_auth_row::UserAuthRow;

impl From<UserAuthRow> for UserAuth {
    fn from(row: UserAuthRow) -> Self {
        UserAuth::new(
            row.id,
            row.username,
            row.email,
            row.password,
            row.salt,
            row.country,
            row.is_active,
            row.is_email_verified,
            row.created_at,
            row.updated_at
        )
    }
}