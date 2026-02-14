use std::sync::Arc;
use sqlx::PgPool;

use crate::application::services::user_service::UserService;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<PgPool>
}