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

use crate::domain::entities::author::Author;
use crate::application::services::author_service::AuthorService;
use crate::infrastructure::app_state::AppState;


pub fn author_routes() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_author))
        .route("/id", get(get_author_by_id))
        .route("/name", get(get_authors_by_name))
        .route("/book", get(get_authors_by_book))
        .route("/gender", get(get_authors_by_gender))
        .route("/publisher", get(get_authors_by_publisher))
        .route("/more_popular", get(more_popular_author))
        .route("/best_valuated", get(best_valuated_author))
        .route("/alter", put(alter_author))
        .route("/delete", delete(delete_author))
        .route("/clear_deleted", get(clear_deleted_authors))
}


async fn create_author(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<(StatusCode, String), (StatusCode, String)> {

    let service = AuthorService::new((*state.db_pool).clone());

    let mut name: Option<String> = None;
    let mut user_id: Option<Uuid> = None;
    let mut file_name: Option<String> = None;
    let mut file_content: Option<Bytes> = None;
    let mut books: Option<Vec<Uuid>> = None;

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
            "avatar" => {
                file_name = Some(field.file_name().unwrap().to_string());
                file_content = Some(field.bytes().await.unwrap());
            }
            "books" => {
                let value = field.text().await.unwrap();
                let parsed: Vec<String> = serde_json::from_str(&value).expect("Erro na obtenção dos ids dos livros");
                books = Some(parsed.iter().map(|f| Uuid::parse_str(f).expect("Erro na conversão de string para Uuid")).collect());
            }
            _ => {}
        }
    }

    match service.create_author(name.unwrap(), user_id.unwrap(), file_name, file_content, books).await {
        Ok(_) => return Ok((StatusCode::CREATED, "Autor criado com sucesso".to_string())),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}


#[derive(Deserialize)]
struct GetAuthorByIdRequest {
    id: Uuid
}

async fn get_author_by_id (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetAuthorByIdRequest>
) -> Result<(StatusCode, Json<Author>), (StatusCode, String)> {
    let service = AuthorService::new((*state.db_pool).clone());

    match service.get_author_by_id(payload.id).await {
        Ok(author) => return Ok((StatusCode::OK, Json(author))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}


#[derive(Deserialize)]
struct GetAuthorsByNameRequest {
    name: String,
    skip: Option<i32>,
    page_size: Option<i32>
}

async fn get_authors_by_name (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetAuthorsByNameRequest>
) -> Result<(StatusCode, Json<Vec<Author>>), (StatusCode, String)> {
    let service = AuthorService::new((*state.db_pool).clone());

    match service.get_author_by_name(payload.name, payload.skip, payload.page_size).await {
        Ok(authors) => return Ok((StatusCode::OK, Json(authors))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}


#[derive(Deserialize)]
struct GetAuthorsByBookRequest {
    book_id: Uuid,
    skip: Option<i32>,
    page_size: Option<i32>
}

async fn get_authors_by_book (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetAuthorsByBookRequest>
) -> Result<(StatusCode, Json<Vec<Author>>), (StatusCode, String)> {
    let service = AuthorService::new((*state.db_pool).clone());

    match service.get_authors_by_book(payload.book_id, payload.skip, payload.page_size).await {
        Ok(authors) => return Ok((StatusCode::OK, Json(authors))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}


#[derive(Deserialize)]
struct GetAuthorsByGenderRequest {
    gender_id: Uuid,
    skip: Option<i32>,
    page_size: Option<i32>
}

async fn get_authors_by_gender (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetAuthorsByGenderRequest>
) -> Result<(StatusCode, Json<Vec<Author>>), (StatusCode, String)> {
    let service = AuthorService::new((*state.db_pool).clone());

    match service.get_authors_by_gender(payload.gender_id, payload.skip, payload.page_size).await {
        Ok(authors) => return Ok((StatusCode::OK, Json(authors))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}


#[derive(Deserialize)]
struct GetAuthorsByPublisherRequest {
    publisher_id: Uuid,
    skip: Option<i32>,
    page_size: Option<i32>
}

async fn get_authors_by_publisher (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetAuthorsByPublisherRequest>
) -> Result<(StatusCode, Json<Vec<Author>>), (StatusCode, String)> {
    let service = AuthorService::new((*state.db_pool).clone());

    match service.get_authors_by_publisher(payload.publisher_id, payload.skip, payload.page_size).await {
        Ok(authors) => return Ok((StatusCode::OK, Json(authors))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}


#[derive(Deserialize)]
struct GetPaginetedRequest {
    skip: Option<i32>,
    page_size: Option<i32>
}

async fn more_popular_author (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetPaginetedRequest>
) -> Result<(StatusCode, Json<Vec<Author>>), (StatusCode, String)> {
    let service = AuthorService::new((*state.db_pool).clone());

    match service.more_popular_author(payload.skip, payload.page_size).await {
        Ok(authors) => return Ok((StatusCode::OK, Json(authors))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}

async fn best_valuated_author (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetPaginetedRequest>
) -> Result<(StatusCode, Json<Vec<Author>>), (StatusCode, String)> {
    let service = AuthorService::new((*state.db_pool).clone());

    match service.best_valuated_author(payload.skip, payload.page_size).await {
        Ok(authors) => return Ok((StatusCode::OK, Json(authors))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}


async fn alter_author (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let mut service = AuthorService::new((*state.db_pool).clone());

    let mut id: Option<Uuid> = None;
    let mut name: Option<String> = None;
    let mut user_id: Option<Uuid> = None;
    let mut file_name: Option<String> = None;
    let mut file_content: Option<Bytes> = None;
    let mut books: Option<Vec<Uuid>> = None;

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
            "avatar" => {
                file_name = Some(field.file_name().unwrap().to_string());
                file_content = Some(field.bytes().await.unwrap());
            }
            "books" => {
                let value = field.text().await.unwrap();
                let parsed: Vec<String> = serde_json::from_str(&value).expect("Erro na obtenção dos ids dos livros");
                books = Some(parsed.iter().map(|f| Uuid::parse_str(f).expect("Erro na conversão de string para Uuid")).collect());
            }
            _ => {}
        }
    }

    match service.alter_author(id.unwrap(), name.unwrap(), user_id.unwrap(), file_name, file_content, books).await {
        Ok(_) => return Ok((StatusCode::OK, "Autor alterado com sucesso".to_string())),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}


#[derive(Deserialize)]
struct DeleteAuthorRequest {
    id: Uuid,
    user_id: Uuid
}

async fn delete_author (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<DeleteAuthorRequest>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let service = AuthorService::new((*state.db_pool).clone());

    match service.delete_author(payload.id, payload.user_id).await {
        Ok(_) => Ok((StatusCode::OK, "Autor removido com sucesso".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}


async fn clear_deleted_authors(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let service = AuthorService::new((*state.db_pool).clone());

    match service.clear_deleted_authors().await {
        Ok(_) => Ok((StatusCode::OK, "Autores excluídos removidos do banco de dados".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}