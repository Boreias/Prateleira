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

use crate::domain::entities::book::Book;
use crate::application::services::book_service::BookService;
use crate::infrastructure::app_state::AppState;


pub fn book_routes() -> Router<AppState> {
    Router::new()
        .route("/create", post(create_book))
        .route("/id", get(get_book_by_id))
        .route("/isbn", get(get_book_by_isbn))
        .route("/name", get(get_books_by_name))
        .route("/author", get(get_books_by_author))
        .route("/best_author_books", get(best_valuated_books_by_author))
        .route("/publisher", get(get_books_by_publisher))
        .route("/best_publisher_books", get(best_valuated_books_by_publisher))
        .route("/gender", get(get_books_by_gender))
        .route("/best_gender_books", get(best_valuated_books_by_gender))
        .route("/smallets_books", get(smallets_books))
        .route("/biggest_books", get(biggest_books))
        .route("/more_popular", get(more_popular_books))
        .route("/best_valuated", get(best_valuated_books))
        .route("/alter", put(alter_book))
        .route("/delete", delete(delete_book))
        .route("/clear_deleted", get(clear_deleted_books))
}


async fn create_book(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    mut multipart: Multipart
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let service = BookService::new((*state.db_pool).clone());

    let mut title: Option<String> = None;
    let mut subtitle: Option<String> = None;
    let mut authors_id: Option<Vec<Uuid>> = None;
    let mut publisher_id: Option<Uuid> = None;
    let mut series_collection: Option<i32> = None;
    let mut volume: Option<i32> = None;
    let mut edition: Option<i32> = None;
    let mut publication_year: Option<i32> = None;
    let mut pages: Option<i32> = None;
    let mut language: Option<String> = None;
    let mut isbn: Option<String> = None;
    let mut genders_id: Option<Vec<Uuid>> = None;
    let mut synopsis: Option<String> = None;
    let mut file_name: Option<String> = None;
    let mut file_content: Option<Bytes> = None;
    let mut user_id: Option<Uuid> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string();

        match field_name.as_str() {
            "title" => {
                title = Some(field.text().await.unwrap());
            }
            "subtitle" => {
                subtitle = Some(field.text().await.unwrap());
            }
            "authors_id" => {
                let value = field.text().await.unwrap();
                authors_id = Some(value
                    .split(',')
                    .map(|s| s.trim())
                    .filter_map(|s| Uuid::parse_str(s).ok())
                    .collect());
            }
            "publisher_id" => {
                let value = field.text().await.unwrap();
                publisher_id = Some(Uuid::parse_str(&value).unwrap());
            }
            "series_collection" => {
                let value = field.text().await.unwrap();
                series_collection = Some(value.parse::<i32>().unwrap());
            }
            "volume" => {
                let value = field.text().await.unwrap();
                volume = Some(value.parse::<i32>().unwrap());
            }
            "edition" => {
                let value = field.text().await.unwrap();
                edition = Some(value.parse::<i32>().unwrap());
            }
            "publication_year" => {
                let value = field.text().await.unwrap();
                publication_year = Some(value.parse::<i32>().unwrap());
            }
            "pages" => {
                let value = field.text().await.unwrap();
                pages = Some(value.parse::<i32>().unwrap());
            }
            "language" => {
                language = Some(field.text().await.unwrap());
            }
            "isbn" => {
                isbn = Some(field.text().await.unwrap());
            }
            "genders_id" => {
                let value = field.text().await.unwrap();
                genders_id = Some(value
                    .split(',')
                    .map(|s| s.trim())
                    .filter_map(|s| Uuid::parse_str(s).ok())
                    .collect());
            }
            "synopsis" => {
                synopsis = Some(field.text().await.unwrap());
            }
            "avatar" => {
                file_name = Some(field.file_name().unwrap().to_string());
                file_content = Some(field.bytes().await.unwrap());
            }
            "user_id" => {
                let value = field.text().await.unwrap();
                user_id = Some(Uuid::parse_str(&value).unwrap());
            }
            _ => {}
        }
    }

    match service.create_book(title.unwrap(), subtitle, authors_id.unwrap(), publisher_id.unwrap(), series_collection, volume, edition, publication_year, pages, language, isbn.unwrap(), genders_id.unwrap(), synopsis, file_name, file_content, user_id.unwrap()).await {
        Ok(_) => return Ok((StatusCode::CREATED, "Livro cadastrado com sucesso".to_string())),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}


#[derive(Deserialize)]
struct GetBookByIdRequest {
    id: Uuid
}

async fn get_book_by_id (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetBookByIdRequest>
) -> Result<(StatusCode, Json<Book>), (StatusCode, String)> {
    let service = BookService::new((*state.db_pool).clone());

    match service.get_book_by_id(payload.id).await {
        Ok(book) => return Ok((StatusCode::OK, Json(book))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}


#[derive(Deserialize)]
struct GetBookByISBNRequest {
    isbn: String
}

async fn get_book_by_isbn (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetBookByISBNRequest>
) -> Result<(StatusCode, Json<Book>), (StatusCode, String)> {
    let service = BookService::new((*state.db_pool).clone());

    match service.get_book_by_isbn(payload.isbn).await {
        Ok(book) => return Ok((StatusCode::OK, Json(book))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}


#[derive(Deserialize)]
struct GetBooksByNameRequest {
    name: String,
    skip: Option<i32>,
    page_size: Option<i32>
}

async fn get_books_by_name (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetBooksByNameRequest>
) -> Result<(StatusCode, Json<Vec<Book>>), (StatusCode, String)> {
    let service = BookService::new((*state.db_pool).clone());

    match service.get_books_by_name(payload.name, payload.skip, payload.page_size).await {
        Ok(books) => return Ok((StatusCode::OK, Json(books))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}


#[derive(Deserialize)]
struct GetBooksByPublisherRequest {
    publisher_id: Uuid,
    skip: Option<i32>,
    page_size: Option<i32>
}

async fn get_books_by_publisher (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetBooksByPublisherRequest>
) -> Result<(StatusCode, Json<Vec<Book>>), (StatusCode, String)> {
    let service = BookService::new((*state.db_pool).clone());

    match service.get_books_by_publisher(payload.publisher_id, payload.skip, payload.page_size).await {
        Ok(books) => return Ok((StatusCode::OK, Json(books))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}

async fn best_valuated_books_by_publisher (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetBooksByPublisherRequest>
) -> Result<(StatusCode, Json<Vec<Book>>), (StatusCode, String)> {
    let service = BookService::new((*state.db_pool).clone());

    match service.best_valuated_books_by_publisher(payload.publisher_id, payload.skip, payload.page_size).await {
        Ok(books) => return Ok((StatusCode::OK, Json(books))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}


#[derive(Deserialize)]
struct GetBooksByAuthorRequest {
    author_id: Uuid,
    skip: Option<i32>,
    page_size: Option<i32>
}

async fn get_books_by_author (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetBooksByAuthorRequest>
) -> Result<(StatusCode, Json<Vec<Book>>), (StatusCode, String)> {
    let service = BookService::new((*state.db_pool).clone());

    match service.get_books_by_author(payload.author_id, payload.skip, payload.page_size).await {
        Ok(books) => return Ok((StatusCode::OK, Json(books))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}

async fn best_valuated_books_by_author (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetBooksByAuthorRequest>
) -> Result<(StatusCode, Json<Vec<Book>>), (StatusCode, String)> {
    let service = BookService::new((*state.db_pool).clone());

    match service.best_valuated_books_by_author(payload.author_id, payload.skip, payload.page_size).await {
        Ok(books) => return Ok((StatusCode::OK, Json(books))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}


#[derive(Deserialize)]
struct GetBooksByGenderRequest {
    gender_id: Uuid,
    skip: Option<i32>,
    page_size: Option<i32>
}

async fn get_books_by_gender (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetBooksByGenderRequest>
) -> Result<(StatusCode, Json<Vec<Book>>), (StatusCode, String)> {
    let service = BookService::new((*state.db_pool).clone());

    match service.get_books_by_gender(payload.gender_id, payload.skip, payload.page_size).await {
        Ok(books) => return Ok((StatusCode::OK, Json(books))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}

async fn best_valuated_books_by_gender (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetBooksByGenderRequest>
) -> Result<(StatusCode, Json<Vec<Book>>), (StatusCode, String)> {
    let service = BookService::new((*state.db_pool).clone());

    match service.best_valuated_books_by_gender(payload.gender_id, payload.skip, payload.page_size).await {
        Ok(books) => return Ok((StatusCode::OK, Json(books))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}


#[derive(Deserialize)]
struct GetPaginetedRequest {
    skip: Option<i32>,
    page_size: Option<i32>
}

async fn more_popular_books (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetPaginetedRequest>
) -> Result<(StatusCode, Json<Vec<Book>>), (StatusCode, String)> {
    let service = BookService::new((*state.db_pool).clone());

    match service.more_popular_books(payload.skip, payload.page_size).await {
        Ok(books) => return Ok((StatusCode::OK, Json(books))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}

async fn best_valuated_books (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetPaginetedRequest>
) -> Result<(StatusCode, Json<Vec<Book>>), (StatusCode, String)> {
    let service = BookService::new((*state.db_pool).clone());

    match service.best_valuated_books(payload.skip, payload.page_size).await {
        Ok(books) => return Ok((StatusCode::OK, Json(books))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}

async fn smallets_books (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetPaginetedRequest>
) -> Result<(StatusCode, Json<Vec<Book>>), (StatusCode, String)> {
    let service = BookService::new((*state.db_pool).clone());

    match service.smallets_books(payload.skip, payload.page_size).await {
        Ok(books) => return Ok((StatusCode::OK, Json(books))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}

async fn biggest_books (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetPaginetedRequest>
) -> Result<(StatusCode, Json<Vec<Book>>), (StatusCode, String)> {
    let service = BookService::new((*state.db_pool).clone());

    match service.biggest_books(payload.skip, payload.page_size).await {
        Ok(books) => return Ok((StatusCode::OK, Json(books))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}


async fn alter_book(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    mut multipart: Multipart
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let service = BookService::new((*state.db_pool).clone());

    let mut id: Option<Uuid> = None;
    let mut title: Option<String> = None;
    let mut subtitle: Option<String> = None;
    let mut authors_id: Option<Vec<Uuid>> = None;
    let mut publisher_id: Option<Uuid> = None;
    let mut series_collection: Option<i32> = None;
    let mut volume: Option<i32> = None;
    let mut edition: Option<i32> = None;
    let mut publication_year: Option<i32> = None;
    let mut pages: Option<i32> = None;
    let mut language: Option<String> = None;
    let mut isbn: Option<String> = None;
    let mut genders_id: Option<Vec<Uuid>> = None;
    let mut synopsis: Option<String> = None;
    let mut file_name: Option<String> = None;
    let mut file_content: Option<Bytes> = None;
    let mut user_id: Option<Uuid> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string();

        match field_name.as_str() {
            "id" => {
                let value = field.text().await.unwrap();
                id = Some(Uuid::parse_str(&value).unwrap());
            }
            "title" => {
                title = Some(field.text().await.unwrap());
            }
            "subtitle" => {
                subtitle = Some(field.text().await.unwrap());
            }
            "authors_id" => {
                let value = field.text().await.unwrap();
                authors_id = Some(value
                    .split(',')
                    .map(|s| s.trim())
                    .filter_map(|s| Uuid::parse_str(s).ok())
                    .collect());
            }
            "publisher_id" => {
                let value = field.text().await.unwrap();
                publisher_id = Some(Uuid::parse_str(&value).unwrap());
            }
            "series_collection" => {
                let value = field.text().await.unwrap();
                series_collection = Some(value.parse::<i32>().unwrap());
            }
            "volume" => {
                let value = field.text().await.unwrap();
                volume = Some(value.parse::<i32>().unwrap());
            }
            "edition" => {
                let value = field.text().await.unwrap();
                edition = Some(value.parse::<i32>().unwrap());
            }
            "publication_year" => {
                let value = field.text().await.unwrap();
                publication_year = Some(value.parse::<i32>().unwrap());
            }
            "pages" => {
                let value = field.text().await.unwrap();
                pages = Some(value.parse::<i32>().unwrap());
            }
            "language" => {
                language = Some(field.text().await.unwrap());
            }
            "isbn" => {
                isbn = Some(field.text().await.unwrap());
            }
            "genders_id" => {
                let value = field.text().await.unwrap();
                genders_id = Some(value
                    .split(',')
                    .map(|s| s.trim())
                    .filter_map(|s| Uuid::parse_str(s).ok())
                    .collect());
            }
            "synopsis" => {
                synopsis = Some(field.text().await.unwrap());
            }
            "avatar" => {
                file_name = Some(field.file_name().unwrap().to_string());
                file_content = Some(field.bytes().await.unwrap());
            }
            "user_id" => {
                let value = field.text().await.unwrap();
                user_id = Some(Uuid::parse_str(&value).unwrap());
            }
            _ => {}
        }
    }

    match service.alter_book(id.unwrap(), title.unwrap(), subtitle, authors_id.unwrap(), publisher_id.unwrap(), series_collection, volume, edition, publication_year, pages, language, isbn.unwrap(), genders_id.unwrap(), synopsis, file_name, file_content, user_id.unwrap()).await {
        Ok(_) => return Ok((StatusCode::OK, "Livro editado com sucesso".to_string())),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}


#[derive(Deserialize)]
struct DeleteBookRequest {
    id: Uuid,
    user_id: Uuid
}

async fn delete_book (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<DeleteBookRequest>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let service = BookService::new((*state.db_pool).clone());

    match service.delete_book(payload.id, payload.user_id).await {
        Ok(_) => Ok((StatusCode::OK, "Livro removido com sucesso".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}


async fn clear_deleted_books (
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let service = BookService::new((*state.db_pool).clone());

    match service.clear_deleted_books().await {
        Ok(_) => Ok((StatusCode::OK, "Livros excluídos removidos do banco de dados".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e))
    }
}