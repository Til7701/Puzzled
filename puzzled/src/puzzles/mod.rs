use crate::config;
use adw::gio::{resources_lookup_data, ResourceLookupFlags};
use once_cell::sync::Lazy;
use puzzle_config::{PuzzleConfigCollection, ReadError};
use std::backtrace::Backtrace;
use std::sync::{Mutex, MutexGuard, TryLockError};
use std::time::Duration;

const CORE_COLLECTIONS: [&str; 6] = [
    "puzzle_a_day",
    "trominoes",
    "hexominoes",
    "circles",
    "sandbox",
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
}

pub fn init() {
    let mut store = PUZZLE_COLLECTION_STORE.lock().unwrap();

    for &collection_name in CORE_COLLECTIONS.iter() {
        let path = format!("/de/til7701/Puzzled/puzzles/{}.json", collection_name);
        let collection = load_core_from_resource(&path);
        store.core_puzzle_collections.push(collection);
    }
}

fn load_core_from_resource(filename: &str) -> PuzzleConfigCollection {
    let data = resources_lookup_data(filename, ResourceLookupFlags::NONE).unwrap();
    let json_str = std::str::from_utf8(&*data).unwrap();
    puzzle_config::load_puzzle_collection_from_json(json_str, config::VERSION).unwrap()
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

pub fn add_community_collection_from_string(json_str: &str) -> Result<(), ReadError> {
    let mut store = get_puzzle_collection_store();
    let collection = puzzle_config::load_puzzle_collection_from_json(json_str, config::VERSION)?;
    store.community_puzzle_collections.push(collection);
    Ok(())
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
        for collection_name in CORE_COLLECTIONS.iter() {
            let json =
                fs::read_to_string(&format!("resources/puzzles/{}.json", collection_name)).unwrap();
            let collection =
                puzzle_config::load_puzzle_collection_from_json(&json, config::VERSION).unwrap();
            assert!(!collection.puzzles().is_empty());
        }
    }

    #[tokio::test]
    async fn test_solve_core_collections() {
        for collection_name in CORE_COLLECTIONS.iter() {
            if collection_name == &"sandbox" {
                // Skip the sandbox collection as it contains puzzles that are not solvable
                continue;
            }

            let json =
                fs::read_to_string(&format!("resources/puzzles/{}.json", collection_name)).unwrap();
            let collection =
                puzzle_config::load_puzzle_collection_from_json(&json, config::VERSION).unwrap();

            for puzzle in collection.puzzles() {
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
