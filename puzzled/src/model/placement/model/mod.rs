use crate::model::placement::PixelPosition;
use crate::model::placement::grid::GridConfig;
use crate::model::placement::tile::PlacedTile;
use crate::model::placement::translation::polyomino::PolyominoPlacementModel;
use puzzle_config::PuzzleConfig;
use puzzled_common::polyform::coord::Coord;
use puzzled_common::polyform::poly::UnitPoly;

#[derive(Debug, Default)]
pub enum PlacementModel {
    #[default]
    Polyomino(PolyominoPlacementModel),
}

impl PlacementModel {
    fn tiles<'a>(&self) -> &'a [impl UnitPoly];
    fn tiles_mut<'a>(&mut self) -> &'a mut [PlacedTile<impl UnitPoly, impl Coord>];

    /// Recalculates tile and board positions for the new area size.
    ///
    /// The given `total_view_size_pixel` must be the actual size of the view the tiles
    /// and board are placed in or should be placed in.
    ///
    /// After this function completed, you can fetch the sizes and positions using the
    /// corresponding getters.
    ///
    /// # Arguments
    ///
    /// * `total_view_size_pixel`: the size of the view in pixels
    /// * `min_cell_size_pixel`: the min cell size for calculating the min area size
    ///
    /// returns: ()
    fn update_pixel_size(&self, total_view_size_pixel: PixelPosition, min_cell_size_pixel: u32);

    /// Returns the minimum size the view needs to place all tiles and board while respecting
    /// the minimum cell size.
    fn min_area_size(&self) -> PixelPosition;

    /// The board position in pixels.
    fn board_pixel_position(&self) -> PixelPosition;

    /// The board position in cells.
    fn board_cell_position(&self) -> &C;

    /// The board size in pixels.
    fn board_size(&self) -> PixelPosition;

    /// The position of the tile with the given index in pixels.
    ///
    /// None, if the tile is being dragged.
    ///
    /// The index is the same as the index of the tile in the [PuzzleModel].
    fn tile_pixel_position(&self, idx: usize) -> Option<PixelPosition>;

    /// The position of the tile with the given index in cells.
    ///
    /// None, if the tile is being dragged.
    ///
    /// The index is the same as the index of the tile in the [PuzzleModel].
    fn tile_cell_position(&self, idx: usize) -> Option<C>;

    /// The size of the tile in pixels.
    fn tile_size(&self, idx: usize) -> PixelPosition;

    /// Updates the tile position by calculating the new position in cells from the given
    /// position in pixels. This calculation snaps the position to the nearest cell
    /// position.
    ///
    /// # Arguments
    ///
    /// * `idx`: the index of the tile
    /// * `position`: the new position in pixels
    ///
    /// returns: ()
    fn update_tile_pixel_position(&self, idx: usize, position: PixelPosition);

    /// Updates the shape for the given tile. This must be called, if the tile rotated
    /// or flipped. The given shape must be a valid result of flipping or rotating the base
    /// shape.
    ///
    /// # Arguments
    ///
    /// * `idx`: the index of the tile to update
    /// * `shape`: the new rotation
    ///
    /// returns: ()
    fn update_tile_shape(&self, idx: usize, shape: P);

    /// Initializes a new hint tile at the given position on the board and with the
    /// given shape.
    /// This replaces the hint tile currently stored in the PlacementModel.
    /// Make sure to call [Self::remove_hint_tile] before this call and remove the
    /// old hint tile from the view.
    ///
    /// # Arguments
    ///
    /// * `position_on_board`: the position of the hint tile relative to the board
    /// * `shape`: the shape of the hint tile
    ///
    /// returns: ()
    fn init_hint_tile(&self, position_on_board: &C, shape: P);

    /// Returns the position of the hint tile in pixels.
    ///
    /// None, if no hint tile is registered.
    fn hint_tile_position(&self) -> Option<PixelPosition>;

    /// Returns the size of the hint tile in pixels.
    ///
    /// None, if no hint tile is registered.
    fn hint_tile_size(&self) -> Option<PixelPosition>;

    /// Removes the hint tile from the placement model.
    /// Positions will no longer be calculated for the old hint tile.
    fn remove_hint_tile(&self);

    /// Connects to the `tile_moved` signal which is emitted when a tile changes positon
    /// or rotation.
    fn connect_tile_moved<F: Fn() + 'static>(&self, callback: F);

    fn emit_tile_moved(&self);

    fn initial_grid_config(puzzle_config: &PuzzleConfig) -> GridConfig<C>;

    /// Calculates how the grid should be laid out based on the current positions of the tiles
    /// and the size of the board.
    ///
    /// This function should ensure, that all tiles are visible and the board is centered.
    /// [Self::update_grid_config()] is called if the grid layout needs to be updated based on the
    /// new calculations.
    fn update_grid_layout(&self);

    /// Calculates the dimensions required to fit all tiles in their current positions.
    fn tiles_required_cells(&self) -> C;

    /// Update the grid configuration and move all elements in case the board offset has changed.
    fn update_grid_config(&self, grid_config: GridConfig<C>, board_position_cells: C);

    /// Moves all elements by the given offset in cells.
    ///
    /// If the new position of an element would be negative, it is set to 0 to ensure that all
    /// elements remain visible.
    fn move_all_elements_by(&self, offset_cells: C);
}
