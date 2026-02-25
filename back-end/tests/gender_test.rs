use std::{sync::Arc, vec};
use std::net::SocketAddr;
use axum::{
    body::Body,
    http::{
        Request, StatusCode,
        header::CONTENT_TYPE
    },
    extract::{ConnectInfo}
};
use mime::APPLICATION_JSON;
use dotenv::dotenv;
use tower::ServiceExt;
use serde_json::json;
use http_body_util::BodyExt;


use back_end::infrastructure::app_state::AppState;
use back_end::infrastructure::db::connection::create_pool;
use back_end::presentation::routes::create_app;
use back_end::domain::entities::gender::Gender;


#[tokio::test]
async fn test_get_gender_by_id_success() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = create_pool(&database_url).await;

    let state = AppState {
        db_pool: Arc::new(pool),
    };

    let app = create_app(state);

    let mut request = Request::builder()
        .uri("/gender/id?id=05fa66f8-808e-448d-8ead-1418ef580153")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    request.extensions_mut().insert(
        ConnectInfo(SocketAddr::from(([127,0,0,1], 3000)))
    );

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_get_gender_by_id_failure() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = create_pool(&database_url).await;

    let state = AppState {
        db_pool: Arc::new(pool),
    };

    let app = create_app(state);

    let mut request = Request::builder()
        .uri("/gender/id?id=67e55044-10b1-426f-9247-bb680e5fe0c8")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    request.extensions_mut().insert(
        ConnectInfo(SocketAddr::from(([127,0,0,1], 3000)))
    );

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
}

#[tokio::test]
async fn test_get_gender_by_name_success() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = create_pool(&database_url).await;

    let gender_name = "Fantasia".to_string();

    let uri = format!("/gender/name?name={}", gender_name.clone());

    let state = AppState {
        db_pool: Arc::new(pool),
    };

    let app = create_app(state);

    let mut request = Request::builder()
        .uri(&uri)
        .method("GET")
        .body(Body::empty())
        .unwrap();

    request.extensions_mut().insert(
        ConnectInfo(SocketAddr::from(([127,0,0,1], 3000)))
    );

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Vec<Gender> = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(body[0].get_name(), gender_name);
}

#[tokio::test]
async fn test_get_gender_by_name_failure() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = create_pool(&database_url).await;

    let state = AppState {
        db_pool: Arc::new(pool),
    };

    let app = create_app(state);

    let mut request = Request::builder()
        .uri("/gender/name?name=ZZZ")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    request.extensions_mut().insert(
        ConnectInfo(SocketAddr::from(([127,0,0,1], 3000)))
    );

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Vec<Gender> = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(body, vec![]);
}

#[tokio::test]
async fn test_complete_gender_flux() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = create_pool(&database_url).await;

    let state = AppState {
        db_pool: Arc::new(pool),
    };

    let app = create_app(state);

    // --------------------------- Criando gênero ---------------------------

    let gender_name = "AAA".to_string();
    let user_id = "67e55044-10b1-426f-9247-bb680e5fe0c8";

    let payload = json!({
        "name": gender_name.clone(),
        "user_id": user_id
    });

    let mut request = Request::builder()
        .uri("/gender/create")
        .method("POST")
        .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
        .body(Body::from(payload.to_string()))
        .unwrap();

    request.extensions_mut().insert(
        ConnectInfo(SocketAddr::from(([127,0,0,1], 3000)))
    );

    let response = app.clone().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    // --------------------------- Selecionando gênero ---------------------------

    let uri = format!("/gender/name?name={}", gender_name.clone());

    let mut request = Request::builder()
        .uri(&uri)
        .method("GET")
        .body(Body::empty())
        .unwrap();

    request.extensions_mut().insert(
        ConnectInfo(SocketAddr::from(([127,0,0,1], 3000)))
    );

    let response = app.clone().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Vec<Gender> = serde_json::from_slice(&bytes).unwrap();
    
    assert_eq!(body[0].get_name(), gender_name);

    // --------------------------- Alterando gênero ---------------------------

    let new_gender_name = "BBB".to_string();

    let book_id = body[0].get_id();

    let payload = json!({
        "id": book_id,
        "name": new_gender_name,
        "user_id": user_id
    });

    let mut request = Request::builder()
        .uri("/gender/alter")
        .method("PUT")
        .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
        .body(Body::from(payload.to_string()))
        .unwrap();

    request.extensions_mut().insert(
        ConnectInfo(SocketAddr::from(([127,0,0,1], 3000)))
    );

    let response = app.clone().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // --------------------------- Deletando gênero ---------------------------

    let uri = format!("/gender/delete?id={}&user_id={}", book_id, user_id);

    let mut request = Request::builder()
        .uri(&uri)
        .method("DELETE")
        .body(Body::empty())
        .unwrap();

    request.extensions_mut().insert(
        ConnectInfo(SocketAddr::from(([127,0,0,1], 3000)))
    );

    let response = app.clone().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // --------------------------- Limpando registros deletados da tabela de gênero ---------------------------

    let mut request = Request::builder()
        .uri("/gender/clear_deleted")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    request.extensions_mut().insert(
        ConnectInfo(SocketAddr::from(([127,0,0,1], 3000)))
    );

    let response = app.clone().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}