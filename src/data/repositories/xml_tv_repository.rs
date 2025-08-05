use crate::data::converters::{channel_converter, program_converter};
use crate::data::sources::api::xmltv_client;
use crate::data::sources::db::postgres_client;
use crate::domain::entities::channel::Channel;
use crate::domain::entities::program::Program;

pub async fn init_xml_tv_data() {
    // Initialize the database connection or any other setup if needed
    println!("Initializing xml tv data...");

    std::thread::spawn(move || async {
        println!("Fetching XML TV data from...");
        let result = xmltv_client::fetch_xmltv_fr().await;
        println!("Found {} channels", result.channels.len());
        println!("Found {} programs", result.programs.len());

        // Drop existing channels before inserting new ones
        postgres_client::drop_channels();
        println!("Existing channels dropped from the database.");

        // Drop existing programs before inserting new ones
        postgres_client::drop_programs();
        println!("Existing programs dropped from the database.");

        let channels: Vec<Channel> = channel_converter::models_to_entities(result.channels);
        let programs: Vec<Program> = program_converter::models_to_entities(result.programs);

        postgres_client::save_channels(channels.clone());
        postgres_client::save_channel_packages(channels.clone(), "FR".to_string());
        println!("Channels saved to the database.");

        let start_time = std::time::Instant::now();
        let mut inserted_programs = 0;
        let program_chunks = programs.chunks(10000);
        for chunk in program_chunks {
            postgres_client::bulk_insert_programs(chunk.to_vec());
            inserted_programs += chunk.len();
            println!("Inserted {} programs of {} into the database.", inserted_programs, programs.len());
        }
        //postgres_client::bulk_insert_programs(programs);
        println!("Programs saved to the database.");
        let elapsed = start_time.elapsed();
        println!("Time taken to insert programs: {:.2?}", elapsed);

        let tnt = xmltv_client::fetch_xmltv_tnt().await;
        let tnt_channels: Vec<Channel> = channel_converter::models_to_entities(tnt.channels);
        println!("Found {} TNT channels", tnt_channels.len());
        postgres_client::save_channel_packages(tnt_channels, "TNT".to_string());
        println!("TNT channels saved to the database.");

    }).join().unwrap().await;

    println!("XML TV data initialization complete.");
}