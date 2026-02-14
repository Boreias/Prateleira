use serde_json::json;
use uuid::Uuid;
use chrono::{NaiveDate, Utc};
use sqlx::{PgPool, Row};
use async_trait::async_trait;
use std::env;
use serde_json;

use crate::domain::irepositories::iauthor_repository::IAuthorRepository;
use crate::domain::entities::book::Book;
use crate::domain::entities::author::Author;
use crate::infrastructure::db::models::author_row::AuthorRow;


pub struct AuthorRepository {
    pool: PgPool
}

impl AuthorRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}


#[async_trait]
impl IAuthorRepository for AuthorRepository {
    async fn create_author(
        &self,
        name: String,
        avatar: String,
        books: Option<Vec<Book>>
    ) -> Result<(), String> {
        let result = sqlx::query("INSERT INTO author (name, avatar) VALUES ($1, $2)")
            .bind(name)
            .bind(avatar)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_author_by_name(&self, name: String) -> Result<Option<Vec<Author>>, String> {
        let rows = sqlx::query("SELECT id, name, avatar FROM author WHERE name = $1")
            .bind(name)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut authors = Vec::new();

        for row in rows {
            let author_row = AuthorRow {
                id: row.get("id"),
                name: row.get("name"),
                avatar: row.get("avatar"),
            };

            let author: Author = author_row.into();

            authors.push(author);
        }

        Ok(Some(authors))
    }

    async fn get_authors_by_book(&self, book_id: i32) -> Result<Option<Vec<Author>>, String> {
        let book_row = sqlx::query("SELECT author_id FROM book_author WHERE book_id = $1")
            .bind(book_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(book_row) = book_row {

            let author_id: String = book_row.get("author_id");
            
            let rows = sqlx::query("SELECT id, name, avatar FROM Author WHERE id = $1")
                .bind(author_id)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            let mut authors = Vec::new();

            for row in rows {
                let author_row = AuthorRow {
                    id: row.get("id"),
                    name: row.get("name"),
                    avatar: row.get("avatar"),
                };

                let author: Author = author_row.into();

                authors.push(author);
            }

            Ok(Some(authors))
        } else {
            Ok(None)
        }
    }

    async fn alter_author(
        &mut self,
        id: String,
        name: String,
        avatar: String,
        books: Option<Vec<Book>>
    ) -> Result<(), String> {
        sqlx::query("UPDATE author SET name = $2, avatar = $3 WHERE id = $1")
            .bind(id)
            .bind(name)
            .bind(avatar)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn delete_author(&self, id: String) -> Result<(), String> {
        sqlx::query("DELETE FROM author WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}