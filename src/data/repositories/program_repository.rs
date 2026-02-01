use crate::data::sources::db::postgres_client;
use crate::domain::entities::program::Program;
use crate::presentation::dtos::Page;

pub fn get_programs_by_channel_id(channel_id: String) -> Page<Program> {
    let programs = postgres_client::find_programs_by_channel_id(&channel_id);
    Page { content: programs }
}

pub(crate) fn get_current_program_by_channel_id(channel_id: String) -> Program {
    postgres_client::find_current_program_by_channel_id(&channel_id)
}

pub(crate) fn get_tonight_program_by_channel_id(channel_id: String) -> Program {
    postgres_client::find_tonight_program_by_channel_id(&channel_id)
}

pub(crate) fn search_programs(query: String) -> Vec<Program> {
    postgres_client::search_programs(query)
}
