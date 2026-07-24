use uuid::Uuid;
use axum::{
    Json,
    Router,
    extract::{
        ConnectInfo,
        Multipart,
        Query,
        State
    },
    http::StatusCode,
    middleware::from_fn_with_state,
    response::IntoResponse,
    routing::{
        delete,
        get,
        post,
        put
    }
};
use serde::{
    Deserialize,
    Serialize
};
use std::net::SocketAddr;
use chrono::NaiveDate;
use axum::body::Bytes;
use tower_cookies::{
    CookieManagerLayer,
    Cookies
};

use crate::domain::entities::user_profile::UserProfile;
use crate::application::services::user_service::UserService;
use crate::infrastructure::location::location::get_location;
use crate::infrastructure::app_state::AppState;
use crate::presentation::middleware::auth::auth;

#[derive(Serialize)]
pub struct ApiResponse {
    pub message: String
}

pub fn user_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/create", post(create_user))
        .route("/validate_email", get(validate_email))
        .route("/id", get(get_user_by_id))
        .route("/email", get(get_user_by_email))
        .route("/name", get(get_user_by_name))
        .route("/auth", get(auth_user)).layer(CookieManagerLayer::new())
        .route("/refresh", get(refresh_user_token))
        .route("/alter", put(update_user)).route_layer(from_fn_with_state(state.clone(), auth))
        .route("/change_password", put(change_password))
        .route("/delete", delete(delete_user)).route_layer(from_fn_with_state(state.clone(), auth))
}


async fn create_user(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    mut multipart: Multipart
) -> impl IntoResponse {
    let service = UserService::new((*state.db_pool).clone());

    let ip = addr.ip().clone();

    let mut username: Option<String> = None;
    let mut email: Option<String> = None;
    let mut password: Option<String> = None;
    let mut name: Option<String> = None;
    let mut bio: Option<String> = None;
    let mut birth_date: Option<NaiveDate> = None;
    let mut file_name: Option<String> = None;
    let mut file_content: Option<Bytes> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string();

        match field_name.as_str() {
            "username" => {
                username = Some(field.text().await.unwrap());
            }
            "email" => {
                email = Some(field.text().await.unwrap());
            }
            "password" => {
                password = Some(field.text().await.unwrap());
            }
            "name" => {
                name = Some(field.text().await.unwrap());
            }
            "bio" => {
                bio = Some(field.text().await.unwrap());
            }
            "birth_date" => {
                let value = field.text().await.unwrap();
                birth_date = Some(NaiveDate::parse_from_str(&value, "%Y-%m-%d").unwrap());
            }
            "avatar" => {
                file_name = Some(field.file_name().unwrap().to_string());
                file_content = Some(field.bytes().await.unwrap());
            }
            _ => {}
        }
    }

    let country = get_location(ip).expect("Erro na obtenção da localização");

    match service.create_user(username.unwrap(), email.unwrap(), password.unwrap(), name.unwrap(), bio, birth_date.unwrap(), country, file_name, file_content).await {
        Ok(_) => return(StatusCode::CREATED, "Usuário registrado com sucesso".to_string()),
        Err(e) => {
            if e == String::from("Nome de usuário já em uso, favor digitar outro") || e == String::from("Email já cadastrado") {
                return(StatusCode::CONFLICT, e.to_string())
            } else {
                return(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}


#[derive(Deserialize)]
struct ValidateEmailRequest {
    email: String,
    token: String
}

async fn validate_email(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<ValidateEmailRequest>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let service = UserService::new((*state.db_pool).clone());

    match service.validate_email(payload.email, payload.token).await {
        Ok(_) => return Ok((StatusCode::OK, String::from("Email verificado com sucesso"))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}


#[derive(Deserialize)]
struct GetUserByIdRequest {
    id: Uuid
}

async fn get_user_by_id(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetUserByIdRequest>
) -> Result<(StatusCode, Json<UserProfile>), (StatusCode, String)> {
    let service = UserService::new((*state.db_pool).clone());

    match service.get_user_by_id(payload.id).await {
        Ok(user) => return Ok((StatusCode::OK, Json(user))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}


#[derive(Deserialize)]
struct GetUserByEmailRequest {
    email: String
}

async fn get_user_by_email(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetUserByEmailRequest>
) -> Result<(StatusCode, Json<UserProfile>), (StatusCode, String)> {
    let service = UserService::new((*state.db_pool).clone());

    match service.get_user_by_email(payload.email).await {
        Ok(user) => {
            if user.is_some() {
                return Ok((StatusCode::OK, Json(user.unwrap())))
            }
            return Err((StatusCode::INTERNAL_SERVER_ERROR, String::from("Email não cadastrado")))
        },
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}


#[derive(Deserialize)]
struct GetUserByNameRequest {
    name: String,
    skip: Option<i32>,
    page_size: Option<i32>
}

async fn get_user_by_name(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<GetUserByNameRequest>
) -> Result<(StatusCode, Json<Vec<UserProfile>>), (StatusCode, String)> {
    let service = UserService::new((*state.db_pool).clone());

    match service.get_user_by_name(payload.name, payload.skip, payload.page_size).await {
        Ok(users) => return Ok((StatusCode::OK, Json(users))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}


#[derive(Deserialize)]
struct AuthUserRequest {
    username_or_email: String,
    password: String
}

async fn auth_user(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<AuthUserRequest>,
    cookies: Cookies
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let service = UserService::new((*state.db_pool).clone());

    let ip = addr.ip().clone();

    let country = get_location(ip).expect("Erro na obtenção da localização");

    match service.auth_user(payload.username_or_email, payload.password, country, cookies).await {
        Ok(_) => return Ok((StatusCode::OK, String::from("Usuário autenticado com sucesso"))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}


#[derive(Deserialize)]
struct RefreshTokenRequest {
    id: Uuid,
    token: String
}

async fn refresh_user_token(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<RefreshTokenRequest>,
    cookies: Cookies
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let service = UserService::new((*state.db_pool).clone());

    match service.refresh_token(payload.id, payload.token, cookies).await {
        Ok(_) => return Ok((StatusCode::OK, String::from("Novo token gerado com sucesso"))),
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}


pub async fn update_user(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    mut multipart: Multipart
) -> impl IntoResponse {
    let mut service = UserService::new((*state.db_pool).clone());

    let mut user_id: Option<Uuid> = None;
    let mut name: Option<String> = None;
    let mut bio: Option<String> = None;
    let mut birth_date: Option<NaiveDate> = None;
    let mut file_name: Option<String> = None;
    let mut file_content: Option<Bytes> = None;

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string();

        match field_name.as_str() {
            "id" => {
                let value = field.text().await.unwrap();
                user_id = Some(Uuid::parse_str(&value).unwrap());
            }
            "name" => {
                name = Some(field.text().await.unwrap());
            }
            "bio" => {
                bio = Some(field.text().await.unwrap());
            }
            "birth_date" => {
                let value = field.text().await.unwrap();
                birth_date = Some(NaiveDate::parse_from_str(&value, "%Y-%m-%d").unwrap());
            }
            "avatar" => {
                file_name = Some(field.file_name().unwrap().to_string());
                file_content = Some(field.bytes().await.unwrap());
            }
            _ => {}
        }
    }

    match service.alter_user_profile(user_id.unwrap(), name.unwrap(), bio, birth_date.unwrap(), file_name, file_content).await {
        Ok(_) => return(StatusCode::OK, "Usuário alterado com sucesso".to_string()),
        Err(e) => return(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    }
}

#[derive(Deserialize)]
struct ChangePasswordRequest {
    id: Uuid,
    new_password: String
}

async fn change_password(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<ChangePasswordRequest>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let mut service = UserService::new((*state.db_pool).clone());

    match service.change_password(payload.id, payload.new_password).await {
        Ok(_) => Ok((StatusCode::OK, "Senha alterada com sucesso".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}



#[derive(Deserialize)]
struct DeleteUserRequest {
    id: Uuid
}

async fn delete_user(
    ConnectInfo(_addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Query(payload): Query<DeleteUserRequest>
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let service = UserService::new((*state.db_pool).clone());

    match service.delete_user(payload.id).await {
        Ok(_) => Ok((StatusCode::OK, "Usuário removido com sucesso".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
    }
}