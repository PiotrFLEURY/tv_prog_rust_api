mod data;
mod domain;
mod presentation;

use crate::data::repositories::xml_tv_repository;
use crate::data::sources::db::postgres_client;
use crate::presentation::routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    postgres_client::init_schema();

    xml_tv_repository::init_xml_tv_data().await;

    let router = routes::create_router();

    // run our app with hyper, listening globally on port 3000
    let listener = match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
        Err(e) => {
            eprintln!("Failed to bind to address: {}", e);
            std::process::exit(1);
        }
        Ok(l) => l,
    };
    match axum::serve(listener, router).await {
        Err(e) => {
            eprintln!("Server error: {}", e);
            std::process::exit(1);
        }
        Ok(_) => {}
    }
}
