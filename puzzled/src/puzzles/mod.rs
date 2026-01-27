use crate::config;
use adw::gio::{resources_lookup_data, ResourceLookupFlags};
use once_cell::sync::Lazy;
use puzzle_config::{PuzzleConfigCollection, ReadError};
use std::backtrace::Backtrace;
use std::sync::{Mutex, MutexGuard, TryLockError};
use std::time::Duration;

const CORE_COLLECTIONS: [&str; 3] = ["puzzle_a_day", "trominoes", "puzzled"];

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
