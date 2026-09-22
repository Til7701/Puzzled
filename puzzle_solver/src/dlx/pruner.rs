use crate::board::Board;
use crate::tile::Tile;
use puzzled_common::polyform::Polyform;

/// The pruner provides some functions to exclude various positions from evaluation.
pub struct Pruner {
    /// The minimum amount of cells any tile has.
    min_tile_size: usize,
}

impl Pruner {
    /// Creates a new pruner that takes the board and the given tiles into account.
    pub fn new(_: &Board, tiles: &[Tile]) -> Self {
        let min_tile_size = tiles
            .iter()
            .map(|tile| {
                tile.base()
                    .count_biggest_connected_area_of_cells_matching(true)
            })
            .min()
            .unwrap_or(0);

        Pruner { min_tile_size }
    }

    /// This method takes a shape where one tile or more have been placed and determines, whether
    /// there are spaces, where no tiles can be placed. This means that if this method returns true,
    /// the given path can be abandoned.
    #[inline(always)]
    pub fn prune_positioned_tile_with_board(&self, shape: &Polyform<()>) -> bool {
        shape.count_smallest_connected_area_of_cells_matching(false) < self.min_tile_size
    }
}
