use std::collections::HashMap;
use crate::data::repositories::program_repository;
use crate::domain::entities::program::Program;
use axum::extract::Query;
use axum::Json;

pub async fn get_programs_by_channel_id(Query(params): Query<HashMap<String, String>>) -> Json<Vec<Program>> {
    let channel_id = match params.get("channelId") {
        Some(id) => id.clone(),
        None => return Json(vec![]), // Return an empty vector if no channel_id is provided
    };
    Json(program_repository::get_programs_by_channel_id(channel_id))
}

pub async fn get_current_program_by_channel_id(Query(params): Query<HashMap<String, String>>) -> Json<Option<Program>> {
    let channel_id = match params.get("channelId") {
        Some(id) => id.clone(),
        None => return Json(None),
    };
    let program = program_repository::get_current_program_by_channel_id(channel_id);
    Json(Some(program))
}

pub async fn get_tonight_program_by_channel_id(Query(params): Query<HashMap<String, String>>) -> Json<Option<Program>> {
    let channel_id = match params.get("channelId") {
        Some(id) => id.clone(),
        None => return Json(None),
    };
    let program = program_repository::get_tonight_program_by_channel_id(channel_id);
    Json(Some(program))
}