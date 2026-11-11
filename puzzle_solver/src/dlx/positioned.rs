use crate::board::Board;
use crate::dlx::pruner::Pruner;
use crate::tile::Tile;
use puzzled_common::polyform::{Polyform, UnitPoly};

/// A tile with all its possible placements on the board represented as bitmasks.
///
/// The bitmasks are 1 for filled cells and 0 for empty cells.
/// If the cell is 1 in the bitmask, it means that the tile occupies that cell on the board.
/// The board itself is not represented in the bitmask.
#[derive(Clone, Debug)]
pub struct PositionedTile {
    all_placements: Vec<Polyform<()>>,
}

impl PositionedTile {
    /// Creates a new PositionedTile from a Tile and a Board.
    ///
    /// The resulting PositionedTile contains all possible placements of the Tile on the Board,
    /// represented as Bitmasks.
    ///
    /// # Arguments
    ///
    /// * `tile`: The Tile to be placed on the Board.
    /// * `board`: The Board on which the Tile will be placed.
    ///
    /// returns: PositionedTile
    pub(crate) fn new(tile: &Tile, board: &Board, pruner: &Pruner) -> Self {
        let all_placements = tile
            .all_rotations
            .iter()
            .flat_map(|rotation| board.get_polyform().place_on_all_positions(rotation))
            .filter(|shape| !pruner.prune_positioned_tile_with_board(shape))
            .map(|mut array| {
                array.remove_parent(board.get_polyform());
                array
            })
            .collect();

        PositionedTile { all_placements }
    }

    /// Returns a reference to Bitmasks representing all possible placements of the Tile on the Board.
    pub fn all_placements(&self) -> &[Polyform<()>] {
        &self.all_placements
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use puzzled_common::polyform::Polyomino;

    #[test]
    fn test_new() {
        let board = Board::new(
            Polyomino::from_bool_slice(&[
                [true, false, false],
                [false, false, false],
                [false, false, false],
                [false, false, false],
            ])
            .into(),
        );
        let tiles = vec![
            Tile::new(
                42,
                Polyomino::from_bool_slice(&[[true, true, true], [true, true, false]]).into(),
            ),
            Tile::new(
                43,
                Polyomino::from_bool_slice(&[[true, true, true], [true, true, true]]).into(),
            ),
        ];
        let pruner = Pruner::new(&board, &tiles);

        let pt0 = PositionedTile::new(&tiles[0], &board, &pruner);
        assert_eq!(pt0.all_placements().len(), 22);

        let pt1 = PositionedTile::new(&tiles[1], &board, &pruner);
        assert_eq!(pt1.all_placements().len(), 5);
    }
}
