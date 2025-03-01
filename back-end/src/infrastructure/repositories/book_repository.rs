use async_trait::async_trait;
use sqlx::{PgPool, Row};
use crate::domain::entities::book::Book;
use crate::domain::irepositories::book_irepository::BookIRepository;


pub struct BookRepository {
    pub _pool: PgPool,
}

impl BookRepository {
    pub fn new(_pool: PgPool) -> Self {
        BookRepository {
            _pool
        }
    }
}

#[async_trait]
impl BookIRepository for BookRepository {
    async fn get_all_books(&self) -> Result<Vec<Book>, String> {
        let mut books: Vec<Book> = Vec::new();
        let mut rows = sqlx::query("SELECT * FROM books")
            .fetch_all(&self._pool)
            .await
            .map_err(|e| e.to_string())?;

        for row in rows.iter_mut() {
            books.push(Book::new(
                row.try_get("id").unwrap(),
                row.try_get("title").unwrap(),
                row.try_get("subtitle").unwrap(),
                row.try_get("author_id").unwrap(),
                row.try_get("publisher_id").unwrap(),
                row.try_get("series_collection").unwrap(),
                row.try_get("volume").unwrap(),
                row.try_get("edition").unwrap(),
                row.try_get("publication_year").unwrap(),
                row.try_get("pages").unwrap(),
                row.try_get("language").unwrap(),
                row.try_get("isbn").unwrap(),
                row.try_get("synopsis").unwrap(),
                row.try_get("cover").unwrap(),
                row.try_get("gender_id").unwrap(),
            ));
        }

        Ok(books)
    }

    async fn get_book_by_id(&self, id: i32) -> Result<Book, String> {
        let row = sqlx::query("SELECT * FROM books WHERE id = $1")
            .bind(id)
            .fetch_one(&self._pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(Book::new(
            row.try_get("id").unwrap(),
            row.try_get("title").unwrap(),
            row.try_get("subtitle").unwrap(),
            row.try_get("author_id").unwrap(),
            row.try_get("publisher_id").unwrap(),
            row.try_get("series_collection").unwrap(),
            row.try_get("volume").unwrap(),
            row.try_get("edition").unwrap(),
            row.try_get("publication_year").unwrap(),
            row.try_get("pages").unwrap(),
            row.try_get("language").unwrap(),
            row.try_get("isbn").unwrap(),
            row.try_get("synopsis").unwrap(),
            row.try_get("cover").unwrap(),
            row.try_get("gender_id").unwrap(),
        ));
    }

    async fn get_book_by_title(&self, title: &str) -> Result<Vec<Book>, String> {
        let mut books: Vec<Book> = Vec::new();
        let mut rows = sqlx::query("SELECT * FROM books WHERE title = $1")
            .bind(title)
            .fetch_all(&self._pool)
            .await
            .map_err(|e| e.to_string())?;

        for row in rows.iter_mut() {
            books.push(Book::new(
                row.try_get("id").unwrap(),
                row.try_get("title").unwrap(),
                row.try_get("subtitle").unwrap(),
                row.try_get("author_id").unwrap(),
                row.try_get("publisher_id").unwrap(),
                row.try_get("series_collection").unwrap(),
                row.try_get("volume").unwrap(),
                row.try_get("edition").unwrap(),
                row.try_get("publication_year").unwrap(),
                row.try_get("pages").unwrap(),
                row.try_get("language").unwrap(),
                row.try_get("isbn").unwrap(),
                row.try_get("synopsis").unwrap(),
                row.try_get("cover").unwrap(),
                row.try_get("gender_id").unwrap(),
            ));
        }

        Ok(books)
    }

    async fn create_book(&self, book: Book) -> Result<(), String> {
        let _result = sqlx::query("INSERT INTO books (title, subtitle, description, cover, author_id, publisher, gender) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(book._title)
            .bind(book._subtitle)
            .bind(book._author_id)
            .bind(book._publisher_id)
            .bind(book._series_collection)
            .bind(book._volume)
            .bind(book._edition)
            .bind(book._publication_year)
            .bind(book._pages)
            .bind(book._language)
            .bind(book._isbn)
            .bind(book._synopsis)
            .bind(book._cover)
            .bind(book._gender_id)
            .execute(&self._pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(());
    }
}