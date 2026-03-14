use crate::model::puzzle::PuzzleModel;
use crate::model::puzzle_meta::PuzzleMeta;
use crate::model::stars;
use adw::glib;
use adw::subclass::prelude::*;
use puzzle_config::{
    PreviewConfig, ProgressionConfig, PuzzleConfigCollection, PuzzleDifficultyConfig,
};

mod imp {
    use super::*;
    use crate::model::puzzle::PuzzleModel;
    use adw::glib::Properties;
    use std::sync::RwLock;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::CollectionModel)]
    pub struct PuzzledCollectionModel {
        pub immutable_inner: RwLock<Option<ImmutableInner>>,
        pub puzzles: RwLock<Vec<PuzzleModel>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledCollectionModel {
        const NAME: &'static str = "PuzzledCollectionModel";
        type Type = CollectionModel;
        type ParentType = glib::Object;

        fn class_init(_: &mut Self::Class) {}

        fn instance_init(_: &glib::subclass::InitializingObject<Self>) {}
    }

    #[glib::derived_properties]
    impl ObjectImpl for PuzzledCollectionModel {}
}

glib::wrapper! {
    pub struct CollectionModel(ObjectSubclass<imp::PuzzledCollectionModel>);
}

impl CollectionModel {
    pub fn new(config: PuzzleConfigCollection, puzzle_meta: PuzzleMeta) -> Self {
        let obj: CollectionModel = glib::Object::builder().build();
        let imp = obj.imp();

        let puzzles: Vec<PuzzleModel> = config
            .puzzles()
            .iter()
            .map(|puzzle_config| PuzzleModel::new(&obj, puzzle_config.clone(), puzzle_meta.clone()))
            .collect();
        imp.puzzles
            .write()
            .expect("Failed to acquire write lock on puzzles")
            .extend(puzzles);

        let inner = ImmutableInner { config };
        imp.immutable_inner
            .write()
            .expect("Failed to acquire write lock on immutable inner")
            .replace(inner);
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

    pub fn author(&self) -> String {
        let imp = self.imp();
        let inner = imp
            .immutable_inner
            .read()
            .expect("Failed to acquire read lock on immutable inner");
        let inner = inner.as_ref().expect("Immutable inner should be set");
        inner.config.author().to_string()
    }

    pub fn version(&self) -> Option<String> {
        let imp = self.imp();
        let inner = imp
            .immutable_inner
            .read()
            .expect("Failed to acquire read lock on immutable inner");
        let inner = inner.as_ref().expect("Immutable inner should be set");
        inner.config.version().clone()
    }

    pub fn preview(&self) -> PreviewConfig {
        let imp = self.imp();
        let inner = imp
            .immutable_inner
            .read()
            .expect("Failed to acquire read lock on immutable inner");
        let inner = inner.as_ref().expect("Immutable inner should be set");
        inner.config.preview().clone()
    }

    pub fn puzzles(&self) -> Vec<PuzzleModel> {
        self.imp()
            .puzzles
            .read()
            .expect("Failed to acquire read lock on puzzles")
            .clone()
    }

    pub fn average_difficulty(&self) -> Option<PuzzleDifficultyConfig> {
        let imp = self.imp();
        let inner = imp
            .immutable_inner
            .read()
            .expect("Failed to acquire read lock on immutable inner");
        let inner = inner.as_ref().expect("Immutable inner should be set");
        inner.config.average_difficulty()
    }

    pub fn progression(&self) -> ProgressionConfig {
        let imp = self.imp();
        let inner = imp
            .immutable_inner
            .read()
            .expect("Failed to acquire read lock on immutable inner");
        let inner = inner.as_ref().expect("Immutable inner should be set");
        inner.config.progression().clone()
    }

    pub fn stars(&self) -> (u32, u32) {
        let (stars_reached, stars_total) = self
            .puzzles()
            .iter()
            .filter(|p| !p.is_unsolvable())
            .map(|p| {
                let solved = p.is_solved();
                let best_hint_count = p.best_hint_count();
                stars::calculate_stars(solved, best_hint_count, &p.difficulty())
            })
            .fold((0, 0), |(reached, total), stars| {
                (reached + stars.reached(), total + stars.total())
            });
        (stars_reached, stars_total)
    }

    pub fn config(&self) -> PuzzleConfigCollection {
        let imp = self.imp();
        let inner = imp
            .immutable_inner
            .read()
            .expect("Failed to acquire read lock on immutable inner");
        let inner = inner.as_ref().expect("Immutable inner should be set");
        inner.config.clone()
    }
}

#[derive(Debug)]
struct ImmutableInner {
    config: PuzzleConfigCollection,
}

impl From<PuzzleConfigCollection> for CollectionModel {
    fn from(collection: PuzzleConfigCollection) -> Self {
        let puzzle_meta = PuzzleMeta::new();
        CollectionModel::new(collection.clone(), puzzle_meta)
    }
}
