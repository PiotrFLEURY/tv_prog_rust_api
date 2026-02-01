use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct XmlTv {
    #[serde(rename = "channel")]
    pub channels: Vec<Channel>,
    #[serde(rename = "programme")]
    pub programs: Vec<Program>,
}

#[derive(Serialize, Deserialize)]
pub struct Channel {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "display-name")]
    pub display_name: DisplayName,
    #[serde(rename = "icon")]
    pub icon: Option<Icon>,
}

#[derive(Serialize, Deserialize)]
pub struct DisplayName {
    #[serde(rename = "#text")]
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct Program {
    pub title: String,
    #[serde(rename = "sub-title")]
    pub sub_title: Option<Vec<String>>,
    #[serde(rename = "@start")]
    pub start: String,
    #[serde(rename = "@stop")]
    pub stop: String,
    #[serde(rename = "@channel")]
    pub channel: String,
    #[serde(rename = "desc")]
    pub description: Option<Description>,
    #[serde(rename = "category")]
    pub categories: Option<Vec<Category>>,
    #[serde(rename = "icon")]
    pub icon: Option<Vec<Icon>>,
    #[serde(rename = "episode-num")]
    pub episode_number: Option<EpisodeNumber>,
    #[serde(rename = "rating")]
    pub rating: Option<Rating>,
}

#[derive(Serialize, Deserialize)]
pub struct Description {
    #[serde(rename = "@lang")]
    pub lang: String,
    #[serde(rename = "#text")]
    pub content: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Category {
    #[serde(rename = "@lang")]
    pub lang: String,
    #[serde(rename = "#text")]
    pub content: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Icon {
    #[serde(rename = "@src")]
    pub src: String,
}

#[derive(Serialize, Deserialize)]
pub struct EpisodeNumber {
    #[serde(rename = "@system")]
    pub system: String,
    #[serde(rename = "#text")]
    pub content: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Rating {
    #[serde(rename = "@system")]
    pub system: String,
    #[serde(rename = "value")]
    pub value: Option<RatingValue>,
    #[serde(rename = "icon")]
    pub icon: Option<Icon>,
}

#[derive(Serialize, Deserialize)]
pub struct RatingValue {
    #[serde(rename = "#text")]
    pub value: Option<String>,
}
