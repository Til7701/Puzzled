use crate::global::state::get_state;
use crate::offset::CellOffset;
use crate::presenter::puzzle_area::board::BoardPresenter;
use crate::presenter::puzzle_area::data::PuzzleAreaData;
use crate::presenter::puzzle_area::puzzle_state::{Cell, PuzzleState, UnusedTile};
use crate::presenter::puzzle_area::tile::TilePresenter;
use crate::presenter::puzzle_area::HighlightMode::{OutOfBounds, Overlapping};
use crate::window::PuzzledWindow;
use gtk::prelude::{FixedExt, GtkWindowExt, WidgetExt};
use gtk::Widget;
use std::cell::RefCell;
use std::mem::take;
use std::rc::Rc;

mod board;
mod data;
mod placement;
pub mod puzzle_state;
mod tile;

pub const WINDOW_TO_BOARD_RATIO: f64 = 2.0;
pub const MIN_CELLS_TO_THE_SIDES_OF_BOARD: u32 = 6;
pub const OVERLAP_HIGHLIGHT_CSS_CLASS: &str = "overlap-highlight";
pub const OUT_OF_BOUNDS_HIGHLIGHT_CSS_CLASS: &str = "out-of-bounds-highlight";

enum HighlightMode {
    Overlapping,
    OutOfBounds,
}

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
        self.window.connect_default_width_notify({
            let self_clone = self.clone();
            move |_| self_clone.update_layout()
        });
        self.window.connect_is_active_notify({
            let self_clone = self.clone();
            move |_| self_clone.update_layout()
        });
        // Currently, this does not work, since the width is not updated yet when this signal is emitted.
        self.window.connect_maximized_notify({
            let self_clone = self.clone();
            move |_| self_clone.update_layout()
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
            self.set_min_width();
        }
    }

    /// Update the layout based on the current state.
    ///
    /// This moves the puzzle area elements according to the current window size.
    pub fn update_layout(&self) {
        self.update_cell_width();
        self.board_presenter.update_layout();
        self.tile_presenter.update_layout();
    }

    fn update_cell_width(&self) {
        let width = self.window.width();
        let grid_config = &mut self.data.borrow_mut().grid_config;
        grid_config.cell_width_pixel = width as u32 / grid_config.grid_h_cell_count;
    }

    fn set_min_width(&self) {
        let min_board_elements_width = self.board_presenter.get_min_element_width();
        let data = self.data.borrow();
        let fixed_min_width = data.grid_config.grid_h_cell_count as i32 * min_board_elements_width;
        data.fixed.set_width_request(fixed_min_width);
    }

    fn clear_elements(&self) {
        let mut data = self.data.borrow_mut();
        let fixed = data.fixed.clone();
        data.elements_in_fixed
            .drain(..)
            .for_each(|e| fixed.remove(&e));
        data.tile_views.clear();
        data.board_view = None;
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
            let tile_position = tile_view.position_cells.ok_or("Tile position not set")?;
            let tile_position = tile_position - board_position + CellOffset(1, 1);
            for (element, offset) in &tile_view.elements_with_offset {
                let element_position = tile_position + (*offset).into();
                if element_position.0 >= 0
                    && element_position.1 >= 0
                    && (element_position.0 as usize) < state.grid.dim().0
                    && (element_position.1 as usize) < state.grid.dim().1
                {
                    let idx: (usize, usize) = element_position.into();
                    let new = match state.grid.get_mut(idx) {
                        None => return Err("Index out of bounds".to_string()),
                        Some(cell_ref) => {
                            let old = take(cell_ref);
                            match old {
                                Cell::Empty(data) => Cell::One(data, element.clone()),
                                Cell::One(data, existing_widget) => {
                                    let widgets = vec![existing_widget, element.clone()];
                                    Cell::Many(data, widgets)
                                }
                                Cell::Many(data, mut widgets) => {
                                    widgets.push(element.clone());
                                    Cell::Many(data, widgets)
                                }
                            }
                        }
                    };
                    state.grid[idx] = new;
                } else {
                    let unused_tile = UnusedTile {
                        id: i,
                        base: tile_view.tile_base.clone(),
                    };
                    state.unused_tiles.insert(unused_tile);
                }
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

    fn highlight(&self, mode: HighlightMode, widget: &Widget) {
        widget.add_css_class(match mode {
            Overlapping => OVERLAP_HIGHLIGHT_CSS_CLASS,
            OutOfBounds => OUT_OF_BOUNDS_HIGHLIGHT_CSS_CLASS,
        });
    }

    fn clear_highlight(&self, widget: &Widget) {
        widget.remove_css_class(OVERLAP_HIGHLIGHT_CSS_CLASS);
        widget.remove_css_class(OUT_OF_BOUNDS_HIGHLIGHT_CSS_CLASS);
    }

    pub fn highlight_invalid_tile_parts(&self, puzzle_state: &PuzzleState) {
        puzzle_state.grid.iter().for_each(|cell| match cell {
            Cell::One(data, widget) => {
                if !data.allowed {
                    self.highlight(OutOfBounds, widget);
                }
            }
            Cell::Many(_, widgets) => widgets.iter().for_each(|w| self.highlight(Overlapping, w)),
            _ => {}
        });
    }

    pub fn clear_highlights(&self) {
        self.data
            .borrow()
            .elements_in_fixed
            .iter()
            .for_each(|element| self.clear_highlight(element));
    }
}
