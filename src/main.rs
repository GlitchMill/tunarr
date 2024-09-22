use std::fs::{self, DirEntry};
use std::path::PathBuf;
use id3::{Tag as ID3Tag, Error as ID3Error};
use lofty::{Probe, ItemKey};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let music_dir = "/home/san/dl/Telegram Desktop"; // specify your folder path here

    // Read the directory
    let paths = fs::read_dir(music_dir)?;

    // Process each file in the directory
    for entry in paths {
        let entry = entry?;
        if let Some(extension) = entry.path().extension() {
            match extension.to_str() {
                Some("mp3") => process_mp3(&entry)?,
                Some("m4a") => process_m4a(&entry)?,
                Some("flac") | Some("ogg") | Some("aac") => process_lofty(&entry)?,
                _ => {
                    println!("Unsupported file format: {:?}", entry.path());
                }
            }
        }
    }

    Ok(())
}

// Process MP3 files using id3 crate
fn process_mp3(entry: &DirEntry) -> Result<(), ID3Error> {
    let path = entry.path();

    // Open the ID3 tag
    let tag = ID3Tag::read_from_path(&path)?;

    // Get the required fields with fallbacks
    let album_name = tag.album().unwrap_or("Unknown Album");
    let album_artist = tag.album_artist()
        .or(tag.artist()) // Fallback to artist if album artist is missing
        .unwrap_or("");    // If both are missing, use empty string

    let new_file_name = if album_artist.is_empty() {
        format!("{}.mp3", album_name)
    } else {
        format!("{} - {}.mp3", album_name, album_artist)
    };

    // Create the album folder in the input file's directory
    let album_folder = create_album_folder(&path, album_name)?;

    // Construct the new file path inside the album folder
    let new_path = album_folder.join(&new_file_name);

    // Rename and move the file to the album folder
    fs::rename(&path, &new_path)?;

    println!("Renamed and moved MP3 file to: {:?}", new_file_name);

    Ok(())
}

// Process M4A files using mp4ameta crate
fn process_m4a(entry: &DirEntry) -> Result<(), Box<dyn std::error::Error>> {
    let path = entry.path();

    // Open the M4A tag
    let tag = mp4ameta::Tag::read_from_path(&path)?;

    let album_name = tag.album().unwrap_or("Unknown Album");
    let album_artist = tag.album_artist()
        .or(tag.artist()) // Fallback to artist if album artist is missing
        .unwrap_or("");

    let new_file_name = if album_artist.is_empty() {
        format!("{}.m4a", album_name)
    } else {
        format!("{} - {}.m4a", album_name, album_artist)
    };

    // Create the album folder in the input file's directory
    let album_folder = create_album_folder(&path, album_name)?;

    // Construct the new file path inside the album folder
    let new_path = album_folder.join(&new_file_name);

    // Rename and move the file to the album folder
    fs::rename(&path, &new_path)?;

    println!("Renamed and moved M4A file to: {:?}", new_file_name);

    Ok(())
}

// Process FLAC, OGG, AAC files using lofty crate
fn process_lofty(entry: &DirEntry) -> Result<(), Box<dyn std::error::Error>> {
    let path = entry.path();

    // Probe and read metadata
    let tagged_file = Probe::open(&path)?.read()?;
    let tag = tagged_file.primary_tag().ok_or("No metadata found")?;

    // Use correct ItemKey variants
    let album_name = tag.get_string(&ItemKey::AlbumTitle)
        .unwrap_or("Unknown Album");
    let album_artist = tag.get_string(&ItemKey::AlbumArtist)
        .or(tag.get_string(&ItemKey::TrackArtist)) // Fallback to track artist
        .unwrap_or("");

    let extension = path.extension().unwrap().to_str().unwrap(); // get file extension

    let new_file_name = if album_artist.is_empty() {
        format!("{}.{}", album_name, extension)
    } else {
        format!("{} - {}.{}", album_name, album_artist, extension)
    };

    // Create the album folder in the input file's directory
    let album_folder = create_album_folder(&path, album_name)?;

    // Construct the new file path inside the album folder
    let new_path = album_folder.join(&new_file_name);

    // Rename and move the file to the album folder
    fs::rename(&path, &new_path)?;

    println!("Renamed and moved {} file to: {:?}", extension.to_uppercase(), new_file_name);

    Ok(())
}

// Helper function to create an album folder in the input file's directory
fn create_album_folder(file_path: &PathBuf, album_name: &str) -> Result<PathBuf, std::io::Error> {
    // Get the parent directory of the file
    let parent_dir = file_path.parent().ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Parent directory not found",
    ))?;

    // Construct the album folder path
    let album_folder = parent_dir.join(album_name);

    // Create the album folder if it doesn't exist
    if !album_folder.exists() {
        fs::create_dir_all(&album_folder)?;  // use create_dir_all to ensure all directories are created
        println!("Created album folder: {:?}", album_folder);
    }

    Ok(album_folder)
}
