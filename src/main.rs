mod data;
mod presentation;
mod domain;

use presentation::routes::router;
use crate::data::repositories::xml_tv_repository;
use crate::data::sources::db::postgres_client;


#[tokio::main]
async fn main() {

    dotenv::dotenv().ok();

    postgres_client::init_schema().await;

    xml_tv_repository::init_xml_tv_data().await;

    let router = router::create_router();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
