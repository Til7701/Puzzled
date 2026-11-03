use crate::model::placement::board::PlacedBoard;
use crate::model::placement::grid::GridConfig;
use crate::model::placement::tile::PlacedTile;
use crate::model::placement::{PixelPosition, initial};
use crate::model::puzzle::PuzzleModel;
use puzzle_config::PuzzleConfig;
use puzzled_common::polyform::Polyform;
use puzzled_common::polyform::coord::Coord;
use puzzled_common::polyform::coord::coord_box::CoordBox;
use puzzled_common::polyform::coord::regular::RegularCoord;
use puzzled_common::polyform::poly::polyomino::Polyomino;
use puzzled_common::polyform::poly::{Poly, UnitPoly};

pub(super) const MIN_CELLS_TO_THE_TOP_OF_BOARD: u32 = 1;
pub(super) const MIN_CELLS_TO_THE_SIDES_OF_BOARD: u32 = 6;
pub(super) const MIN_CELLS_TO_THE_BOTTOM_OF_BOARD: u32 = 6;

#[derive(Debug, Default)]
pub struct PolyominoPlacementModel {
    area_pixel_size: PixelPosition,
    min_area_pixel_size: PixelPosition,
    grid_config: GridConfig<RegularCoord>,
    board: PlacedBoard<RegularCoord>,
    tiles: Vec<PlacedTile<Polyomino<()>, RegularCoord>>,
    hint_tile: Option<PlacedTile<Polyomino<()>, RegularCoord>>,
}

impl PolyominoPlacementModel {
    pub fn new(puzzle_model: &PuzzleModel) -> Self {
        let puzzle_config = puzzle_model.config();

        let grid_config = Self::initial_grid_config(puzzle_config);

        let board_config = puzzle_config.board_config();
        let board = PlacedBoard::new(
            board_config.layout().dim().into(),
            PixelPosition::default(),
            RegularCoord::new(
                MIN_CELLS_TO_THE_SIDES_OF_BOARD,
                MIN_CELLS_TO_THE_TOP_OF_BOARD,
            ),
        );

        let start_positions = initial::calculate_tile_start_positions(
            puzzle_config.tiles(),
            puzzle_config,
            RegularCoord::new(
                MIN_CELLS_TO_THE_SIDES_OF_BOARD,
                MIN_CELLS_TO_THE_TOP_OF_BOARD,
            ),
        );
        let tiles: Vec<PlacedTile<Polyomino<()>, RegularCoord>> = start_positions
            .iter()
            .enumerate()
            .map(move |(i, pos)| {
                let config = &puzzle_model.config().tiles()[i];
                let shape: Polyomino<()> = match config.base() {
                    Polyform::Polyomino(p) => p.clone(),
                    Polyform::Hexomino(_) => unreachable!(),
                };
                let cell_size = shape.dim().into();
                PlacedTile::new(config.name().clone(), shape, cell_size, *pos)
            })
            .collect();

        PolyominoPlacementModel {
            area_pixel_size: PixelPosition(100.0, 100.0),
            min_area_pixel_size: PixelPosition(100.0, 100.0),
            grid_config,
            board,
            tiles,
            hint_tile: None,
        }
    }

    pub fn tiles<'a>(&self) -> &'a [impl UnitPoly] {
        &self.tiles
    }

    pub fn tiles_mut<'a>(&mut self) -> &'a mut [PlacedTile<impl UnitPoly, impl Coord>] {
        &mut self.tiles
    }

    fn update_pixel_size(&self, total_view_size_pixel: PixelPosition, min_cell_size_pixel: u32) {
        todo!()
    }

    fn min_area_size(&self) -> PixelPosition {
        todo!()
    }

    fn board_pixel_position(&self) -> PixelPosition {
        todo!()
    }

    fn board_cell_position(&self) -> &RegularCoord {
        todo!()
    }

    fn board_size(&self) -> PixelPosition {
        todo!()
    }

    fn tile_pixel_position(&self, idx: usize) -> Option<PixelPosition> {
        todo!()
    }

    fn tile_cell_position(&self, idx: usize) -> Option<RegularCoord> {
        todo!()
    }

    fn tile_size(&self, idx: usize) -> PixelPosition {
        todo!()
    }

    fn update_tile_pixel_position(&self, idx: usize, position: PixelPosition) {
        todo!()
    }

    fn update_tile_shape(&self, idx: usize, shape: Polyomino<()>) {
        todo!()
    }

    fn init_hint_tile(&self, position_on_board: &CoordBox, shape: Polyomino<()>) {
        todo!()
    }

    fn hint_tile_position(&self) -> Option<PixelPosition> {
        todo!()
    }

    fn hint_tile_size(&self) -> Option<PixelPosition> {
        todo!()
    }

    fn remove_hint_tile(&self) {
        todo!()
    }

    fn connect_tile_moved<F: Fn() + 'static>(&self, callback: F) {
        todo!()
    }

    fn emit_tile_moved(&self) {
        todo!()
    }

    fn initial_grid_config(puzzle_config: &PuzzleConfig) -> GridConfig<RegularCoord> {
        todo!()
    }

    fn update_grid_layout(&self) {
        todo!()
    }

    fn tiles_required_cells(&self) -> RegularCoord {
        todo!()
    }

    fn update_grid_config(
        &self,
        grid_config: GridConfig<RegularCoord>,
        board_position_cells: RegularCoord,
    ) {
        todo!()
    }

    fn move_all_elements_by(&self, offset_cells: RegularCoord) {
        todo!()
    }
}
