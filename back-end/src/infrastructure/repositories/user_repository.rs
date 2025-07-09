use uuid::Uuid;
use chrono::{NaiveDate, Utc};
use sqlx::{PgPool, Row};
use async_trait::async_trait;
use rand::{TryRngCore};
use rand::rngs::OsRng;

use crate::domain::irepositories::iuser_repository::IUserRepository;
use crate::infrastructure::dto::user_dto::UserDTO;

use crate::infrastructure::crypto::crypto::{derive_password_hash};


pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}


#[async_trait]
impl IUserRepository for UserRepository {
    async fn create_user(
        &self,
        name: String,
        nickname: String,
        email: String,
        password: String,
        birth_date: String,
        avatar: String
    ) -> Result<(), String> {

        let id = Uuid::new_v4();
        let registration_date = Utc::now().date_naive();

        let mut salt = vec![0u8; 16];
        let _ = OsRng.try_fill_bytes(&mut salt);

        let hash_password = derive_password_hash(password, salt.clone());

        sqlx::query("INSERT INTO user (id, name, nickname, email, password, salt, birthDate, registrationDate, avatar) VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9)")
            .bind(id.to_string())
            .bind(name)
            .bind(nickname)
            .bind(email)
            .bind(hash_password)
            .bind(salt)
            .bind(birth_date)
            .bind(registration_date.to_string())
            .bind(avatar)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_user_by_id(&self, id: String) -> Result<Option<UserDTO>, String> {
        let row = sqlx::query("SELECT id, name, nickname, email, birthDate, avatar FROM user WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(row) = row {
            let id = Uuid::parse_str(row.get("id")).map_err(|e| e.to_string())?;
            let birth_date = NaiveDate::parse_from_str(row.get("birthDate"), "%Y-%m-%d")
                .map_err(|e| e.to_string())?;

            Ok(Some(UserDTO {
                id,
                name: row.get("name"),
                nickname: row.get("nickname"),
                email: row.get("email"),
                birth_date,
                avatar: row.get("avatar"),
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_user_by_email(&self, email: String) -> Result<Option<UserDTO>, String> {
        let row = sqlx::query("SELECT id, name, nickname, email, birthDate, avatar FROM user WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(row) = row {
            let id = Uuid::parse_str(row.get("id")).map_err(|e| e.to_string())?;
            let birth_date = NaiveDate::parse_from_str(row.get("birthDate"), "%Y-%m-%d").map_err(|e| e.to_string())?;

            Ok(Some(UserDTO {
                id,
                name: row.get("name"),
                nickname: row.get("nickname"),
                email: row.get("email"),
                birth_date,
                avatar: row.get("avatar"),
            }))
        } else {
            Ok(None)
        }
    }

    async fn get_user_by_nickname(&self, nickname: String) -> Result<Vec<UserDTO>, String> {
        let rows = sqlx::query("SELECT id, name, nickname, email, birthDate, avatar FROM user WHERE nickname = $1")
            .bind(nickname)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut users = Vec::new();

        for row in rows {
            let id = Uuid::parse_str(row.get("id")).map_err(|e| e.to_string())?;
            let birth_date = NaiveDate::parse_from_str(row.get("birthDate"), "%Y-%m-%d").map_err(|e| e.to_string())?;

            let user = UserDTO {
                id,
                name: row.get("name"),
                nickname: row.get("nickname"),
                email: row.get("email"),
                birth_date,
                avatar: row.get("avatar"),
            };

            users.push(user);
        }

        Ok(users)
    }

    async fn alter_user(
        &mut self,
        id: String,
        name: String,
        nickname: String,
        email: String,
        birth_date: String,
        avatar: String
    ) -> Result<(), String> {
        sqlx::query("UPDATE user SET name = $2, nickname = $3, email = $4, birthDate = $5, avatar = $6 WHERE id = $1")
            .bind(id)
            .bind(name)
            .bind(nickname)
            .bind(email)
            .bind(birth_date)
            .bind(avatar)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn delete_user(&self, id: String) -> Result<(), String> {
        sqlx::query("DELETE FROM user WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}