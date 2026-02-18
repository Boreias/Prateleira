use async_trait::async_trait;
use sqlx::{PgPool, Row};

use crate::domain::irepositories::igender_repository::IGenderRepository;
use crate::infrastructure::db::models::gender_row::GenderRow;
use crate::infrastructure::enums::reading_status::ReadingStatus;
use crate::domain::entities::gender::Gender;

pub struct GenderRepository {
    pool: PgPool
}

impl GenderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IGenderRepository for GenderRepository {
    async fn create_gender(
        &self,
        name: String,
        _user_id: i32,
        _books_ids: Option<Vec<i32>>
    ) -> Result<(), String> {
        let gender_row: Option<GenderRow> = sqlx::query_as("SELECT id, name FROM gender WHERE name = $1")
            .bind(name.clone())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if gender_row.is_some() {
            return Err("Gênero já cadastrado".to_string())
        }

        sqlx::query("INSERT INTO gender (name) VALUES ($1)")
            .bind(name)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_gender_by_id(&self, id: i32) -> Result<Gender, String> {
        let gender_row: GenderRow = sqlx::query_as("SELECT id, name FROM gender WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let gender: Gender = gender_row.into();

        Ok(gender)
    }

    async fn get_gender_by_name(&self, name: String, skip: i32, page_size: i32) -> Result<Vec<Gender>, String> {
        let gender_rows: Vec<GenderRow> = sqlx::query_as("SELECT id, name FROM gender WHERE name = $1 LIMIT $2 OFFSET $3")
            .bind(name)
            .bind(page_size)
            .bind(skip)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut genders: Vec<Gender> = Vec::new();

        for row in gender_rows {
            genders.push(row.into());
        }

        Ok(genders)
    }

    async fn get_genders_by_book(&self, book_id: i32, skip: i32, page_size: i32) -> Result<Vec<Gender>, String> {
        let rows = sqlx::query("SELECT gender_id FROM book_gender WHERE book_id = $1")
            .bind(book_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut genders: Vec<Gender> = Vec::new();

        for row in rows {

            let id: i32 = row.get("gender_id");

            let gender_row: GenderRow = sqlx::query_as("SELECT id, name FROM gender WHERE id = $1 LIMIT $2 OFFSET $3")
                .bind(id)
                .bind(page_size)
                .bind(skip)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            let gender: Gender = gender_row.into();

            genders.push(gender);
        }

        Ok(genders)
    }

    async fn get_genders_by_author(&self, author_id: i32, skip: i32, page_size: i32) -> Result<Vec<Gender>, String> {
        let book_rows = sqlx::query("SELECT book_id FROM book_author WHERE author_id = $1")
            .bind(author_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut genders: Vec<Gender> = Vec::new();

        for book_row in book_rows {

            let book_id: i32 = book_row.get("book_id");

            let gender_rows = sqlx::query("SELECT gender_id FROM book_gender WHERE book_id = $1")
                .bind(book_id)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            for row in gender_rows {
                let id: i32 = row.get("gender_id");

                let gender_row: GenderRow = sqlx::query_as("SELECT id, name FROM gender WHERE id = $1 LIMIT $2 OFFSET $3")
                    .bind(id)
                    .bind(page_size)
                    .bind(skip)
                    .fetch_one(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                let gender: Gender = gender_row.into();

                genders.push(gender);
            }
        }

        Ok(genders)
    }

    async fn get_genders_by_publisher(&self, publisher_id: i32, skip: i32, page_size: i32) -> Result<Vec<Gender>, String> {
        let book_rows = sqlx::query("SELECT book_id FROM book_publisher WHERE publisher_id = $1")
            .bind(publisher_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut genders: Vec<Gender> = Vec::new();

        for book_row in book_rows {

            let book_id: i32 = book_row.get("book_id");

            let gender_rows = sqlx::query("SELECT gender_id FROM book_gender WHERE book_id = $1")
                .bind(book_id)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            for row in gender_rows {
                let id: i32 = row.get("gender_id");

                let gender_row: GenderRow = sqlx::query_as("SELECT id, name FROM gender WHERE id = $1 LIMIT $2 OFFSET $3")
                    .bind(id)
                    .bind(page_size)
                    .bind(skip)
                    .fetch_one(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                let gender: Gender = gender_row.into();

                genders.push(gender);
            }
        }

        Ok(genders)
    }

    async fn more_popular_gender(
        &self,
        skip: i32,
        page_size: i32
    ) -> Result<Vec<Gender>, String> {
        let book_rows = sqlx::query(r#"
        SELECT
            book_id,
            COUNT(user_id) as readed_book
        FROM book_user
        WHERE reading_status = $1
        GROUP BY book_id
        ORDER BY readed_book DESC
        LIMIT $2
        OFFSET $3
        "#)
            .bind(ReadingStatus::Lido as i32)
            .bind(page_size)
            .bind(skip)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut genders: Vec<Gender> = Vec::new();

        for book in book_rows {
            let book_id: i32 = book.get("book_id");
            let genders_book = sqlx::query(r#"
            SELECT
                gender_id
            FROM book_gender
            WHERE book_id = $1
            LIMIT $2
            OFFSET $3
            "#)
            .bind(book_id)
            .bind(page_size)
            .bind(skip)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            let genders_id: Vec<i32> = genders_book.iter().map(|row| {
                row.get("id")
            }).collect();

            let param = format!("?{}", ", ?".repeat(genders_id.len()-1));

            let gender_rows: Vec<GenderRow> = sqlx::query_as(r#"
            SELECT
                id, name
            FROM gender
            WHERE id IN ($1)
            LIMIT $2
            OFFSET $3
            "#)
            .bind(param)
            .bind(page_size)
            .bind(skip)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            for row in gender_rows {
                let gender: Gender = row.into();
                genders.push(gender);
            }
        }


        Ok(genders)
    }

    async fn best_valuated_gender(
        &self,
        skip: i32,
        page_size: i32
    ) -> Result<Vec<Gender>, String> {
        let gender_rows = sqlx::query(r#"
        SELECT
            g.id,
            g.name,
            AVG(br.review)::float8 AS genre_average,
            COUNT(br.review) AS total_reviews
        FROM genre g
        JOIN book b ON b.genre_id = g.id
        JOIN book_review br ON br.book_id = b.id
        GROUP BY g.id, g.name
        ORDER BY genre_average DESC
        LIMIT $1
        OFFSET $2
        "#)
            .bind(page_size)
            .bind(skip)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut genders: Vec<Gender> = Vec::new();

        for row in gender_rows {
            genders.push(Gender::new(row.get("g.id"), row.get("g.name")));
        }

        Ok(genders)
    }

    async fn alter_gender(
        &mut self,
        id: i32,
        name: String,
        _user_id: i32,
        _books_ids: Option<Vec<i32>>
    ) -> Result<(), String> {
        sqlx::query("UPDATE gender SET name = $2 WHERE id = $1")
            .bind(id)
            .bind(name)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn delete_gender(&self, id: i32) -> Result<(), String> {
        sqlx::query("DELETE FROM gender WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}