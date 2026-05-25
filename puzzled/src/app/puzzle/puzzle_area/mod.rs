mod board;
mod highlight;
mod hint;
mod layout;
pub mod puzzle_state;
mod tile;

use crate::app::puzzle::puzzle_area::puzzle_state::PuzzleState;
use crate::model::extension::PuzzleTypeExtension;
use crate::model::placement::PlacementModel;
use crate::model::puzzle::PuzzleModel;
use crate::offset::PixelOffset;
use crate::window::PuzzledWindow;
use adw::gio;
use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{glib, Widget};
use log::debug;

const TILE_MOVED_SIGNAL_NAME: &str = "tile-moved";

mod imp {
    use super::*;
    use crate::app::components::board::BoardView;
    use crate::app::components::tile::TileView;
    use crate::model::placement::PlacementModel;
    use adw::glib::subclass::Signal;
    use std::cell::{OnceCell, RefCell};
    use std::sync::OnceLock;

    #[derive(Debug, Default)]
    pub struct PuzzledPuzzleArea {
        pub window: OnceCell<PuzzledWindow>,
        pub(super) placement_model: RefCell<Option<PlacementModel>>,
        pub board: RefCell<Option<BoardView>>,
        pub tiles: RefCell<Vec<TileView>>,
        pub hint_tile: RefCell<Option<TileView>>,
        pub elements_in_fixed: RefCell<Vec<Widget>>,
        pub puzzle: RefCell<Option<PuzzleModel>>,
        pub puzzle_type_extension: RefCell<Option<PuzzleTypeExtension>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledPuzzleArea {
        const NAME: &'static str = "PuzzledPuzzleArea";
        type Type = PuzzleArea;
        type ParentType = gtk::Fixed;
    }

    impl ObjectImpl for PuzzledPuzzleArea {
        fn signals() -> &'static [Signal] {
            static SIGNALS: OnceLock<Vec<Signal>> = OnceLock::new();
            SIGNALS.get_or_init(|| vec![Signal::builder(TILE_MOVED_SIGNAL_NAME).build()])
        }

        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.post_construct_setup_layout();
        }
    }
    impl WidgetImpl for PuzzledPuzzleArea {}
    impl FixedImpl for PuzzledPuzzleArea {}
}

glib::wrapper! {
    pub struct PuzzleArea(ObjectSubclass<imp::PuzzledPuzzleArea>)
        @extends Widget, gtk::Fixed,
         @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gio::ActionGroup, gio::ActionMap;
}

impl PuzzleArea {
    pub fn set_window(&self, window: PuzzledWindow) {
        self.imp()
            .window
            .set(window)
            .expect("Failed to set window for PuzzlePage");
    }

    pub(super) fn add(&self, widget: &Widget, pos: &PixelOffset) {
        self.put(widget, pos.0, pos.1);
        self.imp()
            .elements_in_fixed
            .borrow_mut()
            .push(widget.clone());
    }

    /// Set up the puzzle configuration from the current state.
    ///
    /// This adds the board and tiles to the puzzle area based on the current puzzle configuration.
    /// Final positions and layout are handled in `update_layout()`. Before that, all elements are
    /// added at position (0, 0) and will be moved later.
    pub fn show_puzzle(&self, puzzle: &PuzzleModel) {
        self.imp().puzzle.replace(Some(puzzle.clone()));
        let puzzle_config = puzzle.config();
        self.clear_elements();

        let placement_model = PlacementModel::new(puzzle);
        placement_model.connect_tile_moved({
            let self_clone = self.clone();
            move || self_clone.run_on_tile_moved()
        });
        self.imp().placement_model.replace(Some(placement_model));

        self.setup_board(puzzle_config);

        for (i, tile) in puzzle_config.tiles().iter().enumerate() {
            self.setup_tile(tile, i);
        }

        self.update_highlights();
        self.update_layout();
    }

    pub fn set_puzzle_type_extension(&self, puzzle_type_extension: Option<PuzzleTypeExtension>) {
        self.imp()
            .puzzle_type_extension
            .replace(puzzle_type_extension);
        self.update_highlights();
        self.update_layout();
    }

    pub fn run_on_tile_moved(&self) {
        self.update_highlights();
        self.update_layout();
        self.emit_tile_moved();
    }

    pub fn connect_tile_moved<F: Fn() + 'static>(&self, callback: F) {
        self.connect_local(TILE_MOVED_SIGNAL_NAME, false, move |_| {
            callback();
            None
        });
    }

    fn emit_tile_moved(&self) {
        debug!("Emitting tile moved signal",);
        self.emit_by_name::<()>(TILE_MOVED_SIGNAL_NAME, &[]);
    }

    fn clear_elements(&self) {
        let mut elements_in_fixed = self.imp().elements_in_fixed.borrow_mut();
        elements_in_fixed.drain(..).for_each(|e| self.remove(&e));
        self.imp().tiles.replace(vec![]);
        self.imp().board.replace(None);
        self.remove_hint_tile();
    }

    pub fn extract_puzzle_state(&self) -> Result<PuzzleState, String> {
        let placement_model = self.imp().placement_model.borrow();
        let extension = self.imp().puzzle_type_extension.borrow();
        if let Some(placement_model) = placement_model.as_ref() {
            placement_model.extract_puzzle_state(extension)
        } else {
            Err("No placement model set up".to_owned())
        }
    }
}
