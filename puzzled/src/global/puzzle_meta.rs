use crate::global::state::PuzzleTypeExtension;
use adw::gio::Settings;
use adw::glib::{Variant, VariantDict, VariantTy};
use adw::prelude::{SettingsExt, SettingsExtManual};
use log::{debug, error};
use puzzle_config::{BoardConfig, PuzzleConfigCollection, PuzzleId, Target};

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

    pub fn reset_all(&self) {
        self.settings.reset("solved");
    }

    pub fn is_solved(
        &self,
        collection: &PuzzleConfigCollection,
        puzzle_index: usize,
        extension: &Option<PuzzleTypeExtension>,
    ) -> bool {
        let puzzle_dict = self.get_puzzle_dict("solved", collection);
        let key = puzzle_key(collection, puzzle_index, extension);
        let solved = if let Some(key) = &key {
            puzzle_dict
                .lookup_value(key, Some(VariantTy::BOOLEAN))
                .map(|v| v.get::<bool>().unwrap())
                .unwrap_or(false)
        } else {
            false
        };
        debug!(
            "Get solved={} for collection='{}', puzzle_index={}, key={}",
            solved,
            collection.id(),
            puzzle_index,
            key.unwrap_or_else(|| "none".to_string())
        );
        let backend = self.settings.backend().unwrap();
        dbg!(backend);
        solved
    }

    pub fn set_solved(
        &self,
        solved: bool,
        collection: &PuzzleConfigCollection,
        puzzle_index: usize,
        extension: &Option<PuzzleTypeExtension>,
    ) {
        let collection_dict = self.settings.get::<VariantDict>("solved");
        let puzzle_dict = collection_dict
            .lookup_value(collection.id(), Some(VariantTy::DICTIONARY))
            .map(|v| v.get::<VariantDict>().unwrap())
            .unwrap_or_else(|| VariantDict::new(None));

        let key = puzzle_key(collection, puzzle_index, extension);
        if let Some(key) = &key {
            puzzle_dict.insert(key, &Variant::from(solved));
        }

        collection_dict.insert(collection.id(), &Variant::from(puzzle_dict));
        let result = self.settings.set("solved", &Variant::from(collection_dict));
        match result {
            Ok(_) => {
                debug!(
                    "Set solved={} for collection='{}', puzzle_index={}, key={}",
                    solved,
                    collection.id(),
                    puzzle_index,
                    key.unwrap_or_else(|| "none".to_string())
                )
            }
            Err(_) => {
                error!(
                    "Failed to set solved={} for collection='{}', puzzle_index={}, key={}",
                    solved,
                    collection.id(),
                    puzzle_index,
                    key.unwrap_or_else(|| "none".to_string())
                )
            }
        }
    }

    fn get_puzzle_dict(&self, key: &str, collection: &PuzzleConfigCollection) -> VariantDict {
        let collection_dict = self.settings.get::<VariantDict>(key);
        collection_dict
            .lookup_value(collection.id(), Some(VariantTy::DICTIONARY))
            .map(|v| v.get::<VariantDict>().unwrap())
            .unwrap_or_else(|| VariantDict::new(None))
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
