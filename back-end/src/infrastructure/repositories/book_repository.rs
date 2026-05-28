use async_trait::async_trait;
use axum::body::Bytes;
use tokio::{
    io::AsyncWriteExt,
    fs::remove_file
};
use uuid::Uuid;
use sqlx::{PgPool, Row};

use crate::domain::entities::book::Book;
use crate::domain::entities::author::Author;
use crate::domain::entities::gender::Gender;
use crate::domain::entities::publisher::Publisher;
use crate::domain::irepositories::ibook_repository::IBookRepository;
use crate::infrastructure::db::models::book_row::BookRow;
use crate::infrastructure::db::models::publisher_row::PublisherRow;
use crate::infrastructure::db::models::book_gender_row::BookGenderRow;
use crate::infrastructure::db::models::gender_row::GenderRow;
use crate::infrastructure::db::models::author_row::AuthorRow;
use crate::infrastructure::db::models::book_author_row::BookAuthorRow;
use crate::infrastructure::db::models::image_row::ImageRow;
use crate::infrastructure::enums::reading_status::ReadingStatus;
use crate::infrastructure::common::consts::UPLOADS_IMAGE_PATH;



pub struct BookRepository {
    pool: PgPool
}

impl BookRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}


#[async_trait]
impl IBookRepository for BookRepository {
    async fn create_book (
        &self,
        title: String,
        subtitle: Option<String>,
        authors_id: Vec<Uuid>,
        publisher_id: Uuid,
        series_collection: Option<i32>,
        volume: Option<i32>,
        edition: Option<i32>,
        publication_year: Option<i32>,
        pages: Option<i32>,
        language: Option<String>,
        isbn: String,
        genders_id: Vec<Uuid>,
        synopsis: Option<String>,
        file_name: Option<String>,
        file_content: Option<Bytes>,
        _user_id: Uuid
    ) -> Result<(), String> {
        let book_id = Uuid::new_v4();
        sqlx::query(r#"
            INSERT INTO
                book (id, title, subtitle, publisher_id, series_collection, volume, edition, publication_year, pages, language, isbn, synopsis)
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
            "#
        )
            .bind(book_id)
            .bind(title)
            .bind(subtitle)
            .bind(publisher_id)
            .bind(series_collection)
            .bind(volume)
            .bind(edition)
            .bind(publication_year)
            .bind(pages)
            .bind(language)
            .bind(isbn)
            .bind(synopsis)
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
                    book_image (id, original_name, image_path, book_id)
                VALUES
                    ($1, $2, $3, $4);
                "#
            )
                .bind(image_id)
                .bind(file_name)
                .bind(path)
                .bind(book_id)
                .execute(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        }

        for gender_id in genders_id {
            let book_gender_id = Uuid::new_v4();
            sqlx::query(r#"
                    INSERT INTO
                        book_gender (id, book_id, gender_id)
                    VALUES
                        ($1, $2, $3)
                "#
            )
                .bind(book_gender_id)
                .bind(book_id)
                .bind(gender_id)
                .execute(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        }

        for author_id in authors_id {
            let book_author_id = Uuid::new_v4();
            sqlx::query(r#"
                    INSERT INTO
                        book_author (id, book_id, author_id)
                    VALUES
                        ($1, $2, $3)
                "#
            )
                .bind(book_author_id)
                .bind(book_id)
                .bind(author_id)
                .execute(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    async fn get_book_by_id (&self, book_id: Uuid) -> Result<Book, String> {
        let book_row: BookRow = sqlx::query_as(r#"
                SELECT
                    id, title, subtitle, publisher_id, series_collection, volume, edition, publication_year, pages, language, isbn, synopsis
                FROM
                    book
                WHERE
                    id = $1 AND deleted = false;
            "#
        )
            .bind(book_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut cover: Option<String> = None;

        let image_row: Option<ImageRow> = sqlx::query_as(r#"
            SELECT
                id, original_name, image_path
            FROM
                book_image
            WHERE
                book_id = $1 AND deleted = false;
            "#
        )
            .bind(book_row.id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if image_row.is_some() {
            cover = Some(image_row.unwrap().image_path);
        }


        let mut publisher: Publisher;

        let publisher_row: PublisherRow = sqlx::query_as(r#"
                SELECT
                    id, name, site, email
                FROM
                    publisher
                WHERE
                    id = $1 AND deleted = false;
            "#
        )
        .bind(book_row.publisher_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut avatar: Option<String> = None;

        let image_row: Option<ImageRow> = sqlx::query_as(r#"
            SELECT
                id, original_name, image_path
            FROM
                publisher_image
            WHERE
                publisher_id = $1 AND deleted = false;
            "#
        )
            .bind(publisher_row.id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if image_row.is_some() {
            avatar = Some(image_row.unwrap().image_path);
        }

        publisher = publisher_row.into();
        publisher.set_avatar(avatar);


        let mut authors: Vec<Author> = Vec::new();

        let book_authors_row: Vec<BookAuthorRow> = sqlx::query_as(r#"
                SELECT
                    id, book_id, author_id
                FROM
                    book_author
                WHERE
                    book_id = $1 AND deleted = false;
            "#
        )
            .bind(book_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        for book_author_row in book_authors_row {
            let author_row: AuthorRow = sqlx::query_as(r#"
                    SELECT
                        id, name
                    FROM
                        author
                    WHERE
                        id = $1 AND deleted = false;
                "#
            )
            .bind(book_author_row.author_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            let mut avatar: Option<String> = None;

            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    author_image
                WHERE
                    author_id = $1 AND deleted = false;
                "#
            )
                .bind(author_row.id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                avatar = Some(image_row.unwrap().image_path);
            }

            let mut author: Author = author_row.into();
            author.set_avatar(avatar);

            authors.push(author);
        }


        let mut genders: Vec<Gender> = Vec::new();

        let book_genders_row: Vec<BookGenderRow> = sqlx::query_as(r#"
                SELECT
                    id, book_id, gender_id
                FROM
                    book_gender
                WHERE
                    book_id = $1 AND deleted = false;
            "#
        )
        .bind(book_id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        for book_gender_row in book_genders_row {
            let gender_row: GenderRow = sqlx::query_as(r#"
                    SELECT
                        id, name
                    FROM
                        gender
                    WHERE
                        id = $1 AND deleted = false;
                "#
            )
            .bind(book_gender_row.gender_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            let gender: Gender = gender_row.into();

            genders.push(gender);
        }

        let mut book: Book = book_row.into();
        book.set_cover(cover);

        book.set_publisher(Some(publisher));
        book.set_authors(authors);
        book.set_genders(genders);

        Ok(book)
    }

    async fn get_books_by_name (&self, book_name: String, skip: i32, page_size: i32) -> Result<Vec<Book>, String> {
        let format_name = format!("%{}%", book_name);
        let book_rows: Vec<BookRow> = sqlx::query_as(r#"
                SELECT
                    id, title, subtitle, publisher_id, series_collection, volume, edition, publication_year, pages, language, isbn, synopsis
                FROM
                    book
                WHERE
                    title LIKE $1 AND deleted = false
                LIMIT $2
                OFFSET $3;
            "#
        )
            .bind(format_name)
            .bind(page_size)
            .bind(skip)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut books: Vec<Book> = Vec::new();

        for book_row in book_rows {
            let mut cover: Option<String> = None;

            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    book_image
                WHERE
                    book_id = $1 AND deleted = false;
                "#
            )
                .bind(book_row.id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                cover = Some(image_row.unwrap().image_path);
            }


            let mut publisher: Publisher;

            let publisher_row: PublisherRow = sqlx::query_as(r#"
                    SELECT
                        id, name, site, email
                    FROM
                        publisher
                    WHERE
                        id = $1 AND deleted = false;
                "#
            )
            .bind(book_row.publisher_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            let mut avatar: Option<String> = None;

            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    publisher_image
                WHERE
                    publisher_id = $1 AND deleted = false;
                "#
            )
                .bind(publisher_row.id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                avatar = Some(image_row.unwrap().image_path);
            }

            publisher = publisher_row.into();
            publisher.set_avatar(avatar);


            let mut authors: Vec<Author> = Vec::new();

            let book_authors_row: Vec<BookAuthorRow> = sqlx::query_as(r#"
                    SELECT
                        id, book_id, author_id
                    FROM
                        book_author
                    WHERE
                        book_id = $1 AND deleted = false;
                "#
            )
                .bind(book_row.id)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            for book_author_row in book_authors_row {
                let author_row: AuthorRow = sqlx::query_as(r#"
                        SELECT
                            id, name
                        FROM
                            author
                        WHERE
                            id = $1 AND deleted = false;
                    "#
                )
                .bind(book_author_row.author_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

                let mut avatar: Option<String> = None;

                let image_row: Option<ImageRow> = sqlx::query_as(r#"
                    SELECT
                        id, original_name, image_path
                    FROM
                        author_image
                    WHERE
                        author_id = $1 AND deleted = false;
                    "#
                )
                    .bind(author_row.id)
                    .fetch_optional(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                if image_row.is_some() {
                    avatar = Some(image_row.unwrap().image_path);
                }

                let mut author: Author = author_row.into();
                author.set_avatar(avatar);

                authors.push(author);
            }


            let mut genders: Vec<Gender> = Vec::new();

            let book_genders_row: Vec<BookGenderRow> = sqlx::query_as(r#"
                    SELECT
                        id, book_id, gender_id
                    FROM
                        book_gender
                    WHERE
                        book_id = $1 AND deleted = false;
                "#
            )
            .bind(book_row.id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            for book_gender_row in book_genders_row {
                let gender_row: GenderRow = sqlx::query_as(r#"
                        SELECT
                            id, name
                        FROM
                            gender
                        WHERE
                            id = $1 AND deleted = false;
                    "#
                )
                .bind(book_gender_row.gender_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

                let gender: Gender = gender_row.into();

                genders.push(gender);
            }

            let mut book: Book = book_row.into();
            book.set_cover(cover);

            book.set_publisher(Some(publisher));
            book.set_authors(authors);
            book.set_genders(genders);

            books.push(book);
        }

        Ok(books)
    }

    async fn get_book_by_isbn (&self, isbn: String) -> Result<Book, String> {
        let book_row: BookRow = sqlx::query_as(r#"
                SELECT
                    id, title, subtitle, publisher_id, series_collection, volume, edition, publication_year, pages, language, isbn, synopsis
                FROM
                    book
                WHERE
                    isbn = $1 AND deleted = false;
            "#
        )
            .bind(isbn)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut cover: Option<String> = None;

        let image_row: Option<ImageRow> = sqlx::query_as(r#"
            SELECT
                id, original_name, image_path
            FROM
                book_image
            WHERE
                book_id = $1 AND deleted = false;
            "#
        )
            .bind(book_row.id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if image_row.is_some() {
            cover = Some(image_row.unwrap().image_path);
        }


        let mut publisher: Publisher;

        let publisher_row: PublisherRow = sqlx::query_as(r#"
                SELECT
                    id, name, site, email
                FROM
                    publisher
                WHERE
                    id = $1 AND deleted = false;
            "#
        )
        .bind(book_row.publisher_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut avatar: Option<String> = None;

        let image_row: Option<ImageRow> = sqlx::query_as(r#"
            SELECT
                id, original_name, image_path
            FROM
                publisher_image
            WHERE
                publisher_id = $1 AND deleted = false;
            "#
        )
            .bind(publisher_row.id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if image_row.is_some() {
            avatar = Some(image_row.unwrap().image_path);
        }

        publisher = publisher_row.into();
        publisher.set_avatar(avatar);


        let mut authors: Vec<Author> = Vec::new();

        let book_authors_row: Vec<BookAuthorRow> = sqlx::query_as(r#"
                SELECT
                    (id, book_id, author_id)
                FROM
                    book_author
                WHERE
                    book_id = $1 AND deleted = false;
            "#
        )
            .bind(book_row.id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        for book_author_row in book_authors_row {
            let author_row: AuthorRow = sqlx::query_as(r#"
                    SELECT
                        (id, name)
                    FROM
                        author
                    WHERE
                        id = $1 AND deleted = false;
                "#
            )
            .bind(book_author_row.author_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            let mut avatar: Option<String> = None;

            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    author_image
                WHERE
                    author_id = $1 AND deleted = false;
                "#
            )
                .bind(author_row.id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                avatar = Some(image_row.unwrap().image_path);
            }

            let mut author: Author = author_row.into();
            author.set_avatar(avatar);

            authors.push(author);
        }


        let mut genders: Vec<Gender> = Vec::new();

        let book_genders_row: Vec<BookGenderRow> = sqlx::query_as(r#"
                SELECT
                    id, book_id, gender_id
                FROM
                    book_gender
                WHERE
                    book_id = $1 AND deleted = false;
            "#
        )
        .bind(book_row.id)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        for book_gender_row in book_genders_row {
            let gender_row: GenderRow = sqlx::query_as(r#"
                    SELECT
                        id, name
                    FROM
                        gender
                    WHERE
                        id = $1 AND deleted = false;
                "#
            )
            .bind(book_gender_row.gender_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            let gender: Gender = gender_row.into();

            genders.push(gender);
        }

        let mut book: Book = book_row.into();
        book.set_cover(cover);

        book.set_publisher(Some(publisher));
        book.set_authors(authors);
        book.set_genders(genders);

        Ok(book)
    }

    async fn get_books_by_author(&self, author_id: Uuid, skip: i32, page_size: i32) -> Result<Vec<Book>, String> {
        let book_author_rows: Vec<BookAuthorRow> = sqlx::query_as(r#"
                SELECT
                    id, book_id, author_id
                FROM
                    book_author
                WHERE
                    author_id = $1 AND deleted = false
                LIMIT $2
                OFFSET $3;
            "#
        )
        .bind(author_id)
        .bind(page_size)
        .bind(skip)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut books: Vec<Book> = Vec::new();

        for book_author_row in book_author_rows {
            let book_row: BookRow = sqlx::query_as(r#"
                    SELECT
                        id, title, subtitle, publisher_id, series_collection, volume, edition, publication_year, pages, language, isbn, synopsis
                    FROM
                        book
                    WHERE
                        id = $1 AND deleted = false;
                "#
            )
                .bind(book_author_row.book_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            let mut cover: Option<String> = None;

            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    book_image
                WHERE
                    book_id = $1 AND deleted = false;
                "#
            )
                .bind(book_row.id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                cover = Some(image_row.unwrap().image_path);
            }


            let mut publisher: Publisher;

            let publisher_row: PublisherRow = sqlx::query_as(r#"
                    SELECT
                        id, name, site, email
                    FROM
                        publisher
                    WHERE
                        id = $1 AND deleted = false;
                "#
            )
            .bind(book_row.publisher_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            let mut avatar: Option<String> = None;

            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    publisher_image
                WHERE
                    publisher_id = $1 AND deleted = false;
                "#
            )
                .bind(publisher_row.id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                avatar = Some(image_row.unwrap().image_path);
            }

            publisher = publisher_row.into();
            publisher.set_avatar(avatar);


            let mut authors: Vec<Author> = Vec::new();

            let book_authors_rows: Vec<BookAuthorRow> = sqlx::query_as(r#"
                    SELECT
                        id, book_id, author_id
                    FROM
                        book_author
                    WHERE
                        book_id = $1 AND deleted = false;
                "#
            )
                .bind(book_author_row.book_id)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            for book_author_intern_row in book_authors_rows {
                let author_row: AuthorRow = sqlx::query_as(r#"
                        SELECT
                            id, name
                        FROM
                            author
                        WHERE
                            id = $1 AND deleted = false;
                    "#
                )
                .bind(book_author_intern_row.author_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

                let mut avatar: Option<String> = None;

                let image_row: Option<ImageRow> = sqlx::query_as(r#"
                    SELECT
                        id, original_name, image_path
                    FROM
                        author_image
                    WHERE
                        author_id = $1 AND deleted = false;
                    "#
                )
                    .bind(author_row.id)
                    .fetch_optional(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                if image_row.is_some() {
                    avatar = Some(image_row.unwrap().image_path);
                }

                let mut author: Author = author_row.into();
                author.set_avatar(avatar);

                authors.push(author);
            }


            let mut genders: Vec<Gender> = Vec::new();

            let book_genders_row: Vec<BookGenderRow> = sqlx::query_as(r#"
                    SELECT
                        id, book_id, gender_id
                    FROM
                        book_gender
                    WHERE
                        book_id = $1 AND deleted = false;
                "#
            )
            .bind(book_author_row.book_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            for book_gender_row in book_genders_row {
                let gender_row: GenderRow = sqlx::query_as(r#"
                        SELECT
                            id, name
                        FROM
                            gender
                        WHERE
                            id = $1 AND deleted = false;
                    "#
                )
                .bind(book_gender_row.gender_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

                let gender: Gender = gender_row.into();

                genders.push(gender);
            }

            let mut book: Book = book_row.into();
            book.set_cover(cover);

            book.set_publisher(Some(publisher));
            book.set_authors(authors);
            book.set_genders(genders);

            books.push(book);
        }

        Ok(books)
    }

    async fn best_valuated_books_by_author(&self, author_id: Uuid, skip: i32, page_size: i32) -> Result<Vec<Book>, String> {
        let book_id_rows = sqlx::query(r#"
            SELECT
                b.id,
                AVG(br.review)::float8 AS author_average,
                COUNT(br.review) AS total_reviews
            FROM author a
            WHERE a.id = $1 AND a.deleted = false
            JOIN book_author ba ON ba.author_id = a.id
            JOIN book b ON b.id = ba.book_id
            JOIN book_review br ON br.book_id = b.id
            GROUP BY b.id, b.name
            ORDER BY author_average DESC
            LIMIT $2
            OFFSET $3;
            "#
        )
            .bind(author_id)
            .bind(page_size)
            .bind(skip)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut books: Vec<Book> = Vec::new();

        for book_id_row in book_id_rows {
            let book_id: Uuid = book_id_row.get("b.id");

            let book_row: BookRow = sqlx::query_as(r#"
                    SELECT
                        id, title, subtitle, publisher_id, series_collection, volume, edition, publication_year, pages, language, isbn, synopsis
                    FROM
                        book
                    WHERE
                        id = $1 AND deleted = false;
                "#
            )
                .bind(book_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            let mut cover: Option<String> = None;

            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    book_image
                WHERE
                    book_id = $1 AND deleted = false;
                "#
            )
                .bind(book_row.id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                cover = Some(image_row.unwrap().image_path);
            }


            let mut publisher: Publisher;

            let publisher_row: PublisherRow = sqlx::query_as(r#"
                    SELECT
                        id, name, site, email
                    FROM
                        publisher
                    WHERE
                        id = $1 AND deleted = false;
                "#
            )
            .bind(book_row.publisher_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            let mut avatar: Option<String> = None;

            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    publisher_image
                WHERE
                    publisher_id = $1 AND deleted = false;
                "#
            )
                .bind(publisher_row.id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                avatar = Some(image_row.unwrap().image_path);
            }

            publisher = publisher_row.into();
            publisher.set_avatar(avatar);


            let mut authors: Vec<Author> = Vec::new();

            let book_authors_row: Vec<BookAuthorRow> = sqlx::query_as(r#"
                    SELECT
                        (id, book_id, author_id)
                    FROM
                        book_author
                    WHERE
                        book_id = $1 AND deleted = false;
                "#
            )
                .bind(book_id)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            for book_author_row in book_authors_row {
                let author_row: AuthorRow = sqlx::query_as(r#"
                        SELECT
                            (id, name)
                        FROM
                            author
                        WHERE
                            id = $1 AND deleted = false;
                    "#
                )
                .bind(book_author_row.author_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

                let mut avatar: Option<String> = None;

                let image_row: Option<ImageRow> = sqlx::query_as(r#"
                    SELECT
                        id, original_name, image_path
                    FROM
                        author_image
                    WHERE
                        author_id = $1 AND deleted = false;
                    "#
                )
                    .bind(author_row.id)
                    .fetch_optional(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                if image_row.is_some() {
                    avatar = Some(image_row.unwrap().image_path);
                }

                let mut author: Author = author_row.into();
                author.set_avatar(avatar);

                authors.push(author);
            }


            let mut genders: Vec<Gender> = Vec::new();

            let book_genders_row: Vec<BookGenderRow> = sqlx::query_as(r#"
                    SELECT
                        id, book_id, gender_id
                    FROM
                        book_gender
                    WHERE
                        book_id = $1 AND deleted = false;
                "#
            )
            .bind(book_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            for book_gender_row in book_genders_row {
                let gender_row: GenderRow = sqlx::query_as(r#"
                        SELECT
                            id, name
                        FROM
                            gender
                        WHERE
                            id = $1 AND deleted = false;
                    "#
                )
                .bind(book_gender_row.gender_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

                let gender: Gender = gender_row.into();

                genders.push(gender);
            }

            let mut book: Book = book_row.into();
            book.set_cover(cover);

            book.set_publisher(Some(publisher));
            book.set_authors(authors);
            book.set_genders(genders);

            books.push(book);
        }

        Ok(books)
    }

    async fn get_books_by_publisher(&self, publisher_id: Uuid, skip: i32, page_size: i32) -> Result<Vec<Book>, String> {
        let book_rows: Vec<BookRow> = sqlx::query_as(r#"
                SELECT
                        id, title, subtitle, publisher_id, series_collection, volume, edition, publication_year, pages, language, isbn, synopsis
                    FROM
                        book
                    WHERE
                        publisher_id = $1 AND deleted = false
                    LIMIT $2
                    OFFSET $3;
            "#
        )
        .bind(publisher_id)
        .bind(page_size)
        .bind(skip)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut books: Vec<Book> = Vec::new();

        for book_row in book_rows {
            let mut cover: Option<String> = None;

            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    book_image
                WHERE
                    book_id = $1 AND deleted = false;
                "#
            )
                .bind(book_row.id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                cover = Some(image_row.unwrap().image_path);
            }


            let mut publisher: Publisher;

            let publisher_row: PublisherRow = sqlx::query_as(r#"
                    SELECT
                        id, name, site, email
                    FROM
                        publisher
                    WHERE
                        id = $1 AND deleted = false;
                "#
            )
            .bind(book_row.publisher_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            let mut avatar: Option<String> = None;

            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    publisher_image
                WHERE
                    publisher_id = $1 AND deleted = false;
                "#
            )
                .bind(publisher_row.id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                avatar = Some(image_row.unwrap().image_path);
            }

            publisher = publisher_row.into();
            publisher.set_avatar(avatar);


            let mut authors: Vec<Author> = Vec::new();

            let book_authors_row: Vec<BookAuthorRow> = sqlx::query_as(r#"
                    SELECT
                        id, book_id, author_id
                    FROM
                        book_author
                    WHERE
                        book_id = $1 AND deleted = false;
                "#
            )
                .bind(book_row.id)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            for book_author_row in book_authors_row {
                let author_row: AuthorRow = sqlx::query_as(r#"
                        SELECT
                            id, name
                        FROM
                            author
                        WHERE
                            id = $1 AND deleted = false;
                    "#
                )
                .bind(book_author_row.author_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

                let mut avatar: Option<String> = None;

                let image_row: Option<ImageRow> = sqlx::query_as(r#"
                    SELECT
                        id, original_name, image_path
                    FROM
                        author_image
                    WHERE
                        author_id = $1 AND deleted = false;
                    "#
                )
                    .bind(author_row.id)
                    .fetch_optional(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                if image_row.is_some() {
                    avatar = Some(image_row.unwrap().image_path);
                }

                let mut author: Author = author_row.into();
                author.set_avatar(avatar);

                authors.push(author);
            }


            let mut genders: Vec<Gender> = Vec::new();

            let book_genders_row: Vec<BookGenderRow> = sqlx::query_as(r#"
                    SELECT
                        id, book_id, gender_id
                    FROM
                        book_gender
                    WHERE
                        book_id = $1 AND deleted = false;
                "#
            )
            .bind(book_row.id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            for book_gender_row in book_genders_row {
                let gender_row: GenderRow = sqlx::query_as(r#"
                        SELECT
                            id, name
                        FROM
                            gender
                        WHERE
                            id = $1 AND deleted = false;
                    "#
                )
                .bind(book_gender_row.gender_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

                let gender: Gender = gender_row.into();

                genders.push(gender);
            }

            let mut book: Book = book_row.into();
            book.set_cover(cover);

            book.set_publisher(Some(publisher));
            book.set_authors(authors);
            book.set_genders(genders);

            books.push(book);
        }

        Ok(books)
    }

    async fn best_valuated_books_by_publisher(&self, publisher_id: Uuid, skip: i32, page_size: i32) -> Result<Vec<Book>, String> {
        let book_id_rows = sqlx::query(r#"
            SELECT
                b.id,
                AVG(br.review)::float8 AS publisher_average,
                COUNT(br.review) AS total_reviews
            FROM publisher p
            WHERE p.id = $1 AND p.deleted = false
            JOIN book b ON b.publisher_id = p.id
            JOIN book_review br ON br.book_id = b.id
            GROUP BY b.id, b.name
            ORDER BY publisher_average DESC
            LIMIT $2
            OFFSET $3;
            "#
        )
            .bind(publisher_id)
            .bind(page_size)
            .bind(skip)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut books: Vec<Book> = Vec::new();

        for book_id_row in book_id_rows {
            let book_id: Uuid = book_id_row.get("b.id");

            let book_row: BookRow = sqlx::query_as(r#"
                    SELECT
                        id, title, subtitle, publisher_id, series_collection, volume, edition, publication_year, pages, language, isbn, synopsis
                    FROM
                        book
                    WHERE
                        id = $1 AND deleted = false;
                "#
            )
                .bind(book_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            let mut cover: Option<String> = None;

            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    book_image
                WHERE
                    book_id = $1 AND deleted = false;
                "#
            )
                .bind(book_row.id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                cover = Some(image_row.unwrap().image_path);
            }


            let mut publisher: Publisher;

            let publisher_row: PublisherRow = sqlx::query_as(r#"
                    SELECT
                        id, name, site, email
                    FROM
                        publisher
                    WHERE
                        id = $1 AND deleted = false;
                "#
            )
            .bind(book_row.publisher_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            let mut avatar: Option<String> = None;

            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    publisher_image
                WHERE
                    publisher_id = $1 AND deleted = false;
                "#
            )
                .bind(publisher_row.id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                avatar = Some(image_row.unwrap().image_path);
            }

            publisher = publisher_row.into();
            publisher.set_avatar(avatar);


            let mut authors: Vec<Author> = Vec::new();

            let book_authors_row: Vec<BookAuthorRow> = sqlx::query_as(r#"
                    SELECT
                        (id, book_id, author_id)
                    FROM
                        book_author
                    WHERE
                        book_id = $1 AND deleted = false;
                "#
            )
                .bind(book_id)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            for book_author_row in book_authors_row {
                let author_row: AuthorRow = sqlx::query_as(r#"
                        SELECT
                            (id, name)
                        FROM
                            author
                        WHERE
                            id = $1 AND deleted = false;
                    "#
                )
                .bind(book_author_row.author_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

                let mut avatar: Option<String> = None;

                let image_row: Option<ImageRow> = sqlx::query_as(r#"
                    SELECT
                        id, original_name, image_path
                    FROM
                        author_image
                    WHERE
                        author_id = $1 AND deleted = false;
                    "#
                )
                    .bind(author_row.id)
                    .fetch_optional(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                if image_row.is_some() {
                    avatar = Some(image_row.unwrap().image_path);
                }

                let mut author: Author = author_row.into();
                author.set_avatar(avatar);

                authors.push(author);
            }


            let mut genders: Vec<Gender> = Vec::new();

            let book_genders_row: Vec<BookGenderRow> = sqlx::query_as(r#"
                    SELECT
                        id, book_id, gender_id
                    FROM
                        book_gender
                    WHERE
                        book_id = $1 AND deleted = false;
                "#
            )
            .bind(book_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            for book_gender_row in book_genders_row {
                let gender_row: GenderRow = sqlx::query_as(r#"
                        SELECT
                            id, name
                        FROM
                            gender
                        WHERE
                            id = $1 AND deleted = false;
                    "#
                )
                .bind(book_gender_row.gender_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

                let gender: Gender = gender_row.into();

                genders.push(gender);
            }

            let mut book: Book = book_row.into();
            book.set_cover(cover);

            book.set_publisher(Some(publisher));
            book.set_authors(authors);
            book.set_genders(genders);

            books.push(book);
        }

        Ok(books)
    }

    async fn get_books_by_gender(&self, gender_id: Uuid, skip: i32, page_size: i32) -> Result<Vec<Book>, String> {
        let book_gender_rows: Vec<BookGenderRow> = sqlx::query_as(r#"
                SELECT
                    id, book_id, gender_id
                FROM
                    book_gender
                WHERE
                    gender_id = $1 AND deleted = false
                LIMIT $2
                OFFSET $3;
            "#
        )
        .bind(gender_id)
        .bind(page_size)
        .bind(skip)
        .fetch_all(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        let mut books: Vec<Book> = Vec::new();

        for book_gender_row in book_gender_rows {
            let book_row: BookRow = sqlx::query_as(r#"
                    SELECT
                        id, title, subtitle, publisher_id, series_collection, volume, edition, publication_year, pages, language, isbn, synopsis
                    FROM
                        book
                    WHERE
                        id = $1 AND deleted = false;
                "#
            )
                .bind(book_gender_row.book_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            let mut cover: Option<String> = None;

            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    book_image
                WHERE
                    book_id = $1 AND deleted = false;
                "#
            )
                .bind(book_row.id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                cover = Some(image_row.unwrap().image_path);
            }


            let mut publisher: Publisher;

            let publisher_row: PublisherRow = sqlx::query_as(r#"
                    SELECT
                        id, name, site, email
                    FROM
                        publisher
                    WHERE
                        id = $1 AND deleted = false;
                "#
            )
            .bind(book_row.publisher_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            let mut avatar: Option<String> = None;

            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    publisher_image
                WHERE
                    publisher_id = $1 AND deleted = false;
                "#
            )
                .bind(publisher_row.id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                avatar = Some(image_row.unwrap().image_path);
            }

            publisher = publisher_row.into();
            publisher.set_avatar(avatar);


            let mut authors: Vec<Author> = Vec::new();

            let book_authors_rows: Vec<BookAuthorRow> = sqlx::query_as(r#"
                    SELECT
                        id, book_id, author_id
                    FROM
                        book_author
                    WHERE
                        book_id = $1 AND deleted = false;
                "#
            )
                .bind(book_gender_row.book_id)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            for book_author_intern_row in book_authors_rows {
                let author_row: AuthorRow = sqlx::query_as(r#"
                        SELECT
                            id, name
                        FROM
                            author
                        WHERE
                            id = $1 AND deleted = false;
                    "#
                )
                .bind(book_author_intern_row.author_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

                let mut avatar: Option<String> = None;

                let image_row: Option<ImageRow> = sqlx::query_as(r#"
                    SELECT
                        id, original_name, image_path
                    FROM
                        author_image
                    WHERE
                        author_id = $1 AND deleted = false;
                    "#
                )
                    .bind(author_row.id)
                    .fetch_optional(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                if image_row.is_some() {
                    avatar = Some(image_row.unwrap().image_path);
                }

                let mut author: Author = author_row.into();
                author.set_avatar(avatar);

                authors.push(author);
            }


            let mut genders: Vec<Gender> = Vec::new();

            let book_gender_intern_rows: Vec<BookGenderRow> = sqlx::query_as(r#"
                    SELECT
                        id, book_id, gender_id
                    FROM
                        book_gender
                    WHERE
                        book_id = $1 AND deleted = false;
                "#
            )
            .bind(book_gender_row.book_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            for book_gender_intern_row in book_gender_intern_rows {
                let gender_row: GenderRow = sqlx::query_as(r#"
                        SELECT
                            id, name
                        FROM
                            gender
                        WHERE
                            id = $1 AND deleted = false;
                    "#
                )
                .bind(book_gender_intern_row.gender_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

                let gender: Gender = gender_row.into();

                genders.push(gender);
            }

            let mut book: Book = book_row.into();
            book.set_cover(cover);

            book.set_publisher(Some(publisher));
            book.set_authors(authors);
            book.set_genders(genders);

            books.push(book);
        }

        Ok(books)
    }

    async fn best_valuated_books_by_gender(&self, gender_id: Uuid, skip: i32, page_size: i32) -> Result<Vec<Book>, String> {
        let book_id_rows = sqlx::query(r#"
            SELECT
                b.id,
                AVG(br.review)::float8 AS gender_average,
                COUNT(br.review) AS total_reviews
            FROM gender g
            WHERE g.id = $1 AND g.deleted = false
            JOIN book_gender bg ON g.id = bg.gender_id
            JOIN book b ON b.id = bg.book_id
            JOIN book_review br ON br.book_id = b.id
            GROUP BY b.id, b.name
            ORDER BY gender_average DESC
            LIMIT $2
            OFFSET $3;
            "#
        )
            .bind(gender_id)
            .bind(page_size)
            .bind(skip)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut books: Vec<Book> = Vec::new();

        for book_id_row in book_id_rows {
            let book_id: Uuid = book_id_row.get("b.id");

            let book_row: BookRow = sqlx::query_as(r#"
                    SELECT
                        id, title, subtitle, publisher_id, series_collection, volume, edition, publication_year, pages, language, isbn, synopsis
                    FROM
                        book
                    WHERE
                        id = $1 AND deleted = false;
                "#
            )
                .bind(book_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            let mut cover: Option<String> = None;

            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    book_image
                WHERE
                    book_id = $1 AND deleted = false;
                "#
            )
                .bind(book_row.id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                cover = Some(image_row.unwrap().image_path);
            }


            let mut publisher: Publisher;

            let publisher_row: PublisherRow = sqlx::query_as(r#"
                    SELECT
                        id, name, site, email
                    FROM
                        publisher
                    WHERE
                        id = $1 AND deleted = false;
                "#
            )
            .bind(book_row.publisher_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            let mut avatar: Option<String> = None;

            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    publisher_image
                WHERE
                    publisher_id = $1 AND deleted = false;
                "#
            )
                .bind(publisher_row.id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                avatar = Some(image_row.unwrap().image_path);
            }

            publisher = publisher_row.into();
            publisher.set_avatar(avatar);


            let mut authors: Vec<Author> = Vec::new();

            let book_authors_row: Vec<BookAuthorRow> = sqlx::query_as(r#"
                    SELECT
                        (id, book_id, author_id)
                    FROM
                        book_author
                    WHERE
                        book_id = $1 AND deleted = false;
                "#
            )
                .bind(book_id)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            for book_author_row in book_authors_row {
                let author_row: AuthorRow = sqlx::query_as(r#"
                        SELECT
                            (id, name)
                        FROM
                            author
                        WHERE
                            id = $1 AND deleted = false;
                    "#
                )
                .bind(book_author_row.author_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

                let mut avatar: Option<String> = None;

                let image_row: Option<ImageRow> = sqlx::query_as(r#"
                    SELECT
                        id, original_name, image_path
                    FROM
                        author_image
                    WHERE
                        author_id = $1 AND deleted = false;
                    "#
                )
                    .bind(author_row.id)
                    .fetch_optional(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                if image_row.is_some() {
                    avatar = Some(image_row.unwrap().image_path);
                }

                let mut author: Author = author_row.into();
                author.set_avatar(avatar);

                authors.push(author);
            }


            let mut genders: Vec<Gender> = Vec::new();

            let book_genders_row: Vec<BookGenderRow> = sqlx::query_as(r#"
                    SELECT
                        id, book_id, gender_id
                    FROM
                        book_gender
                    WHERE
                        book_id = $1 AND deleted = false;
                "#
            )
            .bind(book_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            for book_gender_row in book_genders_row {
                let gender_row: GenderRow = sqlx::query_as(r#"
                        SELECT
                            id, name
                        FROM
                            gender
                        WHERE
                            id = $1 AND deleted = false;
                    "#
                )
                .bind(book_gender_row.gender_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

                let gender: Gender = gender_row.into();

                genders.push(gender);
            }

            let mut book: Book = book_row.into();
            book.set_cover(cover);

            book.set_publisher(Some(publisher));
            book.set_authors(authors);
            book.set_genders(genders);

            books.push(book);
        }

        Ok(books)
    }

    async fn smallets_books(&self, skip: i32, page_size: i32) -> Result<Vec<Book>, String> {
        let book_rows: Vec<BookRow> = sqlx::query_as(r#"
                SELECT
                    id, title, subtitle, publisher_id, series_collection, volume, edition, publication_year, pages, language, isbn, synopsis
                FROM
                    book
                WHERE
                    deleted = false
                ORDER BY pages ASC
                LIMIT $1
                OFFSET $2;
            "#
        )
            .bind(page_size)
            .bind(skip)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut books: Vec<Book> = Vec::new();

        for book_row in book_rows {
            let mut cover: Option<String> = None;

            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    book_image
                WHERE
                    book_id = $1 AND deleted = false;
                "#
            )
                .bind(book_row.id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                cover = Some(image_row.unwrap().image_path);
            }


            let mut publisher: Publisher;

            let publisher_row: PublisherRow = sqlx::query_as(r#"
                    SELECT
                        id, name, site, email
                    FROM
                        publisher
                    WHERE
                        id = $1 AND deleted = false;
                "#
            )
            .bind(book_row.publisher_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            let mut avatar: Option<String> = None;

            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    publisher_image
                WHERE
                    publisher_id = $1 AND deleted = false;
                "#
            )
                .bind(publisher_row.id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                avatar = Some(image_row.unwrap().image_path);
            }

            publisher = publisher_row.into();
            publisher.set_avatar(avatar);


            let mut authors: Vec<Author> = Vec::new();

            let book_authors_row: Vec<BookAuthorRow> = sqlx::query_as(r#"
                    SELECT
                        (id, book_id, author_id)
                    FROM
                        book_author
                    WHERE
                        book_id = $1 AND deleted = false;
                "#
            )
                .bind(book_row.id)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            for book_author_row in book_authors_row {
                let author_row: AuthorRow = sqlx::query_as(r#"
                        SELECT
                            (id, name)
                        FROM
                            author
                        WHERE
                            id = $1 AND deleted = false;
                    "#
                )
                .bind(book_author_row.author_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

                let mut avatar: Option<String> = None;

                let image_row: Option<ImageRow> = sqlx::query_as(r#"
                    SELECT
                        id, original_name, image_path
                    FROM
                        author_image
                    WHERE
                        author_id = $1 AND deleted = false;
                    "#
                )
                    .bind(author_row.id)
                    .fetch_optional(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                if image_row.is_some() {
                    avatar = Some(image_row.unwrap().image_path);
                }

                let mut author: Author = author_row.into();
                author.set_avatar(avatar);

                authors.push(author);
            }


            let mut genders: Vec<Gender> = Vec::new();

            let book_genders_row: Vec<BookGenderRow> = sqlx::query_as(r#"
                    SELECT
                        id, book_id, gender_id
                    FROM
                        book_gender
                    WHERE
                        book_id = $1 AND deleted = false;
                "#
            )
            .bind(book_row.id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            for book_gender_row in book_genders_row {
                let gender_row: GenderRow = sqlx::query_as(r#"
                        SELECT
                            id, name
                        FROM
                            gender
                        WHERE
                            id = $1 AND deleted = false;
                    "#
                )
                .bind(book_gender_row.gender_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

                let gender: Gender = gender_row.into();

                genders.push(gender);
            }

            let mut book: Book = book_row.into();
            book.set_cover(cover);

            book.set_publisher(Some(publisher));
            book.set_authors(authors);
            book.set_genders(genders);

            books.push(book);
        }

        Ok(books)
    }

    async fn biggest_books(&self, skip: i32, page_size: i32) -> Result<Vec<Book>, String> {
        let book_rows: Vec<BookRow> = sqlx::query_as(r#"
                SELECT
                    id, title, subtitle, publisher_id, series_collection, volume, edition, publication_year, pages, language, isbn, synopsis
                FROM
                    book
                WHERE
                    deleted = false
                ORDER BY pages DESC
                LIMIT $1
                OFFSET $2;
            "#
        )
            .bind(page_size)
            .bind(skip)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut books: Vec<Book> = Vec::new();

        for book_row in book_rows {
            let mut cover: Option<String> = None;

            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    book_image
                WHERE
                    book_id = $1 AND deleted = false;
                "#
            )
                .bind(book_row.id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                cover = Some(image_row.unwrap().image_path);
            }


            let mut publisher: Publisher;

            let publisher_row: PublisherRow = sqlx::query_as(r#"
                    SELECT
                        id, name, site, email
                    FROM
                        publisher
                    WHERE
                        id = $1 AND deleted = false;
                "#
            )
            .bind(book_row.publisher_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            let mut avatar: Option<String> = None;

            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    publisher_image
                WHERE
                    publisher_id = $1 AND deleted = false;
                "#
            )
                .bind(publisher_row.id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                avatar = Some(image_row.unwrap().image_path);
            }

            publisher = publisher_row.into();
            publisher.set_avatar(avatar);


            let mut authors: Vec<Author> = Vec::new();

            let book_authors_row: Vec<BookAuthorRow> = sqlx::query_as(r#"
                    SELECT
                        (id, book_id, author_id)
                    FROM
                        book_author
                    WHERE
                        book_id = $1 AND deleted = false;
                "#
            )
                .bind(book_row.id)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            for book_author_row in book_authors_row {
                let author_row: AuthorRow = sqlx::query_as(r#"
                        SELECT
                            (id, name)
                        FROM
                            author
                        WHERE
                            id = $1 AND deleted = false;
                    "#
                )
                .bind(book_author_row.author_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

                let mut avatar: Option<String> = None;

                let image_row: Option<ImageRow> = sqlx::query_as(r#"
                    SELECT
                        id, original_name, image_path
                    FROM
                        author_image
                    WHERE
                        author_id = $1 AND deleted = false;
                    "#
                )
                    .bind(author_row.id)
                    .fetch_optional(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                if image_row.is_some() {
                    avatar = Some(image_row.unwrap().image_path);
                }

                let mut author: Author = author_row.into();
                author.set_avatar(avatar);

                authors.push(author);
            }


            let mut genders: Vec<Gender> = Vec::new();

            let book_genders_row: Vec<BookGenderRow> = sqlx::query_as(r#"
                    SELECT
                        id, book_id, gender_id
                    FROM
                        book_gender
                    WHERE
                        book_id = $1 AND deleted = false;
                "#
            )
            .bind(book_row.id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            for book_gender_row in book_genders_row {
                let gender_row: GenderRow = sqlx::query_as(r#"
                        SELECT
                            id, name
                        FROM
                            gender
                        WHERE
                            id = $1 AND deleted = false;
                    "#
                )
                .bind(book_gender_row.gender_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

                let gender: Gender = gender_row.into();

                genders.push(gender);
            }

            let mut book: Book = book_row.into();
            book.set_cover(cover);

            book.set_publisher(Some(publisher));
            book.set_authors(authors);
            book.set_genders(genders);

            books.push(book);
        }

        Ok(books)
    }

    async fn more_popular_books(
        &self,
        skip: i32,
        page_size: i32
    ) -> Result<Vec<Book>, String> {
        let book_id_rows = sqlx::query(r#"
            SELECT
                book_id,
                COUNT(user_id) as readed_book
            FROM book_user
            WHERE reading_status = $1 AND deleted = false
            GROUP BY book_id
            ORDER BY readed_book DESC
            LIMIT $2
            OFFSET $3;
            "#
        )
            .bind(ReadingStatus::Lido as i32)
            .bind(page_size)
            .bind(skip)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut books: Vec<Book> = Vec::new();

        for book_id_row in book_id_rows {
            let book_id: Uuid = book_id_row.get("book_id");

            let book_row: BookRow = sqlx::query_as(r#"
                    SELECT
                        id, title, subtitle, publisher_id, series_collection, volume, edition, publication_year, pages, language, isbn, synopsis
                    FROM
                        book
                    WHERE
                        id = $1 AND deleted = false;
                "#
            )
                .bind(book_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            let mut cover: Option<String> = None;

            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    book_image
                WHERE
                    book_id = $1 AND deleted = false;
                "#
            )
                .bind(book_row.id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                cover = Some(image_row.unwrap().image_path);
            }


            let mut publisher: Publisher;

            let publisher_row: PublisherRow = sqlx::query_as(r#"
                    SELECT
                        id, name, site, email
                    FROM
                        publisher
                    WHERE
                        id = $1 AND deleted = false;
                "#
            )
            .bind(book_row.publisher_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            let mut avatar: Option<String> = None;

            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    publisher_image
                WHERE
                    publisher_id = $1 AND deleted = false;
                "#
            )
                .bind(publisher_row.id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                avatar = Some(image_row.unwrap().image_path);
            }

            publisher = publisher_row.into();
            publisher.set_avatar(avatar);


            let mut authors: Vec<Author> = Vec::new();

            let book_authors_row: Vec<BookAuthorRow> = sqlx::query_as(r#"
                    SELECT
                        (id, book_id, author_id)
                    FROM
                        book_author
                    WHERE
                        book_id = $1 AND deleted = false;
                "#
            )
                .bind(book_id)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            for book_author_row in book_authors_row {
                let author_row: AuthorRow = sqlx::query_as(r#"
                        SELECT
                            (id, name)
                        FROM
                            author
                        WHERE
                            id = $1 AND deleted = false;
                    "#
                )
                .bind(book_author_row.author_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

                let mut avatar: Option<String> = None;

                let image_row: Option<ImageRow> = sqlx::query_as(r#"
                    SELECT
                        id, original_name, image_path
                    FROM
                        author_image
                    WHERE
                        author_id = $1 AND deleted = false;
                    "#
                )
                    .bind(author_row.id)
                    .fetch_optional(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                if image_row.is_some() {
                    avatar = Some(image_row.unwrap().image_path);
                }

                let mut author: Author = author_row.into();
                author.set_avatar(avatar);

                authors.push(author);
            }


            let mut genders: Vec<Gender> = Vec::new();

            let book_genders_row: Vec<BookGenderRow> = sqlx::query_as(r#"
                    SELECT
                        id, book_id, gender_id
                    FROM
                        book_gender
                    WHERE
                        book_id = $1 AND deleted = false;
                "#
            )
            .bind(book_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            for book_gender_row in book_genders_row {
                let gender_row: GenderRow = sqlx::query_as(r#"
                        SELECT
                            id, name
                        FROM
                            gender
                        WHERE
                            id = $1 AND deleted = false;
                    "#
                )
                .bind(book_gender_row.gender_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

                let gender: Gender = gender_row.into();

                genders.push(gender);
            }

            let mut book: Book = book_row.into();
            book.set_cover(cover);

            book.set_publisher(Some(publisher));
            book.set_authors(authors);
            book.set_genders(genders);

            books.push(book);
        }

        Ok(books)
    }


    async fn best_valuated_books(
        &self,
        skip: i32,
        page_size: i32
    ) -> Result<Vec<Book>, String> {
        let book_id_rows = sqlx::query(r#"
            SELECT
                b.id,
                AVG(br.review)::float8 AS book_average,
                COUNT(br.review) AS total_reviews
            FROM book b
            WHERE p.deleted = false
            JOIN book_review br ON br.book_id = b.id
            GROUP BY b.id, b.name
            ORDER BY book_average DESC
            LIMIT $1
            OFFSET $2;
            "#
        )
            .bind(page_size)
            .bind(skip)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut books: Vec<Book> = Vec::new();

        for book_id_row in book_id_rows {
            let book_id: Uuid = book_id_row.get("b.id");

            let book_row: BookRow = sqlx::query_as(r#"
                    SELECT
                        id, title, subtitle, publisher_id, series_collection, volume, edition, publication_year, pages, language, isbn, synopsis
                    FROM
                        book
                    WHERE
                        id = $1 AND deleted = false;
                "#
            )
                .bind(book_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            let mut cover: Option<String> = None;

            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    book_image
                WHERE
                    book_id = $1 AND deleted = false;
                "#
            )
                .bind(book_row.id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                cover = Some(image_row.unwrap().image_path);
            }


            let mut publisher: Publisher;

            let publisher_row: PublisherRow = sqlx::query_as(r#"
                    SELECT
                        id, name, site, email
                    FROM
                        publisher
                    WHERE
                        id = $1 AND deleted = false;
                "#
            )
            .bind(book_row.publisher_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            let mut avatar: Option<String> = None;

            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    publisher_image
                WHERE
                    publisher_id = $1 AND deleted = false;
                "#
            )
                .bind(publisher_row.id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                avatar = Some(image_row.unwrap().image_path);
            }

            publisher = publisher_row.into();
            publisher.set_avatar(avatar);


            let mut authors: Vec<Author> = Vec::new();

            let book_authors_row: Vec<BookAuthorRow> = sqlx::query_as(r#"
                    SELECT
                        (id, book_id, author_id)
                    FROM
                        book_author
                    WHERE
                        book_id = $1 AND deleted = false;
                "#
            )
                .bind(book_id)
                .fetch_all(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            for book_author_row in book_authors_row {
                let author_row: AuthorRow = sqlx::query_as(r#"
                        SELECT
                            (id, name)
                        FROM
                            author
                        WHERE
                            id = $1 AND deleted = false;
                    "#
                )
                .bind(book_author_row.author_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

                let mut avatar: Option<String> = None;

                let image_row: Option<ImageRow> = sqlx::query_as(r#"
                    SELECT
                        id, original_name, image_path
                    FROM
                        author_image
                    WHERE
                        author_id = $1 AND deleted = false;
                    "#
                )
                    .bind(author_row.id)
                    .fetch_optional(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?;

                if image_row.is_some() {
                    avatar = Some(image_row.unwrap().image_path);
                }

                let mut author: Author = author_row.into();
                author.set_avatar(avatar);

                authors.push(author);
            }


            let mut genders: Vec<Gender> = Vec::new();

            let book_genders_row: Vec<BookGenderRow> = sqlx::query_as(r#"
                    SELECT
                        id, book_id, gender_id
                    FROM
                        book_gender
                    WHERE
                        book_id = $1 AND deleted = false;
                "#
            )
            .bind(book_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            for book_gender_row in book_genders_row {
                let gender_row: GenderRow = sqlx::query_as(r#"
                        SELECT
                            id, name
                        FROM
                            gender
                        WHERE
                            id = $1 AND deleted = false;
                    "#
                )
                .bind(book_gender_row.gender_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

                let gender: Gender = gender_row.into();

                genders.push(gender);
            }

            let mut book: Book = book_row.into();
            book.set_cover(cover);

            book.set_publisher(Some(publisher));
            book.set_authors(authors);
            book.set_genders(genders);

            books.push(book);
        }

        Ok(books)
    }

    async fn alter_book(
        &self,
        book_id: Uuid,
        title: String,
        subtitle: Option<String>,
        authors_id: Vec<Uuid>,
        publisher_id: Uuid,
        series_collection: Option<i32>,
        volume: Option<i32>,
        edition: Option<i32>,
        publication_year: Option<i32>,
        pages: Option<i32>,
        language: Option<String>,
        isbn: String,
        genders_id: Vec<Uuid>,
        synopsis: Option<String>,
        file_name: Option<String>,
        file_content: Option<Bytes>,
        _user_id: Uuid
    ) -> Result<(), String> {
        if file_name.is_some() && file_content.is_some() {
            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    book_image
                WHERE
                    book_id = $1 AND deleted = false;
                "#
            )
                .bind(book_id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                remove_file(image_row.clone().unwrap().image_path).await.unwrap();

                sqlx::query(r#"
                    DELETE FROM
                        book_image
                    WHERE
                        id = $1
                    "#
                )
                    .bind(image_row.unwrap().id)
                    .execute(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }

            let image_id = Uuid::new_v4();
            let new_filename = format!("{}.png", Uuid::new_v4());

            let path = format!("./{}/book/{}", UPLOADS_IMAGE_PATH, new_filename);

            let mut file = tokio::fs::File::create(&path)
                .await
                .map_err(|e| e.to_string())?;

            file.write_all(&file_content.unwrap())
                .await
                .map_err(|e| e.to_string())?;

            sqlx::query(r#"
                INSERT INTO
                    book_image (id, original_name, image_path, book_id)
                VALUES
                    ($1, $2, $3, $4);
                "#
            )
                .bind(image_id)
                .bind(file_name)
                .bind(path)
                .bind(book_id)
                .execute(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        } else {
            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    book_image
                WHERE
                    book_id = $1 AND deleted = false;
                "#
            )
                .bind(book_id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if file_name.is_none() && file_content.is_none() && image_row.is_some() {

                remove_file(image_row.clone().unwrap().image_path).await.unwrap();

                sqlx::query(r#"
                    DELETE FROM
                        book_image
                    WHERE
                        id = $1;
                    "#
                )
                    .bind(image_row.unwrap().id)
                    .execute(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?;
            }
        }

        let book_author_rows: Vec<BookAuthorRow> = sqlx::query_as(r#"
                SELECT
                    id, book_id, author_id
                FROM
                    book_author
                WHERE
                    book_id = $1 AND deleted = false;
            "#
        )
            .bind(book_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut continue_old_book_authors: Vec<Uuid> = Vec::new();

        for book_author_row in book_author_rows {
            if !authors_id.contains(&book_author_row.author_id) {
                sqlx::query(r#"
                    UPDATE
                        book_author
                    SET
                        deleted = true
                    WHERE
                        book_id = $1 AND author_id = $2
                    "#
                )
                    .bind(book_id)
                    .bind(book_author_row.author_id)
                    .execute(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?;
            } else {
                continue_old_book_authors.push(book_author_row.author_id);
            }
        }

        let new_book_authors: Vec<Uuid> = authors_id.iter().filter(|a| !continue_old_book_authors.contains(a)).cloned().collect();

        for new_book_author in new_book_authors {
            let book_author_id = Uuid::new_v4();
            sqlx::query(r#"
                    INSERT INTO
                        book_author (id, book_id, author_id)
                    VALUES
                        ($1, $2, $3);
                "#
            )
                .bind(book_author_id)
                .bind(book_id)
                .bind(new_book_author)
                .execute(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        }

        let book_gender_rows: Vec<BookGenderRow> = sqlx::query_as(r#"
                SELECT
                    id, book_id, gender_id
                FROM
                    book_gender
                WHERE
                    book_id = $1 AND deleted = false;
            "#
        )
            .bind(book_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut continue_old_book_genders: Vec<Uuid> = Vec::new();

        for book_gender_row in book_gender_rows {
            if !genders_id.contains(&book_gender_row.gender_id) {
                sqlx::query(r#"
                    UPDATE
                        book_gender
                    SET
                        deleted = true
                    WHERE
                        book_id = $1 AND gender_id = $2
                    "#
                )
                    .bind(book_id)
                    .bind(book_gender_row.gender_id)
                    .execute(&self.pool)
                    .await
                    .map_err(|e| e.to_string())?;
            } else {
                continue_old_book_genders.push(book_gender_row.gender_id);
            }
        }

        let new_book_genders: Vec<Uuid> = genders_id.iter().filter(|a| !continue_old_book_genders.contains(a)).cloned().collect();

        for new_book_gender in new_book_genders {
            let book_gender_id = Uuid::new_v4();
            sqlx::query(r#"
                    INSERT INTO
                        book_gender (id, book_id, gender_id)
                    VALUES
                        ($1, $2, $3);
                "#
            )
                .bind(book_gender_id)
                .bind(book_id)
                .bind(new_book_gender)
                .execute(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        }

        sqlx::query(r#"
            UPDATE
                book
            SET
                title = $2, subtitle = $3, publisher_id = $4, series_collection = $5, volume = $6, edition = $7, publication_year = $8, pages = $9, language = $10, isbn = $11, synopsis = $12
            WHERE
                id = $1 AND deleted = false;
            "#
        )
            .bind(book_id)
            .bind(title)
            .bind(subtitle)
            .bind(publisher_id)
            .bind(series_collection)
            .bind(volume)
            .bind(edition)
            .bind(publication_year)
            .bind(pages)
            .bind(language)
            .bind(isbn)
            .bind(synopsis)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn delete_book(&self, book_id: Uuid, _user_id: Uuid) -> Result<(), String> {
        sqlx::query(r#"
            UPDATE
                book_image
            SET
                deleted = true
            WHERE
                book_id = $1
            "#
        )
            .bind(book_id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query(r#"
            UPDATE
                book_author
            SET
                deleted = true
            WHERE
                book_id = $1
            "#
        )
            .bind(book_id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query(r#"
            UPDATE
                book_gender
            SET
                deleted = true
            WHERE
                book_id = $1
            "#
        )
            .bind(book_id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query(r#"
            UPDATE
                book
            SET
                deleted = true
            WHERE
                id = $1
            "#
        )
            .bind(book_id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn clear_deleted_books(&self) -> Result<(), String> {
        let deleted_images: Vec<ImageRow> = sqlx::query_as(r#"
            SELECT
                id, original_name, image_path
            FROM
                book_image
            WHERE
                deleted = true
            "#
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        for deleted_image in deleted_images {
            remove_file(&deleted_image.image_path).await.unwrap();
        }

        sqlx::query(r#"
            DELETE FROM
                book_image
            WHERE deleted = true
            "#
        )
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query(r#"
            DELETE FROM
                book_author
            WHERE deleted = true
            "#
        )
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query(r#"
            DELETE FROM
                book_gender
            WHERE deleted = true
            "#
        )
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query(r#"
            DELETE FROM
                book
            WHERE deleted = true
            "#
        )
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}