use crate::data::models::Channel as ChannelModel;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_to_entity() {
        // GIVEN
        let model = ChannelModel {
            id: "channel123".to_string(),
            display_name: crate::data::models::DisplayName {
                content: "Test Channel".to_string(),
            },
            icon: Some(crate::data::models::Icon {
                src: "http://example.com/icon.png".to_string(),
            }),
        };

        // WHEN
        let entity = model_to_entity(model);

        // THEN
        assert_eq!(&entity.channel_id, "channel123");
        assert_eq!(&entity.name, "Test Channel");
        assert_eq!(&entity.icon_url, "http://example.com/icon.png");
    }

    #[test]
    fn test_models_to_entities() {
        // GIVEN
        let models = vec![
            ChannelModel {
                id: "channel1".to_string(),
                display_name: crate::data::models::DisplayName {
                    content: "Channel One".to_string(),
                },
                icon: Some(crate::data::models::Icon {
                    src: "http://example.com/icon1.png".to_string(),
                }),
            },
            ChannelModel {
                id: "channel2".to_string(),
                display_name: crate::data::models::DisplayName {
                    content: "Channel Two".to_string(),
                },
                icon: None,
            },
        ];

        // WHEN
        let entities = models_to_entities(models);

        // THEN
        assert_eq!(entities.len(), 2);
        assert_eq!(&entities[0].channel_id, "channel1");
        assert_eq!(&entities[0].name, "Channel One");
        assert_eq!(&entities[0].icon_url, "http://example.com/icon1.png");
        assert_eq!(&entities[1].channel_id, "channel2");
        assert_eq!(&entities[1].name, "Channel Two");
        assert_eq!(&entities[1].icon_url, "");
    }
}
