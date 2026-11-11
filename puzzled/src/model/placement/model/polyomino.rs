use crate::model::placement::board::PlacedBoard;
use crate::model::placement::grid::GridConfig;
use crate::model::placement::prelude::{InternalPlacementModel, SimplePlacementModel};
use crate::model::placement::tile::PlacedTile;
use crate::model::placement::{PixelPosition, initial};
use crate::model::puzzle::PuzzleModel;
use puzzle_config::PuzzleConfig;
use puzzled_common::polyform::{CoordBox, Poly, Polyform, Polyomino, RegularCoord};

pub(in crate::model::placement) const MIN_CELLS_TO_THE_TOP_OF_BOARD: i32 = 1;
pub(in crate::model::placement) const MIN_CELLS_TO_THE_SIDES_OF_BOARD: i32 = 6;
pub(in crate::model::placement) const MIN_CELLS_TO_THE_BOTTOM_OF_BOARD: i32 = 6;

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
        let board_dim = match board_config.layout().dim() {
            CoordBox::Regular(r) => r,
            CoordBox::Hex(_) => {
                unreachable!()
            }
        };
        let board = PlacedBoard::new(
            board_dim,
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
                let cell_size = shape.dim();
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

    fn initial_grid_config(puzzle_config: &PuzzleConfig) -> GridConfig<RegularCoord> {
        let dim = match puzzle_config.board_config().layout().dim() {
            CoordBox::Regular(r) => r,
            CoordBox::Hex(_) => unreachable!(),
        };
        let board_cell_width = dim.x();
        let board_cell_height = dim.y();

        let required_h_cells = board_cell_width + MIN_CELLS_TO_THE_SIDES_OF_BOARD * 2;
        let required_v_cells =
            board_cell_height + MIN_CELLS_TO_THE_TOP_OF_BOARD + MIN_CELLS_TO_THE_BOTTOM_OF_BOARD;
        GridConfig {
            grid_cells: RegularCoord::new(required_h_cells, required_v_cells),
            min_grid_cells: RegularCoord::new(required_h_cells, required_v_cells),
            cell_size_pixel: 1,
        }
    }

    /// Calculates how the grid should be laid out based on the current positions of the tiles
    /// and the size of the board.
    ///
    /// This function should ensure, that all tiles are visible and the board is centered.
    /// [Self::update_grid_config()] is called if the grid layout needs to be updated based on the
    /// new calculations.
    pub fn update_grid_layout(&mut self) {
        let area_pixel_size = self.area_pixel_size;
        let available_width_pixel = area_pixel_size.0;
        let available_height_pixel = area_pixel_size.1;

        let board_size_cells = self.board.cell_size();
        let board_size_cells_with_margin = board_size_cells.add_tuple((
            MIN_CELLS_TO_THE_SIDES_OF_BOARD * 2,
            MIN_CELLS_TO_THE_TOP_OF_BOARD + MIN_CELLS_TO_THE_BOTTOM_OF_BOARD,
        ));
        let tiles_required_cells = self.tiles_required_cells();
        let required_cells = board_size_cells_with_margin.max(tiles_required_cells);

        let cell_width_pixel = (available_width_pixel / required_cells.x() as f64).floor() as i32;
        let cell_height_pixel = (available_height_pixel / required_cells.y() as f64).floor() as i32;
        let cell_size_pixel = cell_width_pixel.min(cell_height_pixel) as u32;

        let grid_h_cell_count = (available_width_pixel / cell_size_pixel as f64).floor() as i32;
        let min_grid_h_cell_count = required_cells.x();
        let grid_v_cell_count = (available_height_pixel / cell_size_pixel as f64).floor() as i32;
        let min_grid_v_cell_count = required_cells.y();

        let board_offset_cells = RegularCoord::new(
            (grid_h_cell_count - board_size_cells.x()) / 2,
            MIN_CELLS_TO_THE_TOP_OF_BOARD,
        );

        let grid_config = GridConfig {
            grid_cells: RegularCoord::new(grid_h_cell_count, grid_v_cell_count),
            min_grid_cells: RegularCoord::new(min_grid_h_cell_count, min_grid_v_cell_count),
            cell_size_pixel,
        };
        let old_grid_config = &self.grid_config;
        let old_board_position_cells = self.board.position_cells();
        if *old_grid_config != grid_config || old_board_position_cells != board_offset_cells {
            self.update_grid_config(grid_config, board_offset_cells);
        }
    }

    /// Calculates the dimensions required to fit all tiles in their current positions.
    fn tiles_required_cells(&self) -> RegularCoord {
        let mut required_cells = RegularCoord::new(0, 0);
        let mut lowest_position_cells = RegularCoord::new(0, 0);
        for tile in self.tiles.iter() {
            let tile_size = tile.cell_size();
            required_cells = required_cells.max(tile_size + tile.position_cells());
            lowest_position_cells = lowest_position_cells.min(tile.position_cells());
        }
        required_cells - lowest_position_cells
    }

    /// Update the grid configuration and move all elements in case the board offset has changed.
    fn update_grid_config(
        &mut self,
        grid_config: GridConfig<RegularCoord>,
        board_position_cells: RegularCoord,
    ) {
        let old_board_position_cells = self.board.position_cells();

        if old_board_position_cells.x() != board_position_cells.x() {
            self.move_all_elements_by(RegularCoord::new(
                board_position_cells.x() - old_board_position_cells.x(),
                0,
            ));
        }

        self.grid_config = grid_config;
        self.board.set_position_cells(board_position_cells);
    }

    /// Moves all elements by the given offset in cells.
    ///
    /// If the new position of an element would be negative, it is set to 0 to ensure that all
    /// elements remain visible.
    fn move_all_elements_by(&mut self, offset_cells: RegularCoord) {
        for tile in self.tiles.iter_mut() {
            let position_cells = tile.position_cells();
            let mut new_position_cells = position_cells + offset_cells;
            if new_position_cells.x() < 0 {
                new_position_cells = new_position_cells.with_x(0);
            }
            if new_position_cells.y() < 0 {
                new_position_cells = new_position_cells.with_y(0);
            }
            tile.set_position_cells(new_position_cells);
        }
        let opt_hint_tile = &mut self.hint_tile;
        if let Some(hint_tile) = opt_hint_tile {
            let new_position_cells = hint_tile.position_cells() + offset_cells;
            hint_tile.set_position_cells(new_position_cells);
        }
    }

    /// Takes all cell positions, calculates pixel positions for them and writes them
    /// back to the tiles and board.
    fn update_pixel_from_cell_data(&mut self) {
        let position_cells = self.board.position_cells();
        self.board
            .set_position_pixel(self.translate_cells_to_pixels(position_cells));
        let size_cells = self.board.cell_size();
        self.board
            .set_pixel_size(self.translate_cells_to_pixels(size_cells));

        for i in 0..self.tiles.len() {
            let (position_pixels, pixel_size) = {
                let tile = &self.tiles[i];
                (
                    self.translate_cells_to_pixels(tile.position_cells()),
                    self.translate_cells_to_pixels(tile.cell_size()),
                )
            };
            let tile = &mut self.tiles[i];
            tile.set_position_pixels(position_pixels);
            tile.set_pixel_size(pixel_size);
        }
        if self.hint_tile.is_some() {
            let (position_pixels, pixel_size) = {
                #[allow(clippy::unnecessary_unwrap)]
                let ht = &self.hint_tile.as_ref().unwrap();
                (
                    self.translate_cells_to_pixels(ht.position_cells()),
                    self.translate_cells_to_pixels(ht.cell_size()),
                )
            };
            #[allow(clippy::unnecessary_unwrap)]
            let ht = &mut self.hint_tile.as_mut().unwrap();
            ht.set_position_pixels(position_pixels);
            ht.set_pixel_size(pixel_size);
        }
    }

    fn translate_pixels_to_cells(&self, position: PixelPosition) -> RegularCoord {
        let cell_size = self.grid_config.cell_size_pixel;
        let pixel = position.div_scalar(cell_size as f64).round();
        RegularCoord::new(pixel.0 as i32, pixel.1 as i32)
    }

    fn translate_cells_to_pixels(&self, position: RegularCoord) -> PixelPosition {
        let cell_size = self.grid_config.cell_size_pixel;
        PixelPosition(position.x() as f64, position.y() as f64).mul_scalar(cell_size as f64)
    }
}

impl InternalPlacementModel<Polyomino<()>, RegularCoord> for PolyominoPlacementModel {
    fn tiles(&self) -> &[PlacedTile<Polyomino<()>, RegularCoord>] {
        &self.tiles
    }

    fn board(&self) -> &PlacedBoard<RegularCoord> {
        &self.board
    }

    fn hint_tile(&self) -> &Option<PlacedTile<Polyomino<()>, RegularCoord>> {
        &self.hint_tile
    }
}

impl SimplePlacementModel<Polyomino<()>, RegularCoord> for PolyominoPlacementModel {
    fn update_pixel_size(
        &mut self,
        total_view_size_pixel: PixelPosition,
        min_cell_size_pixel: u32,
    ) {
        if total_view_size_pixel.0 < 100.0 || total_view_size_pixel.1 < 100.0 {
            return;
        }
        self.area_pixel_size = total_view_size_pixel;
        self.update_grid_layout();
        let min_grid_cells = &self.grid_config.min_grid_cells;
        self.min_area_pixel_size =
            PixelPosition(min_grid_cells.x() as f64, min_grid_cells.y() as f64)
                .mul_scalar(min_cell_size_pixel as f64);
        self.update_pixel_from_cell_data();
    }

    fn min_area_size(&self) -> PixelPosition {
        self.min_area_pixel_size
    }

    fn update_tile_pixel_position(&mut self, idx: usize, position: PixelPosition) {
        let position_cells = self.translate_pixels_to_cells(position);
        let position_pixels = self.translate_cells_to_pixels(position_cells);
        let tile = self.tiles.get_mut(idx).unwrap();
        tile.set_position_pixels(position_pixels);
        tile.set_position_cells(position_cells);
    }

    fn update_tile_dragged(&mut self, idx: usize, dragged: bool) {
        let tile = self.tiles.get_mut(idx).unwrap();
        tile.set_dragged(dragged);
    }

    fn update_tile_shape(&mut self, idx: usize, shape: Polyomino<()>) {
        let old = self.tiles.get_mut(idx).unwrap();
        old.set_cell_size(shape.dim());
        old.set_current_rotation(shape);
    }

    fn prototile_coord(&self, _idx: usize, _pixel_position: PixelPosition) -> Option<CoordBox> {
        Some(RegularCoord::new(0, 0).into()) // FIXME see #255
    }

    fn init_hint_tile(&mut self, position_on_board: RegularCoord, shape: Polyomino<()>) {
        let position = self.board.position_cells() + position_on_board;
        let size = shape.dim();
        self.hint_tile = Some(PlacedTile::new(None, shape, size, position));
    }

    fn remove_hint_tile(&mut self) {
        self.hint_tile = None;
    }
}
