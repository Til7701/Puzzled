use crate::model::puzzle::PuzzleModel;
use crate::model::puzzle_meta::PuzzleMeta;
use crate::model::stars;
use adw::glib;
use adw::prelude::ObjectExt;
use adw::subclass::prelude::*;
use puzzle_config::PuzzleConfigCollection;

const PROGRESS_CHANGED_SIGNAL_NAME: &str = "progress-changed";

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

        fn class_init(_: &mut Self::Class) {}

        fn instance_init(_: &glib::subclass::InitializingObject<Self>) {}
    }

    #[glib::derived_properties]
    impl ObjectImpl for PuzzledCollectionModel {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| vec![Signal::builder(PROGRESS_CHANGED_SIGNAL_NAME).build()])
        }
    }
}

glib::wrapper! {
    pub struct CollectionModel(ObjectSubclass<imp::PuzzledCollectionModel>);
}

impl CollectionModel {
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

    pub fn config(&self) -> &PuzzleConfigCollection {
        self.imp().config.get().unwrap()
    }

    pub fn puzzles(&self) -> &Vec<PuzzleModel> {
        self.imp().puzzles.get().unwrap()
    }

    pub fn stars(&self) -> (u32, u32) {
        // TODO cache
        let (stars_reached, stars_total) = self
            .puzzles()
            .iter()
            .filter(|p| !p.config().is_unsolvable())
            .map(|p| {
                let solved = p.is_solved_default();
                let best_hint_count = p.best_hint_count_default();
                stars::calculate_stars(solved, best_hint_count, &p.config().difficulty())
            })
            .fold((0, 0), |(reached, total), stars| {
                (reached + stars.reached(), total + stars.total())
            });
        (stars_reached, stars_total)
    }

    pub fn mark_all_as_unsolved(&self) {
        for puzzle in self.puzzles() {
            puzzle.mark_as_unsolved();
        }
        self.emit_progress_changed();
    }

    pub fn connect_progress_changed<F: Fn() + 'static>(&self, callback: F) {
        self.connect_local(PROGRESS_CHANGED_SIGNAL_NAME, false, move |_| {
            callback();
            None
        });
    }

    fn emit_progress_changed(&self) {
        self.emit_by_name::<()>(PROGRESS_CHANGED_SIGNAL_NAME, &[]);
    }
}
