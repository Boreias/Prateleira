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
    extract::{ConnectInfo},
    Router
};
use bytes::BytesMut;
use dotenv::dotenv;
use tower::ServiceExt;
use http_body_util::BodyExt;
use uuid::Uuid;


use back_end::infrastructure::app_state::AppState;
use back_end::infrastructure::db::connection::create_pool;
use back_end::presentation::routes::create_app;
use back_end::domain::entities::publisher::Publisher;

const TEST_IMAGE_PATH: &str = "./tests/images/publisher";


async fn create_app_to_test() -> Router {
    dotenv().ok();

    let database_url = std::env::var("TESTE_DATABASE_URL").unwrap();
    let pool = create_pool(&database_url).await;

    let state = AppState {
        db_pool: Arc::new(pool),
    };

    let app = create_app(state);

    return app
}


#[tokio::test]
async fn test_get_publisher_by_id_success() {
    let app = create_app_to_test().await;

    let mut request = Request::builder()
        .uri("/publisher/id?id=acd9ec73-901f-45b8-b121-3c78ba845c61")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    request.extensions_mut().insert(
        ConnectInfo(SocketAddr::from(([127,0,0,1], 3000)))
    );

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Publisher = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(body.get_name(), "Alta Books".to_string());
}


#[tokio::test]
async fn test_get_publisher_by_id_failure() {
    let app = create_app_to_test().await;

    let mut request = Request::builder()
        .uri("/publisher/id?id=67e55044-10b1-426f-9247-bb680e5fe0c8")
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
async fn test_get_publisher_by_name_success() {
    let app = create_app_to_test().await;

    let publisher_name = "Alta Books".to_string();
    let replace_publisher_name = publisher_name.clone().replace(" ", "%20");

    let uri = format!("/publisher/name?name={}", replace_publisher_name);

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
    let body: Vec<Publisher> = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(body[0].get_name(), publisher_name);
}


#[tokio::test]
async fn test_get_publisher_by_name_failure() {
    let app = create_app_to_test().await;

    let mut request = Request::builder()
        .uri("/publisher/name?name=ZZZ")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    request.extensions_mut().insert(
        ConnectInfo(SocketAddr::from(([127,0,0,1], 3000)))
    );

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Vec<Publisher> = serde_json::from_slice(&bytes).unwrap();


    assert_eq!(body, vec![]);
}


#[tokio::test]
async fn test_get_publisher_by_book() {
    let app = create_app_to_test().await;

    let book_id = "3dbd31ad-1967-4a1f-97cd-fcf05bfb4185".to_string();
    let publisher_name = "HarperCollins Brasil".to_string();

    let uri = format!("/publisher/book?book_id={}", book_id);

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
    let body: Publisher = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(body.get_name(), publisher_name);
}


#[tokio::test]
async fn test_get_publisher_by_author() {    
    let app = create_app_to_test().await;

    let author_id = "d309d334-8662-46f0-885e-60ddcb3eec23".to_string();
    let publisher_name = "Pipoca e Nanquim".to_string();

    let uri = format!("/publisher/author?author_id={}", author_id);

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
    let body: Vec<Publisher> = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(body[0].get_name(), publisher_name);
}


#[tokio::test]
async fn test_get_publisher_by_gender() {
    let app = create_app_to_test().await;

    let gender_id = "25580202-27b8-4fc5-a13d-c5ff3c36ecf1".to_string();
    let publisher_name = "Alta Books".to_string();

    let uri = format!("/publisher/gender?gender_id={}", gender_id);

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
    let body: Vec<Publisher> = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(body[0].get_name(), publisher_name);
}


#[tokio::test]
async fn test_flux_without_image() {
    let app = create_app_to_test().await;

    // --------------------------- Criando Editora ---------------------------

    let publisher_name = String::from("O'Reilly");
    let publisher_site = String::from("https://novatec.com.br/");
    let publisher_email = String::from("novatec@novatec.com.br");
    let user_id = Uuid::new_v4();

    let boundary = "----boundary123";

    let body = format!(
        "--{boundary}\r\n\
        Content-Disposition: form-data; name=\"name\"\r\n\r\n\
        {name}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"user_id\"\r\n\r\n\
        {user_id}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"site\"\r\n\r\n\
        {site}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"email\"\r\n\r\n\
        {email}\r\n\
        --{boundary}--\r\n",
        boundary = boundary,
        name = publisher_name,
        user_id = user_id,
        site = publisher_site,
        email = publisher_email
    );

    let mut request = Request::builder()
        .uri("/publisher/create")
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

    // --------------------------- Selecionando editora ---------------------------

    let replace_publisher_name = publisher_name.clone().replace(" ", "%20");

    let uri = format!("/publisher/name?name={}", replace_publisher_name);

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
    let body: Vec<Publisher> = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(body[0].get_name(), publisher_name);

    // --------------------------- Alterando editora ---------------------------

    let new_publisher_name = String::from("novatec");

    let publisher_id = body[0].get_id();

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
        id = publisher_id,
        name = new_publisher_name,
        user_id = user_id
    );

    let mut request = Request::builder()
        .uri("/publisher/alter")
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

    // --------------------------- Deletar editora ---------------------------

    let uri = format!("/publisher/delete?id={}&user_id={}", publisher_id, user_id);

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

    // --------------------------- Limpando registros deletados da tabela de editora ---------------------------

    let mut request = Request::builder()
        .uri("/publisher/clear_deleted")
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
async fn test_complete_publisher_flux_with_images() {
    let app = create_app_to_test().await;

    // --------------------------- Criando Editora ---------------------------

    let publisher_name = String::from("Gen");
    let publisher_site = String::from("https://www.grupogen.com/");
    let publisher_email = String::from("ltc@grupogen.com");
    let user_id = Uuid::new_v4();
    let avatar_file_name = "gen_logo";
    let avatar_path = format!("{}/{}.png", TEST_IMAGE_PATH, avatar_file_name);
    let avatar = read(&avatar_path).unwrap();

    let boundary = "----boundary123";

    let mut body = BytesMut::new();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"name\"\r\n\r\n{}\r\n",
        boundary,
        publisher_name
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"user_id\"\r\n\r\n{}\r\n",
        boundary,
        user_id
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"site\"\r\n\r\n{}\r\n",
        boundary,
        publisher_site
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"email\"\r\n\r\n{}\r\n",
        boundary,
        publisher_email
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
        .uri("/publisher/create")
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

    // --------------------------- Selecionando Editora ---------------------------

    let replace_publisher_name = publisher_name.clone().replace(" ", "%20");

    let uri = format!("/publisher/name?name={}", replace_publisher_name);

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
    let body: Vec<Publisher> = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(body[0].get_name(), publisher_name);

    // --------------------------- Alterando editora ---------------------------

    let new_publisher_name = String::from("GEN LTC");
    let new_publisher_site = String::from("https://www.grupogen.com.br/");
    let new_publisher_email = String::from("ltc@grupogen.com.br");
    let new_avatar_file_name = "gen_logo_2";
    let new_avatar_path = format!("{}/{}.png", TEST_IMAGE_PATH, new_avatar_file_name);
    let new_avatar = read(&new_avatar_path).unwrap();

    let publisher_id = body[0].get_id();

    let mut body = BytesMut::new();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"id\"\r\n\r\n{}\r\n",
        boundary,
        publisher_id
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"name\"\r\n\r\n{}\r\n",
        boundary,
        new_publisher_name
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"user_id\"\r\n\r\n{}\r\n",
        boundary,
        user_id
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"site\"\r\n\r\n{}\r\n",
        boundary,
        new_publisher_site
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"email\"\r\n\r\n{}\r\n",
        boundary,
        new_publisher_email
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
        .uri("/publisher/alter")
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

    // --------------------------- Deletar editora ---------------------------

    let uri = format!("/publisher/delete?id={}&user_id={}", publisher_id, user_id);

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

    // --------------------------- Limpando registros deletados da tabela de editora ---------------------------

    let mut request = Request::builder()
        .uri("/publisher/clear_deleted")
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
async fn test_create_publisher_without_image_edit_add_image() {
    let app = create_app_to_test().await;

    // --------------------------- Criando Editora ---------------------------

    let publisher_name = String::from("Planeta Minotauro");
    let publisher_email = String::from("imprensa@editoraplaneta.com.br");
    let user_id = Uuid::new_v4();

    let boundary = "----boundary123";

    let body = format!(
        "--{boundary}\r\n\
        Content-Disposition: form-data; name=\"name\"\r\n\r\n\
        {name}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"user_id\"\r\n\r\n\
        {user_id}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"email\"\r\n\r\n\
        {email}\r\n\
        --{boundary}--\r\n",
        boundary = boundary,
        name = publisher_name,
        user_id = user_id,
        email = publisher_email
    );

    let mut request = Request::builder()
        .uri("/publisher/create")
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

    // --------------------------- Selecionando editora ---------------------------

    let replace_publisher_name = publisher_name.clone().replace(" ", "%20");

    let uri = format!("/publisher/name?name={}", replace_publisher_name);

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
    let body: Vec<Publisher> = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(body[0].get_name(), publisher_name);

    // --------------------------- Alterando editora ---------------------------

    let avatar_file_name = "minotauro_logo";
    let avatar_path = format!("{}/{}.png", TEST_IMAGE_PATH, avatar_file_name);
    let avatar = read(&avatar_path).unwrap();

    let publisher_id = body[0].get_id();

    let mut body = BytesMut::new();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"id\"\r\n\r\n{}\r\n",
        boundary,
        publisher_id
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"name\"\r\n\r\n{}\r\n",
        boundary,
        publisher_name
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
        .uri("/publisher/alter")
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

    // --------------------------- Deletar editora ---------------------------

    let uri = format!("/publisher/delete?id={}&user_id={}", publisher_id, user_id);

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

    // --------------------------- Limpando registros deletados da tabela de editora ---------------------------

    let mut request = Request::builder()
        .uri("/publisher/clear_deleted")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    request.extensions_mut().insert(
        ConnectInfo(SocketAddr::from(([127,0,0,1], 3000)))
    );

    let response = app.clone().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}