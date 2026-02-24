use std::sync::Arc;
use sqlx::PgPool;


#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<PgPool>
}