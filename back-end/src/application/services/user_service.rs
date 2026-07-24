use sqlx::PgPool;
use uuid::Uuid;
use chrono::NaiveDate;
use axum::body::Bytes;
use tower_cookies::Cookies;

use crate::domain::irepositories::iuser_repository::IUserRepository;
use crate::infrastructure::repositories::user_repository::UserRepository;
use crate::domain::entities::user_profile::UserProfile;



pub struct UserService {
    repository: UserRepository
}

impl UserService {
    pub fn new(pool: PgPool) -> Self {
        let repo = UserRepository::new(pool);
        Self { repository: repo }
    }

    pub async fn create_user(
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
    ) -> Result<(), String> {
        return self.repository.create_user(
            username,
            email,
            password,
            name,
            bio,
            birth_date,
            country,
            file_name,
            file_content
        ).await
    }

    pub async fn validate_email(&self, email: String, token: String) -> Result<(), String> {
        return self.repository.validate_email(email, token).await;
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<UserProfile, String> {
        return self.repository.get_user_by_id(id).await;
    }

    pub async fn get_user_by_email(&self, email: String) -> Result<Option<UserProfile>, String> {
        return self.repository.get_user_by_email(email).await;
    }

    pub async fn get_user_by_name(&self, name: String, skip: Option<i32>, page_size: Option<i32>) -> Result<Vec<UserProfile>, String> {
        return self.repository.get_user_by_name(name, skip.unwrap_or(0), page_size.unwrap_or(20)).await;
    }

    pub async fn auth_user(&self, username_or_email: String, password: String, country: String, cookies: Cookies) -> Result<(), String> {
        return self.repository.auth_user(username_or_email, password, country, cookies).await;
    }

    pub async fn refresh_token(&self, id: Uuid, token: String, cookies: Cookies) -> Result<(), String> {
        return self.repository.refresh_token(id, token, cookies).await;
    }

    pub async fn alter_user_profile(
        &mut self,
        id: Uuid,
        name: String,
        bio: Option<String>,
        birth_date: NaiveDate,
        file_name: Option<String>,
        file_content: Option<Bytes>
    ) -> Result<(), String> {
        return self.repository.alter_user_profile(
            id,
            name,
            bio,
            birth_date,
            file_name,
            file_content
        ).await;
    }

    pub async fn change_password(
        &mut self,
        id: Uuid,
        password: String,
    ) -> Result<(), String> {
        return self.repository.change_password(id, password).await;
    }

    pub async fn activate_user(&self, id: Uuid) -> Result<(), String> {
        return self.repository.activate_user(id).await;
    }

    pub async fn desactivate_user(&self, id: Uuid) -> Result<(), String> {
        return self.repository.desactivate_user(id).await;
    }

    pub async fn delete_user(&self, id: Uuid) -> Result<(), String> {
        return self.repository.delete_user(id).await;
    }
}