use axum::Router;

use crate::infrastructure::app_state::AppState;

// use crate::presentation::controllers::user_controller::user_routes;
// use crate::presentation::controllers::author_controller::author_routes;
use crate::presentation::controllers::gender_controller::gender_routes;

pub fn create_app(state: AppState) -> Router {
    Router::new()
        // .nest("/user", user_routes())
        // .nest("/author", author_routes())
        .nest("/gender", gender_routes())
        .with_state(state)
}