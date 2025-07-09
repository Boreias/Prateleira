use axum::{extract::{Json, State, ConnectInfo}, response::{IntoResponse, Json as JsonResponse}, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use crate::application::services::user_service::UserService;
use crate::app_state::AppState;


#[derive(Deserialize)]
pub struct RegisterUserRequest {
    pub name: String,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub birth_date: String,
    pub avatar: String
}

#[derive(Deserialize)]
pub struct FindUserByIdRequest {
    pub id: String
}

#[derive(Deserialize)]
pub struct FindUserByEmailRequest {
    pub email: String
}

#[derive(Deserialize)]
pub struct FindUserByNicknameRequest {
    pub nickname: String
}

#[derive(Deserialize)]
pub struct AlterUserRequest {
    pub id: String,
    pub name: String,
    pub nickname: String,
    pub email: String,
    pub birth_date: String,
    pub avatar: String
}

#[derive(Deserialize)]
pub struct DeleteUserRequest {
    pub id: String
}


#[derive(Serialize)]
pub struct ApiResponse {
    pub message: String
}

pub async fn register_user(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Json(payload): Json<RegisterUserRequest>
) -> impl IntoResponse {
    let service = UserService::new((*state.db_pool).clone());

    match service.register_user(payload.name, payload.nickname, payload.email, payload.password, payload.birth_date, payload.avatar).await {
        Ok(_) => return(StatusCode::CREATED, "Usuário registrado com sucesso".to_string()),
        Err(e) => {
            if e == "Usuário já cadastrado" {
                return(StatusCode::CONFLICT, e.to_string())
            } else {
                return(StatusCode::INTERNAL_SERVER_ERROR, "Erro interno".to_string())
            }
        }
    }
}

pub async fn find_user_by_id(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Json(payload): Json<FindUserByIdRequest>
) -> impl IntoResponse {
    let service = UserService::new((*state.db_pool).clone());

    match service.get_user_by_id(payload.id).await {
        Ok(Some(user)) => (StatusCode::OK, JsonResponse(user)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Usuário não encontrado").into_response(),
        Err(e) => {
            eprintln!("Erro interno ao buscar usuário: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Erro interno").into_response()
        }
    }
}

pub async fn find_user_by_email(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Json(payload): Json<FindUserByEmailRequest>
) -> impl IntoResponse {
    let service = UserService::new((*state.db_pool).clone());
    match service.get_user_by_email(payload.email).await {
        Ok(Some(user)) => (StatusCode::OK, JsonResponse(user)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "Usuário não encontrado").into_response(),
        Err(e) => {
            eprintln!("Erro interno ao buscar usuário: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Erro interno").into_response()
        }
    }
}

pub async fn find_user_by_nickname(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Json(payload): Json<FindUserByNicknameRequest>
) -> impl IntoResponse {
    let service = UserService::new((*state.db_pool).clone());

    match service.get_user_by_nickname(payload.nickname).await {
        Ok(users) => (StatusCode::OK, JsonResponse(users)).into_response(),
        Err(e) => {
            eprintln!("Erro interno ao buscar usuário: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Erro interno").into_response()
        }
    }
}

pub async fn update_user(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Json(payload): Json<AlterUserRequest>
) -> impl IntoResponse {
    let mut service = UserService::new((*state.db_pool).clone());

    match service.alter_user(payload.id, payload.name, payload.nickname, payload.email, payload.birth_date, payload.avatar).await {
        Ok(_) => return(StatusCode::OK, "Usuário alterado com sucesso".to_string()),
        Err(e) => return(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    }
}

pub async fn remove_user(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Json(payload): Json<DeleteUserRequest>
) -> impl IntoResponse {
    let service = UserService::new((*state.db_pool).clone());

    match service.delete_user(payload.id).await {
        Ok(_) => return(StatusCode::OK, "Usuário removido com sucesso".to_string()),
        Err(e) => return(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    }
}