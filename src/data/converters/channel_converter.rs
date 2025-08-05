use crate::data::models::xmltv::Channel as ChannelModel;
use crate::domain::entities::channel::Channel as ChannelEntity;

pub fn model_to_entity(model: ChannelModel) -> ChannelEntity {
    ChannelEntity {
        id: 0,
        channel_id: model.id,
        name: model.display_name.content,
        icon_url: model.icon.map_or("".to_string(), |icon| icon.src),
    }
}

pub fn models_to_entities(models: Vec<ChannelModel>) -> Vec<ChannelEntity> {
    models.into_iter().map(model_to_entity).collect()
}