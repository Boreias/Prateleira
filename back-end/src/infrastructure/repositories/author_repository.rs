use async_trait::async_trait;
use sqlx::{PgPool, Row};
use crate::domain::entities::author::Author;
use crate::domain::irepositories::author_irepository::AuthorIRepository;


pub struct AuthorRepository {
    pub _pool: PgPool,
}

impl AuthorRepository {
    pub fn new(_pool: PgPool) -> Self {
        AuthorRepository {
            _pool
        }
    }
}

#[async_trait]
impl AuthorIRepository for AuthorRepository {
    async fn get_all_authors(&self) -> Result<Vec<Author>, String> {
        let mut authors: Vec<Author> = Vec::new();
        let mut rows = sqlx::query("SELECT * FROM authors")
            .fetch_all(&self._pool)
            .await
            .map_err(|e| e.to_string())?;

        for row in rows.iter_mut() {
            authors.push(Author::new(
                row.try_get("id").unwrap(),
                row.try_get("name").unwrap(),
                row.try_get("avatar").unwrap(),
                Vec::new()
            ));
        }

        Ok(authors)
    }

    async fn get_author_by_id(&self, id: i32) -> Result<Author, String> {
        let row = sqlx::query("SELECT * FROM authors WHERE id = $1")
            .bind(id)
            .fetch_one(&self._pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(Author::new(
            row.try_get("id").unwrap(),
            row.try_get("name").unwrap(),
            row.try_get("avatar").unwrap(),
            Vec::new()
        ))
    }

    async fn get_author_by_name(&self, name: &str) -> Result<Vec<Author>, String> {
        let mut authors: Vec<Author> = Vec::new();
        let mut rows = sqlx::query("SELECT * FROM authors WHERE name = $1")
            .bind(name)
            .fetch_all(&self._pool)
            .await
            .map_err(|e| e.to_string())?;

        for row in rows.iter_mut() {
            authors.push(Author::new(
                row.try_get("id").unwrap(),
                row.try_get("name").unwrap(),
                row.try_get("avatar").unwrap(),
                Vec::new()
            ));
        }

        Ok(authors)
    }

    async fn create_author(&self, author: Author) -> Result<(), String> {
        let _ = sqlx::query("INSERT INTO authors (name, avatar) VALUES ($1, $2)")
            .bind(author._name)
            .bind(author._avatar)
            .execute(&self._pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}