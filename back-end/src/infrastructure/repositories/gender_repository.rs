use async_trait::async_trait;
use sqlx::{PgPool, Row};
use crate::domain::entities::gender::Gender;
use crate::domain::irepositories::gender_irepository::GenderIRepository;


pub struct GenderRepository {
    pub _pool: PgPool,
}


impl GenderRepository {
    pub fn new(_pool: PgPool) -> Self {
        GenderRepository {
            _pool
        }
    }
}


#[async_trait]
impl GenderIRepository for GenderRepository {
    async fn get_all_genders (&self) -> Result<Vec<Gender>, String> {

        let mut genders: Vec<Gender> = Vec::new();
        let mut rows = sqlx::query("SELECT * FROM genders")
            .fetch_all(&self._pool)
            .await
            .map_err(|e| e.to_string())?;

        for row in rows.iter_mut() {
            genders.push(Genger::new(
                row.try_get("id").unwrap(),
                row.try_get("name").unwrap(),
                row.try_get("books").unwrap(),
            ));
        }

        Ok(genders);
    }

    async fn get_gender_by_id (&self, id: i32) -> Result<Gender, String> {
        let row = sqlx::query
            .bind(id)
            .fetch_one(&self._pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(Gender::new(
            row.try_get("id").unwrap(),
            row.try_get("name").unwrap(),
            row.try_get("books").unwrap(),
        ))
    }

    async fn create_gender (&self, gender: Gender) -> Result<(), String> {
        let _ = sqlx::query("INSERT INTO genders (name) VALUES ($1)")
            .bind(gender._name)
            .execute(&self._pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}