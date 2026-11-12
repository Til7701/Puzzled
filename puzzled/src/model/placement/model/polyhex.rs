use crate::model::placement::PixelPosition;
use crate::model::placement::board::PlacedBoard;
use crate::model::placement::grid::GridConfig;
use crate::model::placement::prelude::{InternalPlacementModel, SimplePlacementModel};
use crate::model::placement::tile::PlacedTile;
use crate::model::puzzle::PuzzleModel;
use puzzled_polyform::{CoordBox, HexCoord, Polyhex};

#[derive(Debug, Default)]
pub struct PolyhexPlacementModel {
    _area_pixel_size: PixelPosition,
    _min_area_pixel_size: PixelPosition,
    _grid_config: GridConfig<HexCoord>,
    _board: PlacedBoard<HexCoord>,
    _tiles: Vec<PlacedTile<Polyhex<()>, HexCoord>>,
    _hint_tile: Option<PlacedTile<Polyhex<()>, HexCoord>>,
}

impl PolyhexPlacementModel {
    pub fn new(_puzzle_model: &PuzzleModel) -> Self {
        todo!()
    }
}

impl InternalPlacementModel<Polyhex<()>, HexCoord> for PolyhexPlacementModel {
    fn tiles(&self) -> &[PlacedTile<Polyhex<()>, HexCoord>] {
        todo!()
    }

    fn board(&self) -> &PlacedBoard<HexCoord> {
        todo!()
    }

    fn hint_tile(&self) -> &Option<PlacedTile<Polyhex<()>, HexCoord>> {
        todo!()
    }
}

impl SimplePlacementModel<Polyhex<()>, HexCoord> for PolyhexPlacementModel {
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

    fn update_tile_shape(&mut self, _idx: usize, _shape: Polyhex<()>) {
        todo!()
    }

    fn prototile_coord(&self, _idx: usize, _pixel_position: PixelPosition) -> Option<CoordBox> {
        todo!()
    }

    fn init_hint_tile(&mut self, _position_on_board: HexCoord, _shape: Polyhex<()>) {
        todo!()
    }

    fn remove_hint_tile(&mut self) {
        todo!()
    }
}
