use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "programs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub channel_id: String,
    pub start_time: DateTime,
    pub end_time: DateTime,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub categories: Option<String>,
    pub icon: Option<String>,
    pub episode_num: Option<String>,
    pub rating_system: Option<String>,
    pub rating_value: Option<String>,
    pub rating_icon: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
