use crate::global::state::PuzzleTypeExtension;
use adw::gio::Settings;
use adw::glib::{Variant, VariantDict, VariantTy};
use adw::prelude::{SettingsExt, SettingsExtManual};
use log::{debug, error};
use puzzle_config::{BoardConfig, PuzzleConfigCollection, Target};

const SOLVED_KEY: &str = "solved";
const HINTS_KEY: &str = "hints";

#[derive(Debug, Clone)]
pub struct PuzzleMeta {
    settings: Settings,
}

impl PuzzleMeta {
    pub fn new() -> Self {
        PuzzleMeta {
            settings: Settings::new("de.til7701.Puzzled.puzzle-meta"),
        }
    }

    pub fn reset_solved(&self) {
        self.settings.reset(SOLVED_KEY);
    }

    pub fn is_solved(
        &self,
        collection: &PuzzleConfigCollection,
        puzzle_index: usize,
        extension: &Option<PuzzleTypeExtension>,
    ) -> bool {
        let variant = self.get_value(SOLVED_KEY, collection, puzzle_index, extension);
        variant.and_then(|v| v.get::<bool>()).unwrap_or(false)
    }

    pub fn set_solved(
        &self,
        solved: bool,
        collection: &PuzzleConfigCollection,
        puzzle_index: usize,
        extension: &Option<PuzzleTypeExtension>,
    ) {
        self.set_value(
            SOLVED_KEY,
            &Variant::from(solved),
            collection,
            puzzle_index,
            extension,
        );
    }

    pub fn hints(
        &self,
        collection: &PuzzleConfigCollection,
        puzzle_index: usize,
        extension: &Option<PuzzleTypeExtension>,
    ) -> Option<u32> {
        let variant = self.get_value(HINTS_KEY, collection, puzzle_index, extension);
        variant.and_then(|v| v.get::<u32>())
    }

    pub fn set_hints(
        &self,
        hints: u32,
        collection: &PuzzleConfigCollection,
        puzzle_index: usize,
        extension: &Option<PuzzleTypeExtension>,
    ) {
        self.set_value(
            HINTS_KEY,
            &Variant::from(hints),
            collection,
            puzzle_index,
            extension,
        );
    }

    fn get_value(
        &self,
        key: &str,
        collection: &PuzzleConfigCollection,
        puzzle_index: usize,
        extension: &Option<PuzzleTypeExtension>,
    ) -> Option<Variant> {
        let (_, puzzle_dict) = self.get_dicts(key, collection);
        let puzzle_key = puzzle_key(collection, puzzle_index, extension);
        let value = if let Some(puzzle_key) = &puzzle_key {
            puzzle_dict.lookup_value(puzzle_key, None)
        } else {
            None
        };
        debug!(
            "Get value for key='{}', collection='{}', puzzle_index={}, puzzle_key={}: {:?}",
            key,
            collection.id(),
            puzzle_index,
            puzzle_key.unwrap_or_else(|| "none".to_string()),
            value
        );
        value
    }

    fn set_value(
        &self,
        key: &str,
        value: &Variant,
        collection: &PuzzleConfigCollection,
        puzzle_index: usize,
        extension: &Option<PuzzleTypeExtension>,
    ) {
        let (collection_dict, puzzle_dict) = self.get_dicts(key, collection);

        let puzzle_key = puzzle_key(collection, puzzle_index, extension);
        if let Some(puzzle_key) = &puzzle_key {
            puzzle_dict.insert(puzzle_key, value);
        }

        collection_dict.insert(collection.id(), &Variant::from(puzzle_dict));
        let result = self.settings.set(key, &Variant::from(collection_dict));
        match result {
            Ok(_) => {
                debug!(
                    "Set value for key='{}', collection='{}', puzzle_index={}, puzzle_key={}",
                    key,
                    collection.id(),
                    puzzle_index,
                    puzzle_key.unwrap_or_else(|| "none".to_string())
                );
            }
            Err(_) => {
                error!(
                    "Failed to set value for key='{}', collection='{}', puzzle_index={}, puzzle_key={}",
                    key,
                    collection.id(),
                    puzzle_index,
                    puzzle_key.unwrap_or_else(|| "none".to_string())
                )
            }
        }
    }

    fn get_dicts(
        &self,
        key: &str,
        collection: &PuzzleConfigCollection,
    ) -> (VariantDict, VariantDict) {
        let collection_dict = self.settings.get::<VariantDict>(key);
        let puzzle_dict = collection_dict
            .lookup_value(collection.id(), Some(VariantTy::DICTIONARY))
            .map(|v| v.get::<VariantDict>().unwrap())
            .unwrap_or_else(|| VariantDict::new(None));
        (collection_dict, puzzle_dict)
    }
}

fn puzzle_key(
    collection: &PuzzleConfigCollection,
    puzzle_index: usize,
    extension: &Option<PuzzleTypeExtension>,
) -> Option<String> {
    let puzzle = collection.puzzles().get(puzzle_index)?;
    let extension_key = extension_key(collection, puzzle_index, extension);
    Some(format!("{}/{}", puzzle.id(), extension_key?))
}

fn extension_key(
    collection: &PuzzleConfigCollection,
    puzzle_index: usize,
    extension: &Option<PuzzleTypeExtension>,
) -> Option<String> {
    match collection.puzzles()[puzzle_index].board_config() {
        BoardConfig::Simple { .. } => Some("simple".to_string()),
        BoardConfig::Area { .. } => {
            let target = match extension {
                Some(PuzzleTypeExtension::Area { target }) => target,
                _ => &None,
            };
            if let Some(target) = target {
                let key = target_key(&target);
                Some(key)
            } else if let Some(target) = collection.puzzles()[puzzle_index]
                .board_config()
                .default_target()
            {
                let key = target_key(&target);
                Some(key)
            } else {
                None
            }
        }
    }
}

fn target_key(target: &Target) -> String {
    let mut key = "".to_string();
    for index in &target.indices {
        key = format!("{}x{}-{}", key, index.0, index.1);
    }
    key
}
