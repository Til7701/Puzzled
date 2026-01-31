use crate::bitmask::Bitmask;
use crate::board::Board;
use crate::tile::Tile;
use banned::BannedBitmask;

mod banned;

pub struct Pruner {
    /// For each relevant bit on the board, a list of banned bitmasks.
    /// The bitmasks can be checked against an index of the current board state if it is
    /// empty.
    banned_bitmasks: Vec<Vec<BannedBitmask>>,
}

impl Pruner {
    /// Creates a new Pruner for use while filling the board with tiles.
    pub fn new_for_filling(board: &Board, tiles: &[Tile]) -> Self {
        let banned_bitmasks = banned::create_banned_bitmasks_for_filling(&board, &tiles);

        Pruner { banned_bitmasks }
    }

    /// Analyzes the current board state and decides whether a solution is still possible.
    /// If a solution is determined to be impossible, it returns true.
    /// Otherwise, it returns false.
    ///
    /// # Arguments
    ///
    /// * `current_board`: The board to analyze.
    ///
    /// returns: bool
    pub fn prune(&self, current_board: &Bitmask) -> bool {
        // TODO start from the first empty cell on the board and end at the last empty cell
        for index in 0..current_board.relevant_bits() {
            if !current_board.get_bit(index) {
                for banned in self.banned_bitmasks[index].iter() {
                    if banned.matches(current_board) {
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::backtracking::pruner::Pruner;
    use crate::bitmask::Bitmask;
    use crate::board::Board;
    use crate::tile::Tile;
    use ndarray::arr2;

    #[test]
    fn test_pruner_trominoes() {
        let board: Board = arr2(&[
            [false, false, false],
            [true, false, true],
            [true, false, false],
        ])
        .into();
        let tiles = vec![
            Tile::new(arr2(&[[true, true, true]])),
            Tile::new(arr2(&[[true, true], [true, false]])),
        ];

        let pruner = Pruner::new_for_filling(&board, &tiles);
        assert!(pruner.banned_bitmasks.len() > 0);

        // Assert prune
        assert!(pruner.prune(&Bitmask::from(&arr2(&[
            [false, true, false],
            [true, false, true],
            [true, false, false],
        ]))));
        assert!(pruner.prune(&Bitmask::from(&arr2(&[
            [true, false, true],
            [true, true, false],
            [true, false, true],
        ]))));
        assert!(pruner.prune(&Bitmask::from(&arr2(&[
            [false, true, false],
            [true, false, true],
            [true, false, false],
        ]))));
        assert!(pruner.prune(&Bitmask::from(&arr2(&[
            [false, true, false],
            [true, false, true],
            [true, true, false],
        ]))));
        assert!(pruner.prune(&Bitmask::from(&arr2(&[
            [false, false, true],
            [true, true, false],
            [true, false, true],
        ]))));
        assert!(pruner.prune(&Bitmask::from(&arr2(&[
            [false, false, false],
            [true, true, false],
            [true, false, true],
        ]))));
        assert!(pruner.prune(&Bitmask::from(&arr2(&[
            [false, true, false],
            [true, true, true],
            [true, true, false],
        ]))));
        assert!(pruner.prune(&Bitmask::from(&arr2(&[
            [false, true, true],
            [true, true, true],
            [true, false, false],
        ]))));

        // Assert not prune
        assert!(!pruner.prune(&Bitmask::from(&arr2(&[
            [false, false, false],
            [true, false, true],
            [true, false, false],
        ]))));
        assert!(!pruner.prune(&Bitmask::from(&arr2(&[
            [false, false, false],
            [true, true, true],
            [true, true, true],
        ]))));
        assert!(!pruner.prune(&Bitmask::from(&arr2(&[
            [true, true, true],
            [true, false, true],
            [true, false, false],
        ]))));
        assert!(!pruner.prune(&Bitmask::from(&arr2(&[
            [true, true, true],
            [true, true, true],
            [true, true, true],
        ]))));
    }

    #[test]
    fn test_pruner_simple() {
        let board: Board = arr2(&[
            [true, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ])
        .into();
        let tiles = vec![
            Tile::new(arr2(&[[false, true, true], [true, true, true]])),
            Tile::new(arr2(&[[true, true, true], [true, true, true]])),
        ];

        let pruner = Pruner::new_for_filling(&board, &tiles);
        assert!(pruner.banned_bitmasks.len() > 0);

        // Assert not prune
        assert!(!pruner.prune(&Bitmask::from(&arr2(&[
            [true, false, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ]))));
        assert!(!pruner.prune(&Bitmask::from(&arr2(&[
            [true, true, false, false],
            [true, true, false, false],
            [true, true, false, false],
        ]))));
        assert!(!pruner.prune(&Bitmask::from(&arr2(&[
            [true, false, true, true],
            [false, false, true, true],
            [false, false, true, true],
        ]))));
        assert!(!pruner.prune(&Bitmask::from(&arr2(&[
            [true, true, true, true],
            [true, true, true, true],
            [true, true, true, true],
        ]))));
    }
}
