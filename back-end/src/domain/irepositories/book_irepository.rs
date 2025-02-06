use async_trait::async_trait;
use crate::domain::entities::book::Book;

#[async_trait]
pub trait BookIRepository {
    async fn get_all_books(&self) -> Result<Vec<Book>, String>;
    async fn get_book_by_id(&self, id: i32) -> Result<Book, String>;
    async fn get_book_by_title(&self, title: &str) -> Result<Vec<Book>, String>;
    // async fn get_book_by_author(&self, author: Author) -> Result<Vec<Book>, String>;
    // async fn get_book_by_publisher(&self, publisher: Publisher) -> Result<Vec<Book>, String>;
    // async fn get_book_by_gender(&self, gender: Gender) -> Result<Vec<Book>, String>;
    async fn create_book(&self, book: Book) -> Result<(), String>;
}