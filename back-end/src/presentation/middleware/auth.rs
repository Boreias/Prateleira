use axum::{
    extract::{ConnectInfo, Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response
};
use std::net::SocketAddr;
use uuid::Uuid;
use axum_extra::extract::cookie::CookieJar;

use crate::infrastructure::app_state::AppState;
use crate::infrastructure::crypto::crypto::validate_jwt;
use crate::infrastructure::db::models::user_auth_row::UserAuthRow;
use crate::infrastructure::location::location::get_location;


pub async fn auth(
    State(state): State<AppState>,
    jar: CookieJar,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_cookie = jar.get("access_token");

    let auth_cookie = if let Some(auth_cookie) = auth_cookie {
        auth_cookie
    } else {
        return Err(StatusCode::UNAUTHORIZED)
    };

    if let Some(user_id) = authorize_current_user(auth_cookie.value()).await {
        if let Some(ConnectInfo(addr)) = req.extensions().get::<ConnectInfo<SocketAddr>>() {
            let country = get_location(addr.ip()).expect("Erro na obtenção da localização");

            let bd = state.db_pool.clone();

            
            let some_user: Option<UserAuthRow> = sqlx::query_as(r#"
                SELECT
                    id, username, email, password_hash, salt, country, is_active, is_email_verified, created_at, updated_at
                FROM
                    user_auth
                WHERE
                    id = $1 AND country = $2;
            "#)
                .bind(user_id)
                .bind(country)
                .fetch_optional(&*bd)
                .await
                .map_err(|e| e.to_string())
                .expect("Erro ao checar os dados do usuário");

            if some_user.is_some() {
                req.extensions_mut().insert(user_id);
                return Ok(next.run(req).await);
            } else {
                return Err(StatusCode::BAD_REQUEST);
            }
        } else {
            return Err(StatusCode::BAD_REQUEST);
        }
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn authorize_current_user(auth_token: &str) -> Option<Uuid> {
    let token_message = validate_jwt(auth_token.to_string()).unwrap();

    Some(token_message.claims.sub)
}