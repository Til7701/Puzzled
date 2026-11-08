use crate::app::puzzle::puzzle_area::puzzle_state::PuzzleState;
use crate::model::extension::PuzzleTypeExtension;
use crate::model::placement::PixelPosition;
use crate::model::placement::board::PlacedBoard;
use crate::model::placement::tile::PlacedTile;
use crate::model::puzzle::PuzzleModel;
use puzzled_common::polyform::{Coord, CoordBox, UnitPoly};
use std::cell::Ref;

pub trait InternalPlacementModel<P: UnitPoly, C: Coord> {
    fn tiles(&self) -> &[PlacedTile<P, C>];
    fn tiles_mut(&mut self) -> &mut [PlacedTile<P, C>];

    fn board(&self) -> &PlacedBoard<C>;
    fn board_mut(&mut self) -> &mut PlacedBoard<C>;

    fn hint_tile(&self) -> &Option<PlacedTile<P, C>>;
}

pub trait SimplePlacementModel<P: UnitPoly, C: Coord> {
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
    fn update_pixel_size(&mut self, total_view_size_pixel: PixelPosition, min_cell_size_pixel: u32);

    /// Returns the minimum size the view needs to place all tiles and board while respecting
    /// the minimum cell size.
    fn min_area_size(&self) -> PixelPosition;

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
    fn update_tile_pixel_position(&mut self, idx: usize, position: PixelPosition);

    /// Marks the tile as being dragged or not.
    ///
    /// If a tile is marked as dragged, its position is not valid and should not be recalculated.
    ///
    /// # Arguments
    ///
    /// * `idx`: the index of the tile
    /// * `dragged`: true, if the tile is being dragged, otherwise false
    ///
    /// returns: ()
    fn update_tile_dragged(&mut self, idx: usize, dragged: bool);

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
    fn update_tile_shape(&mut self, idx: usize, shape: P);

    fn prototile_coord(&self, idx: usize, pixel_position: PixelPosition) -> Option<CoordBox>;

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
    fn init_hint_tile(&mut self, position_on_board: C, shape: P);

    /// Removes the hint tile from the placement model.
    /// Positions will no longer be calculated for the old hint tile.
    fn remove_hint_tile(&mut self);
}

pub trait FullPlacementModel<P: UnitPoly, C: Coord>: SimplePlacementModel<P, C> {
    /// Finds the index of a tile that has the same base shape.
    ///
    /// None, if no tile was found.
    ///
    /// # Arguments
    ///
    /// * `base`: the base shape to search for
    ///
    /// returns: Option<usize>
    fn find_tile_matching_base(&self, base: &P) -> Option<usize>;

    fn extract_puzzle_state(
        &self,
        puzzle: &PuzzleModel,
        puzzle_type_extension: Ref<Option<PuzzleTypeExtension>>,
    ) -> Result<PuzzleState, String>;

    /// The board position in pixels.
    fn board_pixel_position(&self) -> PixelPosition;

    /// The board position in cells.
    fn board_cell_position(&self) -> C;

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

    /// Returns the position of the hint tile in pixels.
    ///
    /// None, if no hint tile is registered.
    fn hint_tile_position(&self) -> Option<PixelPosition>;

    /// Returns the size of the hint tile in pixels.
    ///
    /// None, if no hint tile is registered.
    fn hint_tile_size(&self) -> Option<PixelPosition>;
}
