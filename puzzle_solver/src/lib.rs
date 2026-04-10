use crate::bitmask::Bitmask;
use crate::board::Board;
use crate::plausibility::check;
use crate::result::{Solution, TilePlacement, UnsolvableReason};
use crate::tile::Tile;
use log::debug;
use tokio_util::sync::CancellationToken;

mod backtracking;
mod bitmask;
pub mod board;
mod plausibility;
pub mod result;
pub mod tile;

/// Tries to place all given tiles on the board, filling it completely.
/// If successful, returns a Solution; otherwise, returns an UnsolvableReason.
/// A successful result is reached, if all tiles were placed on the board without overlapping
/// and all empty cells on the board are covered.
///
/// The cancellation token can be used to cancel the operation.
/// The operation may be canceled at any time, in which case it will return
/// after some time. It may still be successful if it was close to finishing.
/// It may also return an error if it was canceled before it could find a solution.
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
    if !check(&board, tiles) {
        debug!("Plausibility check failed.");
        return Err(UnsolvableReason::PlausibilityCheckFailed);
    }

    let mut board = board;
    let trim_sides = board.trim();

    if board.get_array().iter().filter(|c| !*c).count() > Bitmask::max_bits() {
        debug!("Board too large for bitmask representation.");
        return Err(UnsolvableReason::BoardTooLarge);
    }

    let result = backtracking::solve_all_filling(board, tiles, cancel_token).await;
    match &result {
        Ok(solution) => {
            let trim_adjusted_placements: Vec<TilePlacement> = solution
                .placements()
                .iter()
                .map(|placement| {
                    let (x, y) = placement.position();
                    let (trimmed_x, trimmed_y) = (x + trim_sides.lower_x, y + trim_sides.lower_y);
                    TilePlacement::new(
                        placement.base().clone(),
                        placement.rotation().clone(),
                        (trimmed_x, trimmed_y),
                    )
                })
                .collect();
            Ok(Solution::new(trim_adjusted_placements))
        }
        Err(_) => result,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use puzzled_common::shape::shape_square;
    use tokio_util::sync::CancellationToken;

    #[tokio::test]
    async fn test_solve_all_filling_success() {
        let mut board = Board::new((3, 4));
        board[[0, 0]] = true;
        let tiles = vec![
            Tile::new(shape_square(&[[true, true, true], [true, true, false]])),
            Tile::new(shape_square(&[[true, true, true], [true, true, true]])),
        ];

        let result = solve_all_filling(board, &tiles, CancellationToken::new()).await;
        assert!(result.is_ok());
        let solution = result.unwrap();
        let placements = solution.placements();
        assert_eq!(placements.len(), 2);
        let expected_placement_1 = TilePlacement::new(
            shape_square(&[[true, true, true], [true, true, false]]),
            shape_square(&[[false, true], [true, true], [true, true]]),
            (0, 0),
        );
        assert!(placements.contains(&expected_placement_1));
        let expected_placement_2 = TilePlacement::new(
            shape_square(&[[true, true, true], [true, true, true]]),
            shape_square(&[[true, true], [true, true], [true, true]]),
            (0, 2),
        );
        assert!(placements.contains(&expected_placement_2));
    }

    #[tokio::test]
    async fn test_solve_all_filling_success_board_padding() {
        let board = shape_square(&[
            [true, true, true, true, true, true, true],
            [true, true, true, true, true, true, true],
            [true, true, true, true, true, true, true],
            [true, true, false, false, false, true, true],
            [true, false, false, false, false, true, true],
            [true, false, false, false, false, true, true],
            [true, true, true, true, true, true, true],
            [true, true, true, true, true, true, true],
            [true, true, true, true, true, true, true],
            [true, true, true, true, true, true, true],
        ])
        .into();
        let tiles = vec![
            Tile::new(shape_square(&[[true, true, true], [true, true, false]])),
            Tile::new(shape_square(&[[true, true, true], [true, true, true]])),
        ];

        let result = solve_all_filling(board, &tiles, CancellationToken::new()).await;
        assert!(result.is_ok());
        let solution = result.unwrap();
        let placements = solution.placements();
        dbg!(&placements);
        assert_eq!(placements.len(), 2);
        let expected_placement_1 = TilePlacement::new(
            shape_square(&[[true, true, true], [true, true, false]]),
            shape_square(&[[false, true], [true, true], [true, true]]),
            (3, 1),
        );
        assert!(placements.contains(&expected_placement_1));
        let expected_placement_2 = TilePlacement::new(
            shape_square(&[[true, true, true], [true, true, true]]),
            shape_square(&[[true, true], [true, true], [true, true]]),
            (3, 3),
        );
        assert!(placements.contains(&expected_placement_2));
    }

    #[tokio::test]
    async fn test_solve_all_filling_success_one_tile() {
        let mut board = Board::new((3, 2));
        board[[1, 0]] = true;
        let tiles = vec![Tile::new(shape_square(&[
            [true, true, true],
            [true, false, true],
        ]))];

        let result = solve_all_filling(board, &tiles, CancellationToken::new()).await;
        assert!(result.is_ok());
        let solution = result.unwrap();
        let placements = solution.placements();
        assert_eq!(placements.len(), 1);
        let expected_placement_1 = TilePlacement::new(
            shape_square(&[[true, true, true], [true, false, true]]),
            shape_square(&[[true, true], [false, true], [true, true]]),
            (0, 0),
        );
        assert!(placements.contains(&expected_placement_1));
    }

    #[tokio::test]
    async fn test_solve_all_filling_failure() {
        let board = Board::new((3, 4));
        let tiles = vec![
            Tile::new(shape_square(&[[true, true, true], [false, true, true]])),
            Tile::new(shape_square(&[[true, true, true], [true, true, false]])),
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
    async fn test_solve_all_filling_too_few_tiles() {
        let board = Board::new((3, 4));
        let tiles = vec![Tile::new(shape_square(&[
            [true, true, true],
            [true, true, true],
        ]))];

        let result = solve_all_filling(board, &tiles, CancellationToken::new()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_solve_all_filling_too_many_tiles() {
        let board = Board::new((3, 4));
        let tiles = vec![
            Tile::new(shape_square(&[[true, true, true], [true, true, true]])),
            Tile::new(shape_square(&[[true, true, false], [true, false, true]])),
            Tile::new(shape_square(&[[true, false, true], [true, true, true]])),
        ];

        let result = solve_all_filling(board, &tiles, CancellationToken::new()).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_solve_all_filling_failure_with_enough_places_filled() {
        let mut board = Board::new((3, 4));
        board[[0, 0]] = true;
        let tiles = vec![
            Tile::new(shape_square(&[[true, false, true], [true, true, true]])),
            Tile::new(shape_square(&[[true, true, true], [true, true, true]])),
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
        let solution = result.unwrap();
        assert!(solution.placements().is_empty());
    }

    #[tokio::test]
    async fn test_solve_1() {
        let board = shape_square(&[
            [true, true, false, false, true],
            [true, true, false, false, true],
            [true, true, true, false, false],
            [true, false, false, true, true],
            [false, false, false, true, true],
        ])
        .into();
        let tiles = vec![
            Tile::new(shape_square(&[[false, true, true], [true, true, true]])),
            Tile::new(shape_square(&[
                [true, true, false],
                [true, true, false],
                [false, true, true],
            ])),
        ];

        let result = solve_all_filling(board, &tiles, CancellationToken::new()).await;
        assert!(result.is_ok());
        let solution = result.unwrap();
        let placements = solution.placements();
        assert_eq!(placements.len(), 2);
        let expected_placement_1 = TilePlacement::new(
            shape_square(&[[false, true, true], [true, true, true]]),
            shape_square(&[[false, true, true], [true, true, true]]),
            (3, 0),
        );
        assert!(placements.contains(&expected_placement_1));
        let expected_placement_2 = TilePlacement::new(
            shape_square(&[
                [true, true, false],
                [true, true, false],
                [false, true, true],
            ]),
            shape_square(&[
                [true, true, false],
                [true, true, false],
                [false, true, true],
            ]),
            (0, 2),
        );
        assert!(placements.contains(&expected_placement_2));
    }

    #[tokio::test]
    async fn test_solve_tile_can_not_be_placed() {
        let board = shape_square(&[[false, false], [false, false]]).into();
        let tiles = vec![Tile::new(shape_square(&[[true, true, true, true]]))];

        let result = solve_all_filling(board, &tiles, CancellationToken::new()).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(
            error,
            UnsolvableReason::TileCannotBePlaced {
                base: shape_square(&[[true, true, true, true]]),
            }
        );
    }

    #[tokio::test]
    async fn test_solve_all_filling_same_tiles() {
        let board = shape_square(&[
            [true, false, false],
            [false, true, false],
            [false, false, true],
        ])
        .into();
        let tiles = vec![
            Tile::new(shape_square(&[[false, true], [true, true]])),
            Tile::new(shape_square(&[[false, true], [true, true]])),
        ];

        let result = solve_all_filling(board, &tiles, CancellationToken::new()).await;
        assert!(result.is_ok());
        let solution = result.unwrap();
        let placements = solution.placements();
        dbg!(&placements);
        assert_eq!(placements.len(), 2);
        let expected_placement_1 = TilePlacement::new(
            shape_square(&[[false, true], [true, true]]),
            shape_square(&[[true, false], [true, true]]),
            (1, 0),
        );
        assert!(placements.contains(&expected_placement_1));
        let expected_placement_2 = TilePlacement::new(
            shape_square(&[[false, true], [true, true]]),
            shape_square(&[[true, true], [false, true]]),
            (0, 1),
        );
        assert!(placements.contains(&expected_placement_2));
    }

    #[tokio::test]
    async fn test_solve_all_filling_too_large_board() {
        // Increase board size if test fails after increasing the max bits in Bitmask
        let board = Board::new((100, 10));
        let mut tiles = Vec::with_capacity(100);
        for _ in 0..100 {
            tiles.push(Tile::new(shape_square(&[
                [true, true, true, true, true],
                [true, true, true, true, true],
            ])));
        }

        let result: Result<Solution, UnsolvableReason> = tokio::select! {
            result = solve_all_filling(board, &tiles, CancellationToken::new()) => result,
            _ = tokio::time::sleep(std::time::Duration::from_secs(10)) => {
                panic!("Test timed out, this might be because the bitmask size exceeded the board size in this test. Increase the board size in this test and try again.")
            }
        };
        assert!(result.is_err());
        assert_eq!(
            result.expect_err("Expected Error"),
            UnsolvableReason::BoardTooLarge
        );
    }
}
