use crate::array_util;
use crate::backtracking::pruner::Pruner;
use crate::bitmask::Bitmask;
use crate::board::Board;
use crate::tile::Tile;
use log::debug;
use ndarray::Array2;

/// A tile with all its possible placements on the board represented as bitmasks.
///
/// The bitmasks are 1 for filled cells and 0 for empty cells.
/// If the cell is 1 in the bitmask, it means that the tile occupies that cell on the board.
/// The board itself is not represented in the bitmask.
#[derive(Clone)]
pub struct PositionedTile {
    bitmasks: Vec<Bitmask>,
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
        let all_placements: Vec<Array2<bool>> = tile
            .all_rotations
            .iter()
            .flat_map(|rotation| array_util::place_on_all_positions(board.get_array(), rotation))
            .map(|array| {
                let mut array = array.clone();
                array_util::remove_parent(board.get_array(), &mut array);
                array
            })
            .collect();

        let bitmasks: Vec<Bitmask> = all_placements
            .iter()
            .map(|array| Bitmask::from(array))
            .filter(|bitmask| !pruner.prune(bitmask))
            .collect();

        PositionedTile { bitmasks }
    }

    /// Returns a reference to Bitmasks representing all possible placements of the Tile on the Board.
    pub fn bitmasks(&self) -> &[Bitmask] {
        &self.bitmasks
    }

    #[allow(dead_code)]
    fn print_debug(&self, board_width: i32) {
        for bitmask in self.bitmasks.iter() {
            debug!("{}", &bitmask.to_string(board_width));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    #[test]
    fn test_positioned_tile_new() {
        let mut board = Board::new((3, 4));
        board[[0, 0]] = true;
        let tile = Tile::new(arr2(&[[true, true, true], [true, true, false]]));

        let positioned_tile = PositionedTile::new(
            &tile,
            &board,
            &Pruner::new_for_filling(&board, &[tile.clone()]),
        );
        assert_eq!(positioned_tile.bitmasks().len(), 15);

        assert!(!positioned_tile.bitmasks.contains(&Bitmask::from(&arr2(&[
            [false, true, true, false],
            [false, true, true, true],
            [false, false, false, false]
        ]))));
        assert!(!positioned_tile.bitmasks.contains(&Bitmask::from(&arr2(&[
            [false, false, false, false],
            [false, true, true, true],
            [false, true, true, false]
        ]))));
        assert!(!positioned_tile.bitmasks.contains(&Bitmask::from(&arr2(&[
            [false, false, true, false],
            [false, true, true, false],
            [false, true, true, false]
        ]))));
        assert!(!positioned_tile.bitmasks.contains(&Bitmask::from(&arr2(&[
            [false, false, true, false],
            [false, false, true, true],
            [false, false, true, true]
        ]))));
        assert!(!positioned_tile.bitmasks.contains(&Bitmask::from(&arr2(&[
            [false, true, true, false],
            [false, true, true, false],
            [false, false, true, false]
        ]))));
        assert!(!positioned_tile.bitmasks.contains(&Bitmask::from(&arr2(&[
            [false, false, true, true],
            [false, false, true, true],
            [false, false, true, false]
        ]))));
        assert!(!positioned_tile.bitmasks.contains(&Bitmask::from(&arr2(&[
            [false, false, false, false],
            [true, true, true, false],
            [false, true, true, false]
        ]))));

        assert!(positioned_tile.bitmasks.contains(&Bitmask::from(&arr2(&[
            [false, false, false, false],
            [true, true, false, false],
            [true, true, true, false]
        ]))));
        assert!(positioned_tile.bitmasks.contains(&Bitmask::from(&arr2(&[
            [false, false, false, false],
            [false, true, true, false],
            [false, true, true, true]
        ]))));
        assert!(positioned_tile.bitmasks.contains(&Bitmask::from(&arr2(&[
            [false, true, true, true],
            [false, true, true, false],
            [false, false, false, false]
        ]))));
        assert!(positioned_tile.bitmasks.contains(&Bitmask::from(&arr2(&[
            [false, false, false, false],
            [true, true, true, false],
            [true, true, false, false]
        ]))));

        assert!(positioned_tile.bitmasks.contains(&Bitmask::from(&arr2(&[
            [false, true, false, false],
            [true, true, false, false],
            [true, true, false, false]
        ]))));
        assert!(positioned_tile.bitmasks.contains(&Bitmask::from(&arr2(&[
            [false, false, false, true],
            [false, false, true, true],
            [false, false, true, true]
        ]))));

        assert!(positioned_tile.bitmasks.contains(&Bitmask::from(&arr2(&[
            [false, true, false, false],
            [false, true, true, false],
            [false, true, true, false]
        ]))));

        assert!(positioned_tile.bitmasks.contains(&Bitmask::from(&arr2(&[
            [false, false, true, true],
            [false, false, true, true],
            [false, false, false, true]
        ]))));

        assert!(positioned_tile.bitmasks.contains(&Bitmask::from(&arr2(&[
            [false, true, true, false],
            [false, true, true, false],
            [false, true, false, false]
        ]))));

        assert!(positioned_tile.bitmasks.contains(&Bitmask::from(&arr2(&[
            [false, true, true, false],
            [true, true, true, false],
            [false, false, false, false]
        ]))));
        assert!(positioned_tile.bitmasks.contains(&Bitmask::from(&arr2(&[
            [false, false, true, true],
            [false, true, true, true],
            [false, false, false, false]
        ]))));
        assert!(positioned_tile.bitmasks.contains(&Bitmask::from(&arr2(&[
            [false, false, false, false],
            [false, true, true, false],
            [true, true, true, false]
        ]))));
        assert!(positioned_tile.bitmasks.contains(&Bitmask::from(&arr2(&[
            [false, false, false, false],
            [false, false, true, true],
            [false, true, true, true]
        ]))));
        assert!(positioned_tile.bitmasks.contains(&Bitmask::from(&arr2(&[
            [false, true, true, true],
            [false, false, true, true],
            [false, false, false, false]
        ]))));
        assert!(positioned_tile.bitmasks.contains(&Bitmask::from(&arr2(&[
            [false, false, false, false],
            [false, true, true, true],
            [false, false, true, true]
        ]))));
    }

    #[test]
    fn test_positioned_tile_new_no_placements() {
        let board = Board::new((2, 2));
        let tile = Tile::new(arr2(&[
            [true, true, true],
            [true, false, false],
            [false, false, false],
        ]));

        let positioned_tile = PositionedTile::new(
            &tile,
            &board,
            &Pruner::new_for_filling(&board, &[tile.clone()]),
        );
        assert!(positioned_tile.bitmasks.is_empty());
    }

    #[test]
    fn test_positioned_tile_new_full_board() {
        let mut board = Board::new((2, 2));
        board[[0, 0]] = true;
        board[[0, 1]] = true;
        board[[1, 0]] = true;
        board[[1, 1]] = true;
        let tile = Tile::new(arr2(&[[true]]));

        let positioned_tile = PositionedTile::new(
            &tile,
            &board,
            &Pruner::new_for_filling(&board, &[tile.clone()]),
        );
        assert!(positioned_tile.bitmasks.is_empty());
    }

    #[test]
    fn test_positioned_tile_new_duplicates() {
        let board = Board::new((3, 3));
        let tile = Tile::new(arr2(&[[true, true], [true, true]]));

        let positioned_tile = PositionedTile::new(
            &tile,
            &board,
            &Pruner::new_for_filling(&board, &[tile.clone()]),
        );
        assert_eq!(positioned_tile.bitmasks.len(), 4);
    }
}
