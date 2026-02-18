use axum::Router;
use std::{
    net::SocketAddr,
    sync::Arc
};
use std::env;
use dotenv::dotenv;

mod app_state;
use app_state::AppState;


mod infrastructure;
mod application;
mod presentation;
mod domain;

use infrastructure::db::connection::create_pool;
use presentation::controllers::user_controller::user_routes;
use presentation::controllers::author_controller::author_routes;
use presentation::controllers::gender_controller::gender_routes;



#[tokio::main]
async fn main() {

    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("Variável de ambiente DATABASE_URL não definida");

    let pool = create_pool(&database_url).await;

    let state = AppState {
        db_pool: Arc::new(pool)
    };

    let app = Router::new()
        .nest("/user", user_routes())
        .nest("/author", author_routes())
        .nest("/gender", gender_routes())
        .with_state(state);

    let addr = "0.0.0.0:3000".parse::<SocketAddr>().unwrap();
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}