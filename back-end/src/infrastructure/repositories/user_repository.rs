use async_trait::async_trait;
use sqlx::{PgPool, Row};
use crate::domain::entities::user::User;
use crate::domain::irepositories::user_irepository::UserIRepository;

pub struct UserRepository {
    pub _pool: PgPool,
}

impl UserRepository {
    pub fn new(_pool: PgPool) -> Self {
        UserRepository {
            _pool
        }
    }
}


#[async_trait]
impl UserIRepository for UserRepository {
    async fn get_all_users(&self) -> Result<Vec<User>, String> {
        let mut users: Vec<User> = Vec::new();
        let mut rows = sqlx::query("SELECT * FROM users")
            .fetch_all(&self._pool)
            .await
            .map_err(|e| e.to_string())?;

        for row in rows.iter_mut() {
            users.push(User::new(
                row.try_get("id").unwrap(),
                row.try_get("name").unwrap(),
                row.try_get("email").unwrap(),
                row.try_get("password").unwrap(),
                row.try_get("avatar").unwrap(),
                row.try_get("role").unwrap(),
            ));
        }

        Ok(users)
    }

    async fn get_user_by_id(&self, id: i32) -> Result<User, String> {
        let row = sqlx::query("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&self._pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(User::new(
            row.try_get("id").unwrap(),
            row.try_get("name").unwrap(),
            row.try_get("email").unwrap(),
            row.try_get("password").unwrap(),
            row.try_get("avatar").unwrap(),
            row.try_get("role").unwrap(),
        ))
    }

    async fn get_user_by_username(&self, name: &str) -> Result<Vec<User>, String> {
        let mut users: Vec<User> = Vec::new();
        let mut rows = sqlx::query("SELECT * FROM users WHERE username = $1")
            .bind(name)
            .fetch_all(&self._pool)
            .await
            .map_err(|e| e.to_string())?;

        for row in rows.iter_mut() {
            users.push(User::new(
                row.try_get("id").unwrap(),
                row.try_get("name").unwrap(),
                row.try_get("email").unwrap(),
                row.try_get("password").unwrap(),
                row.try_get("avatar").unwrap(),
                row.try_get("role").unwrap(),
            ));
        }

        Ok(users)
    }


    async fn get_user_by_nickname(&self, name: &str) -> Result<Vec<User>, String> {
        let mut users: Vec<User> = Vec::new();
        let mut rows = sqlx::query("SELECT * FROM users WHERE nickname = $1")
            .bind(name)
            .fetch_all(&self._pool)
            .await
            .map_err(|e| e.to_string())?;

        for row in rows.iter_mut() {
            users.push(User::new(
                row.try_get("id").unwrap(),
                row.try_get("name").unwrap(),
                row.try_get("email").unwrap(),
                row.try_get("password").unwrap(),
                row.try_get("avatar").unwrap(),
                row.try_get("role").unwrap(),
            ));
        }

        Ok(users)
    }

    async fn get_user_by_email(&self, email: &str) -> Result<Vec<User>, String> {
        let mut users: Vec<User> = Vec::new();
        let mut rows = sqlx
            .query("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_all(&self._pool)
            .await
            .map_err(|e| e.to_string())?;

        for row in rows.iter_mut() {
            users.push(User::new(
                row.try_get("id").unwrap(),
                row.try_get("name").unwrap(),
                row.try_get("email").unwrap(),
                row.try_get("password").unwrap(),
                row.try_get("avatar").unwrap(),
                row.try_get("role").unwrap(),
            ));
        }

        Ok(users)
    }

    async fn create_user(&self, user: &User) -> Result<(), String> {
        sqlx::query("INSERT INTO users (name, email, password, avatar, role) VALUES ($1, $2, $3, $4, $5)")
            .bind(&user._name)
            .bind(&user._email)
            .bind(&user._password)
            .bind(&user._avatar)
            .bind(&user._role)
            .execute(&self._pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}