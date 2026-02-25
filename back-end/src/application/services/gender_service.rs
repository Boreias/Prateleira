use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::entities::gender::Gender;
use crate::domain::irepositories::igender_repository::IGenderRepository;
use crate::infrastructure::repositories::gender_repository::GenderRepository;


pub struct GenderService {
    repository: GenderRepository
}

impl GenderService {
    pub fn new(pool: PgPool) -> Self {
        let repo = GenderRepository::new(pool);

        Self { repository: repo }
    }

    pub async fn create_gender(
        &self,
        name: String,
        user_id: Uuid,
        books_ids: Option<Vec<Uuid>>
    ) -> Result<(), String> {
        self.repository.create_gender(name, user_id, books_ids).await
    }

    pub async fn get_gender_by_id(&self, id: Uuid) -> Result<Gender, String> {
        self.repository.get_gender_by_id(id).await
    }

    pub async fn get_gender_by_name(&self, name: String, skip: Option<i32>, page_size: Option<i32>) -> Result<Vec<Gender>, String> {
        self.repository.get_gender_by_name(name, skip.unwrap_or(0), page_size.unwrap_or(20)).await
    }

    pub async fn get_genders_by_book(&self, book_id: Uuid, skip: Option<i32>, page_size: Option<i32>) -> Result<Vec<Gender>, String> {
        self.repository.get_genders_by_book(book_id, skip.unwrap_or(0), page_size.unwrap_or(20)).await
    }

    pub async fn get_genders_by_author(&self, author_id: Uuid, skip: Option<i32>, page_size: Option<i32>) -> Result<Vec<Gender>, String> {
        self.repository.get_genders_by_author(author_id, skip.unwrap_or(0), page_size.unwrap_or(20)).await
    }

    pub async fn get_genders_by_publisher(&self, publisher_id: Uuid, skip: Option<i32>, page_size: Option<i32>) -> Result<Vec<Gender>, String> {
        self.repository.get_genders_by_publisher(publisher_id, skip.unwrap_or(0), page_size.unwrap_or(20)).await
    }

    pub async fn more_popular_gender(
        &self,
        skip: Option<i32>,
        page_size: Option<i32>
    ) -> Result<Vec<Gender>, String> {
        self.repository.more_popular_gender(skip.unwrap_or(0), page_size.unwrap_or(20)).await
    }

    pub async fn best_valuated_gender(
        &self,
        skip: Option<i32>,
        page_size: Option<i32>
    ) -> Result<Vec<Gender>, String> {
        self.repository.best_valuated_gender(skip.unwrap_or(0), page_size.unwrap_or(20)).await
    }

    pub async fn alter_gender(
        &mut self,
        id: Uuid,
        name: String,
        user_id: Uuid,
        books_ids: Option<Vec<Uuid>>
    ) -> Result<(), String> {
        self.repository.alter_gender(id, name, user_id, books_ids).await
    }

    pub async fn delete_gender(&self, id: Uuid, user_id: Uuid) -> Result<(), String> {
        self.repository.delete_gender(id, user_id).await
    }

    pub async fn clear_deleted_genders(&self) -> Result<(), String> {
        self.repository.clear_deleted_genders().await
    }
}