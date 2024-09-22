use std::fs::{self, DirEntry};
use std::path::PathBuf;
use id3::{Tag as ID3Tag, Error as ID3Error};
use lofty::probe::Probe; // Ensure this import is present
use lofty::prelude::ItemKey;
use id3::TagLike;
use lofty::file::TaggedFileExt; // Import for primary_tag()


pub fn process_music_files(music_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let paths = fs::read_dir(music_dir)?;

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
    let tag = ID3Tag::read_from_path(&path)?;

    let album_name = tag.album().unwrap_or("Unknown Album");
    let album_artist = tag.album_artist()
        .or(tag.artist())
        .unwrap_or("");

    let new_file_name = if album_artist.is_empty() {
        format!("{}.mp3", album_name)
    } else {
        format!("{} - {}.mp3", album_name, album_artist)
    };

    let album_folder = create_album_folder(&path, album_name)?;
    let new_path = album_folder.join(&new_file_name);
    fs::rename(&path, &new_path)?;

    println!("Renamed and moved MP3 file to: {:?}", new_file_name);
    Ok(())
}

// Process M4A files using mp4ameta crate
fn process_m4a(entry: &DirEntry) -> Result<(), Box<dyn std::error::Error>> {
    let path = entry.path();
    let tag = mp4ameta::Tag::read_from_path(&path)?;

    let album_name = tag.album().unwrap_or("Unknown Album");
    let album_artist = tag.album_artist()
        .or(tag.artist())
        .unwrap_or("");

    let new_file_name = if album_artist.is_empty() {
        format!("{}.m4a", album_name)
    } else {
        format!("{} - {}.m4a", album_name, album_artist)
    };

    let album_folder = create_album_folder(&path, album_name)?;
    let new_path = album_folder.join(&new_file_name);
    fs::rename(&path, &new_path)?;

    println!("Renamed and moved M4A file to: {:?}", new_file_name);
    Ok(())
}

// Process FLAC, OGG, AAC files using lofty crate
fn process_lofty(entry: &DirEntry) -> Result<(), Box<dyn std::error::Error>> {
    let path = entry.path();
    let tagged_file = Probe::open(&path)?.read()?;
    let tag = tagged_file.primary_tag().ok_or("No metadata found")?;

    let album_name = tag.get_string(&ItemKey::AlbumTitle)
        .unwrap_or("Unknown Album");
    let album_artist = tag.get_string(&ItemKey::AlbumArtist)
        .or(tag.get_string(&ItemKey::TrackArtist))
        .unwrap_or("");

    let extension = path.extension().unwrap().to_str().unwrap();
    let new_file_name = if album_artist.is_empty() {
        format!("{}.{}", album_name, extension)
    } else {
        format!("{} - {}.{}", album_name, album_artist, extension)
    };

    let album_folder = create_album_folder(&path, album_name)?;
    let new_path = album_folder.join(&new_file_name);
    fs::rename(&path, &new_path)?;

    println!("Renamed and moved {} file to: {:?}", extension.to_uppercase(), new_file_name);
    Ok(())
}

// Helper function to create an album folder in the input file's directory
fn create_album_folder(file_path: &PathBuf, album_name: &str) -> Result<PathBuf, std::io::Error> {
    let parent_dir = file_path.parent().ok_or(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "Parent directory not found",
    ))?;

    let album_folder = parent_dir.join(album_name);
    if !album_folder.exists() {
        fs::create_dir_all(&album_folder)?;
        println!("Created album folder: {:?}", album_folder);
    }

    Ok(album_folder)
}
