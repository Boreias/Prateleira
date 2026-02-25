use axum::{
    extract::{State, ConnectInfo, Query},
    http::StatusCode,
    routing::{get, post, put, delete},
    Router,
    Json
};
use uuid::Uuid;
use serde::Deserialize;
use std::net::SocketAddr;

use crate::domain::entities::author::Author;
use crate::application::services::author_service::AuthorService;
use crate::infrastructure::app_state::AppState;


pub fn author_routes() -> Router<AppState> {
    Router::new()
}