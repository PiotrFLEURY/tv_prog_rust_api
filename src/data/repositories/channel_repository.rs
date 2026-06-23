use crate::data::sources::db::postgres_client;
use crate::domain::entities::channel::Channel;

pub async fn get_channels_by_package(package: String) -> Result<Vec<Channel>, String> {
    if package == "ALL" {
        return postgres_client::find_all_channels().await;
    }
    postgres_client::find_channels_by_package(package).await
}
