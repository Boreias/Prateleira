use uuid::Uuid;
use async_trait::async_trait;
use axum::body::Bytes;

use crate::domain::entities::publisher::Publisher;


#[async_trait]
pub trait IPublisherRepository {
    async fn create_publisher (
        &self,
        name: String,
        user_id: Uuid,
        site: Option<String>,
        email: Option<String>,
        file_name: Option<String>,
        file_content: Option<Bytes>
    ) -> Result<(), String>;

    async fn get_publisher_by_id (&self, id: Uuid) -> Result<Publisher, String>;

    async fn get_publisher_by_name (&self, name: String, skip: i32, page_size: i32) -> Result<Vec<Publisher>, String>;

    async fn get_publisher_by_book (&self, book_id: Uuid, skip: i32, page_size: i32) -> Result<Publisher, String>;

    async fn get_publishers_by_author (&self, author_id: Uuid, skip: i32, page_size: i32) -> Result<Vec<Publisher>, String>;

    async fn get_publishers_by_gender (&self, gender_id: Uuid, skip: i32, page_size: i32) -> Result<Vec<Publisher>, String>;

    async fn more_popular_publishers(
        &self,
        skip: i32,
        page_size: i32
    ) -> Result<Vec<Publisher>, String>; // TODO: editoras com os livros mais lidos

    async fn best_valuated_publishers(
        &self,
        skip: i32,
        page_size: i32
    ) -> Result<Vec<Publisher>, String>; // TODO: editoras com os livros melhor avaliados

    async fn alter_publisher (
        &self,
        id: Uuid,
        name: String,
        user_id: Uuid,
        site: Option<String>,
        email: Option<String>,
        file_name: Option<String>,
        file_content: Option<Bytes>
    ) -> Result<(), String>;

    async fn delete_publisher (&self, id: Uuid, user_id: Uuid) -> Result<(), String>;

    async fn clear_deleted_publishers(&self) -> Result<(), String>;
}