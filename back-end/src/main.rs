// use axum::{Router, routing::post};
// use sqlx::PgPool;
// use std::net::SocketAddr;
// use dotenv::dotenv;
// use std::env;

// use crate::application::services::author_service::AuthorService;
// use crate::presentation::controllers::author_controller::{register_author};
// use crate::infrastructure::repositories::author_repository::AuthorRepository;

// use crate::application::services::book_service::BookService;
// use crate::presentation::controllers::book_controller::{register_book};
// use crate::infrastructure::repositories::book_repository::BookRepository;

// use crate::application::services::user_service::UserService;
// use crate::presentation::controllers::user_controller::{register_user};
// use crate::infrastructure::repositories::user_repository::UserRepository;

// use crate::application::services::gender_service::GenderService;
// use crate::presentation::controllers::gender_controller::{register_gender};
// use crate::infrastructure::repositories::gender_repository::GenderRepository;

// use crate::application::services::publisher_service::PublisherService;
// use crate::presentation::controllers::publisher_controller::{register_publisher};
// use crate::infrastructure::repositories::publisher_repository::PublisherRepository;


// #[tokio::main]
// async fn main() {
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     let pool = PgPool::connect(&database_url).await.unwrap();

//     let author_repository = AuthorRepository::new(pool.clone());
//     let author_service = AuthorService::new(author_repository);

//     let book_repository = BookRepository::new(pool.clone());
//     let book_service = BookService::new(book_repository);

//     let user_repository = UserRepository::new(pool.clone());
//     let user_service = UserService::new(user_repository);

//     let publiser_repository = PublisherRepository::new(pool.clone());
//     let publisher_service = PublisherService::new(publiser_repository);

//     let gender_repository = GenderRepository::new(pool.clone());
//     let gender_service = GenderService::new(gender_repository);


//     let app = Router::new()
//         .route("/author", post(register_author)).with_state(author_service)
//         .route("/book", post(register_book)).with_state(book_service)
//         .route("/user", post(register_user)).with_state(user_service)
//         .route("/publisher", post(register_publisher)).with_state(publisher_service)
//         .route("/gender", post(register_gender)).with_state(gender_service);
//         // .layer(axum::AddData::new(author_service))
//         // .layer(axum::AddData::new(book_service))
//         // .layer(axum::AddData::new(user_service))
//         // .layer(axum::AddData::new(publisher_service))
//         // .layer(axum::AddData::new(gender_service));


//     let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
//     axum::Server::bind(&addr)
//         .serve(app.into_make_service())
//         .await
//         .unwrap();
// }

use sqlx::PgPool;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL não definida");

    let pool = PgPool::connect(&database_url).await.expect("Falha ao conectar no banco");

    println!("✅ Conectado ao PostgreSQL!");
}
