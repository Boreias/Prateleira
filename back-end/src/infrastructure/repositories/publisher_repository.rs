use async_trait::async_trait;
use axum::body::Bytes;
use tokio::{
    io::AsyncWriteExt,
    fs::remove_file
};
use uuid::Uuid;
use sqlx::{PgPool, Row};

use crate::domain::entities::publisher::Publisher;
use crate::domain::irepositories::ipublisher_repository::IPublisherRepository;
use crate::infrastructure::db::models::publisher_row::PublisherRow;
use crate::infrastructure::db::models::book_author_row::BookAuthorRow;
use crate::infrastructure::db::models::book_gender_row::BookGenderRow;
use crate::infrastructure::db::models::book_publisher_row::BookPublisherRow;
use crate::infrastructure::db::models::image_row::ImageRow;
use crate::infrastructure::enums::reading_status::ReadingStatus;
use crate::infrastructure::common::consts::UPLOADS_IMAGE_PATH;


pub struct PublisherRepository {
    pool: PgPool
}

impl PublisherRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}


#[async_trait]
impl IPublisherRepository for PublisherRepository {
    async fn create_publisher (
        &self,
        name: String,
        _user_id: Uuid,
        site: Option<String>,
        email: Option<String>,
        file_name: Option<String>,
        file_content: Option<Bytes>
    ) -> Result<(), String> {
        let publisher_id = Uuid::new_v4();
        sqlx::query(r#"
            INSERT INTO
                publisher (id, name, site, email)
            VALUES
                ($1, $2, $3, $4);
            "#
        )
            .bind(publisher_id)
            .bind(name)
            .bind(site.unwrap())
            .bind(email.unwrap())
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if file_name.is_some() && file_content.is_some() {
            let image_id = Some(Uuid::new_v4());
            let new_filename = format!("{}.png", Uuid::new_v4());

            let path = format!("./{}/publisher/{}", UPLOADS_IMAGE_PATH, new_filename);

            let mut file = tokio::fs::File::create(&path).await.unwrap();
            file.write_all(&file_content.unwrap()).await.unwrap();

            sqlx::query(r#"
                INSERT INTO
                    publisher_image (id, original_name, image_path, publisher_id)
                VALUES
                    ($1, $2, $3, $4);
                "#
            )
                .bind(image_id.unwrap())
                .bind(file_name)
                .bind(path)
                .bind(publisher_id)
                .execute(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    async fn get_publisher_by_id (&self, id: Uuid) -> Result<Publisher, String> {
        let publisher_row: PublisherRow = sqlx::query_as(r#"
            SELECT
                id, name, site, email
            FROM
                publisher
            WHERE
                id = $1 AND deleted = false;
            "#
        )
            .bind(id)
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

        let mut publisher: Publisher = publisher_row.into();
        publisher.set_avatar(avatar);

        Ok(publisher)
    }

    async fn get_publisher_by_name (&self, name: String, skip: i32, page_size: i32) -> Result<Vec<Publisher>, String> {
        let format_name = format!("%{}%", name);
        let publisher_rows: Vec<PublisherRow> = sqlx::query_as(r#"
            SELECT
                id, name, site, email
            FROM
                publisher
            WHERE
                name LIKE $1 AND deleted = false
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

        let mut publishers = Vec::new();

        for publisher_row in publisher_rows {
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

            let mut publisher: Publisher = publisher_row.into();
            publisher.set_avatar(avatar);

            publishers.push(publisher);
        }

        Ok(publishers)
    }

    async fn get_publisher_by_book (&self, book_id: Uuid, skip: i32, page_size: i32) -> Result<Publisher, String> {
        let book_publisher_row: BookPublisherRow = sqlx::query_as("SELECT publisher_id FROM book_publisher WHERE book_id = $1")
            .bind(book_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let publisher_row: PublisherRow = sqlx::query_as(r#"
            SELECT
                id, name, site, email
            FROM
                publisher
            WHERE
                id = $1 AND deleted = false
            LIMIT $2
            OFFSET $3;
            "#
        )
            .bind(book_publisher_row.publisher_id)
            .bind(page_size)
            .bind(skip)
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

        let mut publisher: Publisher = publisher_row.into();
        publisher.set_avatar(avatar);

        Ok(publisher)
    }

    async fn get_publishers_by_author (&self, author_id: Uuid, skip: i32, page_size: i32) -> Result<Vec<Publisher>, String> {
        let book_rows: Vec<BookAuthorRow> = sqlx::query_as(r#"
            SELECT
                id, book_id, author_id
            FROM
                book_author
            WHERE author_id = $1
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

        let mut publishers: Vec<Publisher> = Vec::new();

        for book_row in book_rows {

            let book_id = book_row.book_id;

            let book_publisher_row: BookPublisherRow = sqlx::query_as(r#"
                SELECT
                    id as book_id, publisher_id
                FROM
                    book
                WHERE
                    book_id = $1
                "#
            )
                .bind(book_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            let publisher_row: PublisherRow = sqlx::query_as(r#"
                SELECT
                    id, name, site, email
                FROM
                    publisher
                WHERE
                    book_id = $1
                "#
            )
                .bind(book_publisher_row.publisher_id)
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

            let mut publisher: Publisher = publisher_row.into();
            publisher.set_avatar(avatar);

            publishers.push(publisher);
        }

        Ok(publishers)
    }

    async fn get_publishers_by_gender (&self, gender_id: Uuid, skip: i32, page_size: i32) -> Result<Vec<Publisher>, String> {
        let book_gender_rows: Vec<BookGenderRow> = sqlx::query_as(r#"
            SELECT
                book_id
            FROM
                book_gender
            WHERE
                gender_id = $1
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

        let mut publishers = Vec::new();

        for book_gender_row in book_gender_rows {
            let publisher_row: BookPublisherRow = sqlx::query_as(r#"
                SELECT
                    id as book_id, publisher_id
                FROM
                    book
                WHERE
                    book_id = $1;
                "#
            )
                .bind(book_gender_row.book_id)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            let publisher_query: PublisherRow = sqlx::query_as(r#"
                SELECT
                    id, name, site, email
                FROM
                    publisher
                WHERE
                    id = $1 AND deleted = false;
                "#
            )
                .bind(publisher_row.publisher_id)
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
                .bind(publisher_row.publisher_id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                avatar = Some(image_row.unwrap().image_path);
            }

            let mut publisher: Publisher = publisher_query.into();
            publisher.set_avatar(avatar);

            publishers.push(publisher);
        }

        Ok(publishers)
    }

    async fn more_popular_publishers(
        &self,
        skip: i32,
        page_size: i32
    ) -> Result<Vec<Publisher>, String> {
        let book_rows = sqlx::query(r#"
            SELECT
                book_id,
                COUNT(user_id) as readed_book
            FROM book_user
            WHERE reading_status = $1
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

        let mut publishers: Vec<Publisher> = Vec::new();

        for book in book_rows {
            let book_id: Uuid = book.get("book_id");
            let publisher_book = sqlx::query(r#"
                SELECT
                    publisher_id
                FROM
                    book_publisher
                WHERE
                    book_id = $1
                LIMIT $2
                OFFSET $3;
                "#
            )
            .bind(book_id)
            .bind(page_size)
            .bind(skip)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

            let publisher_id: Uuid = publisher_book.get("publisher_id");

            let publisher_row: PublisherRow = sqlx::query_as(r#"
                SELECT
                    id, name, site, email
                FROM
                    publisher
                WHERE
                    id = $1 AND deleted = false
                LIMIT $2
                OFFSET $3;
            "#
        )
            .bind(publisher_id)
            .bind(page_size)
            .bind(skip)
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

            let mut publisher: Publisher = publisher_row.into();
            publisher.set_avatar(avatar);

            publishers.push(publisher);
        }

        Ok(publishers)
    }

    async fn best_valuated_publishers(
        &self,
        skip: i32,
        page_size: i32
    ) -> Result<Vec<Publisher>, String> {
        let publisher_rows = sqlx::query(r#"
            SELECT
                p.id,
                p.name,
                p.site,
                p.email,
                AVG(br.review)::float8 AS publisher_average,
                COUNT(br.review) AS total_reviews
            FROM publisher p
            WHERE p.deleted = false
            JOIN book b ON b.publisher_id = p.id
            JOIN book_review br ON br.book_id = b.id
            GROUP BY p.id, p.name
            ORDER BY publisher_average DESC
            LIMIT $1
            OFFSET $2;
            "#
        )
            .bind(page_size)
            .bind(skip)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        let mut publishers: Vec<Publisher> = Vec::new();

        for publisher_row in publisher_rows {
            let mut avatar: Option<String> = None;

            let publisher_id: Uuid = publisher_row.get("a.id");

            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    publisher_image
                WHERE
                    id = $1 AND deleted = false;
                "#)
                .bind(publisher_id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|e| e.to_string())?;

            if image_row.is_some() {
                avatar = Some(image_row.unwrap().image_path);
            }

            let mut publisher: Publisher = Publisher::new(
                publisher_row.get("p.id"),
                publisher_row.get("p.name"),
                publisher_row.get("p.site"),
                publisher_row.get("p.email"),
                None,
                None
            );

            publisher.set_avatar(avatar);

            publishers.push(publisher);
        }

        Ok(publishers)
    }

    async fn alter_publisher (
        &self,
        id: Uuid,
        name: String,
        _user_id: Uuid,
        site: Option<String>,
        email: Option<String>,
        file_name: Option<String>,
        file_content: Option<Bytes>
    ) -> Result<(), String> {

        if file_name.is_some() && file_content.is_some() {
            let image_row: Option<ImageRow> = sqlx::query_as(r#"
                SELECT
                    id, original_name, image_path
                FROM
                    publisher_image
                WHERE
                    publisher_id = $1 AND deleted = false;
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
                        publisher_image
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

            let path = format!("./{}/publisher/{}", UPLOADS_IMAGE_PATH, new_filename);

            let mut file = tokio::fs::File::create(&path).await.unwrap();
            file.write_all(&file_content.unwrap()).await.unwrap();

            sqlx::query(r#"
                INSERT INTO
                    publisher_image (id, original_name, image_path, publisher_id)
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
                    publisher_image
                WHERE
                    publisher_id = $1 AND deleted = false;
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
                        publisher_image
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
                publisher
            SET
                name = $2, site = $3, email = $4
            WHERE
                id = $1 AND deleted = false;
            "#
        )
            .bind(id)
            .bind(name)
            .bind(site)
            .bind(email)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn delete_publisher (&self, id: Uuid, _user_id: Uuid) -> Result<(), String> {
        sqlx::query(r#"
            UPDATE
                publisher_image
            SET
                deleted = true
            WHERE
                publisher_id = $1
            "#
        )
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query(r#"
            UPDATE
                publisher
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

    async fn clear_deleted_publishers(&self) -> Result<(), String> {
        let deleted_images: Vec<ImageRow> = sqlx::query_as(r#"
            SELECT
                id, original_name, image_path
            FROM
                publisher_image
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
                publisher_image
            WHERE deleted = true
            "#
        )
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        sqlx::query(r#"
            DELETE FROM
                publisher
            WHERE deleted = true
            "#
        )
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}