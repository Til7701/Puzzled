use crate::model::collection::CollectionModel;
use crate::model::extension::PuzzleTypeExtension;
use crate::model::puzzle_meta::PuzzleMeta;
use crate::model::stars;
use crate::model::stars::Stars;
use adw::glib;
use adw::prelude::ObjectExt;
use adw::subclass::prelude::*;
use puzzle_config::PuzzleConfig;

const PROGRESS_IMPROVED_SIGNAL_NAME: &str = "progress-improved";
const MARKED_UNSOLVED_SIGNAL_NAME: &str = "marked-unsolved";

mod imp {
    use super::*;
    use adw::glib::subclass::Signal;
    use adw::glib::Properties;
    use std::cell::{OnceCell, RefCell};
    use std::collections::HashMap;
    use std::sync::OnceLock;

    #[derive(Debug, Default, Properties)]
    #[properties(wrapper_type = super::PuzzleModel)]
    pub struct PuzzledPuzzleModel {
        pub(super) collection: OnceCell<CollectionModel>,
        pub(super) config: OnceCell<PuzzleConfig>,
        pub(super) default_extension: OnceCell<PuzzleTypeExtension>,
        pub(super) solved: RefCell<HashMap<Option<PuzzleTypeExtension>, bool>>,
        pub(super) hints_used: RefCell<HashMap<Option<PuzzleTypeExtension>, Option<u32>>>,
        pub(super) stars: RefCell<HashMap<Option<PuzzleTypeExtension>, Stars>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledPuzzleModel {
        const NAME: &'static str = "PuzzledPuzzleModel";
        type Type = PuzzleModel;
        type ParentType = glib::Object;
    }

    #[glib::derived_properties]
    impl ObjectImpl for PuzzledPuzzleModel {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| {
                vec![
                    Signal::builder(PROGRESS_IMPROVED_SIGNAL_NAME).build(),
                    Signal::builder(MARKED_UNSOLVED_SIGNAL_NAME).build(),
                ]
            })
        }
    }
}

glib::wrapper! {
    pub struct PuzzleModel(ObjectSubclass<imp::PuzzledPuzzleModel>);
}

impl PuzzleModel {
    /// Creates a new puzzle model.
    ///
    /// The solved state is cached for each puzzle extension.
    ///
    /// # Arguments
    ///
    /// * `collection`: the collection this puzzle belongs to
    /// * `config`: the config defining the puzzle
    /// * `puzzle_meta`: the puzzle meta to get metadata
    ///
    /// returns: PuzzleModel
    pub fn new(
        collection: &CollectionModel,
        config: PuzzleConfig,
        puzzle_meta: PuzzleMeta,
    ) -> Self {
        let obj: PuzzleModel = glib::Object::builder().build();
        let imp = obj.imp();

        let default_extension = PuzzleTypeExtension::default_for_puzzle(&config);
        imp.default_extension
            .set(default_extension.clone())
            .expect("Failed to set default_extension for PuzzleModel");
        imp.config
            .set(config)
            .expect("Failed to set config for PuzzleModel");
        imp.collection
            .set(collection.clone())
            .expect("Failed to set collection for PuzzleModel");

        let solved = puzzle_meta.is_solved(
            collection.config(),
            obj.config().index(),
            &Some(default_extension.clone()),
        );
        imp.solved
            .borrow_mut()
            .insert(Some(default_extension.clone()), solved);

        let hints = puzzle_meta.hints(
            collection.config(),
            obj.config().index(),
            &Some(default_extension.clone()),
        );
        imp.hints_used
            .borrow_mut()
            .insert(Some(default_extension), hints);

        obj
    }

    /// The config defining the puzzle tiles and board.
    pub fn config(&self) -> &PuzzleConfig {
        self.imp().config.get().unwrap()
    }

    /// The collection this puzzle belongs to.
    pub fn collection(&self) -> &CollectionModel {
        self.imp().collection.get().unwrap()
    }

    /// Checks if the puzzle is solved for a given extension.
    ///
    /// # Arguments
    ///
    /// * `extension`: the extension to check for
    ///
    /// returns: bool
    pub fn is_solved(&self, extension: &Option<PuzzleTypeExtension>) -> bool {
        let imp = self.imp();
        *imp.solved.borrow().get(extension).unwrap_or(&false)
    }

    /// Checks if the puzzle is solved for the default extension.
    ///
    /// returns: bool
    pub fn is_solved_default(&self) -> bool {
        let imp = self.imp();
        *imp.solved
            .borrow()
            .get(&Some(imp.default_extension.get().unwrap().clone()))
            .unwrap_or(&false)
    }

    /// Returns true, if the previous puzzle has been solved for the default target.
    /// False, if not.
    /// None, if there is no previous puzzle.
    pub fn is_previous_solved_default(&self) -> Option<bool> {
        let imp = self.imp();
        let config = imp.config.get().unwrap();

        let this_index = config.index();
        if this_index == 0 {
            return None;
        }

        let collection = imp.collection.get().unwrap();
        Some(collection.puzzles()[this_index - 1].is_solved_default())
    }

    /// Sets the puzzle as solved for the given extension.
    /// The hints are the amount of hints used to solve the puzzle.
    ///
    /// # Arguments
    ///
    /// * `hints`: the amount of hints used to solve the puzzle
    /// * `extension`: the extension the puzzle has been solved for
    ///
    /// returns: ()
    pub fn set_solved(&self, hints: u32, extension: &Option<PuzzleTypeExtension>) {
        let imp = self.imp();
        imp.solved.borrow_mut().insert(extension.clone(), true);
        imp.hints_used
            .borrow_mut()
            .insert(extension.clone(), Some(hints));

        let puzzle_meta = PuzzleMeta::new();
        puzzle_meta.set_solved(
            true,
            self.imp().collection.get().unwrap().config(),
            self.config().index(),
            extension,
        );
        puzzle_meta.set_hints(
            hints,
            self.imp().collection.get().unwrap().config(),
            self.config().index(),
            extension,
        );
        self.emit_progress_improved();
    }

    /// Returns the amount of hints used to solve the puzzle with the given
    /// extension if it has been solved.
    ///
    /// # Arguments
    ///
    /// * `extension`: the extension to get the hint count for.
    ///
    /// returns: Option<u32>
    pub fn best_hint_count(&self, extension: &Option<PuzzleTypeExtension>) -> Option<u32> {
        let imp = self.imp();
        *imp.hints_used.borrow().get(extension).unwrap_or(&None)
    }

    /// Returns the amount of hints used to solve the puzzle with the default extension,
    /// if it has been solved.
    ///
    /// returns: Option<u32>
    pub fn best_hint_count_default(&self) -> Option<u32> {
        let imp = self.imp();
        *imp.hints_used
            .borrow()
            .get(&Some(imp.default_extension.get().unwrap().clone()))
            .unwrap_or(&None)
    }

    /// Returns the stars instance for this puzzle and the given extension.
    ///
    /// # Arguments
    ///
    /// * `extension`: the extension to get the stars for
    ///
    /// returns: Stars
    pub fn stars(&self, extension: &Option<PuzzleTypeExtension>) -> Stars {
        let solved = self.is_solved(extension);
        let best_hint_count = self.best_hint_count(extension);
        let difficulty = self.config().difficulty();
        stars::calculate_stars(solved, best_hint_count, difficulty)
    }

    /// Returns the stars instance for this puzzle and the default extension.
    ///
    /// returns: Stars
    pub fn stars_default(&self) -> Stars {
        let solved = self.is_solved_default();
        let best_hint_count = self.best_hint_count_default();
        let difficulty = self.config().difficulty();
        stars::calculate_stars(solved, best_hint_count, difficulty)
    }

    /// Marks the puzzle as unsolved and emits the signal.
    /// This also clears all caches this model has.
    pub fn mark_as_unsolved(&self) {
        let imp = self.imp();
        imp.solved.borrow_mut().clear();
        imp.hints_used.borrow_mut().clear();
        imp.stars.borrow_mut().clear();
        self.emit_marked_unsolved();
    }

    /// Connects to the `progress_improved` signal.
    /// This signal is emitted, if the solved status changed to true or the hints used improved.
    /// This can be used to update the UI.
    ///
    /// # Arguments
    ///
    /// * `callback`: the callback to call when the signal is emitted
    ///
    /// returns: ()
    pub fn connect_progress_improved<F: Fn() + 'static>(&self, callback: F) {
        self.connect_local(PROGRESS_IMPROVED_SIGNAL_NAME, false, move |_| {
            callback();
            None
        });
    }

    fn emit_progress_improved(&self) {
        self.emit_by_name::<()>(PROGRESS_IMPROVED_SIGNAL_NAME, &[]);
        self.next_puzzle()
            .iter()
            .for_each(|p| p.emit_progress_improved());
    }

    /// Connects to the `marked_unsolved` signal.
    /// This signal is emitted, if the puzzle is marked as unsolved.
    /// This can be used to update the UI.
    ///
    /// # Arguments
    ///
    /// * `callback`: the callback to call when the signal is emitted
    ///
    /// returns: ()
    pub fn connect_marked_unsolved<F: Fn() + 'static>(&self, callback: F) {
        self.connect_local(MARKED_UNSOLVED_SIGNAL_NAME, false, move |_| {
            callback();
            None
        });
    }

    fn emit_marked_unsolved(&self) {
        self.emit_by_name::<()>(MARKED_UNSOLVED_SIGNAL_NAME, &[]);
    }

    /// Returns true, if there is a next puzzle in the collection. False, if not.
    pub fn has_next_puzzle(&self) -> bool {
        let imp = self.imp();
        let self_index = imp.config.get().unwrap().index();
        let collection = imp.collection.get().unwrap();

        collection.puzzles().get(self_index + 1).is_some()
    }

    /// Returns the next puzzle of the collection this puzzle is in.
    /// None, if there is no next puzzle in the collection.
    pub fn next_puzzle(&self) -> Option<&PuzzleModel> {
        let imp = self.imp();
        let self_index = imp.config.get().unwrap().index();
        let collection = imp.collection.get().unwrap();
        collection.puzzles().get(self_index + 1)
    }
}
