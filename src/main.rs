mod album;
mod music_processor;
mod web_server;
use std::env;
use dotenv::dotenv;
use std::error::Error;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let music_dir = env::var("MUSIC_DIR").expect("MUSIC_DIR not set in .env");

    // Process music files
    music_processor::process_music_files(&music_dir)?;

    // Start the web server
    web_server::start_server().await?;

    Ok(())
}

