use sqlx::{
    PgPool,
    Error,
    postgres::PgPoolOptions
};
use std::env;

pub async fn create_pool() -> Result<PgPool, Error> {
    let database_url = env::var("DATABASE_URL")
        .expect("Variável de ambiente DATABASE_URL não definida");

    PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
}