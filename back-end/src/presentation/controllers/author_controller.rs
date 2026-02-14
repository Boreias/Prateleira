use axum::{
    extract::{Json, State, ConnectInfo},
    response::{IntoResponse, Json as JsonResponse},
    http::StatusCode,
    routing::{get, post, put, delete},
    Router
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use crate::application::services::author_service::AuthorService;
use crate::app_state::AppState;


pub fn author_routes() -> Router<AppState> {
    Router::new()
}