use std::{
    sync::Arc,
    vec,
    fs::read_to_string
};
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
use back_end::domain::entities::author::Author;
use uuid::Uuid;

const TEST_IMAGE_PATH: &str = "images/author";


#[tokio::test]
async fn test_get_author_by_id_success() {
    dotenv().ok();

    let database_url = std::env::var("TESTE_DATABASE_URL").unwrap();
    let pool = create_pool(&database_url).await;

    let state = AppState {
        db_pool: Arc::new(pool),
    };

    let app = create_app(state);

    let mut request = Request::builder()
        .uri("/author/id?id=c535765a-12a2-46b8-bd3b-a410a7d8e7b3")
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
async fn test_get_author_by_id_failure() {
    dotenv().ok();

    let database_url = std::env::var("TESTE_DATABASE_URL").unwrap();
    let pool = create_pool(&database_url).await;

    let state = AppState {
        db_pool: Arc::new(pool),
    };

    let app = create_app(state);

    let mut request = Request::builder()
        .uri("/author/id?id=67e55044-10b1-426f-9247-bb680e5fe0c8")
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
async fn test_get_author_by_name_success() {
    dotenv().ok();

    let database_url = std::env::var("TESTE_DATABASE_URL").unwrap();
    let pool = create_pool(&database_url).await;

    let author_name = "J. R. R. Tolkien".to_string();
    let replace_author_name = author_name.clone().replace(" ", "%20");

    let uri = format!("/author/name?name={}", replace_author_name);

    let state = AppState {
        db_pool: Arc::new(pool),
    };

    let app = create_app(state);
    println!("Criei o app");

    let mut request = Request::builder()
        .uri(&uri)
        .method("GET")
        .body(Body::empty())
        .unwrap();
    println!("Criei o request");

    request.extensions_mut().insert(
        ConnectInfo(SocketAddr::from(([127,0,0,1], 3000)))
    );
    println!("Editei o request");

    let response = app.oneshot(request).await.unwrap();
    println!("response: {:?}", response);

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    println!("bytes: {:?}", bytes);
    let body: Vec<Author> = serde_json::from_slice(&bytes).unwrap();
    println!("body: {:?}", body);

    assert_eq!(body[0].get_name(), author_name);
}

#[tokio::test]
async fn test_get_author_by_name_failure() {
    dotenv().ok();

    let database_url = std::env::var("TESTE_DATABASE_URL").unwrap();
    let pool = create_pool(&database_url).await;

    let state = AppState {
        db_pool: Arc::new(pool),
    };

    let app = create_app(state);

    let mut request = Request::builder()
        .uri("/author/name?name=ZZZ")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    request.extensions_mut().insert(
        ConnectInfo(SocketAddr::from(([127,0,0,1], 3000)))
    );

    let response = app.oneshot(request).await.unwrap();

    println!("response: {:?}", response);

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Vec<Author> = serde_json::from_slice(&bytes).unwrap();


    assert_eq!(body, vec![]);
}

// #[tokio::test]
// async fn test_complete_author_flux() {
//     dotenv().ok();

//     let database_url = std::env::var("TESTE_DATABASE_URL").unwrap();
//     let pool = create_pool(&database_url).await;

//     let state = AppState {
//         db_pool: Arc::new(pool),
//     };

//     let app = create_app(state);

//     // --------------------------- Criando Autor ---------------------------

//     let author_name = String::from("Robert E. Howard");
//     let user_id = Uuid::new_v4();
//     let avatar_path = format!("{}/Robert E Howard 1.png", TEST_IMAGE_PATH);
//     let avatar = read_to_string(&avatar_path).unwrap();

//     let payload = json!({
//         "name": author_name.clone(),
//         "user_id": user_id,
//         "avatar": avatar
//     });

//     let mut request = Request::builder()
//         .uri("author/create")
//         .method("POST")
//         .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
//         .body(Body::from(payload.to_string()))
//         .unwrap();

//     request.extensions_mut().insert(
//         ConnectInfo(SocketAddr::from(([127,0,0,1], 3000)))
//     );

//     let response = app.clone().oneshot(request).await.unwrap();

//     assert_eq!(response.status(), StatusCode::CREATED);

//     // --------------------------- Selecionando autor ---------------------------

//     let uri = format!("/author/name?name={}", author_name.clone());

//     let mut request = Request::builder()
//         .uri(&uri)
//         .method("GET")
//         .body(Body::empty())
//         .unwrap();

//     request.extensions_mut().insert(
//         ConnectInfo(SocketAddr::from(([127,0,0,1], 3000)))
//     );

//     let response = app.clone().oneshot(request).await.unwrap();

//     assert_eq!(response.status(), StatusCode::OK);

//     let bytes = response.into_body().collect().await.unwrap().to_bytes();
//     let body: Vec<Author> = serde_json::from_slice(&bytes).unwrap();
    
//     assert_eq!(body[0].get_name(), author_name);

//     // --------------------------- Alterando gênero ---------------------------

//     let new_author_name = String::from("Robert Ervin Howard");
//     let new_avatar_path = format!("{}/Robert E Howard 2.png", TEST_IMAGE_PATH);
//     let new_avatar = read_to_string(&new_avatar_path).unwrap();

//     let author_id = body[0].get_id();

//     let payload = json!({
//         "id": author_id,
//         "name": new_author_name.clone(),
//         "user_id": user_id,
//         "avatar": new_avatar
//     });

//     let mut request = Request::builder()
//         .uri("/author/alter")
//         .method("PUT")
//         .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
//         .body(Body::from(payload.to_string()))
//         .unwrap();

//     request.extensions_mut().insert(
//         ConnectInfo(SocketAddr::from(([127,0,0,1], 3000)))
//     );

//     let response = app.clone().oneshot(request).await.unwrap();

//     assert_eq!(response.status(), StatusCode::OK);
// }