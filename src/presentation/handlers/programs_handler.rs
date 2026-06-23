use crate::data::repositories::program_repository;
use crate::domain::entities::program::Program;
use crate::presentation::dtos::Page;
use axum::Json;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use std::collections::HashMap;

pub async fn get_programs_by_channel_id(
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let channel_id = match params.get("channelId") {
        Some(id) => id.clone(),
        None => {
            return Json(Page {
                content: Vec::<Program>::new(),
            })
            .into_response();
        }
    };
    match program_repository::get_programs_by_channel_id(channel_id).await {
        Ok(page) => Json(page).into_response(),
        Err(e) => {
            eprintln!("Failed to get programs: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Page {
                    content: Vec::<Program>::new(),
                }),
            )
                .into_response()
        }
    }
}

pub async fn get_current_program_by_channel_id(
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let channel_id = match params.get("channelId") {
        Some(id) => id.clone(),
        None => return Json(Option::<Program>::None).into_response(),
    };
    match program_repository::get_current_program_by_channel_id(channel_id).await {
        Ok(program) => Json(program).into_response(),
        Err(e) => {
            eprintln!("Failed to get current program: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Option::<Program>::None),
            )
                .into_response()
        }
    }
}

pub async fn get_tonight_program_by_channel_id(
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let channel_id = match params.get("channelId") {
        Some(id) => id.clone(),
        None => return Json(Option::<Program>::None).into_response(),
    };
    match program_repository::get_tonight_program_by_channel_id(channel_id).await {
        Ok(program) => Json(program).into_response(),
        Err(e) => {
            eprintln!("Failed to get tonight program: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Option::<Program>::None),
            )
                .into_response()
        }
    }
}

pub async fn search_programs(Json(payload): Json<HashMap<String, String>>) -> impl IntoResponse {
    let query = match payload.get("query") {
        Some(q) => q.clone(),
        None => return Json(Vec::<Program>::new()).into_response(),
    };
    if query.trim().is_empty() {
        return Json(Vec::<Program>::new()).into_response();
    }
    match program_repository::search_programs(query).await {
        Ok(programs) => Json(programs).into_response(),
        Err(e) => {
            eprintln!("Failed to search programs: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Vec::<Program>::new()),
            )
                .into_response()
        }
    }
}
