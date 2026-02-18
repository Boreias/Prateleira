use sqlx::PgPool;

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
        user_id: i32,
        books_ids: Option<Vec<i32>>
    ) -> Result<(), String> {
        self.repository.create_gender(name, user_id, books_ids).await
    }

    pub async fn get_gender_by_id(&self, id: i32) -> Result<Gender, String> {
        self.repository.get_gender_by_id(id).await
    }

    pub async fn get_gender_by_name(&self, name: String, skip: i32, page_size: i32) -> Result<Vec<Gender>, String> {
        self.repository.get_gender_by_name(name, skip, page_size).await
    }

    pub async fn get_genders_by_book(&self, book_id: i32, skip: i32, page_size: i32) -> Result<Vec<Gender>, String> {
        self.repository.get_genders_by_book(book_id, skip, page_size).await
    }

    pub async fn get_genders_by_author(&self, author_id: i32, skip: i32, page_size: i32) -> Result<Vec<Gender>, String> {
        self.repository.get_genders_by_author(author_id, skip, page_size).await
    }

    pub async fn get_genders_by_publisher(&self, publisher_id: i32, skip: i32, page_size: i32) -> Result<Vec<Gender>, String> {
        self.repository.get_genders_by_publisher(publisher_id, skip, page_size).await
    }

    pub async fn more_popular_gender(
        &self,
        skip: i32,
        page_size: i32
    ) -> Result<Vec<Gender>, String> {
        self.repository.more_popular_gender(skip, page_size).await
    }

    pub async fn best_valuated_gender(
        &self,
        skip: i32,
        page_size: i32
    ) -> Result<Vec<Gender>, String> {
        self.repository.best_valuated_gender(skip, page_size).await
    }

    pub async fn alter_gender(
        &mut self,
        id: i32,
        name: String,
        user_id: i32,
        books_ids: Option<Vec<i32>>
    ) -> Result<(), String> {
        self.repository.alter_gender(id, name, user_id, books_ids).await
    }

    pub async fn delete_gender(&self, id: i32) -> Result<(), String> {
        self.repository.delete_gender(id).await
    }
}