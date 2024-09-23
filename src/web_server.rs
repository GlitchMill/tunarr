use actix_files as fs;
use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder};
use std::fs as std_fs;
use std::env;
use dotenv::dotenv;
use crate::album::Album;
use base64::engine::general_purpose;
use base64::Engine;
use id3::Tag; 
use lofty::probe::Probe; 
use lofty::file::TaggedFileExt;
use id3::TagLike;
use rand::seq::SliceRandom;
use std::io::Write;
use crate::web_server::std_fs::OpenOptions;
use walkdir::WalkDir;
use futures::stream;
use actix_web::web::Bytes;
use futures::StreamExt;

pub async fn start_server() -> std::io::Result<()> {
    dotenv().ok();
    let music_dir = env::var("MUSIC_DIR").expect("MUSIC_DIR not set in .env");

    HttpServer::new(move || {
        let music_dir_data = web::Data::new(music_dir.clone());

        App::new()
            .app_data(music_dir_data)
            .service(fs::Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(home))
            .route("/albums", web::get().to(get_albums)) 
            .route("/settings", web::get().to(settings))
            .service(save_path) // Register using `.service` with attribute macro
    })
    .bind("127.0.0.1:8280")?
    .run()
    .await
}

#[post("/save-path")]
async fn save_path(path: web::Json<String>) -> HttpResponse {
    let new_path = path.into_inner();
    
    // Read the .env file
    let env_file_path = ".env";  // Assuming the .env file is in the root directory
    let mut env_content = match std_fs::read_to_string(env_file_path) {
        Ok(content) => content,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to read .env file"),
    };

    // Find and update the MUSIC_DIR entry, or append if it doesn't exist
    if let Some(existing_music_dir_index) = env_content.find("MUSIC_DIR=") {
        // Find where the current MUSIC_DIR value ends (newline or end of file)
        if let Some(line_end) = env_content[existing_music_dir_index..].find('\n') {
            let start = existing_music_dir_index;
            let end = existing_music_dir_index + line_end + 1;
            // Replace the existing MUSIC_DIR value with the new path
            env_content.replace_range(start..end, &format!("MUSIC_DIR={}\n", new_path));
        } else {
            // In case MUSIC_DIR is the last entry without a newline
            env_content.replace_range(existing_music_dir_index.., &format!("MUSIC_DIR={}\n", new_path));
        }
    } else {
        // Append if MUSIC_DIR is not found
        env_content.push_str(&format!("MUSIC_DIR={}\n", new_path));
    }

    // Write the updated content back to the .env file
    match OpenOptions::new().write(true).truncate(true).open(env_file_path) {
        Ok(mut file) => {
            if let Err(_) = file.write_all(env_content.as_bytes()) {
                return HttpResponse::InternalServerError().body("Failed to write to .env file");
            }
        }
        Err(_) => return HttpResponse::InternalServerError().body("Failed to open .env file for writing"),
    }

    println!("Updated MUSIC_DIR to: {}", new_path);
    HttpResponse::Ok().body("Path saved and .env updated")
}

// Serve the main HTML page
async fn home() -> HttpResponse {
    let html = std_fs::read_to_string("static/index.html") // Use the renamed import here
        .unwrap_or_else(|_| "<h1>Failed to load page</h1>".to_string());
    HttpResponse::Ok().content_type("text/html").body(html)
}

async fn settings() -> HttpResponse {
    let html = std_fs::read_to_string("static/settings.html") // Make sure to create this file
        .unwrap_or_else(|_| "<h1>Failed to load settings page</h1>".to_string());
    HttpResponse::Ok().content_type("text/html").body(html)
}

pub async fn get_albums(music_dir: web::Data<String>) -> HttpResponse {
    let mut albums = Vec::new();
    let music_dir_path = music_dir.get_ref();

    // Traverse only the subdirectories of the main directory (e.g., "aux")
    for entry in WalkDir::new(music_dir_path)
        .min_depth(2)  // Start checking from second level
        .max_depth(2)  // Limit search to one level deeper
        .into_iter()
        .filter_map(Result::ok)
    {
        if entry.path().is_dir() {
            let album_name = entry.file_name().to_string_lossy().to_string();
            let mut cover_art = None;
            let album_path = entry.path();

            // Collect all files in the album directory (ignoring deeper subdirectories)
            let files: Vec<_> = std::fs::read_dir(&album_path)
                .unwrap()
                .filter_map(Result::ok)
                .filter(|e| e.path().is_file()) // Only consider files
                .collect();

            // Randomly select a file if available for cover art
            if let Some(random_file) = files.choose(&mut rand::thread_rng()) {
                let file_path = random_file.path();

                // Check for cover art in mp3 or flac files
                if file_path.extension().map(|s| s == "mp3").unwrap_or(false) {
                    if let Ok(tag) = Tag::read_from_path(&file_path) {
                        if let Some(frame) = tag.get("APIC") {
                            let content = frame.content();
                            if let Some(picture) = match content {
                                id3::Content::Picture(picture) => Some(picture),
                                _ => None,
                            } {
                                cover_art = Some(general_purpose::STANDARD.encode(&picture.data));
                            }
                        }
                    }
                } else if file_path.extension().map(|s| s == "flac").unwrap_or(false) {
                    if let Ok(probe) = Probe::open(&file_path) {
                        if let Ok(tagged_file) = probe.read() {
                            if let Some(tag) = tagged_file.primary_tag() {
                                if let Some(picture) = tag.pictures().first() {
                                    cover_art = Some(general_purpose::STANDARD.encode(picture.data()));
                                }
                            }
                        }
                    }
                }
            }

            // Push the album to the list, with or without cover art
            albums.push(Album { name: album_name, cover_art });
        }
    }

    HttpResponse::Ok().json(albums)
}