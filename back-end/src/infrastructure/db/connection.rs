use sqlx::{
    PgPool,
    postgres::PgPoolOptions
};


pub async fn create_pool(database_url: &str) -> PgPool {

    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .expect("Falha ao conectar no PostgreSQL")
}


#[cfg(test)]
mod tests {
    // use sqlx::{query, query_as};
    use sqlx::{query_as};
    use std::env;
    use dotenv::dotenv;

    use super::*;

    #[tokio::test]
    async fn test_check_database() {
        // Confere existência do database
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("Variável de ambiente DATABASE_URL não definida");

        let pool = create_pool(&database_url).await;

        let row: (bool,) = query_as("SELECT EXISTS (
            SELECT datname FROM pg_catalog.pg_database WHERE lower(datname) = lower('Prateleira')
        );").fetch_one(&pool).await.unwrap();

        assert!(row.0);
    }

    #[tokio::test]
    async fn test_check_gender_table() {
        // Confere existência das tabelas
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("Variável de ambiente DATABASE_URL não definida");
        let pool = create_pool(&database_url).await;

        let gender: (bool,) = query_as("SELECT EXISTS (
            SELECT 1 
            FROM information_schema.tables 
            WHERE table_schema = 'public' 
            AND table_name = 'gender'
        );").fetch_one(&pool).await.unwrap();

        assert!(gender.0);
    }

    // #[tokio::test]
    // async fn test_check_user_table() {
    //     // Confere existência das tabelas
    //     dotenv().ok();
    //     let database_url = env::var("DATABASE_URL")
    //         .expect("Variável de ambiente DATABASE_URL não definida");
    //     let pool = create_pool(&database_url).await;

    //     let table_result: (bool,) = query_as("SELECT EXISTS (
    //         SELECT 1 
    //         FROM information_schema.tables 
    //         WHERE table_schema = 'public' 
    //         AND table_name = 'user'
    //     );").fetch_one(&pool).await.unwrap();

    //     assert!(table_result.0);
    // }

    // #[tokio::test]
    // async fn test_check_search_index_table() {
    //     // Confere existência das tabelas
    //     dotenv().ok();
    //     let database_url = env::var("DATABASE_URL")
    //         .expect("Variável de ambiente DATABASE_URL não definida");
    //     let pool = create_pool(&database_url).await;

    //     let table_result: (bool,) = query_as("SELECT EXISTS (
    //         SELECT 1 
    //         FROM information_schema.tables 
    //         WHERE table_schema = 'public' 
    //         AND table_name = 'search_index'
    //     );").fetch_one(&pool).await.unwrap();

    //     assert!(table_result.0);
    // }

    // #[tokio::test]
    // async fn test_check_user_password_table() {
    //     // Confere existência das tabelas
    //     dotenv().ok();
    //     let database_url = env::var("DATABASE_URL")
    //         .expect("Variável de ambiente DATABASE_URL não definida");
    //     let pool = create_pool(&database_url).await;

    //     let table_result: (bool,) = query_as("SELECT EXISTS (
    //         SELECT 1 
    //         FROM information_schema.tables 
    //         WHERE table_schema = 'public' 
    //         AND table_name = 'user_password'
    //     );").fetch_one(&pool).await.unwrap();

    //     assert!(table_result.0);
    // }

    // #[tokio::test]
    // async fn test_check_substring_index_table() {
    //     // Confere existência das tabelas
    //     dotenv().ok();
    //     let database_url = env::var("DATABASE_URL")
    //         .expect("Variável de ambiente DATABASE_URL não definida");
    //     let pool = create_pool(&database_url).await;

    //     let table_result: (bool,) = query_as("SELECT EXISTS (
    //         SELECT 1 
    //         FROM information_schema.tables 
    //         WHERE table_schema = 'public' 
    //         AND table_name = 'substring_index'
    //     );").fetch_one(&pool).await.unwrap();

    //     assert!(table_result.0);
    // }

    // #[tokio::test]
    // async fn test_check_author_table() {
    //     // Confere existência das tabelas
    //     dotenv().ok();
    //     let database_url = env::var("DATABASE_URL")
    //         .expect("Variável de ambiente DATABASE_URL não definida");
    //     let pool = create_pool(&database_url).await;

    //     let table_result: (bool,) = query_as("SELECT EXISTS (
    //         SELECT 1 
    //         FROM information_schema.tables 
    //         WHERE table_schema = 'public' 
    //         AND table_name = 'author'
    //     );").fetch_one(&pool).await.unwrap();

    //     assert!(table_result.0);
    // }

    // #[tokio::test]
    // async fn test_check_publisher_table() {
    //     // Confere existência das tabelas
    //     dotenv().ok();
    //     let database_url = env::var("DATABASE_URL")
    //         .expect("Variável de ambiente DATABASE_URL não definida");
    //     let pool = create_pool(&database_url).await;

    //     let table_result: (bool,) = query_as("SELECT EXISTS (
    //         SELECT 1 
    //         FROM information_schema.tables 
    //         WHERE table_schema = 'public' 
    //         AND table_name = 'publisher'
    //     );").fetch_one(&pool).await.unwrap();

    //     assert!(table_result.0);
    // }

    // #[tokio::test]
    // async fn test_check_book_table() {
    //     // Confere existência das tabelas
    //     dotenv().ok();
    //     let database_url = env::var("DATABASE_URL")
    //         .expect("Variável de ambiente DATABASE_URL não definida");
    //     let pool = create_pool(&database_url).await;

    //     let table_result: (bool,) = query_as("SELECT EXISTS (
    //         SELECT 1 
    //         FROM information_schema.tables 
    //         WHERE table_schema = 'public' 
    //         AND table_name = 'book'
    //     );").fetch_one(&pool).await.unwrap();

    //     assert!(table_result.0);
    // }

    // #[tokio::test]
    // async fn test_check_book_author_table() {
    //     // Confere existência das tabelas
    //     dotenv().ok();
    //     let database_url = env::var("DATABASE_URL")
    //         .expect("Variável de ambiente DATABASE_URL não definida");
    //     let pool = create_pool(&database_url).await;

    //     let table_result: (bool,) = query_as("SELECT EXISTS (
    //         SELECT 1 
    //         FROM information_schema.tables 
    //         WHERE table_schema = 'public' 
    //         AND table_name = 'book_author'
    //     );").fetch_one(&pool).await.unwrap();

    //     assert!(table_result.0);
    // }

    // #[tokio::test]
    // async fn test_check_book_gender_table() {
    //     // Confere existência das tabelas
    //     dotenv().ok();
    //     let database_url = env::var("DATABASE_URL")
    //         .expect("Variável de ambiente DATABASE_URL não definida");
    //     let pool = create_pool(&database_url).await;

    //     let table_result: (bool,) = query_as("SELECT EXISTS (
    //         SELECT 1 
    //         FROM information_schema.tables 
    //         WHERE table_schema = 'public' 
    //         AND table_name = 'book_gender'
    //     );").fetch_one(&pool).await.unwrap();

    //     assert!(table_result.0);
    // }

    // #[tokio::test]
    // async fn test_check_reading_status_table() {
    //     // Confere existência das tabelas
    //     dotenv().ok();
    //     let database_url = env::var("DATABASE_URL")
    //         .expect("Variável de ambiente DATABASE_URL não definida");
    //     let pool = create_pool(&database_url).await;

    //     let table_result: (bool,) = query_as("SELECT EXISTS (
    //         SELECT 1 
    //         FROM information_schema.tables 
    //         WHERE table_schema = 'public' 
    //         AND table_name = 'reading_status'
    //     );").fetch_one(&pool).await.unwrap();

    //     assert!(table_result.0);
    // }

    // #[tokio::test]
    // async fn test_check_book_user_table() {
    //     // Confere existência das tabelas
    //     dotenv().ok();
    //     let database_url = env::var("DATABASE_URL")
    //         .expect("Variável de ambiente DATABASE_URL não definida");
    //     let pool = create_pool(&database_url).await;

    //     let table_result: (bool,) = query_as("SELECT EXISTS (
    //         SELECT 1 
    //         FROM information_schema.tables 
    //         WHERE table_schema = 'public' 
    //         AND table_name = 'book_user'
    //     );").fetch_one(&pool).await.unwrap();

    //     assert!(table_result.0);
    // }

    // #[tokio::test]
    // async fn test_check_user_friend_request_table() {
    //     // Confere existência das tabelas
    //     dotenv().ok();
    //     let database_url = env::var("DATABASE_URL")
    //         .expect("Variável de ambiente DATABASE_URL não definida");
    //     let pool = create_pool(&database_url).await;

    //     let table_result: (bool,) = query_as("SELECT EXISTS (
    //         SELECT 1 
    //         FROM information_schema.tables 
    //         WHERE table_schema = 'public' 
    //         AND table_name = 'user_friend_request'
    //     );").fetch_one(&pool).await.unwrap();

    //     assert!(table_result.0);
    // }

    // #[tokio::test]
    // async fn test_check_user_friendship_table() {
    //     // Confere existência das tabelas
    //     dotenv().ok();
    //     let database_url = env::var("DATABASE_URL")
    //         .expect("Variável de ambiente DATABASE_URL não definida");
    //     let pool = create_pool(&database_url).await;

    //     let table_result: (bool,) = query_as("SELECT EXISTS (
    //         SELECT 1 
    //         FROM information_schema.tables 
    //         WHERE table_schema = 'public' 
    //         AND table_name = 'user_friendship'
    //     );").fetch_one(&pool).await.unwrap();

    //     assert!(table_result.0);
    // }

    // #[tokio::test]
    // async fn test_table_author() {
    //     dotenv().ok();
    //     let database_url = env::var("DATABASE_URL")
    //         .expect("Variável de ambiente DATABASE_URL não definida");
    //     let pool = create_pool(&database_url).await;

    //     let id = Uuid::new_v4();
    //     let author_name = "Teste Nome".to_string();

    //     let create_result = query("INSERT INTO Author (id, name) VALUES ($1, $2)")
    //         .bind(id.clone().to_string())
    //         .bind(author_name.clone())
    //         .execute(&pool)
    //         .await
    //         .map_err(|e| e.to_string()).unwrap();

    //     assert_eq!(create_result.rows_affected(), 1);

    //     let select_result: Vec<AuthorRow> = query_as("SELECT id, name FROM Author WHERE id = ?")
    //         .bind(id.clone().to_string())
    //         .fetch_all(&pool)
    //         .await
    //         .unwrap();

    //     assert_eq!(select_result.len(), 1);
    //     assert_eq!(select_result[0].name, author_name);
    // }
}