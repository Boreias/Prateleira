use std::{
    fmt::Write, fs::read, sync::Arc, vec
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
use bytes::BytesMut;
use dotenv::dotenv;
use tower::ServiceExt;
use http_body_util::BodyExt;


use back_end::infrastructure::app_state::AppState;
use back_end::infrastructure::db::connection::create_pool;
use back_end::presentation::routes::create_app;
use back_end::domain::entities::author::Author;
use uuid::Uuid;

const TEST_IMAGE_PATH: &str = "./tests/images/author";


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
        .uri("/author/id?id=cb70ae91-fc1a-4627-a0f4-c5d3523ec5b0")
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
    let body: Vec<Author> = serde_json::from_slice(&bytes).unwrap();

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

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Vec<Author> = serde_json::from_slice(&bytes).unwrap();


    assert_eq!(body, vec![]);
}

#[tokio::test]
async fn test_flux_without_image() {
    dotenv().ok();

    let database_url = std::env::var("TESTE_DATABASE_URL").unwrap();
    let pool = create_pool(&database_url).await;

    let state = AppState {
        db_pool: Arc::new(pool),
    };

    let app = create_app(state);

    // --------------------------- Criando Autor ---------------------------

    let author_name = String::from("Marion Zimmer Bradley");
    let user_id = Uuid::new_v4();

    let boundary = "----boundary123";

    let body = format!(
        "--{boundary}\r\n\
        Content-Disposition: form-data; name=\"name\"\r\n\r\n\
        {name}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"user_id\"\r\n\r\n\
        {user_id}\r\n\
        --{boundary}--\r\n",
        boundary = boundary,
        name = author_name,
        user_id = user_id
    );

    let mut request = Request::builder()
        .uri("/author/create")
        .method("POST")
        .header(
            CONTENT_TYPE,
            format!("multipart/form-data; boundary={}", boundary),
        )
        .body(Body::from(body))
        .unwrap();

    request.extensions_mut().insert(
        ConnectInfo(SocketAddr::from(([127,0,0,1], 3000)))
    );

    let response = app.clone().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    // --------------------------- Selecionando autor ---------------------------

    let replace_author_name = author_name.clone().replace(" ", "%20");

    let uri = format!("/author/name?name={}", replace_author_name);

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
    let body: Vec<Author> = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(body[0].get_name(), author_name);

    // --------------------------- Alterando autor ---------------------------

    let new_author_name = String::from("Marion Z. Bradley");

    let author_id = body[0].get_id();

    let body = format!(
        "--{boundary}\r\n\
        Content-Disposition: form-data; name=\"id\"\r\n\r\n\
        {id}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"name\"\r\n\r\n\
        {name}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"user_id\"\r\n\r\n\
        {user_id}\r\n\
        --{boundary}--\r\n",
        boundary = boundary,
        id = author_id,
        name = new_author_name,
        user_id = user_id
    );

    let mut request = Request::builder()
        .uri("/author/alter")
        .method("PUT")
        .header(
            CONTENT_TYPE,
            format!("multipart/form-data; boundary={}", boundary),
        )
        .body(Body::from(body))
        .unwrap();

    request.extensions_mut().insert(
        ConnectInfo(SocketAddr::from(([127,0,0,1], 3000)))
    );

    let response = app.clone().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // --------------------------- Deletar autor ---------------------------

    let uri = format!("/author/delete?id={}&user_id={}", author_id, user_id);

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
        .uri("/author/clear_deleted")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    request.extensions_mut().insert(
        ConnectInfo(SocketAddr::from(([127,0,0,1], 3000)))
    );

    let response = app.clone().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_complete_author_flux() {
    dotenv().ok();

    let database_url = std::env::var("TESTE_DATABASE_URL").unwrap();
    let pool = create_pool(&database_url).await;

    let state = AppState {
        db_pool: Arc::new(pool),
    };

    let app = create_app(state);

    // --------------------------- Criando Autor ---------------------------

    let author_name = String::from("Robert E. Howard");
    let user_id = Uuid::new_v4();
    let avatar_file_name = "Robert E Howard 1";
    let avatar_path = format!("{}/{}.png", TEST_IMAGE_PATH, avatar_file_name);
    let avatar = read(&avatar_path).unwrap();

    let boundary = "----boundary123";

    let mut body = BytesMut::new();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"name\"\r\n\r\n{}\r\n",
        boundary,
        author_name
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"user_id\"\r\n\r\n{}\r\n",
        boundary,
        user_id
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"avatar\"; filename=\"{}.png\"\r\nContent-Type: image/png\r\n\r\n",
        boundary,
        avatar_file_name
    ).unwrap();

    body.extend_from_slice(&avatar);

    write!(&mut body, "\r\n").unwrap();

    write!(&mut body, "--{}--\r\n", boundary).unwrap();

    let mut request = Request::builder()
        .uri("/author/create")
        .method("POST")
        .header(
            CONTENT_TYPE,
            format!("multipart/form-data; boundary={}", boundary),
        )
        .body(Body::from(body.freeze()))
        .unwrap();

    request.extensions_mut().insert(
        ConnectInfo(SocketAddr::from(([127,0,0,1], 3000)))
    );

    let response = app.clone().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    // --------------------------- Selecionando autor ---------------------------

    let replace_author_name = author_name.clone().replace(" ", "%20");

    let uri = format!("/author/name?name={}", replace_author_name);

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
    let body: Vec<Author> = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(body[0].get_name(), author_name);

    // --------------------------- Alterando autor ---------------------------

    let new_author_name = String::from("Robert Ervin Howard");
    let new_avatar_file_name = "Robert E Howard 2";
    let new_avatar_path = format!("{}/{}.png", TEST_IMAGE_PATH, new_avatar_file_name);
    let new_avatar = read(&new_avatar_path).unwrap();

    let author_id = body[0].get_id();

    let mut body = BytesMut::new();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"id\"\r\n\r\n{}\r\n",
        boundary,
        author_id
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"name\"\r\n\r\n{}\r\n",
        boundary,
        new_author_name
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"user_id\"\r\n\r\n{}\r\n",
        boundary,
        user_id
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"avatar\"; filename=\"{}.png\"\r\nContent-Type: image/png\r\n\r\n",
        boundary,
        new_avatar_file_name
    ).unwrap();

    body.extend_from_slice(&new_avatar);

    write!(&mut body, "\r\n").unwrap();

    write!(&mut body, "--{}--\r\n", boundary).unwrap();

    let mut request = Request::builder()
        .uri("/author/alter")
        .method("PUT")
        .header(
            CONTENT_TYPE,
            format!("multipart/form-data; boundary={}", boundary),
        )
        .body(Body::from(body.freeze()))
        .unwrap();

    request.extensions_mut().insert(
        ConnectInfo(SocketAddr::from(([127,0,0,1], 3000)))
    );

    let response = app.clone().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // --------------------------- Deletar autor ---------------------------

    let uri = format!("/author/delete?id={}&user_id={}", author_id, user_id);

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
        .uri("/author/clear_deleted")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    request.extensions_mut().insert(
        ConnectInfo(SocketAddr::from(([127,0,0,1], 3000)))
    );

    let response = app.clone().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

}