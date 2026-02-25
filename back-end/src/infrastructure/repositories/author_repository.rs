use async_trait::async_trait;
use uuid::Uuid;
use sqlx::{PgPool, Row};

use crate::domain::irepositories::iauthor_repository::IAuthorRepository;
use crate::domain::entities::author::Author;
use crate::infrastructure::db::models::author_row::AuthorRow;
use crate::infrastructure::db::models::book_author_row::BookAuthorRow;
use crate::infrastructure::db::models::book_gender_row::BookGenderRow;
use crate::infrastructure::db::models::book_publisher_row::BookPublisherRow;
use crate::infrastructure::enums::reading_status::ReadingStatus;


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
        _user_id: Uuid,
        _books: Option<Vec<Uuid>>,
    ) -> Result<(), String> {
        sqlx::query("INSERT INTO author (name, avatar) VALUES ($1, $2)")
            .bind(name)
            .bind(avatar)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_author_by_id(&self, id: Uuid) -> Result<Author, String> {
        let author_row: AuthorRow = sqlx::query_as("SELECT id, name FROM author WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let author: Author = author_row.into();

        Ok(author)
    }

    async fn get_author_by_name(&self, name: String, skip: i32, page_size: i32) -> Result<Vec<Author>, String> {
        let rows: Vec<AuthorRow> = sqlx::query_as("SELECT id, name, avatar FROM author WHERE name = $1 LIMIT $2 OFFSET $3")
            .bind(name)
            .bind(page_size)
            .bind(skip)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut authors = Vec::new();

        for row in rows {
            authors.push(row.into());
        }

        Ok(authors)
    }

    async fn get_authors_by_book(&self, book_id: Uuid, skip: i32, page_size: i32) -> Result<Vec<Author>, String> {
        let book_author_rows: Vec<BookAuthorRow> = sqlx::query_as("SELECT author_id FROM book_author WHERE book_id = $1")
            .bind(book_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut authors = Vec::new();

        for book_author_row in book_author_rows {        
            let author_rows: Vec<AuthorRow> = sqlx::query_as("SELECT id, name, avatar FROM Author WHERE id = $1 LIMIT $2 OFFSET $3")
                .bind(book_author_row.author_id)
                .bind(page_size)
                .bind(skip)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            for author_row in author_rows {
                authors.push(author_row.into());
            }
        }

        Ok(authors)
    }

    async fn get_authors_by_gender(&self, gender_id: Uuid, skip: i32, page_size: i32) -> Result<Vec<Author>, String> {
        let book_gender_rows: Vec<BookGenderRow> = sqlx::query_as(r#"
            SELECT
                book_id
            FROM
                book_gender
            WHERE
                gender_id = $1
            LIMIT $2
            OFFSET $3
            "#
        )
            .bind(gender_id)
            .bind(page_size)
            .bind(skip)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut authors = Vec::new();

        for book_gender_row in book_gender_rows {
            let author_rows: Vec<BookAuthorRow> = sqlx::query_as(r#"
                SELECT
                    id, book_id, author_id
                FROM
                    book_author
                WHERE
                    book_id = $1
                "#
            )
                .bind(book_gender_row.book_id)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            for author_row in author_rows {
                let author: AuthorRow = sqlx::query_as(r#"
                    SELECT
                        id, name, avatar
                    FROM
                        author
                    WHERE
                        id = $1
                    "#
                )
                    .bind(author_row.author_id)
                    .fetch_one(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                authors.push(author.into());
            }
        }

        Ok(authors)
    }

    async fn get_authors_by_publisher(&self, publisher_id: Uuid, skip: i32, page_size: i32) -> Result<Vec<Author>, String> {
        let book_publisher_rows: Vec<BookPublisherRow> = sqlx::query_as("SELECT book_id FROM book_publisher WHERE publisher_id = $1")
            .bind(publisher_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut authors: Vec<Author> = Vec::new();

        for book_publisher_row in book_publisher_rows {

            let book_id = book_publisher_row.book_id;

            let author_rows: Vec<BookAuthorRow> = sqlx::query_as("SELECT author_id FROM book_author WHERE book_id = $1")
                .bind(book_id)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            for row in author_rows {
                let id = row.author_id;

                let author_row: AuthorRow = sqlx::query_as("SELECT id, name, avatar FROM author WHERE id = $1 LIMIT $2 OFFSET $3")
                    .bind(id)
                    .bind(page_size)
                    .bind(skip)
                    .fetch_one(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                let author: Author = author_row.into();

                authors.push(author);
            }
        }

        Ok(authors)
    }

    async fn more_popular_author(
        &self,
        skip: i32,
        page_size: i32
    ) -> Result<Vec<Author>, String> {
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
            "#
        )
            .bind(ReadingStatus::Lido as i32)
            .bind(page_size)
            .bind(skip)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut authors: Vec<Author> = Vec::new();

        for book in book_rows {
            let book_id: Uuid = book.get("book_id");
            let authors_book = sqlx::query(r#"
                SELECT
                    author_id
                FROM book_author
                WHERE book_id = $1
                LIMIT $2
                OFFSET $3
                "#
            )
            .bind(book_id)
            .bind(page_size)
            .bind(skip)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            let authors_id: Vec<Uuid> = authors_book.iter().map(|row| {
                row.get("id")
            }).collect();

            let param = format!("?{}", ", ?".repeat(authors_id.len()-1));

            let author_rows: Vec<AuthorRow> = sqlx::query_as(r#"
                SELECT
                    id, name
                FROM author
                WHERE id IN ($1) AND deleted = false
                LIMIT $2
                OFFSET $3
            "#
        )
            .bind(param)
            .bind(page_size)
            .bind(skip)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            for row in author_rows {
                let author: Author = row.into();
                authors.push(author);
            }
        }

        Ok(authors)
    }

    async fn best_valuated_author(
        &self,
        skip: i32,
        page_size: i32
    ) -> Result<Vec<Author>, String> {
        let author_rows = sqlx::query(r#"
            SELECT
                a.id,
                a.name,
                a.avatar
                AVG(br.review)::float8 AS author_average,
                COUNT(br.review) AS total_reviews
            FROM author a
            WHERE a.deleted = false
            JOIN book b ON b.author_id = a.id
            JOIN book_review br ON br.book_id = b.id
            GROUP BY a.id, a.name
            ORDER BY author_average DESC
            LIMIT $1
            OFFSET $2
            "#
        )
            .bind(page_size)
            .bind(skip)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut authors: Vec<Author> = Vec::new();

        for row in author_rows {
            authors.push(Author::new(row.get("a.id"), row.get("a.name"), row.get("a.avatar"), None));
        }

        Ok(authors)
    }

    async fn alter_author(
        &mut self,
        id: Uuid,
        name: String,
        avatar: String,
        _user_id: Uuid,
        _books: Option<Vec<Uuid>>
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

    async fn delete_author(&self, id: Uuid, _user_id: Uuid) -> Result<(), String> {
        sqlx::query(r#"
            UPDATE
                author
            SET
                deleted = true
            WHERE
                id = $1
            "#
        )
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn clear_deleted_authors(&self) -> Result<(), String> {
        sqlx::query(r#"
            DELETE FROM
                author
            WHERE deleted = true
            "#
        )
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}