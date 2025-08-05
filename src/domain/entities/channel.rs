use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Channel {
    /// The id of the channel
    pub id: i32,

    /// The unique identifier for the channel
    #[serde(rename = "channelId")]
    pub channel_id: String,

    /// The name of the channel
    pub name: String,

    /// The URL of the channel's icon
    pub icon_url: String,
}
