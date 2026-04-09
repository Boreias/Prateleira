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
    use sqlx::{
        Pool,
        Postgres,
        Row,
        query,
        query_as
    };
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
    async fn test_check_test_database() {
        // Confere existência do database
        dotenv().ok();
        let database_url = env::var("TESTE_DATABASE_URL")
            .expect("Variável de ambiente TESTE_DATABASE_URL não definida");

        let pool = create_pool(&database_url).await;

        let row: (bool,) = query_as("SELECT EXISTS (
            SELECT datname FROM pg_catalog.pg_database WHERE lower(datname) = lower('Prateleira_teste')
        );").fetch_one(&pool).await.unwrap();

        assert!(row.0);
    }

    async fn check_gender_table_schema(database_url: String) {

        let pool = create_pool(&database_url).await;

        let gender: (bool,) = query_as("SELECT EXISTS (
            SELECT 1 
            FROM information_schema.tables 
            WHERE table_schema = 'public' 
            AND table_name = 'gender'
        );").fetch_one(&pool).await.unwrap();

        assert!(gender.0);

        let schema = query(r#"
            SELECT
                column_name,
                data_type
            FROM
                information_schema.columns
            WHERE
                table_name = 'gender'
            ORDER BY column_name ASC;
        "#).fetch_all(&pool).await.unwrap();

        assert_eq!(schema.len(), 3);

        let column_name: String = schema[0].get("column_name");
        let data_type: String = schema[0].get("data_type");

        assert_eq!(column_name, "deleted".to_string());
        assert_eq!(data_type, "boolean".to_string());

        let column_name: String = schema[1].get("column_name");
        let data_type: String = schema[1].get("data_type");

        assert_eq!(column_name, "id".to_string());
        assert_eq!(data_type, "uuid".to_string());

        let column_name: String = schema[2].get("column_name");
        let data_type: String = schema[2].get("data_type");

        assert_eq!(column_name, "name".to_string());
        assert_eq!(data_type, "character varying".to_string());
    }

    #[tokio::test]
    async fn test_check_gender_table() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("Variável de ambiente DATABASE_URL não definida");
        
        check_gender_table_schema(database_url).await;
    }

    #[tokio::test]
    async fn test_check_gender_test_table() {
        dotenv().ok();
        let database_url = env::var("TESTE_DATABASE_URL")
            .expect("Variável de ambiente TESTE_DATABASE_URL não definida");

        check_gender_table_schema(database_url).await;
    }


    async fn check_author_table_schema(database_url: String) {

        let pool: Pool<Postgres> = create_pool(&database_url).await;

        let table: (bool,) = query_as("SELECT EXISTS (
            SELECT 1 
            FROM information_schema.tables 
            WHERE table_schema = 'public' 
            AND table_name = 'author'
        );").fetch_one(&pool).await.unwrap();

        assert!(table.0);

        let schema = query(r#"
            SELECT
                column_name,
                data_type
            FROM
                information_schema.columns
            WHERE
                table_name = 'author'
            ORDER BY column_name ASC;
        "#).fetch_all(&pool).await.unwrap();

        assert_eq!(schema.len(), 3);

        let column_name: String = schema[0].get("column_name");
        let data_type: String = schema[0].get("data_type");

        assert_eq!(column_name, "deleted".to_string());
        assert_eq!(data_type, "boolean".to_string());

        let column_name: String = schema[1].get("column_name");
        let data_type: String = schema[1].get("data_type");

        assert_eq!(column_name, "id".to_string());
        assert_eq!(data_type, "uuid".to_string());

        let column_name: String = schema[2].get("column_name");
        let data_type: String = schema[2].get("data_type");

        assert_eq!(column_name, "name".to_string());
        assert_eq!(data_type, "character varying".to_string());
    }

    #[tokio::test]
    async fn test_check_author_table() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("Variável de ambiente DATABASE_URL não definida");

        check_author_table_schema(database_url).await;
    }

    #[tokio::test]
    async fn test_check_author_test_table() {
        dotenv().ok();
        let database_url = env::var("TESTE_DATABASE_URL")
            .expect("Variável de ambiente TESTE_DATABASE_URL não definida");

        check_author_table_schema(database_url).await;
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