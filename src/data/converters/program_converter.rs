use crate::data::models::Program as ProgramModel;
use crate::data::sources::db::entities::programs;
use crate::domain::entities::program::Program as ProgramEntity;
use crate::domain::entities::rating::Rating;
use chrono::DateTime;
use sea_orm::Set;

pub fn models_to_entities(models: Vec<ProgramModel>) -> Result<Vec<ProgramEntity>, String> {
    models.into_iter().map(model_to_entity).collect()
}

pub fn model_to_entity(model: ProgramModel) -> Result<ProgramEntity, String> {
    let icon_url = if let Some(icon) = model.icon {
        icon.get(0).map_or(String::new(), |ic| ic.src.clone())
    } else {
        String::new()
    };

    let categories = if let Some(cats) = &model.categories {
        cats.iter()
            .map(|category| category.content.clone().unwrap_or_else(String::new))
            .collect::<Vec<_>>()
            .join(", ")
    } else {
        String::new()
    };

    let episode_num = if let Some(episode) = &model.episode_number {
        episode.content.clone()
    } else {
        Option::from(String::new())
    };

    let rating_value = if let Some(value) = model.rating.as_ref().and_then(|r| r.value.as_ref()) {
        value.value.clone()
    } else {
        Option::from(String::new())
    };

    let rating_icon = if let Some(icon) = model.rating.as_ref().and_then(|r| r.icon.as_ref()) {
        icon.src.clone()
    } else {
        String::new()
    };

    let rating_system = if let Some(system) = model.rating.as_ref().map(|r| r.system.clone()) {
        system
    } else {
        String::new()
    };

    let start_time = DateTime::parse_from_str(model.start.as_str(), "%Y%m%d%H%M%S %z")
        .map_err(|e| format!("Failed to parse start time '{}': {}", model.start, e))?;
    let end_time = DateTime::parse_from_str(model.stop.as_str(), "%Y%m%d%H%M%S %z")
        .map_err(|e| format!("Failed to parse end time '{}': {}", model.stop, e))?;

    Ok(ProgramEntity {
        id: 0,
        channel_id: model.channel,
        start_time,
        end_time,
        title: model.title,
        sub_title: model.sub_title.and_then(|subs| subs.get(0).cloned()), // Take the first subtitle if exists
        description: model.description.and_then(|desc| desc.content),
        categories: Option::from(
            categories
                .split(",")
                .map(|c| c.trim().to_string())
                .collect::<Vec<String>>(),
        ),
        icon_url: Option::from(icon_url),
        episode_num,
        rating: Option::from(Rating {
            system: Option::from(rating_system.clone()),
            value: Option::from(rating_value.clone()),
            icon: Option::from(rating_icon.clone()),
        }),
    })
}

///
/// Convert a SeaORM program model loaded from the database into a domain entity
///
pub fn db_model_to_entity(model: programs::Model) -> ProgramEntity {
    let categories = model
        .categories
        .map(|c| c.split(',').map(|s| s.trim().to_string()).collect());
    ProgramEntity {
        id: model.id,
        channel_id: model.channel_id,
        start_time: from_naive_utc_and_offset(model.start_time),
        end_time: from_naive_utc_and_offset(model.end_time),
        title: model.title,
        sub_title: model.subtitle,
        description: model.description,
        categories,
        icon_url: model.icon,
        episode_num: model.episode_num,
        rating: Some(Rating {
            system: model.rating_system,
            value: model.rating_value,
            icon: model.rating_icon,
        }),
    }
}

///
/// Convert a domain entity into a SeaORM active model ready to be inserted
///
pub fn entity_to_active_model(program: &ProgramEntity) -> programs::ActiveModel {
    let rating = program.rating.clone().unwrap_or(Rating {
        system: None,
        value: None,
        icon: None,
    });
    programs::ActiveModel {
        channel_id: Set(program.channel_id.clone()),
        start_time: Set(program.start_time.naive_utc()),
        end_time: Set(program.end_time.naive_utc()),
        title: Set(program.title.clone()),
        subtitle: Set(program.sub_title.clone()),
        description: Set(program.description.clone()),
        categories: Set(program.categories.as_ref().map(|c| c.join(","))),
        icon: Set(program.icon_url.clone()),
        episode_num: Set(program.episode_num.clone()),
        rating_system: Set(rating.system),
        rating_value: Set(rating.value),
        rating_icon: Set(rating.icon),
        ..Default::default()
    }
}

fn from_naive_utc_and_offset(naive: chrono::NaiveDateTime) -> DateTime<chrono::FixedOffset> {
    let offset = chrono::FixedOffset::east_opt(0).expect("Cannot create offset");
    DateTime::from_naive_utc_and_offset(naive, offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_to_entity() {
        // GIVEN
        let model = ProgramModel {
            channel: "channel123".to_string(),
            start: "20240101080000 +0000".to_string(),
            stop: "20240101090000 +0000".to_string(),
            title: "Test Program".to_string(),
            sub_title: Some(vec!["Episode 1".to_string()]),
            description: Some(crate::data::models::Description {
                lang: "en".to_string(),
                content: Some("This is a test program.".to_string()),
            }),
            categories: Some(vec![crate::data::models::Category {
                lang: "en".to_string(),
                content: Some("Drama".to_string()),
            }]),
            icon: Some(vec![crate::data::models::Icon {
                src: "http://example.com/icon.png".to_string(),
            }]),
            episode_number: Some(crate::data::models::EpisodeNumber {
                system: "EP_SYSTEM".to_string(),
                content: Some("S01E01".to_string()),
            }),
            rating: Some(crate::data::models::Rating {
                system: "MPAA".to_string(),
                value: Some(crate::data::models::RatingValue {
                    value: Some("PG-13".to_string()),
                }),
                icon: Some(crate::data::models::Icon {
                    src: "http://example.com/rating_icon.png".to_string(),
                }),
            }),
        };

        // WHEN
        let entity = model_to_entity(model).unwrap();

        // THEN
        assert_eq!(&entity.channel_id, "channel123");
        assert_eq!(entity.start_time.to_rfc3339(), "2024-01-01T08:00:00+00:00");
        assert_eq!(entity.end_time.to_rfc3339(), "2024-01-01T09:00:00+00:00");
        assert_eq!(&entity.title, "Test Program");
        assert_eq!(entity.sub_title.as_ref().unwrap(), "Episode 1");
        assert_eq!(
            entity.description.as_ref().unwrap(),
            "This is a test program."
        );
        assert_eq!(
            entity.categories.as_ref().unwrap(),
            &vec!["Drama".to_string()]
        );
        assert_eq!(
            entity.icon_url.as_ref().unwrap(),
            "http://example.com/icon.png"
        );
        assert_eq!(entity.episode_num.as_ref().unwrap(), "S01E01");
        let rating = entity.rating.as_ref().unwrap();
        assert_eq!(rating.system.as_ref().unwrap(), "MPAA");
        assert_eq!(rating.value.as_ref().unwrap(), "PG-13");
        assert_eq!(
            rating.icon.as_ref().unwrap(),
            "http://example.com/rating_icon.png"
        );
    }
}
