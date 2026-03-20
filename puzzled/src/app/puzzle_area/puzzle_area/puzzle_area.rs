use crate::model::puzzle::PuzzleModel;
use adw::gio;
use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;

mod imp {
    use super::*;
    use crate::components::board::BoardView;
    use puzzle_solver::tile::Tile;
    use std::cell::RefCell;

    #[derive(Debug, Default)]
    pub struct PuzzledPuzzleArea {
        pub board: RefCell<Option<BoardView>>,
        pub tiles: RefCell<Vec<Tile>>,

        pub puzzle: RefCell<Option<PuzzleModel>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledPuzzleArea {
        const NAME: &'static str = "PuzzledPuzzleArea";
        type Type = PuzzleArea;
        type ParentType = gtk::Fixed;

        fn class_init(klass: &mut Self::Class) {}

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {}
    }

    impl ObjectImpl for PuzzledPuzzleArea {}
    impl WidgetImpl for PuzzledPuzzleArea {}
    impl FixedImpl for PuzzledPuzzleArea {}
}

glib::wrapper! {
    pub struct PuzzleArea(ObjectSubclass<imp::PuzzledPuzzleArea>)
        @extends gtk::Widget, gtk::Fixed,
         @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gio::ActionGroup, gio::ActionMap;
}

impl PuzzleArea {
    pub fn show_puzzle(&self, puzzle: &PuzzleModel) {}
}
