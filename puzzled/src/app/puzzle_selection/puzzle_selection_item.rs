use crate::app::components::board::BoardView;
use crate::app::components::tile::TileView;
use crate::app::puzzle_selection::puzzle_mod::PuzzleModState;
use crate::model::puzzle::PuzzleModel;
use adw::gio;
use adw::glib;
use adw::prelude::ObjectExt;
use adw::subclass::prelude::*;
use gtk::prelude::{BoxExt, FixedExt, WidgetExt};
use gtk::{Align, Fixed, Widget};
use log::{debug, error};
use puzzle_config::{BoardConfig, ProgressionConfig, TileConfig};
use std::thread::sleep;
use std::time::Duration;

/// How many pixels a cell should have in the preview of tiles and boards.
/// This is NOT the total size of the preview.
const PREVIEW_CELL_SIZE: f64 = 20.0;

mod imp {
    use super::*;
    use crate::app::components::info_pill::InfoPill;
    use crate::app::puzzle_selection::puzzle_mod::PuzzleMod;
    use adw::glib::{Properties, derived_properties};
    use std::cell::Cell;

    #[derive(Debug, Default, gtk::CompositeTemplate, Properties)]
    #[template(resource = "/de/til7701/Puzzled/ui/widget/puzzle-selection-item.ui")]
    #[properties(wrapper_type = super::PuzzleSelectionItem)]
    pub struct PuzzledPuzzleSelectionItem {
        #[template_child]
        pub name: TemplateChild<gtk::Label>,
        #[template_child]
        pub puzzle_mod: TemplateChild<PuzzleMod>,
        #[template_child]
        pub description: TemplateChild<gtk::Label>,
        #[template_child]
        pub info_box: TemplateChild<adw::WrapBox>,
        #[template_child]
        pub board_size_pill: TemplateChild<InfoPill>,
        #[template_child]
        pub cell_count_pill: TemplateChild<InfoPill>,
        #[template_child]
        pub tile_count_pill: TemplateChild<InfoPill>,
        #[template_child]
        pub difficulty_pill: TemplateChild<InfoPill>,

        #[template_child]
        pub tile_preview_parent: TemplateChild<gtk::ScrolledWindow>,
        #[template_child]
        pub tile_preview_fixed: TemplateChild<Fixed>,
        #[template_child]
        pub board_preview_parent: TemplateChild<gtk::Box>,
        #[template_child]
        pub board_preview_box: TemplateChild<gtk::Box>,

        #[property(name = "locked", get, set)]
        locked: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledPuzzleSelectionItem {
        const NAME: &'static str = "PuzzledPuzzleSelectionItem";
        type Type = PuzzleSelectionItem;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[derived_properties]
    impl ObjectImpl for PuzzledPuzzleSelectionItem {}
    impl WidgetImpl for PuzzledPuzzleSelectionItem {}
    impl BoxImpl for PuzzledPuzzleSelectionItem {}
}

glib::wrapper! {
    pub struct PuzzleSelectionItem(ObjectSubclass<imp::PuzzledPuzzleSelectionItem>)
        @extends Widget, gtk::Box,
         @implements gtk::Buildable, gtk::Accessible, gtk::ConstraintTarget,
                  gtk::Native, gio::ActionGroup, gio::ActionMap, gtk::Actionable;
}

impl PuzzleSelectionItem {
    /// Creates a new PuzzleSelectionItem.
    ///
    /// It displays information about the puzzle and can be activated to trigger showing
    /// the puzzle area.
    ///
    /// If the state of the puzzles changes e.g. by solving it, this view updates automatically
    /// using the signals of the puzzle model.
    ///
    /// # Arguments
    ///
    /// returns: PuzzleSelectionItem
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    pub fn update(&self, puzzle: &PuzzleModel) {
        debug!("Updating PuzzleSelectionItem: {}", puzzle.config().name());
        sleep(Duration::from_secs(1));
        let imp = self.imp();
        imp.name.set_text(puzzle.config().name());

        self.update_data(puzzle);

        if let Some(description) = puzzle.config().description() {
            imp.description.set_text(description);
        } else {
            self.remove(&imp.description.get());
        }

        if let Some(difficulty) = puzzle.config().difficulty() {
            let label: String = (*difficulty).into();
            imp.difficulty_pill.set_label(label);
        } else {
            imp.info_box.remove(&imp.difficulty_pill.get());
        }

        Self::create_tiles_preview(puzzle.config().tiles(), &imp.tile_preview_fixed.get());
        Self::create_board_preview(puzzle.config().board_config(), &imp.board_preview_box.get());

        puzzle.connect_progress_improved({
            let obj = self.clone();
            let puzzle = puzzle.clone();
            move || {
                obj.update_data(&puzzle);
            }
        });
        puzzle.connect_marked_unsolved({
            let obj = self.clone();
            let puzzle = puzzle.clone();
            move || {
                obj.update_data(&puzzle);
            }
        });
    }

    /// Updates dynamic data of the puzzle.
    /// This should be called, if the puzzle emits signals for relevant changes.
    fn update_data(&self, puzzle: &PuzzleModel) {
        let imp = self.imp();
        let collection = puzzle.collection();
        let stars = puzzle.stars_default();
        let solved = puzzle.is_solved_default();

        let state = {
            let state = match &collection.config().progression() {
                ProgressionConfig::Any => PuzzleModState::Stars(stars),
                ProgressionConfig::Sequential => {
                    let previous_solved = puzzle.is_previous_solved_default().unwrap_or(true);
                    if solved || previous_solved {
                        PuzzleModState::Stars(stars)
                    } else {
                        PuzzleModState::Locked
                    }
                }
            };
            if let PuzzleModState::Stars(_) = state
                && puzzle.config().is_unsolvable()
            {
                PuzzleModState::Unsolvable
            } else {
                state
            }
        };
        imp.puzzle_mod.set_state(&state);
        self.set_locked(state == PuzzleModState::Locked);

        if state != PuzzleModState::Locked || collection.config().preview().show_board_size() {
            let (width, height) = puzzle.config().board_config().layout().dim();
            imp.board_size_pill
                .set_label(format!("{} x {}", width, height));
            let cell_count = puzzle
                .config()
                .board_config()
                .layout()
                .iter()
                .filter(|c| **c)
                .count();
            imp.cell_count_pill.set_label(format!("{}", cell_count));
        } else {
            imp.info_box.remove(&imp.board_size_pill.get());
            imp.info_box.remove(&imp.cell_count_pill.get());
        }

        if state != PuzzleModState::Locked || collection.config().preview().show_tile_count() {
            let tile_count = puzzle.config().tiles().len();
            imp.tile_count_pill.set_label(format!("{}", tile_count));
        } else {
            imp.info_box.remove(&imp.tile_count_pill.get());
        }

        let show_tile_preview =
            state != PuzzleModState::Locked || collection.config().preview().show_tiles();
        let show_board_preview =
            state != PuzzleModState::Locked || collection.config().preview().show_board();

        imp.tile_preview_parent.set_visible(show_tile_preview);
        imp.board_preview_parent.set_visible(show_board_preview);
    }

    fn create_tiles_preview(tiles: &[TileConfig], fixed: &Fixed) {
        let max_tile_cell_height = tiles
            .iter()
            .map(|tile| tile.base().dim().1)
            .max()
            .unwrap_or(1) as i32;
        let mut current_x_offset_cells = 0;

        for (i, tile) in tiles.iter().enumerate() {
            let tile_view = TileView::new(i, tile.base().clone(), tile.color());

            let tile_height = tile.base().dim().1 as i32;
            let y_offset = (max_tile_cell_height - tile_height) as f64 / 2.0;

            fixed.put(
                &tile_view,
                current_x_offset_cells as f64 * PREVIEW_CELL_SIZE,
                y_offset * PREVIEW_CELL_SIZE,
            );
            tile_view.set_width_request((PREVIEW_CELL_SIZE * tile.base().dim().0 as f64) as i32);
            tile_view.set_height_request((PREVIEW_CELL_SIZE * tile.base().dim().1 as f64) as i32);
            let tile_width = tile.base().dim().0;
            let next_x_offset = current_x_offset_cells + tile_width + 1;
            current_x_offset_cells = next_x_offset;
        }
    }

    fn create_board_preview(board: &BoardConfig, preview_box: &gtk::Box) {
        let board_view = BoardView::new(board);

        match board_view {
            Ok(bv) => {
                bv.set_property("halign", Align::Center);
                preview_box.append(&bv);

                let min_element_width = bv.get_min_element_size();
                let size_per_cell = PREVIEW_CELL_SIZE.max(min_element_width as f64);

                bv.set_width_request(size_per_cell as i32 * board.layout().dim().0 as i32);
                bv.set_height_request(size_per_cell as i32 * board.layout().dim().1 as i32);
            }
            Err(e) => {
                error!("Failed to create board preview: {}", e);
            }
        }
    }
}
