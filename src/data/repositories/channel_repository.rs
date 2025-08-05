use crate::data::sources::db::postgres_client as postgres_client;
use crate::domain::entities::channel::Channel;

pub fn get_channels_by_package(package: String) -> Vec<Channel> {
    postgres_client::find_channels_by_package(package)
}
