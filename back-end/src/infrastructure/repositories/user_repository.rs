use serde_json::json;
use uuid::Uuid;
use chrono::{NaiveDate, Utc};
use sqlx::{PgPool, Row};
use async_trait::async_trait;
use std::env;
use serde_json;

use crate::domain::irepositories::iuser_repository::IUserRepository;
use crate::infrastructure::db::models::user_row::UserRow;

use crate::infrastructure::crypto::crypto::{derive_password_hash, generate_salt, simple_hash, encrypt, decrypt};


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

        let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY não definida");

        let id = Uuid::new_v4();

        let hash_email = simple_hash(secret_key.clone().as_bytes(), email);

        sqlx::query("INSERT INTO Search_Index (user_id, field_name, index_value) VALUES ($1, $2, $3)")
            .bind(id.clone().to_string())
            .bind("email".to_string())
            .bind(hash_email)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;


        let salt = generate_salt();

        let hash_password = derive_password_hash(password, salt.clone());

        sqlx::query("INSERT INTO User_Password (user_id, password_hash, salt) VALUES ($1, $2, $3)")
                .bind(id.clone().to_string())
                .bind(hash_password)
                .bind(salt)
                .execute(&self.pool)
                .await
                .map_err(|e| e.to_string())?;


        let registration_date = Utc::now().date_naive();

        let data = json!({
            "name": name,
            "nickname": nickname,
            "birth_date": birth_date,
            "registration_date": registration_date,
            "avatar": avatar
        });

        if let Some(object_data) = data.as_object() {
            for (key, value) in object_data {
                let (hash_data, hash_nonce) = encrypt(secret_key.clone().as_bytes(), value.to_string()).unwrap();
                sqlx::query("INSERT INTO User (user_id, field_name, encryp_value, nonce) VALUES ($1, $2, $3, $4)")
                    .bind(id.clone().to_string())
                    .bind(key)
                    .bind(hash_data)
                    .bind(hash_nonce)
                    .execute(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        } else {
            println!("Erro ao converter json em objeto");
        }

        Ok(())
    }

    async fn get_user_by_id(&self, id: String) -> Result<Option<UserRow>, String> {
        let row = sqlx::query("SELECT id, name, nickname, email, birthDate, avatar FROM user WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(row) = row {
            let id = Uuid::parse_str(row.get("id")).map_err(|e| e.to_string())?;
            let birth_date = NaiveDate::parse_from_str(row.get("birthDate"), "%Y-%m-%d")
                .map_err(|e| e.to_string())?;

            Ok(Some(UserRow {
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

    async fn get_user_by_email(&self, email: String) -> Result<Option<UserRow>, String> {
        let row = sqlx::query("SELECT id, name, nickname, email, birthDate, avatar FROM user WHERE email = $1")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(row) = row {
            let id = Uuid::parse_str(row.get("id")).map_err(|e| e.to_string())?;
            let birth_date = NaiveDate::parse_from_str(row.get("birthDate"), "%Y-%m-%d").map_err(|e| e.to_string())?;

            Ok(Some(UserRow {
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

    async fn get_user_by_nickname(&self, nickname: String) -> Result<Vec<UserRow>, String> {
        let rows = sqlx::query("SELECT id, name, nickname, email, birthDate, avatar FROM user WHERE nickname = $1")
            .bind(nickname)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut users = Vec::new();

        for row in rows {
            let id = Uuid::parse_str(row.get("id")).map_err(|e| e.to_string())?;
            let birth_date = NaiveDate::parse_from_str(row.get("birthDate"), "%Y-%m-%d").map_err(|e| e.to_string())?;

            let user = UserRow {
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