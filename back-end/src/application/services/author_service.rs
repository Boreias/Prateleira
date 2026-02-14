use sqlx::PgPool;

use crate::domain::irepositories::iauthor_repository::IAuthorRepository;
use crate::infrastructure::db::models::author_row::AuthorRow;
use crate::infrastructure::repositories::author_repository::AuthorRepository;


pub struct AuthorService {
    repository: AuthorRepository
}

impl AuthorService {
    pub fn new(pool: PgPool) -> Self {
        let repo = AuthorRepository::new(pool);

        Self { repository: repo }
    }
}