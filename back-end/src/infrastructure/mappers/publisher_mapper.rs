use crate::domain::entities::publisher::Publisher;
use crate::infrastructure::db::models::publisher_row::PublisherRow;

impl From<PublisherRow> for Publisher {
    fn from(row: PublisherRow) -> Self {
        Publisher::new(
            row.id,
            row.name,
            row.site,
            row.email,
            row.avatar,
            None
        )
    }
}