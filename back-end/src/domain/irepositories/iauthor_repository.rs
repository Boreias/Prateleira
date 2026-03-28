use uuid::Uuid;
use async_trait::async_trait;
use axum::body::Bytes;

use crate::domain::entities::author::Author;



#[async_trait]
pub trait IAuthorRepository {
    async fn create_author(
        &self,
        name: String,
        user_id: Uuid,
        file_name: Option<String>,
        file_content: Option<Bytes>,
        books: Option<Vec<Uuid>>,
    ) -> Result<(), String>;

    async fn get_author_by_id(&self, id: Uuid) -> Result<Author, String>;

    async fn get_author_by_name(&self, name: String, skip: i32, page_size: i32) -> Result<Vec<Author>, String>;

    async fn get_authors_by_book(&self, book_id: Uuid, skip: i32, page_size: i32) -> Result<Vec<Author>, String>;

    async fn get_authors_by_gender(&self, gender_id: Uuid, skip: i32, page_size: i32) -> Result<Vec<Author>, String>;

    async fn get_authors_by_publisher(&self, publisher_id: Uuid, skip: i32, page_size: i32) -> Result<Vec<Author>, String>;

    async fn more_popular_author(
        &self,
        skip: i32,
        page_size: i32
    ) -> Result<Vec<Author>, String>;

    async fn best_valuated_author(
        &self,
        skip: i32,
        page_size: i32
    ) -> Result<Vec<Author>, String>;

    async fn alter_author(
        &mut self,
        id: Uuid,
        name: String,
        user_id: Uuid,
        file_name: Option<String>,
        file_content: Option<Bytes>,
        books: Option<Vec<Uuid>>
    ) -> Result<(), String>;

    async fn delete_author(&self, id: Uuid, user_id: Uuid) -> Result<(), String>;

    async fn clear_deleted_authors(&self) -> Result<(), String>;
}