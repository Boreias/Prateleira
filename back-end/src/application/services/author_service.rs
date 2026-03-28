use sqlx::PgPool;
use uuid::Uuid;
use axum::body::Bytes;

use crate::domain::entities::author::Author;
use crate::domain::irepositories::iauthor_repository::IAuthorRepository;
use crate::infrastructure::repositories::author_repository::AuthorRepository;


pub struct AuthorService {
    repository: AuthorRepository
}

impl AuthorService {
    pub fn new(pool: PgPool) -> Self {
        let repo = AuthorRepository::new(pool);

        Self { repository: repo }
    }

    pub async fn create_author(
        &self,
        name: String,
        user_id: Uuid,
        file_name: Option<String>,
        file_content: Option<Bytes>,
        books: Option<Vec<Uuid>>
    ) -> Result<(), String> {
        self.repository.create_author(name, user_id, file_name, file_content, books).await
    }

    pub async fn get_author_by_id(&self, id: Uuid) -> Result<Author, String> {
        self.repository.get_author_by_id(id).await
    }

    pub async fn get_author_by_name(&self, name: String, skip: Option<i32>, page_size: Option<i32>) -> Result<Vec<Author>, String> {
        self.repository.get_author_by_name(name, skip.unwrap_or(0), page_size.unwrap_or(20)).await
    }

    pub async fn get_authors_by_book(&self, book_id: Uuid, skip: Option<i32>, page_size: Option<i32>) -> Result<Vec<Author>, String> {
        self.repository.get_authors_by_book(book_id, skip.unwrap_or(0), page_size.unwrap_or(20)).await
    }

    pub async fn get_authors_by_gender(&self, gender_id: Uuid, skip: Option<i32>, page_size: Option<i32>) -> Result<Vec<Author>, String> {
        self.repository.get_authors_by_gender(gender_id, skip.unwrap_or(0), page_size.unwrap_or(20)).await
    }

    pub async fn get_authors_by_publisher(&self, publisher_id: Uuid, skip: Option<i32>, page_size: Option<i32>) -> Result<Vec<Author>, String> {
        self.repository.get_authors_by_publisher(publisher_id, skip.unwrap_or(0), page_size.unwrap_or(20)).await
    }

    pub async fn more_popular_author(&self, skip: Option<i32>, page_size: Option<i32>) -> Result<Vec<Author>, String> {
        self.repository.more_popular_author(skip.unwrap_or(0), page_size.unwrap_or(20)).await
    }

    pub async fn best_valuated_author(&self, skip: Option<i32>, page_size: Option<i32>) -> Result<Vec<Author>, String> {
        self.repository.best_valuated_author(skip.unwrap_or(0), page_size.unwrap_or(20)).await
    }

    pub async fn alter_author(
        &mut self,
        id: Uuid,
        name: String,
        user_id: Uuid,
        file_name: Option<String>,
        file_content: Option<Bytes>,
        books: Option<Vec<Uuid>>
    ) -> Result<(), String> {
        self.repository.alter_author(id, name, user_id, file_name, file_content, books).await
    }

    pub async fn delete_author(&self, id: Uuid, user_id: Uuid) -> Result<(), String> {
        self.repository.delete_author(id, user_id).await
    }

    pub async fn clear_deleted_authors(&self) -> Result<(), String> {
        self.repository.clear_deleted_authors().await
    }
}