use crate::app::puzzle_area::puzzle_area::placement;
use crate::app::puzzle_area::puzzle_area::puzzle_state::{
    Cell, PuzzleState, TileCellPlacement, UnusedTile,
};
use crate::model::extension::PuzzleTypeExtension;
use crate::model::puzzle::PuzzleModel;
use crate::offset::{CellOffset, PixelOffset};
use crate::window::PuzzledWindow;
use adw::gio;
use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{glib, Widget};
use log::debug;
use std::mem::take;

const TILE_MOVED_SIGNAL_NAME: &str = "tile-moved";

mod imp {
    use super::*;
    use crate::app::puzzle_area::puzzle_area::layout::GridConfig;
    use crate::components::board::BoardView;
    use crate::components::tile::TileView;
    use crate::model::extension::PuzzleTypeExtension;
    use adw::glib::subclass::Signal;
    use std::cell::{OnceCell, RefCell};
    use std::sync::OnceLock;

    #[derive(Debug, Default)]
    pub struct PuzzledPuzzleArea {
        pub board: RefCell<Option<BoardView>>,
        pub tiles: RefCell<Vec<TileView>>,
        pub hint_tile: RefCell<Option<TileView>>,

        pub window: OnceCell<PuzzledWindow>,

        pub grid_config: RefCell<GridConfig>,
        pub elements_in_fixed: RefCell<Vec<Widget>>,
        pub puzzle: RefCell<Option<PuzzleModel>>,
        pub puzzle_type_extension: RefCell<Option<PuzzleTypeExtension>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for PuzzledPuzzleArea {
        const NAME: &'static str = "PuzzledPuzzleArea";
        type Type = PuzzleArea;
        type ParentType = gtk::Fixed;

        fn class_init(klass: &mut Self::Class) {}

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {}
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

        self.imp()
            .grid_config
            .replace(self.initial_grid_config(puzzle_config));

        self.setup_board(puzzle_config);

        let start_positions = placement::calculate_tile_start_positions(
            puzzle_config.tiles(),
            puzzle_config,
            self.imp().grid_config.borrow().board_offset_cells,
        );
        for (i, tile) in puzzle_config.tiles().iter().enumerate() {
            self.setup_tile(tile, i, &start_positions[i]);
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
        let puzzle = self.imp().puzzle.borrow();
        if puzzle.is_none() {
            return Err("No puzzle set".to_string());
        }
        let puzzle = puzzle.as_ref().unwrap();
        let puzzle_config = puzzle.config();

        let mut state = PuzzleState::new(puzzle_config, self.imp().puzzle_type_extension.borrow());

        let tiles = self.imp().tiles.borrow();
        let grid_config = self.imp().grid_config.borrow();
        let board_position = grid_config.board_offset_cells;

        for (i, tile_view) in tiles.iter().enumerate() {
            let tile_position = tile_view
                .position_cells()
                .ok_or_else(|| "Tile position not set".to_string())?;
            let tile_position = tile_position - board_position + CellOffset(1, 1);
            let mut any_cell_on_board = false;
            for ((x, y), cell) in tile_view.current_rotation().indexed_iter() {
                if !*cell {
                    continue;
                }

                let cell_position = tile_position + CellOffset(x as i32, y as i32);
                if cell_position.0 >= 0
                    && cell_position.1 >= 0
                    && (cell_position.0 as usize) < state.grid.dim().0
                    && (cell_position.1 as usize) < state.grid.dim().1
                {
                    let idx: (usize, usize) = cell_position.into();
                    let new = match state.grid.get_mut(idx) {
                        None => return Err("Index out of bounds".to_string()),
                        Some(cell_ref) => {
                            let old = take(cell_ref);
                            let tile_cell_placement = TileCellPlacement {
                                tile_id: i,
                                cell_position: CellOffset(x as i32, y as i32),
                            };
                            match old {
                                Cell::Empty(data) => {
                                    any_cell_on_board = any_cell_on_board || data.is_on_board;
                                    Cell::One(data, tile_cell_placement)
                                }
                                Cell::One(data, existing_widget) => {
                                    any_cell_on_board = any_cell_on_board || data.is_on_board;
                                    let widgets = vec![existing_widget, tile_cell_placement];
                                    Cell::Many(data, widgets)
                                }
                                Cell::Many(data, mut widgets) => {
                                    any_cell_on_board = any_cell_on_board || data.is_on_board;
                                    widgets.push(tile_cell_placement);
                                    Cell::Many(data, widgets)
                                }
                            }
                        }
                    };
                    state.grid[idx] = new;
                }
            }
            if !any_cell_on_board {
                let unused_tile = UnusedTile {
                    id: i,
                    base: tile_view.base().clone(),
                    name: tile_view.name(),
                };
                state.unused_tiles.insert(unused_tile);
            }
        }
        Ok(state)
    }
}
