use actix_files as fs;
use actix_web::{web, App, HttpServer, HttpResponse};
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

pub async fn start_server() -> std::io::Result<()> {
    dotenv().ok();
    let music_dir = env::var("MUSIC_DIR").expect("MUSIC_DIR not set in .env");

    HttpServer::new(move || {
        // Wrap the music_dir in web::Data
        let music_dir_data = web::Data::new(music_dir.clone());

        App::new()
            .app_data(music_dir_data) // Register the music_dir_data
            .service(fs::Files::new("/static", "static").show_files_listing())
            .route("/", web::get().to(home))
            .route("/albums", web::get().to(get_albums)) // This will now call get_albums
    })
    .bind("127.0.0.1:8280")?
    .run()
    .await
}

// Serve the main HTML page
async fn home() -> HttpResponse {
    let html = std_fs::read_to_string("static/index.html") // Use the renamed import here
        .unwrap_or_else(|_| "<h1>Failed to load page</h1>".to_string());
    HttpResponse::Ok().content_type("text/html").body(html)
}

pub async fn get_albums(music_dir: web::Data<String>) -> HttpResponse {
    let mut albums = Vec::new();
    let music_dir_path = music_dir.get_ref();
    for entry in std::fs::read_dir(music_dir_path).unwrap() {
        let entry = entry.unwrap();
        if entry.path().is_dir() {
            let album_name = entry.file_name().to_string_lossy().to_string();
            let mut cover_art = None;

            for file_entry in std_fs::read_dir(entry.path()).unwrap() {
                let file_entry = file_entry.unwrap();
                let file_path = file_entry.path();

                // Handle MP3 files
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
                }

                // Handle FLAC files
                if file_path.extension().map(|s| s == "flac").unwrap_or(false) {
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

            albums.push(Album { name: album_name, cover_art });
        }
    }

    HttpResponse::Ok().json(albums)
}
