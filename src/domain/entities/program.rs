use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use crate::domain::entities::rating::Rating;

#[derive(Deserialize, Serialize, Clone)]
pub struct Program {
    /// The id of the program
    pub id: i32,

    /// The unique identifier for the channel
    #[serde(rename = "channelId")]
    pub channel_id: String,

    /// The start time of the program
    #[serde(rename = "startTime")]
    pub start_time: DateTime<FixedOffset>,

    /// The end time of the program
    #[serde(rename = "endTime")]
    pub end_time: DateTime<FixedOffset>,

    /// The title of the program
    pub title: String,

    /// The subtitle of the program
    #[serde(rename = "subTitle")]
    pub sub_title: Option<String>,

    /// The description of the program
    pub description: Option<String>,

    /// The categories of the program
    pub categories: Option<Vec<String>>,

    /// The URL of the program's icon
    #[serde(rename = "iconUrl")]
    pub icon_url: Option<String>,

    /// The episode number of the program
    #[serde(rename = "episodeNum")]
    pub episode_num: Option<String>,

    /// The rating of the program
    pub rating: Option<Rating>,

}
