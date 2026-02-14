use async_trait::async_trait;
use crate::domain::entities::gender::Gender;


#[async_trait]
pub trait IGenderRepository {
    async fn create_gender(
        &self,
        name: String,
        user_id: i32,
        books_ids: Option<Vec<i32>>
    ) -> Result<(), String>;

    async fn get_gender_by_id(&self, id: i32) -> Result<Gender, String>;

    async fn get_gender_by_name(&self, name: String, skip: i32, page_size: i32) -> Result<Option<Gender>, String>;

    async fn get_genders_by_book(&self, book_id: i32, skip: i32, page_size: i32) -> Result<Option<Vec<Gender>>, String>;

    async fn get_genders_by_author(&self, author_id: i32, skip: i32, page_size: i32) -> Result<Option<Vec<Gender>>, String>;

    async fn get_genders_by_publisher(&self, publisher_id: i32, skip: i32, page_size: i32) -> Result<Option<Vec<Gender>>, String>;

    async fn more_popular_gender(
        &self,
        skip: i32,
        page_size: i32
    ) -> Result<Option<Vec<Gender>>, String>; // TODO: generos com os livros mais lidos

    async fn best_valuated_gender(
        &self,
        skip: i32,
        page_size: i32
    ) -> Result<Option<Vec<Gender>>, String>; // TODO: generos com os livros melhor avaliados

    async fn alter_gender(
        &mut self,
        id: String,
        name: String,
        user_id: i32,
        books_ids: Option<Vec<i32>>
    ) -> Result<(), String>;

    async fn delete_gender(&self, id: String) -> Result<(), String>;
}