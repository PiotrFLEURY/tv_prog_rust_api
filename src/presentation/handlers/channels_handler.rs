use crate::data::repositories::channel_repository;
use crate::domain::entities::channel::Channel;
use axum::Json;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn get_channels_by_package(Path(package): Path<String>) -> impl IntoResponse {
    match channel_repository::get_channels_by_package(package).await {
        Ok(channels) => Json(channels).into_response(),
        Err(e) => {
            eprintln!("Failed to get channels: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Vec::<Channel>::new()),
            )
                .into_response()
        }
    }
}
