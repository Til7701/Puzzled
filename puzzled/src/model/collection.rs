use crate::model::puzzle::PuzzleModel;
use crate::model::puzzle_meta::PuzzleMeta;
use crate::model::stars;
use adw::glib;
use adw::subclass::prelude::*;
use puzzle_config::PuzzleConfigCollection;

mod imp {
    use super::*;
    use crate::model::puzzle::PuzzleModel;
    use adw::glib::Properties;
    use std::cell::OnceCell;

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
    impl ObjectImpl for PuzzledCollectionModel {}
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
            .map(|puzzle_config| PuzzleModel::new(&obj, puzzle_config, puzzle_meta.clone()))
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
}
