use crate::board::Board;
use crate::tile::Tile;
use log::debug;

/// Performs a plausibility check for the given board and tiles.
/// It checks the following conditions:
///
/// 1. The total area of the tiles must equal the area of the board.
/// 2. Each tile must fit within the board dimensions in at least one rotation. TODO
///
/// If all conditions are met, the function returns true; otherwise, it returns false.
///
/// # Arguments
///
/// * `board`: The board the tiles should be placed on.
/// * `tiles`: A slice of tiles to be placed on the board.
///
/// returns: bool
pub(crate) fn check(board: &Board, tiles: &[Tile]) -> bool {
    let board_area = board.get_array().iter().filter(|&&cell| !cell).count();
    let tiles_area: usize = tiles
        .iter()
        .map(|tile| tile.base.iter().filter(|&&cell| cell).count())
        .sum();
    debug!(
        "Plausibility check: board area = {}, tiles area = {}",
        board_area, tiles_area
    );
    tiles_area == board_area
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;
    use crate::tile::Tile;
    use ndarray::arr2;

    #[test]
    fn test_check_passing() {
        let board = Board::new((3, 3));
        let tile1 = Tile::new(arr2(&[[true, true], [true, true], [true, false]]));
        let tile2 = Tile::new(arr2(&[[false, true], [false, true], [true, true]]));
        let tiles = vec![tile1, tile2];

        assert!(check(&board, &tiles));
    }

    #[test]
    fn test_check_failing() {
        let board = Board::new((3, 3));
        let tile1 = Tile::new(arr2(&[[true, true], [true, false]]));
        let tile2 = Tile::new(arr2(&[[false, true], [false, true]]));
        let tiles = vec![tile1, tile2];

        assert!(!check(&board, &tiles));
    }
}
