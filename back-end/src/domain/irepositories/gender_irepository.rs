use async_trait::async_trait;
use crate::domain::entities::gender::Gender;

#[async_trait]
pub trait GenderIRepository {
    async fn get_all_genders(&self) -> Result<Vec<Gender>, String>;
    async fn get_gender_by_id(&self, id: i32) -> Result<Gender, String>;
    async fn create_gender(&self, gender: Gender) -> Result<(), String>;
}