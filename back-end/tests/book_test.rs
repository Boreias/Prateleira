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
use back_end::domain::entities::book::Book;

const TEST_IMAGE_PATH: &str = "./tests/images/book";


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
async fn test_get_book_by_id_success() {
    let app = create_app_to_test().await;

    let mut request = Request::builder()
        .uri("/book/id?id=2579a8cd-e838-4972-85fe-dd2451050719")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    request.extensions_mut().insert(
        ConnectInfo(SocketAddr::from(([127,0,0,1], 3000)))
    );

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Book = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(body.get_title(), "Computação Gráfica".to_string());
}


#[tokio::test]
async fn test_get_book_by_id_failure() {
    let app = create_app_to_test().await;

    let mut request = Request::builder()
        .uri("/book/id?id=67e55044-10b1-426f-9247-bb680e5fe0c8")
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
async fn test_get_book_by_name_success() {
    let app = create_app_to_test().await;

    let book_name = "Código Limpo".to_string();
    let replace_book_name = book_name.clone().replace(" ", "%20");

    let uri = format!("/book/name?name={}", replace_book_name);

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
    let body: Vec<Book> = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(body[0].get_title(), book_name);
}


#[tokio::test]
async fn test_get_book_by_name_failure() {
    let app = create_app_to_test().await;

    let mut request = Request::builder()
        .uri("/book/name?name=ZZZ")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    request.extensions_mut().insert(
        ConnectInfo(SocketAddr::from(([127,0,0,1], 3000)))
    );

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: Vec<Book> = serde_json::from_slice(&bytes).unwrap();


    assert_eq!(body, vec![]);
}


#[tokio::test]
async fn test_get_book_by_publisher_success() {
    let app = create_app_to_test().await;

    let publisher_id = "757ea9a1-ab74-454a-9391-4425c4eb9316".to_string();
    let book_title = "A Saga de Elric".to_string();

    let uri = format!("/book/publisher?publisher_id={}", publisher_id);

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
    let body: Vec<Book> = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(body.len(), 1);
    assert_eq!(body[0].get_title(), book_title);
}


#[tokio::test]
async fn test_get_book_by_author_success() {
    let app = create_app_to_test().await;

    let author_id = "cb70ae91-fc1a-4627-a0f4-c5d3523ec5b0".to_string();
    let book_title = "O Senhor dos Anéis".to_string();

    let uri = format!("/book/author?author_id={}", author_id);

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
    let body: Vec<Book> = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(body.len(), 3);
    assert_eq!(body[0].get_title(), book_title);
}


#[tokio::test]
async fn test_get_book_by_gender_success() {
    let app = create_app_to_test().await;

    let gender_id = "9349c148-2233-4fa8-ab44-7e52faac9923".to_string();
    let book_title = "Pai Rico, Pai Pobre".to_string();

    let uri = format!("/book/gender?gender_id={}", gender_id);

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
    let body: Vec<Book> = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(body.len(), 1);
    assert_eq!(body[0].get_title(), book_title);
}


#[tokio::test]
async fn test_flux_without_image() {
    let app = create_app_to_test().await;

    // --------------------------- Criando Livro ---------------------------

    let book_title = String::from("Hobbit");
    let book_edition = 1;
    let book_pages = 336;
    let book_publication_year = 2019;
    let book_language = String::from("Português");
    let book_publisher_id = String::from("b8ad6e89-69d9-4104-867a-c131a0a08750");
    let book_genders_id = vec![
        String::from("ae7df38c-8328-4077-ad2c-9670f11a9aad"),
        String::from("d0a28604-7d6f-4fa4-8cd2-f5c8ff0951fe"),
        String::from("c26bbe9d-5026-44d4-a096-df60889f8e85"),
        String::from("e930b438-9a28-4c4d-bdd5-cda3e6d3621c")
    ];
    let book_authors_id = vec![
        String::from("cb70ae91-fc1a-4627-a0f4-c5d3523ec5b0")
    ];
    let book_isbn = String::from("978-8595084742");
    let user_id = Uuid::new_v4();

    let boundary = "----boundary123";

    let body = format!(
        "--{boundary}\r\n\
        Content-Disposition: form-data; name=\"title\"\r\n\r\n\
        {title}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"user_id\"\r\n\r\n\
        {user_id}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"edition\"\r\n\r\n\
        {edition}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"pages\"\r\n\r\n\
        {pages}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"publication_year\"\r\n\r\n\
        {publication_year}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"language\"\r\n\r\n\
        {language}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"publisher_id\"\r\n\r\n\
        {publisher_id}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"genders_id\"\r\n\r\n\
        {genders_id}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"isbn\"\r\n\r\n\
        {isbn}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"authors_id\"\r\n\r\n\
        {authors_id}\r\n\
        --{boundary}--\r\n",
        boundary = boundary,
        title = book_title,
        user_id = user_id,
        edition = book_edition,
        pages = book_pages,
        publication_year = book_publication_year,
        language = book_language,
        publisher_id = book_publisher_id,
        genders_id = book_genders_id.join(","),
        authors_id = book_authors_id.join(","),
        isbn = book_isbn
    );

    let mut request = Request::builder()
        .uri("/book/create")
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

    // --------------------------- Selecionando livro ---------------------------

    let replace_book_title = book_title.clone().replace(" ", "%20");

    let uri = format!("/book/name?name={}", replace_book_title);

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
    let body: Vec<Book> = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(body[0].get_title(), book_title);

    // --------------------------- Alterando livro ---------------------------

    let new_book_title = String::from("O Hobbit");

    let book_id = body[0].get_id();

    let body = format!(
        "--{boundary}\r\n\
        Content-Disposition: form-data; name=\"id\"\r\n\r\n\
        {id}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"title\"\r\n\r\n\
        {title}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"user_id\"\r\n\r\n\
        {user_id}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"edition\"\r\n\r\n\
        {edition}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"pages\"\r\n\r\n\
        {pages}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"publication_year\"\r\n\r\n\
        {publication_year}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"language\"\r\n\r\n\
        {language}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"publisher_id\"\r\n\r\n\
        {publisher_id}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"genders_id\"\r\n\r\n\
        {genders_id}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"isbn\"\r\n\r\n\
        {isbn}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"authors_id\"\r\n\r\n\
        {authors_id}\r\n\
        --{boundary}--\r\n",
        boundary = boundary,
        id = book_id,
        title = new_book_title,
        user_id = user_id,
        edition = book_edition,
        pages = book_pages,
        publication_year = book_publication_year,
        language = book_language,
        publisher_id = book_publisher_id,
        genders_id = book_genders_id.join(","),
        authors_id = book_authors_id.join(","),
        isbn = book_isbn
    );

    let mut request = Request::builder()
        .uri("/book/alter")
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

    // --------------------------- Deletar livro ---------------------------

    let uri = format!("/book/delete?id={}&user_id={}", book_id, user_id);

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

    // --------------------------- Limpando registros deletados da tabela de livro ---------------------------

    let mut request = Request::builder()
        .uri("/book/clear_deleted")
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
async fn test_complete_book_flux_with_images() {
    let app = create_app_to_test().await;

    // --------------------------- Criando Livro ---------------------------

    let book_title = String::from("Arvore e Folha");
    let user_id = Uuid::new_v4();
    let book_edition = 1;
    let book_pages = 336;
    let book_publication_year = 2019;
    let book_language = String::from("Português");
    let book_publisher_id = String::from("b8ad6e89-69d9-4104-867a-c131a0a08750");
    let book_genders_id = vec![
        String::from("ae7df38c-8328-4077-ad2c-9670f11a9aad"),
        String::from("d0a28604-7d6f-4fa4-8cd2-f5c8ff0951fe"),
        String::from("c26bbe9d-5026-44d4-a096-df60889f8e85"),
        String::from("e930b438-9a28-4c4d-bdd5-cda3e6d3621c")
    ];
    let book_authors_id = vec![
        String::from("cb70ae91-fc1a-4627-a0f4-c5d3523ec5b0")
    ];
    let book_isbn = String::from("978-8595084742");
    let avatar_file_name = "Arvore e Folha1";
    let avatar_path = format!("{}/{}.png", TEST_IMAGE_PATH, avatar_file_name);
    let avatar = read(&avatar_path).unwrap();

    let boundary = "----boundary123";

    let mut body = BytesMut::new();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"title\"\r\n\r\n{}\r\n",
        boundary,
        book_title
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"user_id\"\r\n\r\n{}\r\n",
        boundary,
        user_id
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"edition\"\r\n\r\n{}\r\n",
        boundary,
        book_edition
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"pages\"\r\n\r\n{}\r\n",
        boundary,
        book_pages
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"publication_year\"\r\n\r\n{}\r\n",
        boundary,
        book_publication_year
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"language\"\r\n\r\n{}\r\n",
        boundary,
        book_language
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"publisher_id\"\r\n\r\n{}\r\n",
        boundary,
        book_publisher_id
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"genders_id\"\r\n\r\n{}\r\n",
        boundary,
        book_genders_id.join(",")
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"authors_id\"\r\n\r\n{}\r\n",
        boundary,
        book_authors_id.join(",")
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"isbn\"\r\n\r\n{}\r\n",
        boundary,
        book_isbn
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
        .uri("/book/create")
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

    // --------------------------- Selecionando Livro ---------------------------

    let replace_title_book = book_title.clone().replace(" ", "%20");

    let uri = format!("/book/name?name={}", replace_title_book);

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
    let body: Vec<Book> = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(body[0].get_title(), book_title);

    // --------------------------- Alterando Livro ---------------------------

    let new_book_title = String::from("Árvore e Folha");
    let new_avatar_file_name = "Arvore e Folha2";
    let new_avatar_path = format!("{}/{}.png", TEST_IMAGE_PATH, new_avatar_file_name);
    let new_avatar = read(&new_avatar_path).unwrap();

    let book_id = body[0].get_id();

    let mut body = BytesMut::new();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"id\"\r\n\r\n{}\r\n",
        boundary,
        book_id
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"title\"\r\n\r\n{}\r\n",
        boundary,
        new_book_title
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"user_id\"\r\n\r\n{}\r\n",
        boundary,
        user_id
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"edition\"\r\n\r\n{}\r\n",
        boundary,
        book_edition
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"pages\"\r\n\r\n{}\r\n",
        boundary,
        book_pages
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"publication_year\"\r\n\r\n{}\r\n",
        boundary,
        book_publication_year
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"language\"\r\n\r\n{}\r\n",
        boundary,
        book_language
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"publisher_id\"\r\n\r\n{}\r\n",
        boundary,
        book_publisher_id
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"genders_id\"\r\n\r\n{}\r\n",
        boundary,
        book_genders_id.join(",")
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"authors_id\"\r\n\r\n{}\r\n",
        boundary,
        book_authors_id.join(",")
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"isbn\"\r\n\r\n{}\r\n",
        boundary,
        book_isbn
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
        .uri("/book/alter")
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

    // --------------------------- Deletar livro ---------------------------

    let uri = format!("/book/delete?id={}&user_id={}", book_id, user_id);

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

    // --------------------------- Limpando registros deletados da tabela de livro ---------------------------

    let mut request = Request::builder()
        .uri("/book/clear_deleted")
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
async fn test_create_book_without_image_edit_add_image() {
    let app = create_app_to_test().await;

    // --------------------------- Criando Livro ---------------------------

    let book_title = String::from("O Feiticeiro de Terramar");
    let user_id = Uuid::new_v4();
    let book_edition = 1;
    let book_pages = 208;
    let book_publication_year = 2022;
    let book_language = String::from("Português");
    let book_publisher_id = String::from("acd9ec73-901f-45b8-b121-3c78ba845c61");
    let book_genders_id = vec![
        String::from("ae7df38c-8328-4077-ad2c-9670f11a9aad"),
        String::from("d0a28604-7d6f-4fa4-8cd2-f5c8ff0951fe"),
        String::from("c26bbe9d-5026-44d4-a096-df60889f8e85"),
        String::from("e930b438-9a28-4c4d-bdd5-cda3e6d3621c")
    ];
    let book_authors_id = vec![
        String::from("f7dc9b34-877f-4abc-84f3-4c85303538a5")
    ];
    let book_isbn = String::from("978-6586015423");

    let boundary = "----boundary123";

    let body = format!(
        "--{boundary}\r\n\
        Content-Disposition: form-data; name=\"title\"\r\n\r\n\
        {title}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"user_id\"\r\n\r\n\
        {user_id}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"edition\"\r\n\r\n\
        {edition}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"pages\"\r\n\r\n\
        {pages}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"publication_year\"\r\n\r\n\
        {publication_year}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"language\"\r\n\r\n\
        {language}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"publisher_id\"\r\n\r\n\
        {publisher_id}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"genders_id\"\r\n\r\n\
        {genders_id}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"isbn\"\r\n\r\n\
        {isbn}\r\n\
        --{boundary}\r\n\
        Content-Disposition: form-data; name=\"authors_id\"\r\n\r\n\
        {authors_id}\r\n\
        --{boundary}--\r\n",
        boundary = boundary,
        title = book_title,
        user_id = user_id,
        edition = book_edition,
        pages = book_pages,
        publication_year = book_publication_year,
        language = book_language,
        publisher_id = book_publisher_id,
        genders_id = book_genders_id.join(","),
        authors_id = book_authors_id.join(","),
        isbn = book_isbn
    );

    let mut request = Request::builder()
        .uri("/book/create")
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

    // --------------------------- Selecionando livro ---------------------------

    let replace_book_title = book_title.clone().replace(" ", "%20");

    let uri = format!("/book/name?name={}", replace_book_title);

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
    let body: Vec<Book> = serde_json::from_slice(&bytes).unwrap();

    assert_eq!(body[0].get_title(), book_title);

    // --------------------------- Alterando livro ---------------------------

    let avatar_file_name = "O Feiticeiro de TerraMar";
    let avatar_path = format!("{}/{}.png", TEST_IMAGE_PATH, avatar_file_name);
    let avatar = read(&avatar_path).unwrap();

    let book_id = body[0].get_id();

    let mut body = BytesMut::new();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"id\"\r\n\r\n{}\r\n",
        boundary,
        book_id
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"title\"\r\n\r\n{}\r\n",
        boundary,
        book_title
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"user_id\"\r\n\r\n{}\r\n",
        boundary,
        user_id
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"edition\"\r\n\r\n{}\r\n",
        boundary,
        book_edition
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"pages\"\r\n\r\n{}\r\n",
        boundary,
        book_pages
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"publication_year\"\r\n\r\n{}\r\n",
        boundary,
        book_publication_year
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"language\"\r\n\r\n{}\r\n",
        boundary,
        book_language
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"publisher_id\"\r\n\r\n{}\r\n",
        boundary,
        book_publisher_id
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"genders_id\"\r\n\r\n{}\r\n",
        boundary,
        book_genders_id.join(",")
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"authors_id\"\r\n\r\n{}\r\n",
        boundary,
        book_authors_id.join(",")
    ).unwrap();

    write!(
        &mut body,
        "--{}\r\nContent-Disposition: form-data; name=\"isbn\"\r\n\r\n{}\r\n",
        boundary,
        book_isbn
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
        .uri("/book/alter")
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

    // --------------------------- Deletar livro ---------------------------

    let uri = format!("/book/delete?id={}&user_id={}", book_id, user_id);

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

    // --------------------------- Limpando registros deletados da tabela de livro ---------------------------

    let mut request = Request::builder()
        .uri("/book/clear_deleted")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    request.extensions_mut().insert(
        ConnectInfo(SocketAddr::from(([127,0,0,1], 3000)))
    );

    let response = app.clone().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}