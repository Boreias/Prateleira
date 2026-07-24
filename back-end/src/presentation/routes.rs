use axum::{
    Router,
    http::{Method, HeaderValue}
};
use tower_http::{
    services::ServeDir,
    cors::{Any, CorsLayer}
};


use crate::infrastructure::app_state::AppState;

use crate::presentation::controllers::user_controller::user_routes;
use crate::presentation::controllers::author_controller::author_routes;
use crate::presentation::controllers::gender_controller::gender_routes;
use crate::presentation::controllers::publisher_controller::publisher_routes;
use crate::presentation::controllers::book_controller::book_routes;


pub fn create_app(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_headers(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]);

    Router::new()
        .nest("/user", user_routes(state.clone()))
        .nest("/author", author_routes())
        .nest_service("/uploads", ServeDir::new("infrastructure/uploads"))
        .nest("/gender", gender_routes())
        .nest("/publisher", publisher_routes())
        .nest("/book", book_routes())
        .layer(cors)
        .with_state(state)
}