use crate::offset::{CellOffset, PixelOffset};
use crate::presenter::board::BoardPresenter;
use crate::presenter::tile::TilePresenter;
use crate::puzzle_state::{Cell, PuzzleState};
use crate::state::get_state;
use crate::view::{BoardView, TileView};
use gtk::prelude::{FixedExt, WidgetExt};
use gtk::{Fixed, Widget};
use std::cell::RefCell;
use std::mem::take;
use std::rc::Rc;

pub const WINDOW_TO_BOARD_RATIO: f64 = 2.5;
pub const OVERLAP_HIGHLIGHT_CSS_CLASS: &str = "overlap-highlight";

/// Configuration for the puzzle grid layout.
#[derive(Debug, Default)]
pub struct GridConfig {
    pub grid_h_cell_count: u32,
    pub cell_width_pixel: u32,
    pub board_offset_cells: CellOffset,
}

#[derive(Debug, Default)]
pub struct PuzzleAreaData {
    pub fixed: Option<Fixed>,
    pub elements_in_fixed: Vec<Widget>,
    pub board_view: Option<BoardView>,
    pub tile_views: Vec<TileView>,
    pub grid_config: GridConfig,
}

impl PuzzleAreaData {
    pub fn add_to_fixed(&mut self, widget: &Widget, pos: &PixelOffset) {
        match &self.fixed {
            Some(fixed) => {
                fixed.put(widget, pos.0, pos.1);
                self.elements_in_fixed.push(widget.clone());
            }
            None => {}
        }
    }
}

#[derive(Debug, Clone)]
pub struct PuzzleAreaPresenter {
    data: Rc<RefCell<PuzzleAreaData>>,
    board_presenter: BoardPresenter,
    tile_presenter: TilePresenter,
}

impl PuzzleAreaPresenter {
    pub fn set_view(&self, view: Fixed) {
        self.data.borrow_mut().fixed = Some(view);
        self.clear_elements();
    }

    /// Set up the puzzle configuration from the current state.
    ///
    /// This adds the board and tiles to the puzzle area based on the current puzzle configuration.
    /// Final positions and layout are handled in `update_layout()`. Before that, all elements are
    /// added at position (0, 0) and will be moved later.
    pub fn setup_puzzle_config_from_state(&self) {
        self.clear_elements();

        let state = get_state();
        let puzzle_config = &state.puzzle_config;

        self.board_presenter.setup(puzzle_config);
        let mut position_start = CellOffset(1, 1);
        for tile in puzzle_config.tiles.iter() {
            self.tile_presenter.setup(
                tile,
                &position_start,
                Rc::new({
                    let self_clone = self.clone();
                    move || self_clone.update_highlights()
                }),
            );

            let (rows, cols) = tile.base.dim();
            position_start.0 += (rows + 1) as i32;
            if position_start.0 > 10 {
                position_start.0 = 1;
                position_start.1 += (cols + 1) as i32;
            }
        }

        self.update_layout();
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
        let width = {
            let data = self.data.borrow();
            match &data.fixed {
                Some(fixed) => fixed.width(),
                None => 0,
            }
        };

        let grid_config = &mut self.data.borrow_mut().grid_config;
        grid_config.cell_width_pixel = width as u32 / grid_config.grid_h_cell_count;
    }

    fn clear_elements(&self) {
        let mut data = self.data.borrow_mut();
        if let Some(fixed) = data.fixed.clone() {
            data.elements_in_fixed
                .drain(..)
                .for_each(|e| fixed.remove(&e));
            data.tile_views.clear();
            data.board_view = None;
        }
    }

    pub fn extract_puzzle_state(&self) -> Result<PuzzleState, String> {
        let mut state = PuzzleState::new(&get_state().puzzle_config);
        let data = self.data.borrow();
        let board_position = data.grid_config.board_offset_cells;

        for tile_view in &data.tile_views {
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

    fn highlight(&self, widget: &Widget) {
        widget.set_opacity(0.5);
        widget.add_css_class(OVERLAP_HIGHLIGHT_CSS_CLASS);
    }

    fn clear_highlight(&self, widget: &Widget) {
        widget.set_opacity(1.0);
        widget.remove_css_class(OVERLAP_HIGHLIGHT_CSS_CLASS);
    }

    pub fn highlight_invalid_tile_parts(&self, puzzle_state: &PuzzleState) {
        puzzle_state.grid.iter().for_each(|cell| match cell {
            Cell::One(data, widget) => {
                if !data.allowed {
                    self.highlight(widget);
                }
            }
            Cell::Many(_, widgets) => widgets.iter().for_each(|w| self.highlight(w)),
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

impl Default for PuzzleAreaPresenter {
    fn default() -> Self {
        let data = Rc::new(RefCell::new(PuzzleAreaData::default()));
        let mut board_presenter = BoardPresenter::default();
        board_presenter.set_data(data.clone());
        let mut tile_presenter = TilePresenter::default();
        tile_presenter.set_data(data.clone());
        Self {
            data,
            board_presenter,
            tile_presenter,
        }
    }
}
