use crate::model::collection::CollectionModel;
use crate::model::puzzle_meta::PuzzleMeta;
use crate::model::stars::Stars;
use adw::glib;
use adw::subclass::prelude::*;
use puzzle_config::{BoardConfig, PuzzleConfig, PuzzleDifficultyConfig, TileConfig};
use std::collections::HashMap;

mod imp {
    use super::*;
    use adw::glib::Properties;
    use std::sync::RwLock;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::PuzzleModel)]
    pub struct PuzzledPuzzleModel {
        pub(super) immutable_inner: RwLock<Option<ImmutableInner>>,
        pub(super) collection_id: RwLock<Option<String>>,
        pub(super) solved: RwLock<bool>,
        pub(super) hints_used: RwLock<Option<u32>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledPuzzleModel {
        const NAME: &'static str = "PuzzledPuzzleModel";
        type Type = PuzzleModel;
        type ParentType = glib::Object;

        fn class_init(_: &mut Self::Class) {}

        fn instance_init(_: &glib::subclass::InitializingObject<Self>) {}
    }

    #[glib::derived_properties]
    impl ObjectImpl for PuzzledPuzzleModel {}
}

glib::wrapper! {
    pub struct PuzzleModel(ObjectSubclass<imp::PuzzledPuzzleModel>);
}

impl PuzzleModel {
    pub fn new(
        collection: &CollectionModel,
        config: PuzzleConfig,
        puzzle_meta: PuzzleMeta,
    ) -> Self {
        let obj: PuzzleModel = glib::Object::builder().build();
        let imp = obj.imp();

        let inner = ImmutableInner { config };
        imp.immutable_inner
            .write()
            .expect("Failed to acquire write lock on immutable inner")
            .replace(inner);

        imp.collection_id
            .write()
            .expect("Failed to acquire write lock on collection_id")
            .replace(collection.id());

        let solved = puzzle_meta.is_solved(
            &collection.config(),
            obj.index(),
            &None, // TODO: pass the actual extension
        );

        obj
    }

    pub fn id(&self) -> String {
        let imp = self.imp();
        let inner = imp
            .immutable_inner
            .read()
            .expect("Failed to acquire read lock on immutable inner");
        let inner = inner.as_ref().expect("Immutable inner should be set");
        inner.config.id().to_string()
    }

    pub fn index(&self) -> usize {
        let imp = self.imp();
        let inner = imp
            .immutable_inner
            .read()
            .expect("Failed to acquire read lock on immutable inner");
        let inner = inner.as_ref().expect("Immutable inner should be set");
        inner.config.index()
    }

    pub fn name(&self) -> String {
        let imp = self.imp();
        let inner = imp
            .immutable_inner
            .read()
            .expect("Failed to acquire read lock on immutable inner");
        let inner = inner.as_ref().expect("Immutable inner should be set");
        inner.config.name().to_string()
    }

    pub fn description(&self) -> Option<String> {
        let imp = self.imp();
        let inner = imp
            .immutable_inner
            .read()
            .expect("Failed to acquire read lock on immutable inner");
        let inner = inner.as_ref().expect("Immutable inner should be set");
        inner.config.description().clone()
    }

    pub fn is_solved(&self) -> bool {
        let imp = self.imp();
        *imp.solved
            .read()
            .expect("Failed to acquire read lock on solved")
    }

    pub fn set_solved(&self, hints: u32) {
        todo!()
    }

    pub fn is_unsolvable(&self) -> bool {
        let imp = self.imp();
        let inner = imp
            .immutable_inner
            .read()
            .expect("Failed to acquire read lock on immutable inner");
        let inner = inner.as_ref().expect("Immutable inner should be set");
        inner.config.is_unsolvable()
    }

    pub fn difficulty(&self) -> Option<PuzzleDifficultyConfig> {
        let imp = self.imp();
        let inner = imp
            .immutable_inner
            .read()
            .expect("Failed to acquire read lock on immutable inner");
        let inner = inner.as_ref().expect("Immutable inner should be set");
        *inner.config.difficulty()
    }

    pub fn best_hint_count(&self) -> Option<u32> {
        let imp = self.imp();
        *imp.hints_used
            .read()
            .expect("Failed to acquire read lock on hints_used")
    }

    pub fn stars(&self) -> Stars {
        todo!()
    }

    pub fn board_config(&self) -> BoardConfig {
        let imp = self.imp();
        let inner = imp
            .immutable_inner
            .read()
            .expect("Failed to acquire read lock on immutable inner");
        let inner = inner.as_ref().expect("Immutable inner should be set");
        inner.config.board_config().clone()
    }

    pub fn tiles(&self) -> Vec<TileConfig> {
        let imp = self.imp();
        let inner = imp
            .immutable_inner
            .read()
            .expect("Failed to acquire read lock on immutable inner");
        let inner = inner.as_ref().expect("Immutable inner should be set");
        inner.config.tiles().clone()
    }

    pub fn additional_info(&self) -> Option<HashMap<String, String>> {
        let imp = self.imp();
        let inner = imp
            .immutable_inner
            .read()
            .expect("Failed to acquire read lock on immutable inner");
        let inner = inner.as_ref().expect("Immutable inner should be set");
        inner.config.additional_info().clone()
    }
}

#[derive(Debug)]
struct ImmutableInner {
    config: PuzzleConfig,
}
