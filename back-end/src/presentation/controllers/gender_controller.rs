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

use crate::domain::entities::gender::Gender;
use crate::application::services::gender_service::GenderService;
use crate::infrastructure::app_state::AppState;



pub fn gender_routes() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_gender))
        .route("/id", get(get_gender_by_id))
        .route("/name", get(get_gender_by_name))
        .route("/book", get(get_genders_by_book))
        .route("/author", get(get_genders_by_author))
        .route("/publisher", get(get_genders_by_publisher))
        .route("/more_popular", get(more_popular_gender))
        .route("/best_valuated", get(best_valuated_gender))
        .route("/alter", put(alter_gender))
        .route("/delete", delete(delete_gender))
        .route("/clear_deleted", get(clear_deleted_genders))
}


#[derive(Deserialize)]
struct CreateGenderResquest {
    name: String,
    user_id: Uuid,
    books_ids: Option<Vec<Uuid>>
}

async fn create_gender(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Json(payload): Json<CreateGenderResquest>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let service = GenderService::new((*state.db_pool).clone());

    match service.create_gender(payload.name, payload.user_id, payload.books_ids).await {
        Ok(_) => return Ok((StatusCode::CREATED, "Gênero criado com sucesso".to_string())),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}


#[derive(Deserialize)]
struct GetGenderByIdRequest {
    id: Uuid
}

async fn get_gender_by_id(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetGenderByIdRequest>
) -> Result<(StatusCode, Json<Gender>), (StatusCode, String)> {
    let service = GenderService::new((*state.db_pool).clone());

    match service.get_gender_by_id(payload.id).await {
        Ok(gender) => return Ok((StatusCode::OK, Json(gender))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}


#[derive(Deserialize)]
struct GetGenderByNameRequest {
    name: String,
    skip: Option<i32>,
    page_size: Option<i32>
}

async fn get_gender_by_name (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetGenderByNameRequest>
) -> Result<(StatusCode, Json<Vec<Gender>>), (StatusCode, String)> {
    let service = GenderService::new((*state.db_pool).clone());

    match service.get_gender_by_name(payload.name, payload.skip, payload.page_size).await {
        Ok(genders) => return Ok((StatusCode::OK, Json(genders))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}

#[derive(Deserialize)]
struct GetGenderByBookRequest {
    book_id: Uuid,
    skip: Option<i32>,
    page_size: Option<i32>
}

async fn get_genders_by_book (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetGenderByBookRequest>
) -> Result<(StatusCode, Json<Vec<Gender>>), (StatusCode, String)> {
    let service = GenderService::new((*state.db_pool).clone());

    match service.get_genders_by_book(payload.book_id, payload.skip, payload.page_size).await {
        Ok(genders) => return Ok((StatusCode::OK, Json(genders))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}


#[derive(Deserialize)]
struct GetGenderByAuthorRequest {
    author_id: Uuid,
    skip: Option<i32>,
    page_size: Option<i32>
}

async fn get_genders_by_author (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetGenderByAuthorRequest>
) -> Result<(StatusCode, Json<Vec<Gender>>), (StatusCode, String)> {
    let service = GenderService::new((*state.db_pool).clone());

    match service.get_genders_by_author(payload.author_id, payload.skip, payload.page_size).await {
        Ok(genders) => return Ok((StatusCode::OK, Json(genders))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}


#[derive(Deserialize)]
struct GetGenderByPublisherRequest {
    publisher_id: Uuid,
    skip: Option<i32>,
    page_size: Option<i32>
}

async fn get_genders_by_publisher (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetGenderByPublisherRequest>
) -> Result<(StatusCode, Json<Vec<Gender>>), (StatusCode, String)> {
    let service = GenderService::new((*state.db_pool).clone());

    match service.get_genders_by_publisher(payload.publisher_id, payload.skip, payload.page_size).await {
        Ok(genders) => return Ok((StatusCode::OK, Json(genders))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}


#[derive(Deserialize)]
struct GetPaginetedRequest {
    skip: Option<i32>,
    page_size: Option<i32>
}

async fn more_popular_gender (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetPaginetedRequest>
) -> Result<(StatusCode, Json<Vec<Gender>>), (StatusCode, String)> {
    let service = GenderService::new((*state.db_pool).clone());

    match service.more_popular_gender(payload.skip, payload.page_size).await {
        Ok(genders) => return Ok((StatusCode::OK, Json(genders))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}

async fn best_valuated_gender (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetPaginetedRequest>
) -> Result<(StatusCode, Json<Vec<Gender>>), (StatusCode, String)> {
    let service = GenderService::new((*state.db_pool).clone());

    match service.best_valuated_gender(payload.skip, payload.page_size).await {
        Ok(genders) => return Ok((StatusCode::OK, Json(genders))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}


#[derive(Deserialize)]
struct AlterGenderRequest {
    id: Uuid,
    name: String,
    user_id: Uuid,
    books_ids: Option<Vec<Uuid>>
}

async fn alter_gender (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Json(payload): Json<AlterGenderRequest>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let mut service = GenderService::new((*state.db_pool).clone());

    match service.alter_gender(payload.id, payload.name, payload.user_id, payload.books_ids).await {
        Ok(_) => return Ok((StatusCode::OK, "Gênero atualizado com sucesso".to_string())),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}


#[derive(Deserialize)]
struct DeleteGenderRequest {
    id: Uuid,
    user_id: Uuid
}

async fn delete_gender (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<DeleteGenderRequest>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let service = GenderService::new((*state.db_pool).clone());

    match service.delete_gender(payload.id, payload.user_id).await {
        Ok(_) => return Ok((StatusCode::OK, "Gênero excluído com sucesso".to_string())),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}


async fn clear_deleted_genders (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let service = GenderService::new((*state.db_pool).clone());

    match service.clear_deleted_genders().await {
        Ok(_) => return Ok((StatusCode::OK, "Gêneros excluídos foram removidos do banco com sucesso".to_string())),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}