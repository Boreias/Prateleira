use async_trait::async_trait;
use crate::domain::entities::author::Author;

#[async_trait]
pub trait AuthorIRepository {
    async fn get_all_authors(&self) -> Result<Vec<Author>, String>;
    async fn get_author_by_id(&self, id: i32) -> Result<Author, String>;
    async fn get_author_by_name(&self, name: &str) -> Result<Vec<Author>, String>;
    async fn create_author(&self, author: Author) -> Result<(), String>;
}