use crate::data::converters::program_converter;
use crate::data::sources::db::schema::SCHEMA_CREATION_QUERY;
use crate::data::sources::db::sql_queries::{
    DELETE_CHANNELS_QUERY, DELETE_PACKAGES_QUERY, DELETE_PROGRAMS_QUERY,
    FIND_CURRENT_PROGRAM_BY_CHANNEL_ID_QUERY, FIND_PROGRAMS_BY_CHANNEL_ID_QUERY,
    FIND_TONIGHT_PROGRAM_BY_CHANNEL_ID_QUERY, INSERT_CHANNEL_QUERY, INSERT_PACKAGE_QUERY,
    SELECT_ALL_CHANNELS_QUERY, SELECT_CHANNELS_QUERY,
};
use crate::domain::entities::channel::Channel;
use crate::domain::entities::program::Program;
use crate::domain::entities::rating::Rating;
use chrono::Timelike;
use dotenv::var;
use postgres::{Client, Error, NoTls};

///
/// Get a database connection
///
pub fn client() -> Client {
    let connection_string =
        var("CONNECTION_STRING").expect("DATABASE_URL must be set in the environment variables");
    Client::connect(connection_string.as_str(), NoTls).expect("Unable to connect to the database")
}

pub fn init_schema() {
    thread_exec(move || -> Result<(), Error> {
        println!("Initializing database schema...");
        let mut client = client();
        client.batch_execute(SCHEMA_CREATION_QUERY)?;
        Ok(())
    })
    .expect("Unable to initialize database schema");
    println!("Database schema initialized.");
}

fn thread_exec<F, R>(f: F) -> R
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    std::thread::spawn(move || f())
        .join()
        .expect("Unable to execute query")
}

pub fn drop_channels() {
    println!("Dropping all channels from the database...");
    thread_exec(|| -> Result<(), Error> {
        let mut client = client();
        client.execute(DELETE_PACKAGES_QUERY, &[])?;
        client.execute(DELETE_CHANNELS_QUERY, &[])?;
        Ok(())
    })
    .expect("Unable to drop channels from the database");
    println!("All channels dropped from the database.");
}

pub fn save_channels(channels: Vec<Channel>) {
    thread_exec(move || -> Result<(), Error> {
        let mut client = client();
        for channel in &channels {
            println!("Inserting channel: {}", channel.channel_id);
            client.execute(
                INSERT_CHANNEL_QUERY,
                &[&channel.channel_id, &channel.name, &channel.icon_url],
            )?;
        }
        Ok(())
    })
    .expect("Unable to save channels to the database");
}

pub fn save_channel_packages(channels: Vec<Channel>, package: String) {
    thread_exec(move || -> Result<(), Error> {
        let mut client = client();
        for channel in &channels {
            println!(
                "Inserting channel package for channel_id: {}",
                channel.channel_id
            );
            client.execute(INSERT_PACKAGE_QUERY, &[&channel.channel_id, &package])?;
        }
        Ok(())
    })
    .expect("Unable to save channel packages to the database");
}

pub fn find_all_channels() -> Vec<Channel> {
    thread_exec(move || -> Result<Vec<Channel>, Error> {
        let mut channels = Vec::new();
        let select = client().query(SELECT_ALL_CHANNELS_QUERY, &[])?;
        for row in select {
            let channel = Channel {
                id: row.get(0),
                channel_id: row.get(1),
                name: row.get(2),
                icon_url: row.get(3),
            };
            channels.push(channel);
        }
        Ok(channels)
    })
    .expect("Unable to find all channels")
}

pub fn find_channels_by_package(package: String) -> Vec<Channel> {
    thread_exec(move || -> Result<Vec<Channel>, Error> {
        let mut channels = Vec::new();
        let select = client().query(SELECT_CHANNELS_QUERY, &[&package])?;
        for row in select {
            let channel = Channel {
                id: row.get(0),
                channel_id: row.get(1),
                name: row.get(2),
                icon_url: row.get(3),
            };
            channels.push(channel);
        }
        Ok(channels)
    })
    .expect("Unable to find channels by package")
}

pub fn drop_programs() {
    println!("Dropping all programs from the database...");
    thread_exec(move || -> Result<(), Error> {
        let mut client = client();
        client.execute(DELETE_PROGRAMS_QUERY, &[])?;
        Ok(())
    })
    .expect("Unable to drop programs from the database");
}

pub fn bulk_insert_programs(programs: Vec<Program>) {
    println!(
        "Bulk inserting {} programs to the database...",
        programs.len()
    );
    thread_exec(move || -> Result<(), Error> {
        let mut client = client();
        let insert_query = "INSERT INTO PROGRAMS (\
CHANNEL_ID, START_TIME, END_TIME, TITLE, SUBTITLE, DESCRIPTION, CATEGORIES, \
ICON, EPISODE_NUM, RATING_SYSTEM, RATING_VALUE, RATING_ICON) \
VALUES ";
        let values: Vec<String> = programs
            .iter()
            .map(|program| {
                let rating = program.rating.as_ref().unwrap_or_else(|| &Rating {
                    icon: None,
                    system: None,
                    value: None,
                });
                format!(
                    "('{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}')",
                    program.channel_id,
                    program.start_time.naive_utc(),
                    program.end_time.naive_utc(),
                    escape_string(&program.title),
                    escape_string(program.sub_title.as_deref().unwrap_or("")),
                    escape_string(program.description.as_deref().unwrap_or("")),
                    escape_string(
                        program
                            .categories
                            .as_deref()
                            .unwrap_or(&vec![])
                            .join(",")
                            .as_str()
                    ),
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
        client.batch_execute(&full_query)?;
        println!("Bulk insert completed.");
        Ok(())
    })
    .expect("Unable to bulk insert programs to the database");
}

pub fn escape_string(input: &str) -> String {
    input.replace('\'', "''")
}

pub fn find_programs_by_channel_id(channel_id: &str) -> Vec<Program> {
    let channel = String::from(channel_id);
    thread_exec(move || -> Result<Vec<Program>, Error> {
        let mut programs = Vec::new();
        let mut client = client();
        let rows = client.query(FIND_PROGRAMS_BY_CHANNEL_ID_QUERY, &[&channel])?;
        for row in rows {
            let program = program_converter::row_to_entity(&row);
            programs.push(program);
        }
        Ok(programs)
    })
    .expect(&format!(
        "Cannot find programs for channel id {}",
        channel_id
    ))
}

pub fn find_current_program_by_channel_id(channel_id: &str) -> Program {
    let channel = String::from(channel_id);
    thread_exec(move || -> Result<Program, Error> {
        let mut client = client();
        let row = client.query_one(FIND_CURRENT_PROGRAM_BY_CHANNEL_ID_QUERY, &[&channel])?;
        let program = program_converter::row_to_entity(&row);
        Ok(program)
    })
    .expect(&format!(
        "Unable to find current program by channel id {}",
        channel_id
    ))
}

pub fn find_tonight_program_by_channel_id(channel_id: &str) -> Program {
    let channel = String::from(channel_id);
    thread_exec(move || -> Result<Program, Error> {
        let mut client = client();
        // Tonight 20:30
        let target_time = chrono::Local::now()
            .with_hour(20)
            .and_then(|dt| dt.with_minute(30))
            .unwrap_or_else(|| chrono::Local::now());
        let row = client.query_one(
            FIND_TONIGHT_PROGRAM_BY_CHANNEL_ID_QUERY,
            &[&channel, &target_time.naive_utc()],
        )?;
        let program = program_converter::row_to_entity(&row);
        Ok(program)
    })
    .expect(&format!(
        "Unable to find tonight program by channel id {}",
        channel_id
    ))
}

pub fn search_programs(query_string: String) -> Vec<Program> {
    if !query_valid(query_string.clone()) {
        println!("Invalid query string: {}", query_string);
        return vec![];
    }
    thread_exec(move || -> Result<Vec<Program>, Error> {
        let mut programs = Vec::new();
        let mut client = client();
        let query = format!("%{}%", query_string);
        let rows = client
            .query(
                "SELECT * FROM PROGRAMS WHERE TITLE ILIKE $1 OR SUBTITLE ILIKE $1 OR DESCRIPTION ILIKE $1",
                &[&query],
            )
            ?;
        for row in rows {
            let program = program_converter::row_to_entity(&row);
            programs.push(program);
        }
        Ok(programs)
    })
    .expect("Unable to search programs by query string")
}

fn query_valid(query: String) -> bool {
    // The only allowed characers a lower case letters (ASCII 97-122)
    for c in query.chars() {
        if !(c.is_ascii_lowercase() || c.is_ascii_whitespace()) {
            return false;
        }
    }
    true
}
