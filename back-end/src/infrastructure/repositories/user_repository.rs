use uuid::Uuid;
use chrono::{NaiveDate, Utc};
use sqlx::{PgPool, Row};
use async_trait::async_trait;
use axum::{
    body::Bytes
};
use tokio::{
    io::AsyncWriteExt,
    fs::remove_file
};
use std::env;
use tower_cookies::{Cookie, Cookies, cookie::SameSite::Lax};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        SaltString,
        PasswordHash,
        PasswordHasher,
        PasswordVerifier,
    },
    Argon2,
};


use crate::{domain::irepositories::iuser_repository::IUserRepository, infrastructure::db::models::user_refresh_token_row::UserRefreshTokenRow};
use crate::domain::entities::user_auth::UserAuth;
use crate::domain::entities::user_profile::UserProfile;
use crate::infrastructure::db::models::user_auth_row::UserAuthRow;
use crate::infrastructure::db::models::user_profile_row::UserProfileRow;
use crate::infrastructure::db::models::user_info_row::UserInfoRow;
use crate::infrastructure::db::models::image_row::ImageRow;
use crate::infrastructure::common::consts::UPLOADS_IMAGE_PATH;

use crate::infrastructure::crypto::crypto::{derive_password_hash, generate_salt, generate_jwt};
use crate::infrastructure::email::mail::{send_validation_email, send_identity_verification};


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
        let check_credentials: Option<UserAuthRow> = sqlx::query_as(r#"
            SELECT
                id, username, email, password, salt, country, is_active, is_email_verified, created_at, updated_at
            FROM
                user_auth
            WHERE
                username = $1 OR email = $2;
        "#)
            .bind(username.clone())
            .bind(email.clone())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if check_credentials.is_some() {
            let credentials: UserAuth = check_credentials.unwrap().into();
            if username == credentials.get_username() {
                return Err(String::from("Nome de usuário já em uso, favor digitar outro"));
            }
            if email == credentials.get_email() {
                return Err(String::from("Email já cadastrado"));
            }
        }

        let user_id: Uuid = Uuid::new_v4();
        let actual_date: NaiveDate = Utc::now().date_naive();

        let salt = SaltString::generate(&mut OsRng);
        let passwork_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .expect("Erro ao gerar hash da senha")
            .to_string();

        sqlx::query(r#"
            INSERT INTO
                user_auth (id, username, email, password, salt, country, is_active, is_email_verified, created_at, updated_at)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11);
        "#)
            .bind(user_id)
            .bind(username)
            .bind(email.clone())
            .bind(passwork_hash)
            .bind(salt.to_string())
            .bind(country)
            .bind(true)
            .bind(false)
            .bind(actual_date)
            .bind(actual_date)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query(r#"
            INSERT INTO
                user_profile (id, name, bio, birth_date)
            VALUES
                ($1, $2, $3, $4);
        "#)
            .bind(user_id)
            .bind(name.clone())
            .bind(bio)
            .bind(birth_date)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if file_name.is_some() && file_content.is_some() {
            let image_id = Uuid::new_v4();
            let new_filename = format!("{}.png", Uuid::new_v4());

            let path = format!("./{}/publisher/{}", UPLOADS_IMAGE_PATH, new_filename);

            let mut file = tokio::fs::File::create(&path)
                .await
                .map_err(|e| e.to_string())?;

            file.write_all(&file_content.unwrap())
                .await
                .map_err(|e| e.to_string())?;

            sqlx::query(r#"
                INSERT INTO
                    user_image (id, original_name, image_path, user_id)
                VALUES
                    ($1, $2, $3, $4);
                "#
            )
                .bind(image_id)
                .bind(file_name)
                .bind(path)
                .bind(user_id)
                .execute(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        }

        let token = String::from_utf8(generate_salt()).expect("Erro ao gerar token para validação do email");
        let user_email_validation_id = Uuid::new_v4();

        let base_url_validation_email = env::var("VALIDATION_URL").expect("Url de verificação do email não definido");

        let url_validation_email = format!("{}email={}&token={}", base_url_validation_email, email.clone(), token.clone());

        sqlx::query(r#"
            INSERT INTO
                user_email_verification (id, email, token)
            VALUES
                ($1, $2, $3);
        "#)
            .bind(user_email_validation_id)
            .bind(email.clone())
            .bind(token.clone())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        send_validation_email(&email, &name, &url_validation_email).expect("Erro ao enviar email");

        Ok(())
    }

    async fn validate_email(&self, email: String, token: String) -> Result<(), String> {
        let email_verification = sqlx::query(r#"
            SELECT
                id, email, token
            FROM
                user_email_verification
            WHERE
                token = $1 AND email = $2;
        "#)
            .bind(token)
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if email_verification.is_none() {
            return Err(String::from("Token inválido"));
        }

        let user_id: Uuid = email_verification.unwrap().get("id");

        sqlx::query(r#"
            UPDATE
                user_auth
            SET
                is_email_verified = true
            WHERE
                id = $1;
        "#)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query(r#"
            DELETE FROM
                user_email_verification
            WHERE
                id = $1;
        "#)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_user_by_id(&self, id: Uuid) -> Result<UserProfile, String> {
        let user_profile: UserProfileRow = sqlx::query_as(r#"
            SELECT
                up.id, up.name, up.bio, up.birth_date, up.avatar, up.created_at, up.updated_at
            FROM
                user_profile AS up
            INNER JOIN
                user_auth AS au
            WHERE
                up.id = $1 AND au.is_active = TRUE;
        "#)
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let user: UserProfile = user_profile.into();

        Ok(user)
    }

    async fn get_user_by_email(&self, email: String) -> Result<Option<UserProfile>, String> {
        let user_profile: Option<UserProfileRow> = sqlx::query_as(r#"
            SELECT
                up.id, up.name, up.bio, up.birth_date, up.avatar, up.created_at, up.updated_at
            FROM
                user_profile AS up
            INNER JOIN
                user_auth AS au
            WHERE
                au.email = $1 AND au.is_active = TRUE;
        "#)
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        if user_profile.is_some() {
            let user: UserProfile = user_profile.unwrap().into();

            return Ok(Some(user))
        }

        Ok(None)
    }

    async fn get_user_by_name(&self, name: String, skip: i32, page_size: i32) -> Result<Vec<UserProfile>, String> {
        let format_name = format!("%{}%", name);
        let user_profiles: Vec<UserProfileRow> = sqlx::query_as(r#"
            SELECT
                up.id, up.name, up.bio, up.birth_date, up.avatar, up.created_at, up.updated_at
            FROM
                user_profile AS up
            INNER JOIN
                user_auth AS au
            WHERE
                up.name LIKE $1 AND au.is_active = TRUE
            LIMIT $2
            OFFSET $3;
        "#)
        .bind(format_name)
        .bind(skip)
        .bind(page_size)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut users: Vec<UserProfile> = Vec::new();

        for user_profile in user_profiles {
            let user: UserProfile = user_profile.into();

            users.push(user);
        }

        Ok(users)
    }

    async fn auth_user(&self, username_or_email: String, password: String, country: String, cookies: Cookies) -> Result<(), String> {
        let user_auth_row: Option<UserAuthRow> = sqlx::query_as(r#"
            SELECT
                id, username, email, password_hash, salt, country, is_active, is_email_verified, created_at, updated_at
            FROM
                user_auth
            WHERE
                username = $1 OR email = $1;
        "#)
            .bind(username_or_email)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if user_auth_row.is_some() {
            let user_auth: UserAuth = user_auth_row.unwrap().into();

            if !user_auth.get_is_email_verified() {
                return Err(String::from("Email não verificado"));
            }

            if user_auth.get_country() != country {
                let user_info_row: UserInfoRow = sqlx::query_as(r#"
                    SELECT
                        up.id, up.name, up.bio, up.birth_date, up.avatar, up.created_at, up.updated_at
                    FROM
                        user_profile AS up
                    INNER JOIN
                        user_auth AS au
                    WHERE
                        up.id = $1 AND au.is_active = TRUE;
                "#)
                .bind(user_auth.get_id())
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

                let token = String::from_utf8(generate_salt()).expect("Erro ao gerar token para validação do email");
                let user_email_validation_id = Uuid::new_v4();

                let base_url_validation_email = env::var("VALIDATION_URL").expect("Url de verificação do email não definido");

                let url_validation_email = format!("{}email={}&token={}", base_url_validation_email, user_auth.get_email().clone(), token.clone());

                sqlx::query(r#"
                    INSERT INTO
                        user_email_verification (id, email, token)
                    VALUES
                        ($1, $2, $3);
                "#)
                    .bind(user_email_validation_id)
                    .bind(user_auth.get_email().clone())
                    .bind(token.clone())
                    .execute(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                send_identity_verification(&user_auth.get_email(), &user_info_row.name, &url_validation_email).expect("Erro ao enviar email");
                return Err(String::from("Localização diferente"));
            }

            let password_hash = user_auth.get_password().clone();
            let parsed_hash = PasswordHash::new(&password_hash).expect("Erro ao ");

            Argon2::default()
                .verify_password(
                    password.as_bytes(),
                    &parsed_hash,
                )
                .expect("Senha incorreta");

            let token = generate_jwt(user_auth.get_id()).expect("Erro ao gerar token");

            let cookie = Cookie::build(("access_token", token))
                .path("/")
                .secure(true)
                .http_only(true)
                .same_site(Lax)
                .build();

            cookies.add(cookie);

            return Ok(());
        }

        return Err(String::from("Usuário inválido"));
    }

    async fn refresh_token(&self, id: Uuid, token: String, cookies: Cookies) -> Result<(), String> {
        let user_refresh_token_row: Option<UserRefreshTokenRow> = sqlx::query_as(r#"
            SELECT
                id, token, expire_at
            FROM
                user_refresh_token
            WHERE
                id = $1 AND token = $2
        "#)
            .bind(id)
            .bind(token)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if user_refresh_token_row.is_none() {
            return Err("Token não encontrado, favor realizar login".to_string());
        }

        let user_refresh_token = user_refresh_token_row.unwrap();

        if user_refresh_token.expire_at < Utc::now().naive_utc() {
            return Err("Token expirado, favor realizar login".to_string());
        }

        let access_token = generate_jwt(user_refresh_token.id).expect("Erro ao gerar token");

        let cookie = Cookie::build(("access_token", access_token))
            .path("/")
            .secure(true)
            .http_only(true)
            .same_site(Lax)
            .build();

        cookies.add(cookie);

        return Ok(());
    }

    async fn alter_user_profile(
        &mut self,
        id: Uuid,
        name: String,
        bio: Option<String>,
        birth_date: NaiveDate,
        file_name: Option<String>,
        file_content: Option<Bytes>
    ) -> Result<(), String> {
        let check_user_row: Option<UserProfileRow> = sqlx::query_as(r#"
            SELECT
                up.id, up.name, up.bio, up.birth_date, up.avatar
            FROM
                user_profile AS up
            INNER JOIN
                user_auth AS au
            ON
                au.id = up.id
            WHERE
                up.id = $1 AND au.is_active = true AND au.is_email_verified = true;
        "#)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if check_user_row.is_none() {
            return Err(String::from("Usuário não encontrado"));
        }

        if file_name.is_some() && file_content.is_some() {
            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    user_image
                WHERE
                    user_id = $1;
                "#
            )
                .bind(id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                remove_file(image_row.clone().unwrap().image_path).await.unwrap();

                sqlx::query(r#"
                    DELETE FROM
                        user_image
                    WHERE
                        id = $1
                    "#
                )
                    .bind(image_row.unwrap().id)
                    .execute(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }

            let image_id = Some(Uuid::new_v4());
            let new_filename = format!("{}.png", Uuid::new_v4());

            let path = format!("./{}/user/{}", UPLOADS_IMAGE_PATH, new_filename);

            let mut file = tokio::fs::File::create(&path)
                .await
                .map_err(|e| e.to_string())?;

            file.write_all(&file_content.unwrap())
                .await
                .map_err(|e| e.to_string())?;

            sqlx::query(r#"
                INSERT INTO
                    user_image (id, original_name, image_path, user_id)
                VALUES
                    ($1, $2, $3, $4);
                "#
            )
                .bind(image_id.unwrap())
                .bind(file_name)
                .bind(path)
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        } else {
            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    user_image
                WHERE
                    user_id = $1;
                "#
            )
                .bind(id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if file_name.is_none() && file_content.is_none() && image_row.is_some() {

                remove_file(image_row.clone().unwrap().image_path).await.unwrap();

                sqlx::query(r#"
                    DELETE FROM
                        user_image
                    WHERE
                        id = $1
                    "#
                )
                    .bind(image_row.unwrap().id)
                    .execute(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }

        sqlx::query(r#"
            UPDATE
                user_profile
            SET
                name = $1, bio = $2, birth_date = $3
            WHERE
                id = $4;
        "#)
            .bind(name)
            .bind(bio)
            .bind(birth_date)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn change_password(
        &mut self,
        id: Uuid,
        password: String,
    ) -> Result<(), String> {
        let check_user_row: Option<UserAuthRow> = sqlx::query_as(r#"
            SELECT
                id, username, email, password_hash, salt, is_active, is_email_verified, created_at, updated_at
            FROM
                user_auth
            WHERE
                id = $1 AND is_active = true AND is_email_verified = true;
        "#)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if check_user_row.is_none() {
            return Err(String::from("Usuário não encontrado"));
        }

        let check_user: UserAuth = check_user_row.unwrap().into();

        let new_password_vec = derive_password_hash(password, check_user.get_salt().into_bytes());
        let new_password = String::from_utf8(new_password_vec).expect("Erro ao criptografar senha");

        let updated_at: NaiveDate = Utc::now().date_naive();

        sqlx::query(r#"
            UPDATE
                user_auth
            SET
                password_hash = $1, updated_at = $2
            WHERE
                id = $3;
        "#)
            .bind(new_password)
            .bind(updated_at)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn activate_user(&self, id: Uuid) -> Result<(), String> {
        sqlx::query(r#"
            UPDATE
                user_auth
            SET
                is_active = true
            WHERE id = $1;
        "#)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn desactivate_user(&self, id: Uuid) -> Result<(), String> {
        sqlx::query(r#"
            UPDATE
                user_auth
            SET
                is_active = false
            WHERE id = $1;
        "#)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn delete_user(&self, id: Uuid) -> Result<(), String> {
        let user_auth_row: UserAuthRow = sqlx::query_as(r#"
            SELECT
                id, username, email, password_hash, password_algorithm, is_active, is_email_verified, created_at, updated_at
            FROM
                user_auth
            WHERE
                id = $1;
        "#)
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let user_auth: UserAuth = user_auth_row.into();

        if user_auth.get_is_active() {
            return Err(String::from("O usuário ainda está ativo, para deletar a conta é necessário estar inativo"));
        }

        sqlx::query(r#"
            DELETE FROM
                user_profile
            WHERE
                id = $1;
        "#)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query(r#"
            DELETE FROM 
                user_auth
            WHERE
                id = $1;
        "#)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}