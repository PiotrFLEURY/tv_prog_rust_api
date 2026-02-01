use crate::data::models::Program as ProgramModel;
use crate::domain::entities::program::Program as ProgramEntity;
use crate::domain::entities::rating::Rating;
use chrono::DateTime;

pub fn models_to_entities(models: Vec<ProgramModel>) -> Vec<ProgramEntity> {
    models.into_iter().map(model_to_entity).collect()
}

pub fn model_to_entity(model: ProgramModel) -> ProgramEntity {
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

    ProgramEntity {
        id: 0,
        channel_id: model.channel,
        start_time: DateTime::parse_from_str(model.start.as_str(), "%Y%m%d%H%M%S %z")
            .expect("Failed to parse start time"),
        end_time: DateTime::parse_from_str(model.stop.as_str(), "%Y%m%d%H%M%S %z")
            .expect("Failed to parse end time"),
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
    }
}

pub fn row_to_entity(row: &postgres::Row) -> ProgramEntity {
    let row_categories: Option<String> = row.get(7);
    let categories: Option<Vec<String>> =
        row_categories.map(|c| c.split(',').map(|s| s.trim().to_string()).collect());
    ProgramEntity {
        id: row.get(0),
        channel_id: row.get(1),
        start_time: DateTime::from_naive_utc_and_offset(
            row.get(2),
            chrono::FixedOffset::east_opt(0).unwrap(),
        ),
        end_time: DateTime::from_naive_utc_and_offset(
            row.get(3),
            chrono::FixedOffset::east_opt(0).unwrap(),
        ),
        title: row.get(4),
        sub_title: row.get(5),
        description: row.get(6),
        categories,
        icon_url: row.get(8),
        episode_num: row.get(9),
        rating: Some(Rating {
            system: row.get(10),
            value: row.get(11),
            icon: row.get(12),
        }),
    }
}
