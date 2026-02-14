mod community;

use crate::config;
use crate::puzzles::community::save_community_collection;
use adw::gio::{resources_lookup_data, ResourceLookupFlags};
use log::error;
use once_cell::sync::Lazy;
use puzzle_config::{JsonLoader, PuzzleConfigCollection, ReadError};
use std::backtrace::Backtrace;
use std::sync::{Mutex, MutexGuard, TryLockError};
use std::time::Duration;

const CORE_COLLECTIONS: [&str; 5] = [
    "puzzle_a_day",
    "trominoes",
    "hexominoes",
    "circles",
    "puzzled",
];

static PUZZLE_COLLECTION_STORE: Lazy<Mutex<PuzzleCollectionStore>> =
    Lazy::new(|| Mutex::new(PuzzleCollectionStore::default()));

#[derive(Debug, Default)]
pub struct PuzzleCollectionStore {
    core_puzzle_collections: Vec<PuzzleConfigCollection>,
    community_puzzle_collections: Vec<PuzzleConfigCollection>,
}

impl PuzzleCollectionStore {
    pub fn core_puzzle_collections(&self) -> &[PuzzleConfigCollection] {
        &self.core_puzzle_collections
    }

    pub fn community_puzzle_collections(&self) -> &[PuzzleConfigCollection] {
        &self.community_puzzle_collections
    }

    pub fn add_community_collection_from_string(
        &mut self,
        json_str: &str,
    ) -> Result<(), ReadError> {
        let json_loader = create_json_loader();
        let collection = json_loader.load_puzzle_collection(json_str)?;
        self.remove_community_collection(collection.id());
        save_community_collection(collection.id(), json_str);
        self.community_puzzle_collections.push(collection);
        Ok(())
    }

    pub fn remove_community_collection(&mut self, collection_id: &str) {
        self.community_puzzle_collections
            .retain(|collection| collection.id() != collection_id);
        community::delete_community_collection(collection_id);
    }
}

pub fn init() {
    let mut store = PUZZLE_COLLECTION_STORE.lock().unwrap();
    let json_loader = create_json_loader();

    for &collection_name in CORE_COLLECTIONS.iter() {
        let path = format!("/de/til7701/Puzzled/puzzles/{}.json", collection_name);
        let collection = load_core_from_resource(&path, &json_loader);
        store.core_puzzle_collections.push(collection);
    }

    let community_collections = community::load_community_collections();
    for json_str in community_collections {
        let collection = match json_loader.load_puzzle_collection(&json_str) {
            Ok(collection) => collection,
            Err(e) => {
                error!(
                    "Failed to load community puzzle collection from JSON string: {:?}",
                    e
                );
                continue;
            }
        };
        store.community_puzzle_collections.push(collection);
    }
}

fn load_core_from_resource(filename: &str, json_loader: &JsonLoader) -> PuzzleConfigCollection {
    let json_str = read_resource(filename);
    match json_loader.load_puzzle_collection(&json_str) {
        Ok(collection) => collection,
        Err(e) => panic!(
            "Failed to load core puzzle collection from '{}': {:?}",
            filename, e
        ),
    }
}

fn create_json_loader() -> JsonLoader {
    let predefined_json_str = read_resource("/de/til7701/Puzzled/predefined.json");
    puzzle_config::create_json_loader(&predefined_json_str, config::VERSION).unwrap()
}

fn read_resource(filename: &str) -> String {
    let data = resources_lookup_data(filename, ResourceLookupFlags::NONE).unwrap();
    std::str::from_utf8(&*data).unwrap().to_string()
}

pub fn get_puzzle_collection_store() -> MutexGuard<'static, PuzzleCollectionStore> {
    match PUZZLE_COLLECTION_STORE.try_lock() {
        Ok(guard) => guard,
        Err(TryLockError::WouldBlock) => {
            eprintln!(
                "get_puzzle_collection_store: mutex busy (possible deadlock). PID={} Backtrace:\n{:?}",
                std::process::id(),
                Backtrace::force_capture()
            );
            std::thread::sleep(Duration::from_secs(2));
            PUZZLE_COLLECTION_STORE.lock().unwrap()
        }
        Err(TryLockError::Poisoned(_)) => PUZZLE_COLLECTION_STORE.lock().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use puzzle_config::BoardConfig;
    use puzzle_solver::board::Board;
    use puzzle_solver::tile::Tile;
    use std::fs;
    use tokio_util::sync::CancellationToken;

    #[test]
    fn test_load_core_collections() {
        let predefined_json_str =
            fs::read_to_string(&"resources/predefined.json".to_string()).unwrap();
        let json_loader =
            puzzle_config::create_json_loader(&predefined_json_str, config::VERSION).unwrap();

        for collection_name in CORE_COLLECTIONS.iter() {
            let json =
                fs::read_to_string(&format!("resources/puzzles/{}.json", collection_name)).unwrap();
            let collection = json_loader.load_puzzle_collection(&json).unwrap();
            assert!(!collection.puzzles().is_empty());
        }
    }

    #[tokio::test]
    async fn test_solve_core_collections() {
        let predefined_json_str =
            fs::read_to_string(&"resources/predefined.json".to_string()).unwrap();
        let json_loader =
            puzzle_config::create_json_loader(&predefined_json_str, config::VERSION).unwrap();

        // (collection_id, puzzle_name) pairs to skip because they are known to be unsolvable or take too long
        let skip_list = [("de.til7701.Puzzled.Puzzled", "Large Sandbox")];

        for collection_name in CORE_COLLECTIONS.iter() {
            let json =
                fs::read_to_string(&format!("resources/puzzles/{}.json", collection_name)).unwrap();
            let collection = json_loader.load_puzzle_collection(&json).unwrap();

            for puzzle in collection.puzzles() {
                if skip_list.contains(&(collection.id(), puzzle.name())) {
                    // Skip puzzles that are known to be unsolvable or take too long
                    continue;
                }

                if puzzle.tiles().len() > 12 {
                    // Skip puzzles with too many tiles to avoid long test times
                    continue;
                }

                let board_config = &puzzle.board_config();
                match board_config {
                    BoardConfig::Simple { layout } => {
                        let board: Board = layout.map(|e| !e).clone().into();
                        let tiles: Vec<Tile> = puzzle
                            .tiles()
                            .iter()
                            .map(|tile_config| Tile::new(tile_config.base().clone()))
                            .collect();
                        let result = puzzle_solver::solve_all_filling(
                            board,
                            &tiles,
                            CancellationToken::new(),
                        )
                        .await;
                        assert!(
                            result.is_ok(),
                            "Failed to solve puzzle '{}' in collection '{}'",
                            puzzle.name(),
                            collection_name
                        );
                    }
                    BoardConfig::Area { .. } => {}
                }
            }
        }
    }
}
