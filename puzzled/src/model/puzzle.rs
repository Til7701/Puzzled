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

        fn class_init(_: &mut Self::Class) {}

        fn instance_init(_: &glib::subclass::InitializingObject<Self>) {}
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
            &collection.config(),
            obj.config().index(),
            &Some(default_extension.clone()),
        );
        imp.solved
            .borrow_mut()
            .insert(Some(default_extension.clone()), solved);

        let hints = puzzle_meta.hints(
            &collection.config(),
            obj.config().index(),
            &Some(default_extension.clone()),
        );
        imp.hints_used
            .borrow_mut()
            .insert(Some(default_extension), hints);

        obj
    }

    pub fn config(&self) -> &PuzzleConfig {
        self.imp().config.get().unwrap()
    }

    pub fn collection(&self) -> &CollectionModel {
        self.imp().collection.get().unwrap()
    }

    pub fn is_solved(&self, extension: &Option<PuzzleTypeExtension>) -> bool {
        let imp = self.imp();
        *imp.solved.borrow().get(extension).unwrap_or(&false)
    }

    pub fn is_solved_default(&self) -> bool {
        let imp = self.imp();
        *imp.solved
            .borrow()
            .get(&Some(imp.default_extension.get().unwrap().clone()))
            .unwrap_or(&false)
    }

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

    pub fn best_hint_count(&self, extension: &Option<PuzzleTypeExtension>) -> Option<u32> {
        let imp = self.imp();
        *imp.hints_used.borrow().get(extension).unwrap_or(&None)
    }

    pub fn best_hint_count_default(&self) -> Option<u32> {
        let imp = self.imp();
        *imp.hints_used
            .borrow()
            .get(&Some(imp.default_extension.get().unwrap().clone()))
            .unwrap_or(&None)
    }

    pub fn stars(&self, extension: &Option<PuzzleTypeExtension>) -> Stars {
        let solved = self.is_solved(extension);
        let best_hint_count = self.best_hint_count(extension);
        let difficulty = self.config().difficulty();
        stars::calculate_stars(solved, best_hint_count, &difficulty)
    }

    pub fn stars_default(&self) -> Stars {
        let solved = self.is_solved_default();
        let best_hint_count = self.best_hint_count_default();
        let difficulty = self.config().difficulty();
        stars::calculate_stars(solved, best_hint_count, &difficulty)
    }

    pub fn mark_as_unsolved(&self) {
        let imp = self.imp();
        imp.solved.borrow_mut().clear();
        imp.hints_used.borrow_mut().clear();
        imp.stars.borrow_mut().clear();
        self.emit_marked_unsolved();
    }

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

    pub fn connect_marked_unsolved<F: Fn() + 'static>(&self, callback: F) {
        self.connect_local(MARKED_UNSOLVED_SIGNAL_NAME, false, move |_| {
            callback();
            None
        });
    }

    fn emit_marked_unsolved(&self) {
        self.emit_by_name::<()>(MARKED_UNSOLVED_SIGNAL_NAME, &[]);
    }

    pub fn has_next_puzzle(&self) -> bool {
        let imp = self.imp();
        let self_index = imp.config.get().unwrap().index();
        let collection = imp.collection.get().unwrap();

        collection.puzzles().get(self_index + 1).is_some()
    }

    pub fn next_puzzle(&self) -> Option<&PuzzleModel> {
        let imp = self.imp();
        let self_index = imp.config.get().unwrap().index();
        let collection = imp.collection.get().unwrap();
        collection.puzzles().get(self_index + 1)
    }
}
