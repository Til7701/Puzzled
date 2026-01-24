use crate::offset::{CellOffset, PixelOffset};
use crate::view::board::BoardView;
use crate::view::tile::TileView;
use gtk::prelude::FixedExt;
use gtk::{Fixed, Widget};

#[derive(Debug, Default)]
pub struct PuzzleAreaData {
    pub fixed: Fixed,
    pub elements_in_fixed: Vec<Widget>,
    pub board_view: Option<BoardView>,
    pub tile_views: Vec<TileView>,
    pub grid_config: GridConfig,
}

impl PuzzleAreaData {
    pub fn add_to_fixed(&mut self, widget: &Widget, pos: &PixelOffset) {
        self.fixed.put(widget, pos.0, pos.1);
        self.elements_in_fixed.push(widget.clone());
    }
}

/// Configuration for the puzzle grid layout.
#[derive(Debug)]
pub struct GridConfig {
    pub grid_h_cell_count: u32,
    pub cell_width_pixel: u32,
    pub board_offset_cells: CellOffset,
}

impl Default for GridConfig {
    fn default() -> Self {
        GridConfig {
            grid_h_cell_count: 1,
            cell_width_pixel: 1,
            board_offset_cells: CellOffset(0, 0),
        }
    }
}
