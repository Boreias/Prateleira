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


    async fn check_author_image_table_schema(database_url: String) {

        let pool: Pool<Postgres> = create_pool(&database_url).await;

        let table: (bool,) = query_as("SELECT EXISTS (
            SELECT 1 
            FROM information_schema.tables 
            WHERE table_schema = 'public' 
            AND table_name = 'author_image'
        );").fetch_one(&pool).await.unwrap();

        assert!(table.0);

        let schema = query(r#"
            SELECT
                column_name,
                data_type
            FROM
                information_schema.columns
            WHERE
                table_name = 'author_image'
            ORDER BY column_name ASC;
        "#).fetch_all(&pool).await.unwrap();

        assert_eq!(schema.len(), 5);

        let column_name: String = schema[0].get("column_name");
        let data_type: String = schema[0].get("data_type");

        assert_eq!(column_name, "author_id".to_string());
        assert_eq!(data_type, "uuid".to_string());

        let column_name: String = schema[1].get("column_name");
        let data_type: String = schema[1].get("data_type");

        assert_eq!(column_name, "deleted".to_string());
        assert_eq!(data_type, "boolean".to_string());

        let column_name: String = schema[2].get("column_name");
        let data_type: String = schema[2].get("data_type");

        assert_eq!(column_name, "id".to_string());
        assert_eq!(data_type, "uuid".to_string());

        let column_name: String = schema[3].get("column_name");
        let data_type: String = schema[3].get("data_type");

        assert_eq!(column_name, "image_path".to_string());
        assert_eq!(data_type, "text".to_string());

        let column_name: String = schema[4].get("column_name");
        let data_type: String = schema[4].get("data_type");

        assert_eq!(column_name, "original_name".to_string());
        assert_eq!(data_type, "character varying".to_string());
    }

    #[tokio::test]
    async fn test_check_author_image_table() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("Variável de ambiente DATABASE_URL não definida");

        check_author_image_table_schema(database_url).await;
    }

    #[tokio::test]
    async fn test_check_author_image_image_test_table() {
        dotenv().ok();
        let database_url = env::var("TESTE_DATABASE_URL")
            .expect("Variável de ambiente TESTE_DATABASE_URL não definida");

        check_author_image_table_schema(database_url).await;
    }


    async fn check_publisher_table_schema(database_url: String) {

        let pool: Pool<Postgres> = create_pool(&database_url).await;

        let table: (bool,) = query_as("SELECT EXISTS (
            SELECT 1 
            FROM information_schema.tables 
            WHERE table_schema = 'public' 
            AND table_name = 'publisher'
        );").fetch_one(&pool).await.unwrap();

        assert!(table.0);

        let schema = query(r#"
            SELECT
                column_name,
                data_type
            FROM
                information_schema.columns
            WHERE
                table_name = 'publisher'
            ORDER BY column_name ASC;
        "#).fetch_all(&pool).await.unwrap();

        assert_eq!(schema.len(), 5);

        let column_name: String = schema[0].get("column_name");
        let data_type: String = schema[0].get("data_type");

        assert_eq!(column_name, "deleted".to_string());
        assert_eq!(data_type, "boolean".to_string());

        let column_name: String = schema[1].get("column_name");
        let data_type: String = schema[1].get("data_type");

        assert_eq!(column_name, "email".to_string());
        assert_eq!(data_type, "character varying".to_string());

        let column_name: String = schema[2].get("column_name");
        let data_type: String = schema[2].get("data_type");

        assert_eq!(column_name, "id".to_string());
        assert_eq!(data_type, "uuid".to_string());

        let column_name: String = schema[3].get("column_name");
        let data_type: String = schema[3].get("data_type");

        assert_eq!(column_name, "name".to_string());
        assert_eq!(data_type, "character varying".to_string());

        let column_name: String = schema[4].get("column_name");
        let data_type: String = schema[4].get("data_type");

        assert_eq!(column_name, "site".to_string());
        assert_eq!(data_type, "character varying".to_string());
    }

    #[tokio::test]
    async fn test_check_publisher_table() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("Variável de ambiente DATABASE_URL não definida");

        check_publisher_table_schema(database_url).await;
    }

    #[tokio::test]
    async fn test_check_publisher_test_table() {
        dotenv().ok();
        let database_url = env::var("TESTE_DATABASE_URL")
            .expect("Variável de ambiente TESTE_DATABASE_URL não definida");

        check_publisher_table_schema(database_url).await;
    }


    async fn check_publisher_image_table_schema(database_url: String) {

        let pool: Pool<Postgres> = create_pool(&database_url).await;

        let table: (bool,) = query_as("SELECT EXISTS (
            SELECT 1 
            FROM information_schema.tables 
            WHERE table_schema = 'public' 
            AND table_name = 'publisher_image'
        );").fetch_one(&pool).await.unwrap();

        assert!(table.0);

        let schema = query(r#"
            SELECT
                column_name,
                data_type
            FROM
                information_schema.columns
            WHERE
                table_name = 'publisher_image'
            ORDER BY column_name ASC;
        "#).fetch_all(&pool).await.unwrap();

        assert_eq!(schema.len(), 5);

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

        assert_eq!(column_name, "image_path".to_string());
        assert_eq!(data_type, "text".to_string());

        let column_name: String = schema[3].get("column_name");
        let data_type: String = schema[3].get("data_type");

        assert_eq!(column_name, "original_name".to_string());
        assert_eq!(data_type, "character varying".to_string());

        let column_name: String = schema[4].get("column_name");
        let data_type: String = schema[4].get("data_type");

        assert_eq!(column_name, "publisher_id".to_string());
        assert_eq!(data_type, "uuid".to_string());
    }

    #[tokio::test]
    async fn test_check_publisher_image_table() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("Variável de ambiente DATABASE_URL não definida");

        check_publisher_image_table_schema(database_url).await;
    }

    #[tokio::test]
    async fn test_check_publisher_image_test_table() {
        dotenv().ok();
        let database_url = env::var("TESTE_DATABASE_URL")
            .expect("Variável de ambiente TESTE_DATABASE_URL não definida");

        check_publisher_image_table_schema(database_url).await;
    }


    async fn check_book_table_schema(database_url: String) {

        let pool: Pool<Postgres> = create_pool(&database_url).await;

        let table: (bool,) = query_as("SELECT EXISTS (
            SELECT 1 
            FROM information_schema.tables 
            WHERE table_schema = 'public' 
            AND table_name = 'book'
        );").fetch_one(&pool).await.unwrap();

        assert!(table.0);

        let schema = query(r#"
            SELECT
                column_name,
                data_type
            FROM
                information_schema.columns
            WHERE
                table_name = 'book'
            ORDER BY column_name ASC;
        "#).fetch_all(&pool).await.unwrap();

        assert_eq!(schema.len(), 13);

        let column_name: String = schema[0].get("column_name");
        let data_type: String = schema[0].get("data_type");

        assert_eq!(column_name, "deleted".to_string());
        assert_eq!(data_type, "boolean".to_string());

        let column_name: String = schema[1].get("column_name");
        let data_type: String = schema[1].get("data_type");

        assert_eq!(column_name, "edition".to_string());
        assert_eq!(data_type, "integer".to_string());

        let column_name: String = schema[2].get("column_name");
        let data_type: String = schema[2].get("data_type");

        assert_eq!(column_name, "id".to_string());
        assert_eq!(data_type, "uuid".to_string());

        let column_name: String = schema[3].get("column_name");
        let data_type: String = schema[3].get("data_type");

        assert_eq!(column_name, "isbn".to_string());
        assert_eq!(data_type, "character varying".to_string());

        let column_name: String = schema[4].get("column_name");
        let data_type: String = schema[4].get("data_type");

        assert_eq!(column_name, "language".to_string());
        assert_eq!(data_type, "character varying".to_string());

        let column_name: String = schema[5].get("column_name");
        let data_type: String = schema[5].get("data_type");

        assert_eq!(column_name, "pages".to_string());
        assert_eq!(data_type, "integer".to_string());

        let column_name: String = schema[6].get("column_name");
        let data_type: String = schema[6].get("data_type");

        assert_eq!(column_name, "publication_year".to_string());
        assert_eq!(data_type, "integer".to_string());

        let column_name: String = schema[7].get("column_name");
        let data_type: String = schema[7].get("data_type");

        assert_eq!(column_name, "publisher_id".to_string());
        assert_eq!(data_type, "uuid".to_string());

        let column_name: String = schema[8].get("column_name");
        let data_type: String = schema[8].get("data_type");

        assert_eq!(column_name, "series_collection".to_string());
        assert_eq!(data_type, "integer".to_string());

        let column_name: String = schema[9].get("column_name");
        let data_type: String = schema[9].get("data_type");

        assert_eq!(column_name, "subtitle".to_string());
        assert_eq!(data_type, "character varying".to_string());

        let column_name: String = schema[10].get("column_name");
        let data_type: String = schema[10].get("data_type");

        assert_eq!(column_name, "synopsis".to_string());
        assert_eq!(data_type, "text".to_string());

        let column_name: String = schema[11].get("column_name");
        let data_type: String = schema[11].get("data_type");

        assert_eq!(column_name, "title".to_string());
        assert_eq!(data_type, "character varying".to_string());

        let column_name: String = schema[12].get("column_name");
        let data_type: String = schema[12].get("data_type");

        assert_eq!(column_name, "volume".to_string());
        assert_eq!(data_type, "integer".to_string());
    }

    #[tokio::test]
    async fn test_check_book_table() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("Variável de ambiente DATABASE_URL não definida");

        check_book_table_schema(database_url).await;
    }

    #[tokio::test]
    async fn test_check_book_test_table() {
        dotenv().ok();
        let database_url = env::var("TESTE_DATABASE_URL")
            .expect("Variável de ambiente TESTE_DATABASE_URL não definida");

        check_book_table_schema(database_url).await;
    }


    async fn check_book_image_table_schema(database_url: String) {

        let pool: Pool<Postgres> = create_pool(&database_url).await;

        let table: (bool,) = query_as("SELECT EXISTS (
            SELECT 1 
            FROM information_schema.tables 
            WHERE table_schema = 'public' 
            AND table_name = 'book_image'
        );").fetch_one(&pool).await.unwrap();

        assert!(table.0);

        let schema = query(r#"
            SELECT
                column_name,
                data_type
            FROM
                information_schema.columns
            WHERE
                table_name = 'book_image'
            ORDER BY column_name ASC;
        "#).fetch_all(&pool).await.unwrap();

        assert_eq!(schema.len(), 5);

        let column_name: String = schema[0].get("column_name");
        let data_type: String = schema[0].get("data_type");

        assert_eq!(column_name, "book_id".to_string());
        assert_eq!(data_type, "uuid".to_string());

        let column_name: String = schema[1].get("column_name");
        let data_type: String = schema[1].get("data_type");

        assert_eq!(column_name, "deleted".to_string());
        assert_eq!(data_type, "boolean".to_string());

        let column_name: String = schema[2].get("column_name");
        let data_type: String = schema[2].get("data_type");

        assert_eq!(column_name, "id".to_string());
        assert_eq!(data_type, "uuid".to_string());

        let column_name: String = schema[3].get("column_name");
        let data_type: String = schema[3].get("data_type");

        assert_eq!(column_name, "image_path".to_string());
        assert_eq!(data_type, "text".to_string());

        let column_name: String = schema[4].get("column_name");
        let data_type: String = schema[4].get("data_type");

        assert_eq!(column_name, "original_name".to_string());
        assert_eq!(data_type, "character varying".to_string());
    }

    #[tokio::test]
    async fn test_check_book_image_table() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("Variável de ambiente DATABASE_URL não definida");

        check_book_image_table_schema(database_url).await;
    }

    #[tokio::test]
    async fn test_check_book_image_test_table() {
        dotenv().ok();
        let database_url = env::var("TESTE_DATABASE_URL")
            .expect("Variável de ambiente TESTE_DATABASE_URL não definida");

        check_book_image_table_schema(database_url).await;
    }


    async fn check_book_gender_table_schema(database_url: String) {

        let pool: Pool<Postgres> = create_pool(&database_url).await;

        let table: (bool,) = query_as("SELECT EXISTS (
            SELECT 1 
            FROM information_schema.tables 
            WHERE table_schema = 'public' 
            AND table_name = 'book_gender'
        );").fetch_one(&pool).await.unwrap();

        assert!(table.0);

        let schema = query(r#"
            SELECT
                column_name,
                data_type
            FROM
                information_schema.columns
            WHERE
                table_name = 'book_gender'
            ORDER BY column_name ASC;
        "#).fetch_all(&pool).await.unwrap();

        assert_eq!(schema.len(), 4);

        let column_name: String = schema[0].get("column_name");
        let data_type: String = schema[0].get("data_type");

        assert_eq!(column_name, "book_id".to_string());
        assert_eq!(data_type, "uuid".to_string());

        let column_name: String = schema[1].get("column_name");
        let data_type: String = schema[1].get("data_type");

        assert_eq!(column_name, "deleted".to_string());
        assert_eq!(data_type, "boolean".to_string());

        let column_name: String = schema[2].get("column_name");
        let data_type: String = schema[2].get("data_type");

        assert_eq!(column_name, "gender_id".to_string());
        assert_eq!(data_type, "uuid".to_string());

        let column_name: String = schema[3].get("column_name");
        let data_type: String = schema[3].get("data_type");

        assert_eq!(column_name, "id".to_string());
        assert_eq!(data_type, "uuid".to_string());
    }

    #[tokio::test]
    async fn test_check_book_gender_table() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("Variável de ambiente DATABASE_URL não definida");

        check_book_gender_table_schema(database_url).await;
    }

    #[tokio::test]
    async fn test_check_book_gender_test_table() {
        dotenv().ok();
        let database_url = env::var("TESTE_DATABASE_URL")
            .expect("Variável de ambiente TESTE_DATABASE_URL não definida");

        check_book_gender_table_schema(database_url).await;
    }


    async fn check_book_author_table_schema(database_url: String) {

        let pool: Pool<Postgres> = create_pool(&database_url).await;

        let table: (bool,) = query_as("SELECT EXISTS (
            SELECT 1 
            FROM information_schema.tables
            WHERE table_schema = 'public'
            AND table_name = 'book_author'
        );").fetch_one(&pool).await.unwrap();

        assert!(table.0);

        let schema = query(r#"
            SELECT
                column_name,
                data_type
            FROM
                information_schema.columns
            WHERE
                table_name = 'book_author'
            ORDER BY column_name ASC;
        "#).fetch_all(&pool).await.unwrap();

        assert_eq!(schema.len(), 4);

        let column_name: String = schema[0].get("column_name");
        let data_type: String = schema[0].get("data_type");

        assert_eq!(column_name, "author_id".to_string());
        assert_eq!(data_type, "uuid".to_string());

        let column_name: String = schema[1].get("column_name");
        let data_type: String = schema[1].get("data_type");

        assert_eq!(column_name, "book_id".to_string());
        assert_eq!(data_type, "uuid".to_string());

        let column_name: String = schema[2].get("column_name");
        let data_type: String = schema[2].get("data_type");

        assert_eq!(column_name, "deleted".to_string());
        assert_eq!(data_type, "boolean".to_string());

        let column_name: String = schema[3].get("column_name");
        let data_type: String = schema[3].get("data_type");

        assert_eq!(column_name, "id".to_string());
        assert_eq!(data_type, "uuid".to_string());
    }

    #[tokio::test]
    async fn test_check_book_author_table() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("Variável de ambiente DATABASE_URL não definida");

        check_book_author_table_schema(database_url).await;
    }

    #[tokio::test]
    async fn test_check_book_author_test_table() {
        dotenv().ok();
        let database_url = env::var("TESTE_DATABASE_URL")
            .expect("Variável de ambiente TESTE_DATABASE_URL não definida");

        check_book_author_table_schema(database_url).await;
    }


    async fn check_user_auth_table_schema(database_url: String) {

        let pool: Pool<Postgres> = create_pool(&database_url).await;

        let table: (bool,) = query_as("SELECT EXISTS (
            SELECT 1 
            FROM information_schema.tables
            WHERE table_schema = 'public'
            AND table_name = 'user_auth'
        );").fetch_one(&pool).await.unwrap();

        assert!(table.0);

        let schema = query(r#"
            SELECT
                column_name,
                data_type
            FROM
                information_schema.columns
            WHERE
                table_name = 'user_auth'
            ORDER BY column_name ASC;
        "#).fetch_all(&pool).await.unwrap();

        assert_eq!(schema.len(), 10);

        let column_name: String = schema[0].get("column_name");
        let data_type: String = schema[0].get("data_type");

        assert_eq!(column_name, "country".to_string());
        assert_eq!(data_type, "character varying".to_string());

        let column_name: String = schema[1].get("column_name");
        let data_type: String = schema[1].get("data_type");

        assert_eq!(column_name, "created_at".to_string());
        assert_eq!(data_type, "timestamp without time zone".to_string());

        let column_name: String = schema[2].get("column_name");
        let data_type: String = schema[2].get("data_type");

        assert_eq!(column_name, "deleted_at".to_string());
        assert_eq!(data_type, "timestamp without time zone".to_string());

        let column_name: String = schema[3].get("column_name");
        let data_type: String = schema[3].get("data_type");

        assert_eq!(column_name, "email".to_string());
        assert_eq!(data_type, "character varying".to_string());

        let column_name: String = schema[4].get("column_name");
        let data_type: String = schema[4].get("data_type");

        assert_eq!(column_name, "id".to_string());
        assert_eq!(data_type, "uuid".to_string());

        let column_name: String = schema[5].get("column_name");
        let data_type: String = schema[5].get("data_type");

        assert_eq!(column_name, "is_email_verified".to_string());
        assert_eq!(data_type, "boolean".to_string());

        let column_name: String = schema[6].get("column_name");
        let data_type: String = schema[6].get("data_type");

        assert_eq!(column_name, "password".to_string());
        assert_eq!(data_type, "text".to_string());

        let column_name: String = schema[7].get("column_name");
        let data_type: String = schema[7].get("data_type");

        assert_eq!(column_name, "salt".to_string());
        assert_eq!(data_type, "character varying".to_string());

        let column_name: String = schema[8].get("column_name");
        let data_type: String = schema[8].get("data_type");

        assert_eq!(column_name, "updated_at".to_string());
        assert_eq!(data_type, "timestamp without time zone".to_string());

        let column_name: String = schema[9].get("column_name");
        let data_type: String = schema[9].get("data_type");

        assert_eq!(column_name, "username".to_string());
        assert_eq!(data_type, "character varying".to_string());
    }

    #[tokio::test]
    async fn test_check_user_auth_table() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("Variável de ambiente DATABASE_URL não definida");

        check_user_auth_table_schema(database_url).await;
    }

    #[tokio::test]
    async fn test_check_user_auth_test_table() {
        dotenv().ok();
        let database_url = env::var("TESTE_DATABASE_URL")
            .expect("Variável de ambiente TESTE_DATABASE_URL não definida");

        check_user_auth_table_schema(database_url).await;
    }


    async fn check_user_profile_table_schema(database_url: String) {

        let pool: Pool<Postgres> = create_pool(&database_url).await;

        let table: (bool,) = query_as("SELECT EXISTS (
            SELECT 1 
            FROM information_schema.tables
            WHERE table_schema = 'public'
            AND table_name = 'user_profile'
        );").fetch_one(&pool).await.unwrap();

        assert!(table.0);

        let schema = query(r#"
            SELECT
                column_name,
                data_type
            FROM
                information_schema.columns
            WHERE
                table_name = 'user_profile'
            ORDER BY column_name ASC;
        "#).fetch_all(&pool).await.unwrap();

        assert_eq!(schema.len(), 4);

        let column_name: String = schema[0].get("column_name");
        let data_type: String = schema[0].get("data_type");

        assert_eq!(column_name, "bio".to_string());
        assert_eq!(data_type, "text".to_string());

        let column_name: String = schema[1].get("column_name");
        let data_type: String = schema[1].get("data_type");

        assert_eq!(column_name, "birth_date".to_string());
        assert_eq!(data_type, "date".to_string());

        let column_name: String = schema[2].get("column_name");
        let data_type: String = schema[2].get("data_type");

        assert_eq!(column_name, "id".to_string());
        assert_eq!(data_type, "uuid".to_string());

        let column_name: String = schema[3].get("column_name");
        let data_type: String = schema[3].get("data_type");

        assert_eq!(column_name, "name".to_string());
        assert_eq!(data_type, "character varying".to_string());
    }

    #[tokio::test]
    async fn test_check_user_profile_table() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("Variável de ambiente DATABASE_URL não definida");

        check_user_profile_table_schema(database_url).await;
    }

    #[tokio::test]
    async fn test_check_user_profile_test_table() {
        dotenv().ok();
        let database_url = env::var("TESTE_DATABASE_URL")
            .expect("Variável de ambiente TESTE_DATABASE_URL não definida");

        check_user_profile_table_schema(database_url).await;
    }


    async fn check_user_refresh_token_table_schema(database_url: String) {

        let pool: Pool<Postgres> = create_pool(&database_url).await;

        let table: (bool,) = query_as("SELECT EXISTS (
            SELECT 1 
            FROM information_schema.tables
            WHERE table_schema = 'public'
            AND table_name = 'user_refresh_token'
        );").fetch_one(&pool).await.unwrap();

        assert!(table.0);

        let schema = query(r#"
            SELECT
                column_name,
                data_type
            FROM
                information_schema.columns
            WHERE
                table_name = 'user_refresh_token'
            ORDER BY column_name ASC;
        "#).fetch_all(&pool).await.unwrap();

        assert_eq!(schema.len(), 3);

        let column_name: String = schema[0].get("column_name");
        let data_type: String = schema[0].get("data_type");

        assert_eq!(column_name, "expire_at".to_string());
        assert_eq!(data_type, "timestamp without time zone".to_string());

        let column_name: String = schema[1].get("column_name");
        let data_type: String = schema[1].get("data_type");

        assert_eq!(column_name, "id".to_string());
        assert_eq!(data_type, "uuid".to_string());

        let column_name: String = schema[2].get("column_name");
        let data_type: String = schema[2].get("data_type");

        assert_eq!(column_name, "token".to_string());
        assert_eq!(data_type, "text".to_string());
    }

    #[tokio::test]
    async fn test_check_user_refresh_token_table() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("Variável de ambiente DATABASE_URL não definida");

        check_user_refresh_token_table_schema(database_url).await;
    }

    #[tokio::test]
    async fn test_check_user_refresh_token_test_table() {
        dotenv().ok();
        let database_url = env::var("TESTE_DATABASE_URL")
            .expect("Variável de ambiente TESTE_DATABASE_URL não definida");

        check_user_refresh_token_table_schema(database_url).await;
    }


    async fn check_user_email_verification_table_schema(database_url: String) {

        let pool: Pool<Postgres> = create_pool(&database_url).await;

        let table: (bool,) = query_as("SELECT EXISTS (
            SELECT 1 
            FROM information_schema.tables
            WHERE table_schema = 'public'
            AND table_name = 'user_email_verification'
        );").fetch_one(&pool).await.unwrap();

        assert!(table.0);

        let schema = query(r#"
            SELECT
                column_name,
                data_type
            FROM
                information_schema.columns
            WHERE
                table_name = 'user_email_verification'
            ORDER BY column_name ASC;
        "#).fetch_all(&pool).await.unwrap();

        assert_eq!(schema.len(), 2);

        let column_name: String = schema[0].get("column_name");
        let data_type: String = schema[0].get("data_type");

        assert_eq!(column_name, "id".to_string());
        assert_eq!(data_type, "uuid".to_string());

        let column_name: String = schema[1].get("column_name");
        let data_type: String = schema[1].get("data_type");

        assert_eq!(column_name, "token".to_string());
        assert_eq!(data_type, "character varying".to_string());
    }

    #[tokio::test]
    async fn test_check_user_email_verification_table() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("Variável de ambiente DATABASE_URL não definida");

        check_user_email_verification_table_schema(database_url).await;
    }

    #[tokio::test]
    async fn test_check_user_email_verification_test_table() {
        dotenv().ok();
        let database_url = env::var("TESTE_DATABASE_URL")
            .expect("Variável de ambiente TESTE_DATABASE_URL não definida");

        check_user_email_verification_table_schema(database_url).await;
    }


    async fn check_user_image_table_schema(database_url: String) {

        let pool: Pool<Postgres> = create_pool(&database_url).await;

        let table: (bool,) = query_as("SELECT EXISTS (
            SELECT 1 
            FROM information_schema.tables
            WHERE table_schema = 'public'
            AND table_name = 'user_image'
        );").fetch_one(&pool).await.unwrap();

        assert!(table.0);

        let schema = query(r#"
            SELECT
                column_name,
                data_type
            FROM
                information_schema.columns
            WHERE
                table_name = 'user_image'
            ORDER BY column_name ASC;
        "#).fetch_all(&pool).await.unwrap();

        assert_eq!(schema.len(), 4);

        let column_name: String = schema[0].get("column_name");
        let data_type: String = schema[0].get("data_type");

        assert_eq!(column_name, "id".to_string());
        assert_eq!(data_type, "uuid".to_string());

        let column_name: String = schema[1].get("column_name");
        let data_type: String = schema[1].get("data_type");

        assert_eq!(column_name, "image_path".to_string());
        assert_eq!(data_type, "text".to_string());

        let column_name: String = schema[2].get("column_name");
        let data_type: String = schema[2].get("data_type");

        assert_eq!(column_name, "original_name".to_string());
        assert_eq!(data_type, "character varying".to_string());

        let column_name: String = schema[3].get("column_name");
        let data_type: String = schema[3].get("data_type");

        assert_eq!(column_name, "user_id".to_string());
        assert_eq!(data_type, "uuid".to_string());
    }

    #[tokio::test]
    async fn test_check_user_image_table() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("Variável de ambiente DATABASE_URL não definida");

        check_user_image_table_schema(database_url).await;
    }

    #[tokio::test]
    async fn test_check_user_image_test_table() {
        dotenv().ok();
        let database_url = env::var("TESTE_DATABASE_URL")
            .expect("Variável de ambiente TESTE_DATABASE_URL não definida");

        check_user_image_table_schema(database_url).await;
    }
}