use crate::presentation::handlers::channels_handler::get_channels_by_package;
use crate::presentation::handlers::programs_handler::{
    get_current_program_by_channel_id, get_programs_by_channel_id,
    get_tonight_program_by_channel_id, search_programs,
};
use axum::routing::post;
use axum::{Router, routing::get};

pub fn create_router() -> Router {
    Router::new()
        .route("/channels/{package}", get(get_channels_by_package))
        .route("/programs", get(get_programs_by_channel_id))
        .route("/programs/current", get(get_current_program_by_channel_id))
        .route("/programs/tonight", get(get_tonight_program_by_channel_id))
        .route("/programs/search", post(search_programs))
        .fallback(get(|| async { "Not Found" }))
}
