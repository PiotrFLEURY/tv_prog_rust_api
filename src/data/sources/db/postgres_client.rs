use crate::data::converters::{channel_converter, program_converter};
use crate::data::sources::db::connection;
use crate::data::sources::db::entities::{channel_packages, channels, programs};
use crate::data::sources::db::schema::SCHEMA_CREATION_QUERY;
use crate::domain::entities::channel::Channel;
use crate::domain::entities::program::Program;
use chrono::{Duration, Timelike, Utc};
use sea_orm::sea_query::Expr;
use sea_orm::sea_query::extension::postgres::PgExpr;
use sea_orm::{
    ColumnTrait, Condition, ConnectionTrait, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};

const INSERT_CHUNK_SIZE: usize = 1000;

pub async fn init_schema() -> Result<(), String> {
    println!("Initializing database schema...");
    let db = connection::get().await?;
    db.execute_unprepared(SCHEMA_CREATION_QUERY)
        .await
        .map_err(|e| e.to_string())?;
    println!("Database schema initialized.");
    Ok(())
}

pub async fn drop_channels() -> Result<(), String> {
    println!("Dropping all channels from the database...");
    let db = connection::get().await?;
    channel_packages::Entity::delete_many()
        .exec(db)
        .await
        .map_err(|e| e.to_string())?;
    channels::Entity::delete_many()
        .exec(db)
        .await
        .map_err(|e| e.to_string())?;
    println!("All channels dropped from the database.");
    Ok(())
}

pub async fn save_channels(channels_to_save: Vec<Channel>) -> Result<(), String> {
    if channels_to_save.is_empty() {
        return Ok(());
    }
    let db = connection::get().await?;
    let models: Vec<channels::ActiveModel> = channels_to_save
        .iter()
        .map(|channel| {
            println!("Inserting channel: {}", channel.channel_id);
            channels::ActiveModel {
                channel_id: Set(channel.channel_id.clone()),
                display_name: Set(channel.name.clone()),
                icon: Set(Some(channel.icon_url.clone())),
                ..Default::default()
            }
        })
        .collect();
    for chunk in models.chunks(INSERT_CHUNK_SIZE) {
        channels::Entity::insert_many(chunk.to_vec())
            .exec(db)
            .await
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub async fn save_channel_packages(
    channels_to_save: Vec<Channel>,
    package: String,
) -> Result<(), String> {
    if channels_to_save.is_empty() {
        return Ok(());
    }
    let db = connection::get().await?;
    let models: Vec<channel_packages::ActiveModel> = channels_to_save
        .iter()
        .map(|channel| {
            println!(
                "Inserting channel package for channel_id: {}",
                channel.channel_id
            );
            channel_packages::ActiveModel {
                channel_id: Set(channel.channel_id.clone()),
                package_id: Set(package.clone()),
                ..Default::default()
            }
        })
        .collect();
    for chunk in models.chunks(INSERT_CHUNK_SIZE) {
        channel_packages::Entity::insert_many(chunk.to_vec())
            .exec(db)
            .await
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub async fn find_all_channels() -> Result<Vec<Channel>, String> {
    let db = connection::get().await?;
    let models = channels::Entity::find()
        .all(db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(models
        .into_iter()
        .map(channel_converter::db_model_to_entity)
        .collect())
}

pub async fn find_channels_by_package(package: String) -> Result<Vec<Channel>, String> {
    let db = connection::get().await?;
    let channel_ids: Vec<String> = channel_packages::Entity::find()
        .filter(channel_packages::Column::PackageId.eq(package))
        .all(db)
        .await
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(|package| package.channel_id)
        .collect();
    let models = channels::Entity::find()
        .filter(channels::Column::ChannelId.is_in(channel_ids))
        .all(db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(models
        .into_iter()
        .map(channel_converter::db_model_to_entity)
        .collect())
}

pub async fn drop_programs() -> Result<(), String> {
    println!("Dropping all programs from the database...");
    let db = connection::get().await?;
    programs::Entity::delete_many()
        .exec(db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub async fn bulk_insert_programs(programs_to_save: Vec<Program>) -> Result<(), String> {
    if programs_to_save.is_empty() {
        return Ok(());
    }
    println!(
        "Bulk inserting {} programs to the database...",
        programs_to_save.len()
    );
    let db = connection::get().await?;
    let models: Vec<programs::ActiveModel> = programs_to_save
        .iter()
        .map(program_converter::entity_to_active_model)
        .collect();
    for chunk in models.chunks(INSERT_CHUNK_SIZE) {
        programs::Entity::insert_many(chunk.to_vec())
            .exec(db)
            .await
            .map_err(|e| e.to_string())?;
    }
    println!("Bulk insert completed.");
    Ok(())
}

pub async fn find_programs_by_channel_id(channel_id: &str) -> Result<Vec<Program>, String> {
    let db = connection::get().await?;
    let now = Utc::now().naive_utc();
    let models = programs::Entity::find()
        .filter(programs::Column::ChannelId.eq(channel_id))
        .filter(programs::Column::StartTime.gte(now))
        .order_by_asc(programs::Column::StartTime)
        .limit(100)
        .all(db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(models
        .into_iter()
        .map(program_converter::db_model_to_entity)
        .collect())
}

pub async fn find_current_program_by_channel_id(
    channel_id: &str,
) -> Result<Option<Program>, String> {
    let db = connection::get().await?;
    let now = Utc::now().naive_utc();
    let model = programs::Entity::find()
        .filter(programs::Column::ChannelId.eq(channel_id))
        .filter(programs::Column::StartTime.lte(now))
        .filter(programs::Column::EndTime.gte(now))
        .one(db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(model.map(program_converter::db_model_to_entity))
}

pub async fn find_tonight_program_by_channel_id(
    channel_id: &str,
) -> Result<Option<Program>, String> {
    let db = connection::get().await?;
    // Tonight 20:30
    let target_time = chrono::Local::now()
        .with_hour(20)
        .and_then(|dt| dt.with_minute(30))
        .unwrap_or_else(chrono::Local::now)
        .naive_utc();
    let candidates = programs::Entity::find()
        .filter(programs::Column::ChannelId.eq(channel_id))
        .filter(programs::Column::StartTime.gte(target_time))
        .order_by_asc(programs::Column::StartTime)
        .limit(50)
        .all(db)
        .await
        .map_err(|e| e.to_string())?;
    // duration is at least 30 minutes
    let program = candidates
        .into_iter()
        .find(|model| model.end_time - model.start_time >= Duration::minutes(30));
    Ok(program.map(program_converter::db_model_to_entity))
}

pub async fn search_programs(query_string: String) -> Result<Vec<Program>, String> {
    if !query_valid(&query_string) {
        println!("Invalid query string: {}", query_string);
        return Ok(vec![]);
    }
    let db = connection::get().await?;
    let pattern = format!("%{}%", query_string);
    let models = programs::Entity::find()
        .filter(
            Condition::any()
                .add(Expr::col(programs::Column::Title).ilike(pattern.clone()))
                .add(Expr::col(programs::Column::Subtitle).ilike(pattern.clone()))
                .add(Expr::col(programs::Column::Description).ilike(pattern)),
        )
        .all(db)
        .await
        .map_err(|e| e.to_string())?;
    Ok(models
        .into_iter()
        .map(program_converter::db_model_to_entity)
        .collect())
}

fn query_valid(query: &str) -> bool {
    // The only allowed characers a lower case letters (ASCII 97-122)
    query
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_whitespace())
}
