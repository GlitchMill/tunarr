
# 🎧 Tunarr

**Tunarr** is a Rust-based application that helps organize your music files by renaming them and placing them into album-specific folders. It supports various file formats like `.mp3`, `.flac`, `.aac`, `.ogg`, and `.m4a`. 🚀

## Features 🌟

- 📁 Automatically creates folders based on album names.
- 🎵 Renames music files using the pattern: `{Album Name} - {Artist}`.
- 🖇️ Handles multiple file formats: `.mp3`, `.flac`, `.aac`, `.ogg`, and `.m4a`.
- 🔄 Falls back to using the artist's name if the album artist is not available.
- 🔄 Uses a default name if both album and artist are missing.
- 🌐 Includes a web interface for easier interaction and album browsing.

## Getting Started 🚀

### Prerequisites 📜

- Rust installed on your machine. If you haven't installed Rust yet, follow the [official Rust installation guide](https://www.rust-lang.org/tools/install).

### Installation 🛠️

1. Clone the repository:
   ```bash
   git clone https://github.com/glitchmill/tunarr.git
   cd tunarr
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

### Usage 🎤

Run **Tunarr** by specifying the music folder:
```bash
cargo run --release -- <path_to_your_music_folder>
```

### Example 📂

```bash
cargo run --release -- ./music
```

This will sort all music files in the `./music` directory, creating folders based on album names and renaming the files accordingly.

### Web UI 🌐

You can start the web server to interact with **Tunarr** through a browser:
```bash
cargo run --release
```
Access the web interface at `http://127.0.0.1:8280` to view albums and cover art.

### Roadmap 🗺️
Upcoming Features 🚀
- 🎶 Support for additional audio formats: Extend compatibility to more audio formats.  
- 🔍 Enhanced metadata extraction: Improve how metadata is read, including lyrics and artwork.  
- 📦 GUI version: Develop a graphical user interface for easier usage.  
- 🔄 Batch processing: Allow for batch sorting of multiple folders at once.  
- 🎨 Custom naming patterns: Enable users to define their own file naming conventions.  

## Contributing 🤝

Contributions are welcome! If you'd like to contribute to **Tunarr**, please fork the repository and submit a pull request.

## License 📜

This project is licensed under the GPL V3 License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments 🙏

- Thanks to the [lofty](https://crates.io/crates/lofty) and [id3](https://crates.io/crates/id3) crates for handling audio metadata.
- Inspiration from the need for organized music libraries! 🎶