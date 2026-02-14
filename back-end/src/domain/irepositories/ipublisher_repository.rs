use async_trait::async_trait;

use crate::domain::entities::publisher::Publisher;


#[async_trait]
pub trait IPublisherRepository {
    async fn create_publiser (
        &self,
        id: i32,
        name: String,
        user_id: i32,
        site: Option<String>,
        email: Option<String>,
        avatar: Option<String>,
        books_ids: Option<Vec<i32>>
    ) -> Result<(), String>;

    async fn get_publisher_by_id (&self, publisher_id: i32) -> Result<Publisher, String>;

    async fn get_publisher_by_name (&self, publisher_name: String, skip: i32, page_size: i32) -> Result<Vec<Publisher>, String>;

    async fn get_publisher_by_book (&self, book_id: i32) -> Result<Publisher, String>;

    async fn get_publishers_by_author (&self, author_id: i32, skip: i32, page_size: i32) -> Result<Option<Vec<Publisher>>, String>;

    async fn get_publishers_by_gender (&self, gender_id: i32, skip: i32, page_size: i32) -> Result<Option<Vec<Publisher>>, String>;

    async fn more_popular_publishers(
        &self,
        skip: i32,
        page_size: i32
    ) -> Result<Option<Vec<Publisher>>, String>; // TODO: editoras com os livros mais lidos

    async fn best_valuated_publishers(
        &self,
        skip: i32,
        page_size: i32
    ) -> Result<Option<Vec<Publisher>>, String>; // TODO: editoras com os livros melhor avaliados

    async fn alter_publisher (
        &self,
        name: String,
        user_id: i32,
        site: Option<String>,
        email: Option<String>,
        avatar: Option<String>,
        books_ids: Option<Vec<i32>>
    ) -> Result<(), String>;

    async fn delete_publisher (&self, publisher_id: i32) -> Result<(), String>;
}