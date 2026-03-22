use crate::model::puzzle::PuzzleModel;
use crate::model::puzzle_meta::PuzzleMeta;
use crate::model::stars;
use crate::model::store::with_puzzle_collection_store;
use adw::glib;
use adw::prelude::ObjectExt;
use adw::subclass::prelude::*;
use puzzle_config::PuzzleConfigCollection;

const PROGRESS_CHANGED_SIGNAL_NAME: &str = "progress-changed";
const DELETED_SIGNAL_NAME: &str = "deleted";

mod imp {
    use super::*;
    use crate::model::puzzle::PuzzleModel;
    use adw::glib::subclass::Signal;
    use adw::glib::Properties;
    use std::cell::OnceCell;
    use std::sync::OnceLock;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::CollectionModel)]
    pub struct PuzzledCollectionModel {
        pub config: OnceCell<PuzzleConfigCollection>,
        pub puzzles: OnceCell<Vec<PuzzleModel>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledCollectionModel {
        const NAME: &'static str = "PuzzledCollectionModel";
        type Type = CollectionModel;
        type ParentType = glib::Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for PuzzledCollectionModel {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder(PROGRESS_CHANGED_SIGNAL_NAME).build(),
                    Signal::builder(DELETED_SIGNAL_NAME).build(),
                ]
            })
        }
    }
}

glib::wrapper! {
    pub struct CollectionModel(ObjectSubclass<imp::PuzzledCollectionModel>);
}

impl CollectionModel {
    /// Creates a new CollectionModel from the given config.
    /// It provides access to metadata and caches it to reduce calls to the backend.
    /// This constructor also creates models for the puzzles which can be accessed
    /// using the [Self::puzzles()] method.
    ///
    /// The [PuzzleMeta] instance is used to get the initial data from the backend.
    /// This should be reused when creating multiple models at once.
    ///
    /// # Arguments
    ///
    /// * `config`: the config describing the collection
    /// * `puzzle_meta`: used to get metadata
    ///
    /// returns: CollectionModel
    pub fn new(config: PuzzleConfigCollection, puzzle_meta: &PuzzleMeta) -> Self {
        let obj: CollectionModel = glib::Object::builder().build();
        let imp = obj.imp();

        let puzzle_configs = config.puzzles().clone();

        imp.config
            .set(config)
            .expect("Failed to set config for CollectionModel");

        let puzzles: Vec<PuzzleModel> = puzzle_configs
            .into_iter()
            .map(|puzzle_config| {
                let puzzle = PuzzleModel::new(&obj, puzzle_config, puzzle_meta.clone());

                puzzle.connect_progress_improved({
                    let collection = obj.clone();
                    move || {
                        collection.emit_progress_changed();
                    }
                });

                puzzle
            })
            .collect();
        imp.puzzles
            .set(puzzles)
            .expect("Failed to set puzzles for CollectionModel");

        obj
    }

    /// Returns the config defining the puzzles and metadata of the collection.
    pub fn config(&self) -> &PuzzleConfigCollection {
        self.imp().config.get().unwrap()
    }

    /// Returns the puzzle models of the collection.
    pub fn puzzles(&self) -> &Vec<PuzzleModel> {
        self.imp().puzzles.get().unwrap()
    }

    /// Calculates the stars reached for the puzzles in this collection and how many
    /// can be reached in total.
    ///
    /// returns: (reached_stars, total_stars)
    pub fn stars(&self) -> (u32, u32) {
        let (stars_reached, stars_total) = self
            .puzzles()
            .iter()
            .filter(|p| !p.config().is_unsolvable())
            .map(|p| {
                let solved = p.is_solved_default();
                let best_hint_count = p.best_hint_count_default();
                stars::calculate_stars(solved, best_hint_count, p.config().difficulty())
            })
            .fold((0, 0), |(reached, total), stars| {
                (reached + stars.reached(), total + stars.total())
            });
        (stars_reached, stars_total)
    }

    /// Marks all puzzles as unsolved and emits the `progress_changed` signal for UIs to
    /// update.
    pub fn mark_all_as_unsolved(&self) {
        for puzzle in self.puzzles() {
            puzzle.mark_as_unsolved();
        }
        self.emit_progress_changed();
    }

    /// Connects to the `progress_changes` signal.
    /// This is emitted, when the solved status of a puzzle changes or
    /// all puzzles are marked as unsolved.
    ///
    /// # Arguments
    ///
    /// * `callback`: the callback to call, when the signal is emitted
    ///
    /// returns: ()
    pub fn connect_progress_changed<F: Fn() + 'static>(&self, callback: F) {
        self.connect_local(PROGRESS_CHANGED_SIGNAL_NAME, false, move |_| {
            callback();
            None
        });
    }

    fn emit_progress_changed(&self) {
        self.emit_by_name::<()>(PROGRESS_CHANGED_SIGNAL_NAME, &[]);
    }

    /// Deletes the collection from the collection store and emits the `deleted` signal.
    pub fn delete(&self) {
        with_puzzle_collection_store(|store| {
            store.remove_community_collection(self.config().id());
        });
        self.emit_deleted();
    }

    /// Connects to the `deleted` signal.
    /// This is emitted, when the collection is deleted.
    ///
    /// # Arguments
    ///
    /// * `callback`: the callback to call, when the signal is emitted
    ///
    /// returns: ()
    pub fn connect_deleted<F: Fn() + 'static>(&self, callback: F) {
        self.connect_local(DELETED_SIGNAL_NAME, false, move |_| {
            callback();
            None
        });
    }

    fn emit_deleted(&self) {
        self.emit_by_name::<()>(DELETED_SIGNAL_NAME, &[]);
    }
}
