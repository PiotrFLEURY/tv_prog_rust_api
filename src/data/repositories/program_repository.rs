use crate::data::sources::db::postgres_client;
use crate::domain::entities::program::Program;
use crate::presentation::dtos::Page;

pub async fn get_programs_by_channel_id(channel_id: String) -> Result<Page<Program>, String> {
    let programs = postgres_client::find_programs_by_channel_id(&channel_id).await?;
    Ok(Page { content: programs })
}

pub async fn get_current_program_by_channel_id(
    channel_id: String,
) -> Result<Option<Program>, String> {
    postgres_client::find_current_program_by_channel_id(&channel_id).await
}

pub async fn get_tonight_program_by_channel_id(
    channel_id: String,
) -> Result<Option<Program>, String> {
    postgres_client::find_tonight_program_by_channel_id(&channel_id).await
}

pub async fn search_programs(query: String) -> Result<Vec<Program>, String> {
    postgres_client::search_programs(query).await
}
