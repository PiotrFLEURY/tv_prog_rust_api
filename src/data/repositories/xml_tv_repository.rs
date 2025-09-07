use crate::data::converters::{channel_converter, program_converter};
use crate::data::sources::api::xmltv_client;
use crate::data::sources::db::postgres_client;
use crate::domain::entities::channel::Channel;
use crate::domain::entities::program::Program;

pub async fn init_xml_tv_data() {
    // Initialize the database connection or any other setup if needed
    println!("Initializing xml tv data...");

    std::thread::spawn(move || async {
        let start_time = std::time::Instant::now();
        println!("Fetching XML TV data from...");
        let result = xmltv_client::fetch_xmltv_all().await;
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
        postgres_client::save_channel_packages(channels.clone(), "ALL".to_string());
        println!("Channels saved to the database.");


        let mut inserted_programs = 0;
        let program_chunks = programs.chunks(10000);
        for chunk in program_chunks {
            postgres_client::bulk_insert_programs(chunk.to_vec());
            inserted_programs += chunk.len();
            println!("Inserted {} programs of {} into the database.", inserted_programs, programs.len());
        }
        println!("Programs saved to the database.");



        let existing_channel_ids = channels.into_iter()
            .map(|c| c.channel_id.clone())
            .collect::<Vec<String>>();

        let fr = xmltv_client::fetch_xmltv_fr().await;
        let fr_channels: Vec<Channel> = channel_converter::models_to_entities(fr.channels);
        println!("Found {} FR channels", fr_channels.len());
        let known_fr_channels = fr_channels.iter()
            .filter(|c| existing_channel_ids.contains(&c.channel_id))
            .cloned()
            .collect::<Vec<Channel>>();
        println!("Known FR channels: {}", known_fr_channels.len());
        postgres_client::save_channel_packages(known_fr_channels, "FR".to_string());
        let unknown_fr_channels = fr_channels.iter()
            .filter(|c| !existing_channel_ids.contains(&c.channel_id))
            .cloned()
            .collect::<Vec<Channel>>();
        println!("Unknown FR channels: {}", unknown_fr_channels.len());
        postgres_client::save_channels(unknown_fr_channels.clone());
        postgres_client::save_channel_packages(unknown_fr_channels, "FR".to_string());
        let unknown_fr_programs = fr.programs.into_iter()
            .filter(|p| !existing_channel_ids.contains(&p.channel))
            .map(|p| {
                program_converter::model_to_entity(p)
            })
            .collect::<Vec<Program>>();
        if unknown_fr_programs.len()>0 {
            println!("Found {} unknown FR channels", unknown_fr_programs.len());
            postgres_client::bulk_insert_programs(unknown_fr_programs);
        } else {
            println!("No unknown FR channels found");
        }
        println!("FR channels saved to the database.");

        let tnt = xmltv_client::fetch_xmltv_tnt().await;
        let tnt_channels: Vec<Channel> = channel_converter::models_to_entities(tnt.channels);
        println!("Found {} TNT channels", tnt_channels.len());
        let known_tnt_channels = tnt_channels.iter()
            .filter(|c| existing_channel_ids.contains(&c.channel_id))
            .cloned()
            .collect::<Vec<Channel>>();
        println!("Known TNT channels: {}", known_tnt_channels.len());
        postgres_client::save_channel_packages(known_tnt_channels, "TNT".to_string());
        println!("TNT channels saved to the database.");

        let elapsed = start_time.elapsed();
        println!("Time taken to init database: {:.2?}", elapsed);
    }).join().unwrap().await;

    println!("XML TV data initialization complete.");
}