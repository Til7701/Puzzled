use crate::app::puzzle::puzzle_area::PuzzleArea;
use crate::offset::CellOffset;
use crate::window::{MIN_WINDOW_HEIGHT, MIN_WINDOW_WIDTH};
use adw::glib;
use adw::subclass::prelude::ObjectSubclassIsExt;
use gtk::prelude::{WidgetExt, WidgetExtManual};
use puzzle_config::PuzzleConfig;
use std::cell::Cell;
use std::rc::Rc;

const MIN_CELLS_TO_THE_TOP_OF_BOARD: i32 = 1;
const MIN_CELLS_TO_THE_SIDES_OF_BOARD: i32 = 6;
const MIN_CELLS_TO_THE_BOTTOM_OF_BOARD: i32 = 6;

/// Configuration for the puzzle grid layout.
#[derive(Debug, Eq, PartialEq)]
pub struct GridConfig {
    pub grid_h_cell_count: u32,
    pub grid_v_cell_count: u32,
    pub min_grid_h_cell_count: u32,
    pub min_grid_v_cell_count: u32,
    pub cell_size_pixel: u32,
    pub board_offset_cells: CellOffset,
}

impl Default for GridConfig {
    fn default() -> Self {
        GridConfig {
            grid_h_cell_count: 1,
            grid_v_cell_count: 1,
            min_grid_h_cell_count: 1,
            min_grid_v_cell_count: 1,
            cell_size_pixel: 1,
            board_offset_cells: CellOffset(0, 0),
        }
    }
}

impl PuzzleArea {
    pub fn post_construct_setup_layout(&self) {
        // May have to use window here, since the size may change unpredictably
        self.add_tick_callback({
            let self_clone = self.clone();
            let last = Rc::new(Cell::new((self.width(), self.height())));
            move |window, _| {
                let (w, h) = last.get();
                let window_width = window.width();
                let window_height = window.height();
                if window_width != w || window_height != h {
                    last.set((window_width, window_height));
                    self_clone.update_layout();
                }
                glib::ControlFlow::Continue
            }
        });
    }

    pub(crate) fn initial_grid_config(&self, puzzle_config: &PuzzleConfig) -> GridConfig {
        let board_cell_width = puzzle_config.board_config().layout().dim().0 as i32;
        let board_cell_height = puzzle_config.board_config().layout().dim().1 as i32;

        let required_h_cells = board_cell_width + MIN_CELLS_TO_THE_SIDES_OF_BOARD * 2;
        let required_v_cells =
            board_cell_height + MIN_CELLS_TO_THE_TOP_OF_BOARD + MIN_CELLS_TO_THE_BOTTOM_OF_BOARD;
        GridConfig {
            grid_h_cell_count: required_h_cells as u32,
            grid_v_cell_count: required_v_cells as u32,
            min_grid_h_cell_count: required_h_cells as u32,
            min_grid_v_cell_count: required_v_cells as u32,
            cell_size_pixel: 1,
            board_offset_cells: CellOffset(
                MIN_CELLS_TO_THE_SIDES_OF_BOARD,
                MIN_CELLS_TO_THE_TOP_OF_BOARD,
            ),
        }
    }

    /// Update the layout based on the current state.
    ///
    /// This moves the puzzle area elements according to the current window size.
    pub fn update_layout(&self) {
        self.update_grid_layout();
        self.set_min_size();
        self.update_board_layout();
        self.update_tile_layout();
    }

    /// Calculates how the grid should be laid out based on the current positions of the tiles
    /// and the size of the board.
    ///
    /// This function should ensure, that all tiles are visible and the board is centered.
    /// [Self::update_grid_config()] is called if the grid layout needs to be updated based on the
    /// new calculations.
    fn update_grid_layout(&self) {
        let available_width_pixel = self.imp().window.get().unwrap().width() as f64;
        let available_height_pixel = {
            let mut header_height = self
                .imp()
                .window
                .get()
                .unwrap()
                .puzzle_area_nav_page()
                .header_bar()
                .height() as f64;
            if header_height == 0.0 {
                header_height = 40.0;
            }
            self.imp().window.get().unwrap().height() as f64 - header_height
        };

        let board_size_cells = self.board_size_cells();
        let board_size_cells_with_margin = board_size_cells.add_tuple((
            MIN_CELLS_TO_THE_SIDES_OF_BOARD * 2,
            MIN_CELLS_TO_THE_TOP_OF_BOARD + MIN_CELLS_TO_THE_BOTTOM_OF_BOARD,
        ));
        let tiles_required_cells = self.tiles_required_cells();
        let required_cells = board_size_cells_with_margin.max(tiles_required_cells);

        let cell_width_pixel = (available_width_pixel / required_cells.0 as f64).floor() as u32;
        let cell_height_pixel = (available_height_pixel / required_cells.1 as f64).floor() as u32;
        let cell_size_pixel = cell_width_pixel.min(cell_height_pixel);

        let grid_h_cell_count = (available_width_pixel / cell_size_pixel as f64).floor() as u32;
        let min_grid_h_cell_count = required_cells.0 as u32;
        let grid_v_cell_count = (available_height_pixel / cell_size_pixel as f64).floor() as u32;
        let min_grid_v_cell_count = required_cells.1 as u32;

        let board_offset_cells = CellOffset(
            ((grid_h_cell_count - board_size_cells.0 as u32) / 2) as i32,
            MIN_CELLS_TO_THE_TOP_OF_BOARD,
        );

        let grid_config = GridConfig {
            grid_h_cell_count,
            grid_v_cell_count,
            min_grid_h_cell_count,
            min_grid_v_cell_count,
            cell_size_pixel,
            board_offset_cells,
        };
        let old_grid_config = self.imp().grid_config.borrow();
        if *old_grid_config != grid_config {
            drop(old_grid_config);
            self.update_grid_config(grid_config);
        }
    }

    /// Get the dimensions of the board in cells.
    fn board_size_cells(&self) -> CellOffset {
        let puzzle = self.imp().puzzle.borrow();
        let board_size = puzzle
            .as_ref()
            .map(|p| p.config().board_config().layout().dim())
            .unwrap_or((1, 1));
        CellOffset(board_size.0 as i32, board_size.1 as i32)
    }

    /// Calculates the dimensions required to fit all tiles in their current positions.
    fn tiles_required_cells(&self) -> CellOffset {
        let tiles = self.imp().tiles.borrow();
        let mut required_cells = CellOffset(0, 0);
        let mut lowest_position_cells = CellOffset(0, 0);
        for tile_view in tiles.iter() {
            let tile_size: CellOffset = tile_view.base().dim().into();
            required_cells =
                required_cells.max(tile_size + tile_view.position_cells().unwrap_or_default());
            lowest_position_cells = lowest_position_cells
                .min(tile_view.position_cells().unwrap_or(lowest_position_cells));
        }
        required_cells - lowest_position_cells
    }

    /// Update the grid configuration and move all elements in case the board offset has changed.
    fn update_grid_config(&self, grid_config: GridConfig) {
        let old_grid_config = self.imp().grid_config.borrow();

        if old_grid_config.board_offset_cells.0 != grid_config.board_offset_cells.0 {
            self.move_all_elements_by(CellOffset(
                grid_config.board_offset_cells.0 - old_grid_config.board_offset_cells.0,
                0,
            ));
        }

        drop(old_grid_config);
        self.imp().grid_config.replace(grid_config);
    }

    /// Moves all elements by the given offset in cells.
    ///
    /// If the new position of an element would be negative, it is set to 0 to ensure that all
    /// elements remain visible.
    fn move_all_elements_by(&self, offset_cells: CellOffset) {
        let tiles = self.imp().tiles.borrow();
        for tile_view in tiles.iter() {
            if let Some(position_cells) = tile_view.position_cells() {
                let mut new_position_cells = position_cells + offset_cells;
                if new_position_cells.0 < 0 {
                    new_position_cells.0 = 0;
                }
                if new_position_cells.1 < 0 {
                    new_position_cells.1 = 0;
                }
                tile_view.set_position_cells(Some(new_position_cells));
            }
        }
        let hint_tile = self.imp().hint_tile.borrow();
        if let Some(hint_tile_view) = hint_tile.as_ref()
            && let Some(position_cells) = hint_tile_view.position_cells()
        {
            let new_position_cells = position_cells + offset_cells;
            hint_tile_view.set_position_cells(Some(new_position_cells));
        }
    }

    /// Sets the minimum size of the window based on the current grid configuration.
    ///
    /// This has to be set on the window instead of the Fixed, since the AdwBreakpointBin
    /// that everything is wrapped in, does not work well with changing width requests
    /// if the children.
    fn set_min_size(&self) {
        if !self
            .imp()
            .window
            .get()
            .unwrap()
            .outer_view()
            .shows_content()
        {
            return;
        }

        let min_board_elements_width = self.get_min_element_width();
        let grid_config = self.imp().grid_config.borrow();

        let fixed_min_width = grid_config.min_grid_h_cell_count as i32 * min_board_elements_width;
        self.imp()
            .window
            .get()
            .unwrap()
            .set_width_request(fixed_min_width.max(MIN_WINDOW_WIDTH));
        let fixed_min_height = grid_config.min_grid_v_cell_count as i32 * min_board_elements_width;
        self.imp().window.get().unwrap().set_height_request(
            (fixed_min_height
                + self
                    .imp()
                    .window
                    .get()
                    .unwrap()
                    .puzzle_area_nav_page()
                    .header_bar()
                    .height())
            .max(MIN_WINDOW_HEIGHT),
        );
    }
}
