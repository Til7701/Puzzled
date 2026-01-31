use crate::bitmask::Bitmask;
use crate::board::Board;
use crate::plausibility::check;
use crate::result::{Solution, UnsolvableReason};
use crate::tile::Tile;
use log::debug;
use tokio_util::sync::CancellationToken;

mod array_util;
mod backtracking;
mod bitmask;
pub mod board;
mod plausibility;
pub mod result;
pub mod tile;

/// Tries to place all given tiles on the board, filling it completely.
/// If successful, returns a Solution; otherwise, returns an UnsolvableReason.
/// TODO the solution currently does not contain the placements.
/// A successful result is reached, if all tiles were placed on the board without overlapping
/// and all empty cells on the board are covered.
///
/// The cancellation token can be used to cancel the operation.
/// The operation may be cancelled at any time, in which case it will return
/// after some time. It may still be successful if it was close to finishing.
/// It may also return an error if it was cancelled before it could find a solution.
///
/// # Arguments
///
/// * `board`: The board to place the tiles on to fill it completely.
/// * `tiles`: The tiles to place on the board.
/// * `cancel_token`: A cancellation token to cancel the operation.
///
/// returns: Result<Solution, UnsolvableReason>
///
/// # Examples
///
/// ```
/// use ndarray::arr2;
/// use puzzle_solver::board::Board;
/// use puzzle_solver::tile::Tile;
/// use puzzle_solver::solve_all_filling;
/// use tokio_util::sync::CancellationToken;
///
/// let mut board = Board::new((3, 4));
/// board[[0, 0]] = true;
/// let tiles = vec![
///     Tile::new(arr2(&[[true, true, true], [true, true, true]])),
///     Tile::new(arr2(&[[true, true, true], [true, true, false]])),
/// ];
/// let cancel_token = CancellationToken::new();
///
/// let result = tokio::runtime::Runtime::new().unwrap().block_on(solve_all_filling(board, &tiles, cancel_token));
/// assert!(result.is_ok());
/// ```
pub async fn solve_all_filling(
    board: Board,
    tiles: &[Tile],
    cancel_token: CancellationToken,
) -> Result<Solution, UnsolvableReason> {
    if !check(&board, &tiles) {
        debug!("Plausibility check failed.");
        return Err(UnsolvableReason::NoFit);
    }

    let mut board = board;
    board.trim();

    if board.get_array().iter().filter(|c| !*c).count() > Bitmask::max_bits() {
        debug!("Board too large for bitmask representation.");
        return Err(UnsolvableReason::BoardTooLarge);
    }

    let result = backtracking::solve_all_filling(board, tiles, cancel_token).await;
    match &result {
        Ok(solution) => {
            for placement in solution.placements() {
                debug!("Placement at position {:?}", placement.position());
                array_util::debug_print(&placement.rotation());
            }
        }
        Err(_) => {}
    };
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;
    use tokio_util::sync::CancellationToken;

    #[tokio::test]
    async fn test_solve_all_filling_success() {
        let mut board = Board::new((3, 4));
        board[[0, 0]] = true;
        let tiles = vec![
            Tile::new(arr2(&[[true, true, true], [true, true, true]])),
            Tile::new(arr2(&[[true, true, true], [true, true, false]])),
        ];

        let result = solve_all_filling(board, &tiles, CancellationToken::new()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_solve_all_filling_failure() {
        let board = Board::new((3, 4));
        let tiles = vec![
            Tile::new(arr2(&[[true, true, true], [false, true, true]])),
            Tile::new(arr2(&[[true, true, true], [true, true, false]])),
        ];

        let result = solve_all_filling(board, &tiles, CancellationToken::new()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_solve_all_filling_no_tiles() {
        let board = Board::new((3, 4));
        let tiles = vec![];

        let result = solve_all_filling(board, &tiles, CancellationToken::new()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_solve_all_filling_one_tile() {
        let board = Board::new((3, 4));
        let tiles = vec![Tile::new(arr2(&[[true, true, true], [true, true, true]]))];

        let result = solve_all_filling(board, &tiles, CancellationToken::new()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_solve_all_filling_too_many_tiles() {
        let board = Board::new((3, 4));
        let tiles = vec![
            Tile::new(arr2(&[[true, true, true], [true, true, true]])),
            Tile::new(arr2(&[[true, true, false], [true, false, true]])),
            Tile::new(arr2(&[[true, false, true], [true, true, true]])),
        ];

        let result = solve_all_filling(board, &tiles, CancellationToken::new()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_solve_all_filling_failure_with_enough_places_filled() {
        let mut board = Board::new((3, 4));
        board[[0, 0]] = true;
        let tiles = vec![
            Tile::new(arr2(&[[true, false, true], [true, true, true]])),
            Tile::new(arr2(&[[true, true, true], [true, true, true]])),
        ];

        let result = solve_all_filling(board, &tiles, CancellationToken::new()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_solve_all_filling_solved_without_tiles() {
        let mut board = Board::new((3, 3));
        for i in 0..3 {
            for j in 0..3 {
                board[[i, j]] = true;
            }
        }
        let tiles = vec![];

        let result = solve_all_filling(board, &tiles, CancellationToken::new()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_solve_1() {
        let board = arr2(&[
            [true, true, false, false, true],
            [true, true, false, false, true],
            [true, true, true, false, false],
            [true, false, false, true, true],
            [false, false, false, true, true],
        ])
        .into();
        let tiles = vec![
            Tile::new(arr2(&[[false, true, true], [true, true, true]])),
            Tile::new(arr2(&[
                [true, true, false],
                [true, true, false],
                [false, true, true],
            ])),
        ];

        let result = solve_all_filling(board, &tiles, CancellationToken::new()).await;
        assert!(result.is_ok());
    }
}
