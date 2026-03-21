use crate::app::puzzle_area::puzzle_area::puzzle_area::PuzzleArea;
use crate::app::puzzle_area::puzzle_area::puzzle_state::{Cell, PuzzleState};
use crate::components::tile::DrawingMode;
use adw::subclass::prelude::ObjectSubclassIsExt;

impl PuzzleArea {
    pub fn update_highlights(&self) {
        self.clear_highlights();
        let puzzle_state = self.extract_puzzle_state();
        if let Ok(puzzle_state) = puzzle_state {
            self.highlight_invalid_tile_parts(&puzzle_state);
        }
    }

    fn clear_highlights(&self) {
        let tile_views = self.imp().tiles.borrow();
        for tile_view in tile_views.iter() {
            tile_view.reset_drawing_modes();
        }
    }

    pub fn highlight_invalid_tile_parts(&self, puzzle_state: &PuzzleState) {
        let tile_views = self.imp().tiles.borrow();

        puzzle_state.grid.iter().for_each(|cell| match cell {
            Cell::One(data, tile_cell_placement) => {
                if !data.allowed
                    && let Some(tile_view) = tile_views.get(tile_cell_placement.tile_id)
                {
                    tile_view.set_drawing_mode_at(
                        tile_cell_placement.cell_position.0 as usize,
                        tile_cell_placement.cell_position.1 as usize,
                        DrawingMode::OutOfBounds,
                    );
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
}
