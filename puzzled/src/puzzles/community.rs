use adw::glib;
use log::{error, info};
use std::path::PathBuf;

pub fn save_community_collection(collection_id: &str, json_str: &str) {
    let puzzles_dir = get_dir();
    let file_path = puzzles_dir.join(format!("{}.json", collection_id));
    if let Err(e) = std::fs::write(&file_path, json_str) {
        error!("Failed to save community collection to file: {}", e);
    }
}

pub fn load_community_collections() -> Vec<String> {
    let puzzles_dir = get_dir();
    let mut collections = Vec::new();

    if let Ok(entries) = std::fs::read_dir(&puzzles_dir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    if let Some(ext) = entry.path().extension() {
                        if ext == "json" {
                            if let Ok(json_str) = std::fs::read_to_string(entry.path()) {
                                collections.push(json_str);
                            } else {
                                error!(
                                    "Failed to read community collection file: {:?}",
                                    entry.path()
                                );
                            }
                        }
                    }
                }
            }
        }
    } else {
        error!("Failed to read puzzles directory: {:?}", puzzles_dir);
    }

    collections
}

pub fn delete_community_collection(collection_id: &str) {
    let puzzles_dir = get_dir();
    let file_path = puzzles_dir.join(format!("{}.json", collection_id));
    if let Err(e) = std::fs::remove_file(&file_path) {
        info!("Failed to delete community collection file: {}", e);
    }
}

fn get_dir() -> PathBuf {
    let xdg_data_dir = glib::user_data_dir();
    let puzzles_dir = xdg_data_dir.join("puzzled").join("community_puzzles");
    if let Err(e) = std::fs::create_dir_all(&puzzles_dir) {
        error!("Failed to create puzzles directory: {}", e);
    }
    puzzles_dir
}
