use axum::{
    routing::{get, post, put, delete}, Router
};
use std::{net::SocketAddr, sync::Arc};
use dotenv::dotenv;

mod app_state;
use app_state::AppState;


mod infrastructure {
    pub mod crypto;
    pub mod db;
    pub mod dto;
    pub mod email;
    pub mod repositories;
}

mod application {
    pub mod services;
}

mod presentation {
    pub mod controllers;
}

mod domain {
    pub mod entities;
    pub mod irepositories;
}

use infrastructure::db::connection::create_pool;
use application::services::user_service::UserService;
use presentation::controllers::user_controller::{
    register_user,
    find_user_by_id,
    find_user_by_email,
    find_user_by_nickname,
    update_user,
    remove_user
};



#[tokio::main]
async fn main() {

    dotenv().ok();

    let pool = create_pool().await.expect("Erro ao conectar ao banco");

    let user_service = Arc::new(UserService::new(pool.clone()));

    let state = AppState {
        db_pool: Arc::new(pool),
        user_service,
    };


    let app = Router::new()
        .route("/", get( || async { "Hello, world!!" }))
        .route("/registerUser", post(register_user))
        .route("/findUserById", get(find_user_by_id))
        .route("/findUserByEmail", get(find_user_by_email))
        .route("/findUserByNickname", get(find_user_by_nickname))
        .route("/alterUser", put(update_user))
        .route("/deleteUser", delete(remove_user))
        .with_state(state);

    let addr = "0.0.0.0:3000".parse::<SocketAddr>().unwrap();
    let listner = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listner, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}