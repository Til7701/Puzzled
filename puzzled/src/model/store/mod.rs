mod community;

use crate::config;
use crate::model::collection::CollectionModel;
use crate::model::puzzle_meta::PuzzleMeta;
use crate::model::store::community::save_community_collection;
use adw::gio::{resources_lookup_data, ResourceLookupFlags};
use log::error;
use once_cell::sync::OnceCell;
use puzzle_config::{JsonLoader, Predefined, PuzzleConfigCollection, ReadError};
use std::cell::RefCell;

const CORE_COLLECTIONS: [&str; 9] = [
    "puzzle_a_day",
    "trominoes",
    "tetrominoes",
    "pentominoes",
    "hexominoes",
    "chess",
    "circles",
    "recursive_construction",
    "puzzled",
];

thread_local! {
    static PUZZLE_COLLECTION_STORE: RefCell<PuzzleCollectionStore> =
    RefCell::new(PuzzleCollectionStore::default());
}

pub fn with_puzzle_collection_store<R>(f: impl FnOnce(&mut PuzzleCollectionStore) -> R) -> R {
    PUZZLE_COLLECTION_STORE.with(|store| f(&mut store.borrow_mut()))
}

/// Provides access to the core and community puzzle collections.
///
/// Get the singleton by calling [get_puzzle_collection_store]().
/// The store is initialized by calling [init]() once at application startup.
#[derive(Debug, Default)]
pub struct PuzzleCollectionStore {
    core_puzzle_collections: Vec<CollectionModel>,
    community_puzzle_collections: Vec<CollectionModel>,
}

impl PuzzleCollectionStore {
    pub fn core_puzzle_collections(&self) -> &[CollectionModel] {
        &self.core_puzzle_collections
    }

    pub fn community_puzzle_collections(&self) -> &[CollectionModel] {
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
        self.community_puzzle_collections
            .push(CollectionModel::new(collection, &PuzzleMeta::new()));
        Ok(())
    }

    pub fn remove_community_collection(&mut self, collection_id: &str) {
        self.community_puzzle_collections
            .retain(|collection| collection.config().id() != collection_id);
        community::delete_community_collection(collection_id);
    }

    pub fn mark_all_as_unsolved(&self) {
        PuzzleMeta::new().reset();
        for collection in &self.core_puzzle_collections {
            collection.mark_all_as_unsolved();
        }
        for collection in &self.community_puzzle_collections {
            collection.mark_all_as_unsolved();
        }
    }
}

/// Must be called once at application startup to load the core and community puzzle collections into the store.
///
/// A second call has undefined behavior.
pub fn init() {
    PUZZLE_COLLECTION_STORE.with_borrow_mut(|store| {
        let json_loader = create_json_loader();
        let puzzle_meta = PuzzleMeta::new();

        for &collection_name in CORE_COLLECTIONS.iter() {
            let path = format!("/de/til7701/Puzzled/puzzles/{}.json", collection_name);
            let collection = load_core_from_resource(&path, &json_loader);
            store
                .core_puzzle_collections
                .push(CollectionModel::new(collection, &puzzle_meta));
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
            store
                .community_puzzle_collections
                .push(CollectionModel::new(collection, &puzzle_meta));
        }
    });
}

/// Loads a core puzzle collection from a resource file using the provided JsonLoader.
///
/// Panics if the resource cannot be found or if loading the collection fails.
///
/// # Arguments
///
/// * `filename`: The path to the resource file containing the puzzle collection JSON. This must be a valid resource path, e.g., "/de/til7701/Puzzled/puzzles/puzzle_a_day.json".
/// * `json_loader`: The JsonLoader instance to use.
///
/// returns: PuzzleConfigCollection
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

/// Creates a JsonLoader and adds predefined tiles from the predefined JSON resource.
fn create_json_loader() -> JsonLoader {
    let predefined_json_str = read_resource("/de/til7701/Puzzled/predefined.json");
    puzzle_config::create_json_loader(&predefined_json_str, config::VERSION).unwrap()
}

/// Convenience function to read a resource file as a string.
///
/// Panics if the resource cannot be found or read.
fn read_resource(filename: &str) -> String {
    let data = resources_lookup_data(filename, ResourceLookupFlags::NONE).unwrap();
    std::str::from_utf8(&data).unwrap().to_string()
}

static PREDEFINED: OnceCell<Predefined> = OnceCell::new();

pub fn get_predefined<'a>() -> &'a Predefined {
    PREDEFINED.get_or_init(|| {
        let predefined_json_str = read_resource("/de/til7701/Puzzled/predefined.json");
        puzzle_config::get_predefined(&predefined_json_str, config::VERSION)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use puzzle_config::{BoardConfig, PuzzleConfig, PuzzleId};
    use puzzle_solver::board::Board;
    use puzzle_solver::tile::Tile;
    use std::collections::{HashMap, HashSet};
    use std::fs;
    use std::hash::{DefaultHasher, Hash, Hasher};
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

    /// Ensures solvability of puzzles in core collections that are not known to be unsolvable or take too long to solve.
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn test_solve_core_collections() {
        let predefined_json_str =
            fs::read_to_string(&"resources/predefined.json".to_string()).unwrap();
        let json_loader =
            puzzle_config::create_json_loader(&predefined_json_str, config::VERSION).unwrap();

        // (collection_id, puzzle_name) pairs to skip because they are known to be unsolvable or take too long
        let skip_list = [
            ("de.til7701.Puzzled.RecursiveConstruction", "T4 x 3"), // Takes too long to solve
            ("de.til7701.Puzzled.Hexominoes", "Holes"),             // Takes too long to solve
            ("de.til7701.Puzzled.PuzzleADay", "4-Digit Year"),      // Unknown if solvable
        ];

        for collection_name in CORE_COLLECTIONS.iter() {
            let json =
                fs::read_to_string(&format!("resources/puzzles/{}.json", collection_name)).unwrap();
            let collection = json_loader.load_puzzle_collection(&json).unwrap();

            for puzzle in collection.puzzles() {
                if puzzle.is_unsolvable() || skip_list.contains(&(collection.id(), puzzle.name())) {
                    // Skip puzzles that are known to be unsolvable or take too long
                    continue;
                }

                if puzzle.tiles().len() > 12 {
                    // Skip puzzles with too many tiles to avoid long test times
                    println!(
                        "Skipping puzzle '{}' in collection '{}' because it has too many tiles ({}).",
                        puzzle.name(),
                        collection_name,
                        puzzle.tiles().len()
                    );
                    continue;
                }

                println!(
                    "Testing puzzle '{}' in collection '{}'",
                    puzzle.name(),
                    collection_name
                );
                let start_time = std::time::Instant::now();

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

                let elapsed = start_time.elapsed();
                println!(
                    "Solved puzzle '{}' in collection '{}' in {:.2?}",
                    puzzle.name(),
                    collection_name,
                    elapsed
                );
            }
        }
    }

    /// Ensures unique collection ids
    #[test]
    fn test_core_collections_ids() {
        let predefined_json_str =
            fs::read_to_string(&"resources/predefined.json".to_string()).unwrap();
        let json_loader =
            puzzle_config::create_json_loader(&predefined_json_str, config::VERSION).unwrap();

        for collection_name in CORE_COLLECTIONS.iter() {
            let json =
                fs::read_to_string(&format!("resources/puzzles/{}.json", collection_name)).unwrap();
            let collection = json_loader.load_puzzle_collection(&json).unwrap();
            assert!(!collection.puzzles().is_empty());
            let mut set: HashSet<PuzzleId> = HashSet::new();
            for puzzle in collection.puzzles() {
                let id = puzzle.id();
                assert!(
                    set.insert(id.clone()),
                    "Duplicate puzzle ID '{}' in collection '{}'",
                    id,
                    collection_name
                );
            }
        }
    }

    /// Ensures unique puzzle names in collections
    #[test]
    fn test_core_collections_names() {
        let predefined_json_str =
            fs::read_to_string(&"resources/predefined.json".to_string()).unwrap();
        let json_loader =
            puzzle_config::create_json_loader(&predefined_json_str, config::VERSION).unwrap();

        for collection_name in CORE_COLLECTIONS.iter() {
            let json =
                fs::read_to_string(&format!("resources/puzzles/{}.json", collection_name)).unwrap();
            let collection = json_loader.load_puzzle_collection(&json).unwrap();
            assert!(!collection.puzzles().is_empty());
            let mut set: HashSet<&str> = HashSet::new();
            for puzzle in collection.puzzles() {
                let name = puzzle.name();
                assert!(
                    set.insert(name),
                    "Duplicate puzzle Name '{}' in collection '{}'",
                    name,
                    collection_name
                );
            }
        }
    }

    /// Ensures unique puzzle ids in collections
    #[test]
    fn test_core_collections_unique_puzzles() {
        let predefined_json_str =
            fs::read_to_string(&"resources/predefined.json".to_string()).unwrap();
        let json_loader =
            puzzle_config::create_json_loader(&predefined_json_str, config::VERSION).unwrap();

        let mut set: HashMap<u64, String> = HashMap::new();
        for collection_name in CORE_COLLECTIONS.iter() {
            let json =
                fs::read_to_string(&format!("resources/puzzles/{}.json", collection_name)).unwrap();
            let collection = json_loader.load_puzzle_collection(&json).unwrap();
            for puzzle in collection.puzzles() {
                let puzzle_identifier = format!("{}:{}", collection_name, puzzle.id());
                let mut hasher = DefaultHasher::new();
                PuzzleConfig::hash(puzzle, &mut hasher);
                let hash = hasher.finish();
                assert!(
                    !set.contains_key(&hash),
                    "Duplicate puzzle detected: {} and {}",
                    set.get(&hash).unwrap(),
                    puzzle_identifier
                );
                set.insert(hash, puzzle_identifier);
            }
        }
    }
}
