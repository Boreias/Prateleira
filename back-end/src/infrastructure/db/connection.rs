use sqlx::{
    PgPool,
    Error,
    postgres::PgPoolOptions
};
use std::env;

pub async fn create_pool() -> PgPool {
    let database_url = env::var("DATABASE_URL")
        .expect("Variável de ambiente DATABASE_URL não definida");

    PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Falha ao conectar no PostgreSQL")
}


#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use sqlx::{query, query_as, query_scalar};
    use uuid::Uuid;

    use super::*;

    #[tokio::test]
    async fn test_check_database() {
        // Confere existência do database
        let pool = create_pool().await;

        let row: (bool,) = query_as("SELECT EXISTS (
            SELECT datname FROM pg_catalog.pg_database WHERE lower(datname) = lower('Prateleira')
        );").fetch_one(&pool).await.unwrap();

        assert!(row.0);
    }

    #[tokio::test]
    async fn test_check_tables() {
        // Confere existência das tabelas
        let pool = create_pool().await;

        let user: (bool,) = query_as("SELECT EXISTS (
            SELECT 1
            FROM pg_tables
            WHERE schemaname = 'public' -- Or your specific schema
            AND tablename = 'User'
        );").fetch_one(&pool).await.unwrap();

        let search_index: (bool,) = query_as("SELECT EXISTS (
            SELECT 1
            FROM pg_tables
            WHERE schemaname = 'public' -- Or your specific schema
            AND tablename = 'Search_Index'
        );").fetch_one(&pool).await.unwrap();

        let user_password: (bool,) = query_as("SELECT EXISTS (
            SELECT 1
            FROM pg_tables
            WHERE schemaname = 'public' -- Or your specific schema
            AND tablename = 'User_Password'
        );").fetch_one(&pool).await.unwrap();

        let substring_index: (bool,) = query_as("SELECT EXISTS (
            SELECT 1
            FROM pg_tables
            WHERE schemaname = 'public' -- Or your specific schema
            AND tablename = 'Substring_Index'
        );").fetch_one(&pool).await.unwrap();

        let author: (bool,) = query_as("SELECT EXISTS (
            SELECT 1
            FROM pg_tables
            WHERE schemaname = 'public' -- Or your specific schema
            AND tablename = 'Author'
        );").fetch_one(&pool).await.unwrap();

        let publisher: (bool,) = query_as("SELECT EXISTS (
            SELECT 1
            FROM pg_tables
            WHERE schemaname = 'public' -- Or your specific schema
            AND tablename = 'Publisher'
        );").fetch_one(&pool).await.unwrap();

        let gender: (bool,) = query_as("SELECT EXISTS (
            SELECT 1
            FROM pg_tables
            WHERE schemaname = 'public' -- Or your specific schema
            AND tablename = 'Gender'
        );").fetch_one(&pool).await.unwrap();

        let book: (bool,) = query_as("SELECT EXISTS (
            SELECT 1
            FROM pg_tables
            WHERE schemaname = 'public' -- Or your specific schema
            AND tablename = 'Book'
        );").fetch_one(&pool).await.unwrap();

        let book_author: (bool,) = query_as("SELECT EXISTS (
            SELECT 1
            FROM pg_tables
            WHERE schemaname = 'public' -- Or your specific schema
            AND tablename = 'BookAuthor'
        );").fetch_one(&pool).await.unwrap();

        let book_gender: (bool,) = query_as("SELECT EXISTS (
            SELECT 1
            FROM pg_tables
            WHERE schemaname = 'public' -- Or your specific schema
            AND tablename = 'BookGender'
        );").fetch_one(&pool).await.unwrap();

        let reading_status: (bool,) = query_as("SELECT EXISTS (
            SELECT 1
            FROM pg_tables
            WHERE schemaname = 'public' -- Or your specific schema
            AND tablename = 'ReadingStatus'
        );").fetch_one(&pool).await.unwrap();

        let book_user: (bool,) = query_as("SELECT EXISTS (
            SELECT 1
            FROM pg_tables
            WHERE schemaname = 'public' -- Or your specific schema
            AND tablename = 'BookUser'
        );").fetch_one(&pool).await.unwrap();

        let user_friend_request: (bool,) = query_as("SELECT EXISTS (
            SELECT 1
            FROM pg_tables
            WHERE schemaname = 'public' -- Or your specific schema
            AND tablename = 'UserFriendRequest'
        );").fetch_one(&pool).await.unwrap();

        let user_friendship: (bool,) = query_as("SELECT EXISTS (
            SELECT 1
            FROM pg_tables
            WHERE schemaname = 'public' -- Or your specific schema
            AND tablename = 'UserFriendship'
        );").fetch_one(&pool).await.unwrap();

        assert!(user.0);
        assert!(search_index.0);
        assert!(user_password.0);
        assert!(substring_index.0);
        assert!(author.0);
        assert!(publisher.0);
        assert!(gender.0);
        assert!(book.0);
        assert!(book_author.0);
        assert!(book_gender.0);
        assert!(reading_status.0);
        assert!(book_user.0);
        assert!(user_friend_request.0);
        assert!(user_friendship.0);
    }

    #[tokio::test]
    async fn test_table_author() {
        let pool = create_pool().await;

        let id = Uuid::new_v4();
        let author_name = "Teste Nome".to_string();

        let create_result = query("INSERT INTO Author (id, name) VALUES ($1, $2)")
            .bind(id.clone().to_string())
            .bind(author_name.clone())
            .execute(&pool)
            .await
            .map_err(|e| e.to_string()).unwrap();

        assert_eq!(create_result.rows_affected(), 1);

        let select_result = sqlx::query!("SELECT id, name FROM Author WHERE id = ?", id.clone().to_string())
            .fetch_all(&pool)
            .await
            .unwrap();

        assert_eq!(select_result.len(), 1);
        assert_eq!(select_result[0].name, author_name);
    }
}