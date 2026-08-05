use crate::domain::entities::user_profile::UserProfile;
use crate::infrastructure::db::models::user_profile_row::UserProfileRow;


impl From<UserProfileRow> for UserProfile {
    fn from(row: UserProfileRow) -> Self {
        UserProfile::new(
            row.id,
            row.name,
            row.bio,
            row.birth_date,
            None
        )
    }
}