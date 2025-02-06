use crate::domain::entities::author::Author;
use crate::domain::irepositories::author_irepository::AuthorIRepository;
use async_trait::async_trait;

pub struct AuthorService<T: AuthorIRepository> {
    repository: T,
}

impl<T: AuthorIRepository> AuthorService<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub async fn register_author(&self, name: String, avatar: String) -> Result<(), String> {
        let author = Author::new(name, avatar);
        self.repository.create_author(&author).await
    }
}