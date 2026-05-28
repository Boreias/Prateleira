use sqlx::PgPool;
use uuid::Uuid;
use axum::body::Bytes;

use crate::domain::entities::book::Book;
use crate::domain::irepositories::ibook_repository::IBookRepository;
use crate::infrastructure::repositories::book_repository::BookRepository;


pub struct BookService {
    repository: BookRepository
}

impl BookService {
    pub fn new(pool: PgPool) -> Self {
        let repo = BookRepository::new(pool);

        Self { repository: repo }
    }

    pub async fn create_book (
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
    ) -> Result<(), String> {
        self.repository.create_book(
            title,
            subtitle,
            authors_id,
            publisher_id,
            series_collection,
            volume,
            edition,
            publication_year,
            pages,
            language,
            isbn,
            genders_id,
            synopsis,
            file_name,
            file_content,
            user_id
        ).await
    }

    pub async fn get_book_by_id (&self, book_id: Uuid) -> Result<Book, String> {
        self.repository.get_book_by_id(book_id).await
    }

    pub async fn get_books_by_name (&self, book_name: String, skip: Option<i32>, page_size: Option<i32>) -> Result<Vec<Book>, String> {
        self.repository.get_books_by_name(book_name, skip.unwrap_or(0), page_size.unwrap_or(20)).await
    }

    pub async fn get_book_by_isbn (&self, isbn: String) -> Result<Book, String> {
        self.repository.get_book_by_isbn(isbn).await
    }

    pub async fn get_books_by_author(&self, author_id: Uuid, skip: Option<i32>, page_size: Option<i32>) -> Result<Vec<Book>, String> {
        self.repository.get_books_by_author(author_id, skip.unwrap_or(0), page_size.unwrap_or(20)).await
    }

    pub async fn best_valuated_books_by_author(&self, author_id: Uuid, skip: Option<i32>, page_size: Option<i32>) -> Result<Vec<Book>, String> {
        self.repository.best_valuated_books_by_author(author_id, skip.unwrap_or(0), page_size.unwrap_or(20)).await
    }

    pub async fn get_books_by_publisher(&self, publisher_id: Uuid, skip: Option<i32>, page_size: Option<i32>) -> Result<Vec<Book>, String> {
        self.repository.get_books_by_publisher(publisher_id, skip.unwrap_or(0), page_size.unwrap_or(20)).await
    }

    pub async fn best_valuated_books_by_publisher(&self, publisher_id: Uuid, skip: Option<i32>, page_size: Option<i32>) -> Result<Vec<Book>, String> {
        self.repository.best_valuated_books_by_publisher(publisher_id, skip.unwrap_or(0), page_size.unwrap_or(20)).await
    }

    pub async fn get_books_by_gender(&self, gender_id: Uuid, skip: Option<i32>, page_size: Option<i32>) -> Result<Vec<Book>, String> {
        self.repository.get_books_by_gender(gender_id, skip.unwrap_or(0), page_size.unwrap_or(20)).await
    }

    pub async fn best_valuated_books_by_gender(&self, gender_id: Uuid, skip: Option<i32>, page_size: Option<i32>) -> Result<Vec<Book>, String> {
        self.repository.best_valuated_books_by_gender(gender_id, skip.unwrap_or(0), page_size.unwrap_or(20)).await
    }

    pub async fn smallets_books(&self, skip: Option<i32>, page_size: Option<i32>) -> Result<Vec<Book>, String> {
        self.repository.smallets_books(skip.unwrap_or(0), page_size.unwrap_or(20)).await
    }

    pub async fn biggest_books(&self, skip: Option<i32>, page_size: Option<i32>) -> Result<Vec<Book>, String> {
        self.repository.biggest_books(skip.unwrap_or(0), page_size.unwrap_or(20)).await
    }

    pub async fn more_popular_books(
        &self,
        skip: Option<i32>,
        page_size: Option<i32>
    ) -> Result<Vec<Book>, String> {
        self.repository.more_popular_books(skip.unwrap_or(0), page_size.unwrap_or(20)).await
    }

    pub async fn best_valuated_books(
        &self,
        skip: Option<i32>,
        page_size: Option<i32>
    ) -> Result<Vec<Book>, String> {
        self.repository.best_valuated_books(skip.unwrap_or(0), page_size.unwrap_or(20)).await
    }

    pub async fn alter_book(
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
    ) -> Result<(), String> {
        self.repository.alter_book(
            book_id,
            title,
            subtitle,
            authors_id,
            publisher_id,
            series_collection,
            volume,
            edition,
            publication_year,
            pages,
            language,
            isbn,
            genders_id,
            synopsis,
            file_name,
            file_content,
            user_id
        ).await
    }

    pub async fn delete_book(&self, book_id: Uuid, user_id: Uuid) -> Result<(), String> {
        self.repository.delete_book(book_id, user_id).await
    }

    pub async fn clear_deleted_books(&self) -> Result<(), String> {
        self.repository.clear_deleted_books().await
    }
}