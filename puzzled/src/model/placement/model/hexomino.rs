use crate::model::placement::PixelPosition;
use crate::model::placement::board::PlacedBoard;
use crate::model::placement::prelude::{InternalPlacementModel, SimplePlacementModel};
use crate::model::placement::tile::PlacedTile;
use crate::model::puzzle::PuzzleModel;
use puzzled_common::polyform::{CoordBox, HexCoord, Hexomino};

#[derive(Debug, Default)]
pub struct HexominoPlacementModel {}

impl HexominoPlacementModel {
    pub fn new(_puzzle_model: &PuzzleModel) -> Self {
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
        _total_view_size_pixel: PixelPosition,
        _min_cell_size_pixel: u32,
    ) {
        todo!()
    }

    fn min_area_size(&self) -> PixelPosition {
        todo!()
    }

    fn update_tile_pixel_position(&mut self, _idx: usize, _position: PixelPosition) {
        todo!()
    }

    fn update_tile_dragged(&mut self, _idx: usize, _dragged: bool) {
        todo!()
    }

    fn update_tile_shape(&mut self, _idx: usize, _shape: Hexomino<()>) {
        todo!()
    }

    fn prototile_coord(&self, _idx: usize, _pixel_position: PixelPosition) -> Option<CoordBox> {
        todo!()
    }

    fn init_hint_tile(&mut self, _position_on_board: HexCoord, _shape: Hexomino<()>) {
        todo!()
    }

    fn remove_hint_tile(&mut self) {
        todo!()
    }
}
