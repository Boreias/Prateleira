use async_trait::async_trait;
use crate::domain::entities::user::User;

#[async_trait]
pub trait UserIRepository {
    async fn create_user(&self, user: &User) -> Result<(), String>;
    async fn get_user_by_username(&self, email: &str) -> Result<Option<User>, String>;
    async fn get_user_by_nickname(&self, email: &str) -> Result<Option<User>, String>;
    async fn get_user_by_email(&self, email: &str) -> Result<Option<User>, String>;
}