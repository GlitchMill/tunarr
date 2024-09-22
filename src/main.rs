mod album;
mod music_processor;
mod web_server;
use std::env;
use dotenv::dotenv;
use std::error::Error;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Start the web server
    web_server::start_server().await?;

    Ok(())
}

