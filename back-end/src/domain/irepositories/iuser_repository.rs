use async_trait::async_trait;
use crate::infrastructure::db::models::user_row::UserRow;


#[async_trait]
pub trait IUserRepository {
    async fn create_user(
        &self,
        name: String,
        nickname: String,
        email: String,
        password: String,
        birth_date: String,
        avatar: String
    ) -> Result<(), String>;

    async fn get_user_by_id(&self, id: String) -> Result<Option<UserRow>, String>;

    async fn get_user_by_email(&self, email: String) -> Result<Option<UserRow>, String>;

    async fn get_user_by_nickname(&self, nickname: String) -> Result<Vec<UserRow>, String>;

    async fn alter_user(
        &mut self,
        id: String,
        name: String,
        nickname: String,
        email: String,
        birth_date: String,
        avatar: String
    ) -> Result<(), String>;

    async fn delete_user(&self, id: String) -> Result<(), String>;
}