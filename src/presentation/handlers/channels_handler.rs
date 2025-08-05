use crate::data::repositories::channel_repository;
use crate::domain::entities::channel::Channel;
use axum::extract::Path;
use axum::Json;

pub async fn get_channels_by_package(Path(package): Path<String>) -> Json<Vec<Channel>> {
    Json(channel_repository::get_channels_by_package(package))
}