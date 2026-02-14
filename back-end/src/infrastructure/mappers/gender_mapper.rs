use crate::domain::entities::gender::Gender;
use crate::infrastructure::db::models::gender_row::GenderRow;

impl From<GenderRow> for Gender {
    fn from(row: GenderRow) -> Self {
        Gender::new(
            row.id,
            row.name
        )
    }
}
