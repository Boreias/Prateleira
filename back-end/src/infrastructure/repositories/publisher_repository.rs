use async_trait::async_trait;
use sqlx::{PgPool, Row};
use crate::domain::entities::publisher::Publisher;
use crate::domain::irepositories::publisher_irepository::PublisherIRepository;


pub struct PublisherRepository {
    pub _pool: PgPool,
}


impl PublisherRepository {
    pub fn new(_pool: PgPool) -> Self {
        PublisherRepository {
            _pool
        }
    }
}


#[async_trait]
impl PublisherIRepository for PublisherRepository {
    async fn get_all_publishers(&self) -> Result<Vec<Publisher>, String> {
        let mut publishers: Vec<Publisher> = Vec::new();
        let mut rows = sqlx::query("SELECT * FROM publishers")
            .fetch_all(&self._pool)
            .await
            .map_err(|e| e.to_string())?;

        for row in rows.iter_mut() {
            publishers.push(Publisher::new(
                row.try_get("id").unwrap(),
                row.try_get("name").unwrap(),
                row.try_get("avatar").unwrap(),
                Vec::new()
            ));
        }

        Ok(publishers)
    }

    async fn get_publisher_by_id(&self, id: i32) -> Result<Publisher, String> {
        let row = sqlx::query("SELECT * FROM publishers WHERE id = $1")
            .bind(id)
            .fetch_one(&self._pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(Publisher::new(
            row.try_get("id").unwrap(),
            row.try_get("name").unwrap(),
            row.try_get("avatar").unwrap(),
            Vec::new()
        ))
    }

    async fn get_publisher_by_name(&self, name: &str) -> Result<Vec<Publisher>, String> {
        let mut publishers: Vec<Publisher> = Vec::new();
        let mut rows = sqlx::query("SELECT * FROM publishers WHERE name = $1")
            .bind(name)
            .fetch_all(&self._pool)
            .await
            .map_err(|e| e.to_string())?;

        for row in rows.iter_mut() {
            publishers.push(Publisher::new(
                row.try_get("id").unwrap(),
                row.try_get("name").unwrap(),
                row.try_get("avatar").unwrap(),
                Vec::new()
            ));
        }

        Ok(publishers)
    }

    async fn create_publisher(&self, publisher: Publisher) -> Result<(), String> {
        sqlx::query("INSERT INTO publishers (name, avatar) VALUES ($1, $2)")
            .bind(publisher._name)
            .bind(publisher._avatar)
            .execute(&self._pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}