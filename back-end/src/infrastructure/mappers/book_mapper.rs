use crate::domain::entities::book::Book;
use crate::infrastructure::db::models::book_row::BookRow;

impl From<BookRow> for Book {
    fn from(row: BookRow) -> Self {
        Book::new(
            row.id,
            row.title,
            row.subtitle,
            Vec::new(),
            None,
            row.series_collection,
            row.volume,
            row.edition,
            row.publication_year,
            row.pages,
            row.language,
            row.isbn,
            Vec::new(),
            row.synopsis,
            row.cover
        )
    }
}