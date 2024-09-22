mod album;
mod music_processor;
mod web_server;

use std::error::Error;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let music_dir = "/home/san/dl/Telegram Desktop"; // specify your folder path here

    // Process music files
    music_processor::process_music_files(music_dir)?;

    // Start the web server
    web_server::start_server().await?;

    Ok(())
}
