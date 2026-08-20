use adw::glib;
use log::{error, info};
use std::path::PathBuf;

pub fn save_community_collection(collection_id: &str, json_str: &str) {
    let puzzles_dir = get_xdg_puzzles_data_dir();
    let file_path = puzzles_dir.join(format!("{}.json", collection_id));
    if let Err(e) = std::fs::write(&file_path, json_str) {
        error!("Failed to save community collection to file: {}", e);
    }
}

pub fn load_community_collections() -> Vec<String> {
    let puzzles_dir = get_xdg_puzzles_data_dir();
    let mut collections = Vec::new();

    if let Ok(entries) = std::fs::read_dir(&puzzles_dir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type()
                && file_type.is_file()
                && let Some(ext) = entry.path().extension()
                && ext == "json"
            {
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
    } else {
        error!("Failed to read puzzles directory: {:?}", puzzles_dir);
    }

    collections
}

pub fn delete_community_collection(collection_id: &str) {
    let puzzles_dir = get_xdg_puzzles_data_dir();
    let file_path = puzzles_dir.join(format!("{}.json", collection_id));
    if !file_path.exists() {
        info!(
            "Community collection file does not exist, nothing to delete: {:?}",
            file_path
        );
        return;
    }
    if let Err(e) = std::fs::remove_file(&file_path) {
        error!("Failed to delete community collection file: {}", e);
    }
}

fn get_xdg_puzzles_data_dir() -> PathBuf {
    let xdg_data_dir = glib::user_data_dir();
    let puzzles_dir = xdg_data_dir.join("puzzled").join("community_puzzles");
    if let Err(e) = std::fs::create_dir_all(&puzzles_dir) {
        error!("Failed to create puzzles directory: {}", e);
    }
    puzzles_dir
}

#[cfg(test)]
mod test {
    use crate::model::store::community::{delete_community_collection, get_xdg_puzzles_data_dir, load_community_collections, save_community_collection};
    use std::path::PathBuf;

    #[test]
    fn test_save_delete_community_collection() {
        save_community_collection("test", r#"{"test": true}"#);

        let puzzles_dir = get_xdg_puzzles_data_dir();
        println!("{:?}", puzzles_dir);

        let file_path = puzzles_dir.join("test.json");
        assert!(file_path.exists());

        let all_collections_json = load_community_collections();
        assert!(all_collections_json.contains(&r#"{"test": true}"#.to_string()));

        delete_community_collection("test");

        let all_collections_json = load_community_collections();
        assert!(!all_collections_json.contains(&r#"{"test": true}"#.to_string()));
    }

    #[test]
    fn test_get_xdg_puzzles_data_dir() {
        let buf = get_xdg_puzzles_data_dir();

        // Constructing strings this way to avoid search and replace refactoring errors. See #222
        let puzzled = concat!("puz", "zled");
        let community_puzzles = concat!("commun", "ity_puz", "zles");

        assert!(buf.ends_with(PathBuf::from(puzzled).join(community_puzzles)));
    }
}
