use std::{fs::{self}, path::{Path, PathBuf}};

const AUDIO_EXTENSIONS: &[&str] = &[
    "mp3", "flac", "wav", "aiff", "aif",
    "ogg", "opus", "m4a", "aac",
    "dsf", "dff",
];

fn is_audio(path: &Path) -> bool {
    path.extension()
        .map_or(false, |ext| AUDIO_EXTENSIONS.contains(&ext.to_str().unwrap_or("")))
}

pub fn read_dir_deep(
    directory: &str,
    files: &mut Vec<PathBuf>
) {
    let path_directory: PathBuf = PathBuf::from(&directory);
    let entries = match fs::read_dir(&path_directory) {
        Ok(e) => e,
        Err(e) => {
            log::error!("Impossible de lire le dossier {:?}: {}", path_directory, e);
            return;
        }
    };

    for entry in entries.flatten() {
        let path: PathBuf = entry.path();

        if path.is_dir() {
            if let Some(p) = path.to_str() {
                read_dir_deep(p, files);
            }
        } else if is_audio(&path) {
            files.push(path);
        }
    }
}