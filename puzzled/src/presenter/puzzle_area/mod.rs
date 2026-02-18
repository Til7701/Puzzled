use crate::global::state::get_state;
use crate::offset::CellOffset;
use crate::presenter::puzzle_area::board::BoardPresenter;
use crate::presenter::puzzle_area::data::{GridConfig, PuzzleAreaData};
use crate::presenter::puzzle_area::puzzle_state::{
    Cell, PuzzleState, TileCellPlacement, UnusedTile,
};
use crate::presenter::puzzle_area::tile::TilePresenter;
use crate::view::tile::{DrawingMode, TileView};
use crate::window::PuzzledWindow;
use adw::glib;
use gtk::prelude::{FixedExt, WidgetExt, WidgetExtManual};
use puzzle_config::ColorConfig;
use puzzle_solver::result::TilePlacement;
use std::cell::RefCell;
use std::mem::take;
use std::rc::Rc;

mod board;
mod data;
mod placement;
pub mod puzzle_state;
mod tile;

const MIN_CELLS_TO_THE_TOP_OF_BOARD: i32 = 1;
const MIN_CELLS_TO_THE_SIDES_OF_BOARD: i32 = 6;
const MIN_CELLS_TO_THE_BOTTOM_OF_BOARD: i32 = 6;

#[derive(Debug, Clone)]
pub struct PuzzleAreaPresenter {
    window: PuzzledWindow,
    data: Rc<RefCell<PuzzleAreaData>>,
    board_presenter: BoardPresenter,
    tile_presenter: TilePresenter,
}

impl PuzzleAreaPresenter {
    pub fn new(window: &PuzzledWindow) -> Self {
        let data = Rc::new(RefCell::new(PuzzleAreaData::default()));
        data.borrow_mut().fixed = window.puzzle_area_nav_page().grid();

        let mut board_presenter = BoardPresenter::default();
        board_presenter.set_data(data.clone());
        let mut tile_presenter = TilePresenter::default();
        tile_presenter.set_data(data.clone());

        Self {
            window: window.clone(),
            data,
            board_presenter,
            tile_presenter,
        }
    }

    pub fn setup(&self) {
        self.window.add_tick_callback({
            let self_clone = self.clone();
            let last = Rc::new(std::cell::Cell::new((
                self.window.width(),
                self.window.height(),
            )));
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

    /// Set up the puzzle configuration from the current state.
    ///
    /// This adds the board and tiles to the puzzle area based on the current puzzle configuration.
    /// Final positions and layout are handled in `update_layout()`. Before that, all elements are
    /// added at position (0, 0) and will be moved later.
    pub fn show_puzzle(&self, on_position_changed: Rc<dyn Fn()>) {
        self.clear_elements();

        let state = get_state();
        if let Some(puzzle_config) = &state.puzzle_config {
            self.board_presenter.setup(puzzle_config);

            let start_positions = placement::calculate_tile_start_positions(
                &puzzle_config.tiles(),
                puzzle_config,
                self.data.borrow().grid_config.board_offset_cells,
            );
            for (i, tile) in puzzle_config.tiles().iter().enumerate() {
                self.tile_presenter.setup(
                    tile,
                    i,
                    &start_positions[i],
                    Rc::new({
                        let self_clone = self.clone();
                        let on_position_changed = on_position_changed.clone();
                        move || {
                            self_clone.update_highlights();
                            on_position_changed();
                        }
                    }),
                );
            }

            drop(state);
            self.update_highlights();
            self.update_layout();
            self.set_min_size();
        }
    }

    /// Update the layout based on the current state.
    ///
    /// This moves the puzzle area elements according to the current window size.
    pub fn update_layout(&self) {
        self.update_grid_layout();
        self.board_presenter.update_layout();
        self.tile_presenter.update_layout();
    }

    fn update_grid_layout(&self) {
        let available_width_pixel = self.window.width() as f64;
        let available_height_pixel = self.window.height() as f64
            - self.window.puzzle_area_nav_page().header_bar().height() as f64;

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
        let min_grid_h_cell_count = board_size_cells_with_margin.0 as u32;
        let min_grid_v_cell_count = board_size_cells_with_margin.1 as u32;
        let grid_v_cell_count = (available_height_pixel / cell_size_pixel as f64).floor() as u32;

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
        let data = self.data.borrow();
        if data.grid_config != grid_config {
            drop(data);
            self.update_grid_config(grid_config);
        }
    }

    fn board_size_cells(&self) -> CellOffset {
        let state = get_state();
        let board_size = state
            .puzzle_config
            .as_ref()
            .map(|c| c.board_config().layout().dim())
            .unwrap_or((1, 1));
        CellOffset(board_size.0 as i32, board_size.1 as i32)
    }

    fn tiles_required_cells(&self) -> CellOffset {
        let data = self.data.borrow();
        let tile_views = &data.tile_views;
        let mut required_cells = CellOffset(0, 0);
        let mut lowest_position_cells = CellOffset(0, 0);
        for tile_view in tile_views {
            let tile_size: CellOffset = tile_view.base().dim().into();
            required_cells =
                required_cells.max(tile_size + tile_view.position_cells().unwrap_or_default());
            lowest_position_cells = lowest_position_cells
                .min(tile_view.position_cells().unwrap_or(lowest_position_cells));
        }
        required_cells - lowest_position_cells
    }

    fn update_grid_config(&self, grid_config: GridConfig) {
        let mut data = self.data.borrow_mut();

        if data.grid_config.board_offset_cells.0 != grid_config.board_offset_cells.0 {
            self.move_all_elements_by(
                &data,
                CellOffset(
                    grid_config.board_offset_cells.0 - data.grid_config.board_offset_cells.0,
                    0,
                ),
            );
        }

        data.grid_config = grid_config;
        drop(data);
        self.set_min_size();
    }

    fn move_all_elements_by(&self, data: &PuzzleAreaData, offset_cells: CellOffset) {
        for tile_view in &data.tile_views {
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
        if let Some(hint_tile_view) = &data.hint_tile_view {
            if let Some(position_cells) = hint_tile_view.position_cells() {
                let new_position_cells = position_cells + offset_cells;
                hint_tile_view.set_position_cells(Some(new_position_cells));
            }
        }
    }

    fn set_min_size(&self) {
        let min_board_elements_width = self.board_presenter.get_min_element_width();
        let data = self.data.borrow();

        let fixed_min_width =
            data.grid_config.min_grid_h_cell_count as i32 * min_board_elements_width;
        self.window.set_width_request(fixed_min_width);
        let fixed_min_height =
            data.grid_config.min_grid_v_cell_count as i32 * min_board_elements_width;
        self.window.set_height_request(
            fixed_min_height + self.window.puzzle_area_nav_page().header_bar().height(),
        );
    }

    fn clear_elements(&self) {
        let mut data = self.data.borrow_mut();
        let fixed = data.fixed.clone();
        data.elements_in_fixed
            .drain(..)
            .for_each(|e| fixed.remove(&e));
        data.tile_views.clear();
        data.board_view = None;
        if let Some(tile_view) = &data.hint_tile_view {
            fixed.remove(tile_view);
        }
        data.hint_tile_view = None;
    }

    pub fn extract_puzzle_state(&self) -> Result<PuzzleState, String> {
        let state = get_state();
        let mut state = PuzzleState::new(
            &state.puzzle_config.clone().unwrap(),
            &state.puzzle_type_extension,
        );
        let data = self.data.borrow();
        let board_position = data.grid_config.board_offset_cells;

        for (i, tile_view) in data.tile_views.iter().enumerate() {
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
                };
                state.unused_tiles.insert(unused_tile);
            }
        }
        Ok(state)
    }

    pub fn update_highlights(&self) {
        self.clear_highlights();
        let puzzle_state = self.extract_puzzle_state();
        if let Ok(puzzle_state) = puzzle_state {
            self.highlight_invalid_tile_parts(&puzzle_state);
        }
    }

    fn clear_highlights(&self) {
        let data = self.data.borrow();
        let tile_views = &data.tile_views;
        for tile_view in tile_views {
            tile_view.reset_drawing_modes();
        }
    }

    pub fn highlight_invalid_tile_parts(&self, puzzle_state: &PuzzleState) {
        let data = self.data.borrow();
        let tile_views = &data.tile_views;

        puzzle_state.grid.iter().for_each(|cell| match cell {
            Cell::One(data, tile_cell_placement) => {
                if !data.allowed {
                    if let Some(tile_view) = tile_views.get(tile_cell_placement.tile_id) {
                        tile_view.set_drawing_mode_at(
                            tile_cell_placement.cell_position.0 as usize,
                            tile_cell_placement.cell_position.1 as usize,
                            DrawingMode::OutOfBounds,
                        );
                    }
                }
            }
            Cell::Many(_, tile_cell_placements) => {
                for tile_cell_placement in tile_cell_placements {
                    if let Some(tile_view) = tile_views.get(tile_cell_placement.tile_id) {
                        tile_view.set_drawing_mode_at(
                            tile_cell_placement.cell_position.0 as usize,
                            tile_cell_placement.cell_position.1 as usize,
                            DrawingMode::Overlapping,
                        );
                    }
                }
            }
            _ => {}
        });
    }

    /// Show the placement of a tile as a hint.
    pub fn show_hint_tile(&self, placement: &TilePlacement) {
        let mut data = self.data.borrow_mut();
        let tile_matching_base = data
            .tile_views
            .iter()
            .find(|t| t.base().eq(placement.base()));
        if tile_matching_base.is_none() {
            return;
        }
        let tile_matching_base = tile_matching_base.unwrap();
        let color = tile_matching_base.color().with_alpha(0.5);
        let color_config = ColorConfig::new(
            (color.red() * 255.0) as u8,
            (color.green() * 255.0) as u8,
            (color.blue() * 255.0) as u8,
            (color.alpha() * 255.0) as u8,
        );
        let tile_view = self.create_hint_tile(placement, color_config, &data);
        if let Some(tile_matching_base) = &data.hint_tile_view {
            data.fixed.remove(tile_matching_base);
        }
        data.hint_tile_view = Some(tile_view.clone());
        data.fixed.put(&tile_view, 0.0, 0.0);
        drop(data);
        self.update_layout();
    }

    fn create_hint_tile(
        &self,
        placement: &TilePlacement,
        color_config: ColorConfig,
        data: &PuzzleAreaData,
    ) -> TileView {
        let tile_view = TileView::new(usize::MAX, placement.rotation().clone(), color_config);

        tile_view.set_position_cells(Some(
            data.grid_config.board_offset_cells + placement.position().into() - CellOffset(1, 1), // Plus 1, 1 because the puzzle state has a border of one cell to provide information for highlighting
        ));

        let click_gesture = gtk::GestureClick::new();
        click_gesture.connect_pressed({
            let self_clone = self.clone();
            move |_, _, _, _| {
                self_clone.remove_hint_tile();
            }
        });
        tile_view.add_controller(click_gesture);

        tile_view
    }

    /// Remove the hint tile from the puzzle area, if one is currently shown.
    pub fn remove_hint_tile(&self) {
        let mut data = self.data.borrow_mut();
        if let Some(tile_view) = &data.hint_tile_view {
            data.fixed.remove(tile_view);
        }
        data.hint_tile_view = None;
    }
}
