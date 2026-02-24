use std::{
    net::SocketAddr,
    sync::Arc
};
use tokio::net::TcpListener;
use std::env;
use dotenv::dotenv;

use back_end::infrastructure::app_state::AppState;
use back_end::infrastructure::db::connection::create_pool;
use back_end::presentation::routes::create_app;



#[tokio::main]
async fn main() {

    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("Variável de ambiente DATABASE_URL não definida");

    let pool = create_pool(&database_url).await;

    let state = AppState {
        db_pool: Arc::new(pool)
    };

    let app = create_app(state);

    let addr = "0.0.0.0:3000".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(addr).await.unwrap();
    
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}