use async_trait::async_trait;

use crate::domain::entities::book::Book;


#[async_trait]
pub trait IBookRepository {
    async fn create_book (
        &self,
        title: String,
        subtitle: Option<String>,
        authors_id: Vec<i32>,
        publisher_id: i32,
        series_collection: Option<i8>,
        volume: Option<i8>,
        edition: Option<i8>,
        publication_year: i16,
        pages: i32,
        language: String,
        isbn: String,
        genders_id: Vec<i32>,
        synopsis: String,
        cover: String,
        user_id: i32
    ) -> Result<(), String>;

    async fn get_book_by_id (&self, book_id: i32) -> Result<Book, String>;

    async fn get_books_by_author(&self, author_id: i32, skip: i32, page_size: i32) -> Result<Option<Vec<Book>>, String>;

    async fn best_books_by_author(&self, author_id: i32, skip: i32, page_size: i32) -> Result<Option<Vec<Book>>, String>;

    async fn get_books_by_publisher(&self, publisher_id: i32, skip: i32, page_size: i32) -> Result<Option<Vec<Book>>, String>;

    async fn best_books_by_publisher(&self, publisher_id: i32, skip: i32, page_size: i32) -> Result<Option<Vec<Book>>, String>;

    async fn get_books_by_gender(&self, gender_id: i32, skip: i32, page_size: i32) -> Result<Option<Vec<Book>>, String>;

    async fn best_books_by_gender(&self, gender_id: i32, skip: i32, page_size: i32) -> Result<Option<Vec<Book>>, String>;

    async fn more_popular_book(
        &self,
        skip: i32,
        page_size: i32
    ) -> Result<Option<Vec<Book>>, String>; // TODO: livros mais lidos

    async fn best_valuated_book(
        &self,
        skip: i32,
        page_size: i32
    ) -> Result<Option<Vec<Book>>, String>;// TODO: livros melhor avaliados

    async fn alter_book(
        &self,
        book_id: i32,
        title: String,
        subtitle: Option<String>,
        authors_id: Vec<i32>,
        publisher_id: i32,
        series_collection: Option<i8>,
        volume: Option<i8>,
        edition: Option<i8>,
        publication_year: i16,
        pages: i32,
        language: String,
        isbn: String,
        genders_id: Vec<i32>,
        synopsis: String,
        cover: String,
        user_id: i32
    ) -> Result<(), String>;

    async fn delete_book(&self, book_id: i32) -> Result<(), String>;
}