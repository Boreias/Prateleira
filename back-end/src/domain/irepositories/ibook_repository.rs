use uuid::Uuid;
use async_trait::async_trait;
use axum::body::Bytes;

use crate::domain::entities::book::Book;


#[async_trait]
pub trait IBookRepository {
    async fn create_book (
        &self,
        title: String,
        subtitle: Option<String>,
        authors_id: Vec<Uuid>,
        publisher_id: Uuid,
        series_collection: Option<i32>,
        volume: Option<i32>,
        edition: Option<i32>,
        publication_year: Option<i32>,
        pages: Option<i32>,
        language: Option<String>,
        isbn: String,
        genders_id: Vec<Uuid>,
        synopsis: Option<String>,
        file_name: Option<String>,
        file_content: Option<Bytes>,
        user_id: Uuid
    ) -> Result<(), String>;

    async fn get_book_by_id (&self, book_id: Uuid) -> Result<Book, String>;

    async fn get_books_by_name (&self, book_name: String, skip: i32, page_size: i32) -> Result<Vec<Book>, String>;

    async fn get_book_by_isbn (&self, isbn: String) -> Result<Book, String>;

    async fn get_books_by_author(&self, author_id: Uuid, skip: i32, page_size: i32) -> Result<Vec<Book>, String>;

    async fn best_valuated_books_by_author(&self, author_id: Uuid, skip: i32, page_size: i32) -> Result<Vec<Book>, String>;

    async fn get_books_by_publisher(&self, publisher_id: Uuid, skip: i32, page_size: i32) -> Result<Vec<Book>, String>;

    async fn best_valuated_books_by_publisher(&self, publisher_id: Uuid, skip: i32, page_size: i32) -> Result<Vec<Book>, String>;

    async fn get_books_by_gender(&self, gender_id: Uuid, skip: i32, page_size: i32) -> Result<Vec<Book>, String>;

    async fn best_valuated_books_by_gender(&self, gender_id: Uuid, skip: i32, page_size: i32) -> Result<Vec<Book>, String>;

    async fn smallets_books(&self, skip: i32, page_size: i32) -> Result<Vec<Book>, String>;

    async fn biggest_books(&self, skip: i32, page_size: i32) -> Result<Vec<Book>, String>;

    async fn more_popular_books(
        &self,
        skip: i32,
        page_size: i32
    ) -> Result<Vec<Book>, String>;

    async fn best_valuated_books(
        &self,
        skip: i32,
        page_size: i32
    ) -> Result<Vec<Book>, String>;

    async fn alter_book(
        &self,
        book_id: Uuid,
        title: String,
        subtitle: Option<String>,
        authors_id: Vec<Uuid>,
        publisher_id: Uuid,
        series_collection: Option<i32>,
        volume: Option<i32>,
        edition: Option<i32>,
        publication_year: Option<i32>,
        pages: Option<i32>,
        language: Option<String>,
        isbn: String,
        genders_id: Vec<Uuid>,
        synopsis: Option<String>,
        file_name: Option<String>,
        file_content: Option<Bytes>,
        user_id: Uuid
    ) -> Result<(), String>;

    async fn delete_book(&self, book_id: Uuid, user_id: Uuid) -> Result<(), String>;

    async fn clear_deleted_books(&self) -> Result<(), String>;
}