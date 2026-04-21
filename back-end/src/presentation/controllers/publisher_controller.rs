use axum::{
    body::Bytes,
    extract::{State, ConnectInfo, Query, Multipart},
    http::StatusCode,
    routing::{get, post, put, delete},
    Router,
    Json
};
use uuid::Uuid;
use serde::Deserialize;
use std::net::SocketAddr;

use crate::domain::entities::publisher::Publisher;
use crate::application::services::publisher_service::PublisherService;
use crate::infrastructure::app_state::AppState;


pub fn publisher_routes() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_publisher))
        .route("/id", get(get_publisher_by_id))
        .route("/name", get(get_publishers_by_name))
        .route("/book", get(get_publisher_by_book))
        .route("/gender", get(get_publishers_by_gender))
        .route("/author", get(get_publishers_by_author))
        .route("/more_popular", get(more_popular_publishers))
        .route("/best_valuated", get(best_valuated_publishers))
        .route("/alter", put(alter_publisher))
        .route("/delete", delete(delete_publisher))
        .route("/clear_deleted", get(clear_deleted_publishers))
}


async fn create_publisher(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    mut multipart: Multipart
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let service = PublisherService::new((*state.db_pool).clone());

    let mut name: Option<String> = None;
    let mut user_id: Option<Uuid> = None;
    let mut site: Option<String> = None;
    let mut email: Option<String> = None;
    let mut file_name: Option<String> = None;
    let mut file_content: Option<Bytes> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string();

        match field_name.as_str() {
            "name" => {
                name = Some(field.text().await.unwrap());
            }
            "user_id" => {
                let value = field.text().await.unwrap();
                user_id = Some(Uuid::parse_str(&value).unwrap());
            }
            "site" => {
                site = Some(field.text().await.unwrap());
            }
            "email" => {
                email = Some(field.text().await.unwrap());
            }
            "avatar" => {
                file_name = Some(field.file_name().unwrap().to_string());
                file_content = Some(field.bytes().await.unwrap());
            }
            _ => {}
        }
        
    }

    match service.create_publisher(name.unwrap(), user_id.unwrap(), site, email, file_name, file_content).await {
        Ok(_) => return Ok((StatusCode::CREATED, "Editora criada com sucesso".to_string())),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}


#[derive(Deserialize)]
struct GetPublisherByIdRequest {
    id: Uuid
}

async fn get_publisher_by_id (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetPublisherByIdRequest>
) -> Result<(StatusCode, Json<Publisher>), (StatusCode, String)> {
    let service = PublisherService::new((*state.db_pool).clone());

    match service.get_publisher_by_id(payload.id).await {
        Ok(publisher) => return Ok((StatusCode::OK, Json(publisher))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}


#[derive(Deserialize)]
struct GetPublishersByNameRequest {
    name: String,
    skip: Option<i32>,
    page_size: Option<i32>
}

async fn get_publishers_by_name (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetPublishersByNameRequest>
) -> Result<(StatusCode, Json<Vec<Publisher>>), (StatusCode, String)> {
    let service = PublisherService::new((*state.db_pool).clone());

    match service.get_publisher_by_name(payload.name, payload.skip, payload.page_size).await {
        Ok(publishers) => return Ok((StatusCode::OK, Json(publishers))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}


#[derive(Deserialize)]
struct GetPublishersByBookRequest {
    book_id: Uuid,
    skip: Option<i32>,
    page_size: Option<i32>
}

async fn get_publisher_by_book (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetPublishersByBookRequest>
) -> Result<(StatusCode, Json<Publisher>), (StatusCode, String)> {
    let service = PublisherService::new((*state.db_pool).clone());

    match service.get_publisher_by_book(payload.book_id, payload.skip, payload.page_size).await {
        Ok(publisher) => return Ok((StatusCode::OK, Json(publisher))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}


#[derive(Deserialize)]
struct GetPublishersByGenderRequest {
    gender_id: Uuid,
    skip: Option<i32>,
    page_size: Option<i32>
}

async fn get_publishers_by_gender (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetPublishersByGenderRequest>
) -> Result<(StatusCode, Json<Vec<Publisher>>), (StatusCode, String)> {
    let service = PublisherService::new((*state.db_pool).clone());

    match service.get_publishers_by_gender(payload.gender_id, payload.skip, payload.page_size).await {
        Ok(publishers) => return Ok((StatusCode::OK, Json(publishers))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}


#[derive(Deserialize)]
struct GetPublishersByAuthorRequest {
    author_id: Uuid,
    skip: Option<i32>,
    page_size: Option<i32>
}

async fn get_publishers_by_author (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetPublishersByAuthorRequest>
) -> Result<(StatusCode, Json<Vec<Publisher>>), (StatusCode, String)> {
    let service = PublisherService::new((*state.db_pool).clone());

    match service.get_publishers_by_author(payload.author_id, payload.skip, payload.page_size).await {
        Ok(publishers) => return Ok((StatusCode::OK, Json(publishers))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}


#[derive(Deserialize)]
struct GetPaginetedRequest {
    skip: Option<i32>,
    page_size: Option<i32>
}

async fn more_popular_publishers (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetPaginetedRequest>
) -> Result<(StatusCode, Json<Vec<Publisher>>), (StatusCode, String)> {
    let service = PublisherService::new((*state.db_pool).clone());

    match service.more_popular_publishers(payload.skip, payload.page_size).await {
        Ok(publishers) => return Ok((StatusCode::OK, Json(publishers))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}

async fn best_valuated_publishers (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetPaginetedRequest>
) -> Result<(StatusCode, Json<Vec<Publisher>>), (StatusCode, String)> {
    let service = PublisherService::new((*state.db_pool).clone());

    match service.best_valuated_publishers(payload.skip, payload.page_size).await {
        Ok(publishers) => return Ok((StatusCode::OK, Json(publishers))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}


async fn alter_publisher(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    mut multipart: Multipart
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let service = PublisherService::new((*state.db_pool).clone());

    let mut id: Option<Uuid> = None;
    let mut name: Option<String> = None;
    let mut user_id: Option<Uuid> = None;
    let mut site: Option<String> = None;
    let mut email: Option<String> = None;
    let mut file_name: Option<String> = None;
    let mut file_content: Option<Bytes> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string();

        match field_name.as_str() {
            "id" => {
                let value = field.text().await.unwrap();
                id = Some(Uuid::parse_str(&value).unwrap());
            }
            "name" => {
                name = Some(field.text().await.unwrap());
            }
            "user_id" => {
                let value = field.text().await.unwrap();
                user_id = Some(Uuid::parse_str(&value).unwrap());
            }
            "site" => {
                site = Some(field.text().await.unwrap());
            }
            "email" => {
                email = Some(field.text().await.unwrap());
            }
            "avatar" => {
                file_name = Some(field.file_name().unwrap().to_string());
                file_content = Some(field.bytes().await.unwrap());
            }
            _ => {}
        }
        
    }

    match service.alter_publisher(id.unwrap(), name.unwrap(), user_id.unwrap(), site, email, file_name, file_content).await {
        Ok(_) => return Ok((StatusCode::OK, "Editora alterada com sucesso".to_string())),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}


#[derive(Deserialize)]
struct DeletePublisherRequest {
    id: Uuid,
    user_id: Uuid
}

async fn delete_publisher (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<DeletePublisherRequest>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let service = PublisherService::new((*state.db_pool).clone());

    match service.delete_publisher(payload.id, payload.user_id).await {
        Ok(_) => Ok((StatusCode::OK, "Editora removida com sucesso".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}


async fn clear_deleted_publishers (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let service = PublisherService::new((*state.db_pool).clone());

    match service.clear_deleted_publishers().await {
        Ok(_) => Ok((StatusCode::OK, "Editoras excluídas removidas do banco de dados".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}