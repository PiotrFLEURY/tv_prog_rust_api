use crate::data::converters::program_converter;
use crate::data::sources::db::schema::SCHEMA_CREATION_QUERY;
use crate::data::sources::db::sql_queries::{DELETE_CHANNELS_QUERY, DELETE_PACKAGES_QUERY, DELETE_PROGRAMS_QUERY, FIND_CURRENT_PROGRAM_BY_CHANNEL_ID_QUERY, FIND_PROGRAMS_BY_CHANNEL_ID_QUERY, FIND_TONIGHT_PROGRAM_BY_CHANNEL_ID_QUERY, INSERT_CHANNEL_QUERY, INSERT_PACKAGE_QUERY, SELECT_ALL_CHANNELS_QUERY, SELECT_CHANNELS_QUERY};
use crate::domain::entities::channel::Channel;
use crate::domain::entities::program::Program;
use chrono::Timelike;
use dotenv::var;
use postgres::{Client, NoTls};

///
/// Get a database connection
///
pub fn client() -> Client {
    let connection_string =
        var("CONNECTION_STRING").expect("DATABASE_URL must be set in the environment variables");
    Client::connect(connection_string.as_str(), NoTls).unwrap()
}

pub async fn init_schema() {
    std::thread::spawn(move || {
        println!("Initializing database schema...");
        let mut client = client();
        client.batch_execute(SCHEMA_CREATION_QUERY).unwrap();
        println!("Database schema initialized.");
    })
    .join()
    .unwrap();
}

pub fn drop_channels() {
    println!("Dropping all channels from the database...");
    std::thread::spawn(move || {
        let mut client = client();
        client.execute(DELETE_PACKAGES_QUERY, &[]).unwrap();
        client.execute(DELETE_CHANNELS_QUERY, &[]).unwrap();
    })
    .join()
    .unwrap();
}

pub fn save_channels(channels: Vec<Channel>) {
    std::thread::spawn(move || {
        let mut client = client();
        for channel in &channels {
            client
                .execute(
                    INSERT_CHANNEL_QUERY,
                    &[&channel.channel_id, &channel.name, &channel.icon_url],
                )
                .unwrap();
        }
    })
    .join()
    .unwrap();
}

pub fn save_channel_packages(channels: Vec<Channel>, package: String) {
    std::thread::spawn(move || {
        let mut client = client();
        for channel in channels {
            println!("Inserting channel package for channel_id: {}", channel.channel_id);
            client
                .execute(INSERT_PACKAGE_QUERY, &[&channel.channel_id, &package])
                .unwrap();
        }
    })
        .join()
        .unwrap();
}


pub(crate) fn find_all_channels() -> Vec<Channel> {
    std::thread::spawn(move || {
        let mut channels = Vec::new();
        for row in client().query(SELECT_ALL_CHANNELS_QUERY, &[]).unwrap() {
            let channel = Channel {
                id: row.get(0),
                channel_id: row.get(1),
                name: row.get(2),
                icon_url: row.get(3),
            };
            channels.push(channel);
        }
        channels
    })
        .join()
        .unwrap()
}

pub(crate) fn find_channels_by_package(package: String) -> Vec<Channel> {
    std::thread::spawn(move || {
        let mut channels = Vec::new();
        for row in client().query(SELECT_CHANNELS_QUERY, &[&package]).unwrap() {
            let channel = Channel {
                id: row.get(0),
                channel_id: row.get(1),
                name: row.get(2),
                icon_url: row.get(3),
            };
            channels.push(channel);
        }
        channels
    })
    .join()
    .unwrap()
}

pub fn drop_programs() {
    println!("Dropping all programs from the database...");
    std::thread::spawn(move || {
        let mut client = client();
        client.execute(DELETE_PROGRAMS_QUERY, &[]).unwrap();
    })
    .join()
    .unwrap();
}

pub fn bulk_insert_programs(programs: Vec<Program>) {
    println!(
        "Bulk inserting {} programs to the database...",
        programs.len()
    );
    std::thread::spawn(move || {
        let mut client = client();
        let insert_query = "INSERT INTO PROGRAMS (\
CHANNEL_ID, START_TIME, END_TIME, TITLE, SUBTITLE, DESCRIPTION, CATEGORIES, \
ICON, EPISODE_NUM, RATING_SYSTEM, RATING_VALUE, RATING_ICON) \
VALUES ";
        let values: Vec<String> = programs
            .iter()
            .map(|program| {
                let rating = program.rating.as_ref().unwrap();
                format!(
                    "('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}')",
                    program.channel_id,
                    program.start_time.naive_utc(),
                    program.end_time.naive_utc(),
                    escape_string(&program.title),
                    escape_string(program.sub_title.as_deref().unwrap_or("")),
                    escape_string(program.description.as_deref().unwrap_or("")),
                    escape_string(program.categories.as_deref().unwrap_or(&vec![]).join(",").as_str()),
                    escape_string(program.icon_url.as_deref().unwrap_or("")),
                    escape_string(program.episode_num.as_deref().unwrap_or("")),
                    escape_string(rating.system.as_ref().unwrap_or(&"".to_string()).as_str()),
                    escape_string(rating.value.as_ref().unwrap_or(&"".to_string()).as_str()),
                    escape_string(rating.icon.as_ref().unwrap_or(&"".to_string()).as_str())
                )
            })
            .collect();
        let full_query = format!("{} {}", insert_query, values.join(", "));
        //println!("Executing bulk insert query: {}", full_query);
        client.batch_execute(&full_query).unwrap();
        println!("Bulk insert completed.");
    })
    .join()
    .unwrap();
}

pub fn escape_string(input: &str) -> String {
    input.replace('\'', "''")
}

pub(crate) fn find_programs_by_channel_id(channel_id: String) -> Vec<Program> {
    std::thread::spawn(move || {
        let mut programs = Vec::new();
        let mut client = client();
        let rows = client
            .query(FIND_PROGRAMS_BY_CHANNEL_ID_QUERY, &[&channel_id])
            .unwrap();
        for row in rows {
            let program = program_converter::row_to_entity(&row);
            programs.push(program);
        }
        programs
    })
    .join()
    .unwrap()
}

pub(crate) fn find_current_program_by_channel_id(channel_id: String) -> Program {
    std::thread::spawn(move || {
        let mut client = client();
        let row = client
            .query_one(FIND_CURRENT_PROGRAM_BY_CHANNEL_ID_QUERY, &[&channel_id])
            .unwrap();
        let program = program_converter::row_to_entity(&row);
        program
    })
    .join()
    .unwrap()
}

pub(crate) fn find_tonight_program_by_channel_id(channel_id: String) -> Program {
    std::thread::spawn(move || {
        let mut client = client();
        // Tonight 20:30
        let target_time = chrono::Local::now()
            .with_hour(20)
            .and_then(|dt| dt.with_minute(30))
            .unwrap_or_else(|| chrono::Local::now());
        let row = client
            .query_one(
                FIND_TONIGHT_PROGRAM_BY_CHANNEL_ID_QUERY,
                &[&channel_id, &target_time.naive_utc()],
            )
            .unwrap();
        let program = program_converter::row_to_entity(&row);
        program
    })
    .join()
    .unwrap()
}