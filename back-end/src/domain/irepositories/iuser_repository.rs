use uuid::Uuid;
use async_trait::async_trait;
use chrono::NaiveDate;
use axum::body::Bytes;
use tower_cookies::{Cookies};

use crate::domain::entities::user_profile::UserProfile;


#[async_trait]
pub trait IUserRepository {
    async fn create_user(
        &self,
        username: String,
        email: String,
        password: String,
        name: String,
        bio: Option<String>,
        birth_date: NaiveDate,
        country: String,
        file_name: Option<String>,
        file_content: Option<Bytes>
    ) -> Result<(), String>;

    async fn validate_email(&self, email: String, token: String) -> Result<(), String>;

    async fn get_user_by_id(&self, id: Uuid) -> Result<UserProfile, String>;

    async fn get_user_by_email(&self, email: String) -> Result<Option<UserProfile>, String>;

    async fn get_user_by_name(&self, name: String, skip: i32, page_size: i32) -> Result<Vec<UserProfile>, String>;

    async fn auth_user(&self, username_or_email: String, password: String, country: String, cookies: Cookies) -> Result<(), String>;

    async fn refresh_token(&self, id: Uuid, token: String, cookies: Cookies) -> Result<(), String>;

    async fn alter_user_profile(
        &mut self,
        id: Uuid,
        name: String,
        bio: Option<String>,
        birth_date: NaiveDate,
        file_name: Option<String>,
        file_content: Option<Bytes>
    ) -> Result<(), String>;

    async fn change_password(
        &mut self,
        id: Uuid,
        password: String,
    ) -> Result<(), String>;

    async fn activate_user(&self, id: Uuid) -> Result<(), String>;

    async fn desactivate_user(&self, id: Uuid) -> Result<(), String>;

    async fn delete_user(&self, id: Uuid) -> Result<(), String>;
}