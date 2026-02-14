use sqlx::PgPool;

use crate::domain::irepositories::iuser_repository::IUserRepository;
use crate::infrastructure::db::models::user_row::UserRow;
use crate::infrastructure::repositories::user_repository::UserRepository;



pub struct UserService {
    repository: UserRepository
}

impl UserService {
    pub fn new(pool: PgPool) -> Self {
        let repo = UserRepository::new(pool);
        Self { repository: repo }
    }

    pub async fn register_user(
        &self,
        name: String,
        nickname: String,
        email: String,
        password: String,
        birth_date: String,
        avatar: String
    ) -> Result<(), String> {
        if self.repository.get_user_by_email(email.clone()).await?.is_some() {
            return Err("Usuário já cadastrado".to_string());
        }

        self.repository.create_user(name, nickname, email, password, birth_date, avatar).await
    }

    pub async fn get_user_by_id(&self, id: String) -> Result<Option<UserRow>, String> {
        self.repository.get_user_by_id(id).await
    }

    pub async fn get_user_by_email(&self, email: String) -> Result<Option<UserRow>, String> {
        self.repository.get_user_by_email(email).await
    }

    pub async fn get_user_by_nickname(&self, nickname: String) -> Result<Vec<UserRow>, String> {
        self.repository.get_user_by_nickname(nickname).await
    }

    pub async fn alter_user(
            &mut self,
            id: String,
            name: String,
            nickname: String,
            email: String,
            birth_date: String,
            avatar: String
        ) -> Result<(), String> {
        self.repository.alter_user(
            id,
            name,
            nickname,
            email,
            birth_date,
            avatar
        ).await
    }

    pub async fn delete_user(&self, id: String) -> Result<(), String> {
        self.repository.delete_user(id).await
    }
}