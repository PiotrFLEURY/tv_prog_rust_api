use crate::data::sources::db::postgres_client;
use crate::domain::entities::program::Program;

pub fn get_programs_by_channel_id(channel_id: String) -> Vec<Program> {
    postgres_client::find_programs_by_channel_id(channel_id)
}

pub(crate) fn get_current_program_by_channel_id(channel_id: String) -> Program {
    postgres_client::find_current_program_by_channel_id(channel_id)
}

pub(crate) fn get_tonight_program_by_channel_id(channel_id: String) -> Program {
    postgres_client::find_tonight_program_by_channel_id(channel_id)
}