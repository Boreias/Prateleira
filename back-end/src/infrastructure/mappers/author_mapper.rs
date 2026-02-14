use crate::domain::entities::author::Author;
use crate::infrastructure::db::models::author_row::AuthorRow;

impl From<AuthorRow> for Author {
    fn from(row: AuthorRow) -> Self {
        Author::new(
            row.id,
            row.name,
            row.avatar,
            None, // livros carregados separadamente
        )
    }
}
