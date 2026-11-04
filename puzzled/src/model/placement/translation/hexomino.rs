use crate::model::placement::PixelPosition;
use crate::model::placement::board::PlacedBoard;
use crate::model::placement::t::{InternalPlacementModel, SimplePlacementModel};
use crate::model::placement::tile::PlacedTile;
use crate::model::puzzle::PuzzleModel;
use puzzled_common::polyform::coord::hex::HexCoord;
use puzzled_common::polyform::poly::hexomino::Hexomino;

pub(super) const MIN_CELLS_TO_THE_TOP_OF_BOARD: u32 = 1;
pub(super) const MIN_CELLS_TO_THE_SIDES_OF_BOARD: u32 = 6;
pub(super) const MIN_CELLS_TO_THE_BOTTOM_OF_BOARD: u32 = 6;

#[derive(Debug, Default)]
pub struct HexominoPlacementModel {}

impl HexominoPlacementModel {
    pub fn new(puzzle_model: &PuzzleModel) -> Self {
        todo!()
    }
}

impl InternalPlacementModel<Hexomino<()>, HexCoord> for HexominoPlacementModel {
    fn tiles(&self) -> &[PlacedTile<Hexomino<()>, HexCoord>] {
        todo!()
    }

    fn tiles_mut(&mut self) -> &mut [PlacedTile<Hexomino<()>, HexCoord>] {
        todo!()
    }

    fn board(&self) -> &PlacedBoard<HexCoord> {
        todo!()
    }

    fn board_mut(&mut self) -> &mut PlacedBoard<HexCoord> {
        todo!()
    }

    fn hint_tile(&self) -> &Option<PlacedTile<Hexomino<()>, HexCoord>> {
        todo!()
    }
}

impl SimplePlacementModel<Hexomino<()>, HexCoord> for HexominoPlacementModel {
    fn update_pixel_size(
        &mut self,
        total_view_size_pixel: PixelPosition,
        min_cell_size_pixel: u32,
    ) {
        todo!()
    }

    fn min_area_size(&self) -> PixelPosition {
        todo!()
    }

    fn update_tile_pixel_position(&mut self, idx: usize, position: PixelPosition) {
        todo!()
    }

    fn update_tile_dragged(&mut self, idx: usize, dragged: bool) {
        todo!()
    }

    fn update_tile_shape(&mut self, idx: usize, shape: Hexomino<()>) {
        todo!()
    }

    fn init_hint_tile(&mut self, position_on_board: &HexCoord, shape: Hexomino<()>) {
        todo!()
    }

    fn remove_hint_tile(&mut self) {
        todo!()
    }
}
