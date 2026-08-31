use crate::board::Board;
use crate::plausibility::check;
use crate::result::{Solution, TilePlacement, UnsolvableReason};
use crate::tile::Tile;
use cancellation_token::CancellationToken;
use log::debug;

pub mod board;
pub mod cancellation_token;
mod dlx;
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
/// use puzzle_solver::board::Board;
/// use puzzle_solver::tile::Tile;
/// use puzzle_solver::cancellation_token::CancellationToken;
/// use puzzled_common::polyform::Polyform::polyomino_from_bool_slice;
///
///    let board = Board::new(Polyform::polyomino_from_bool_slice(&[
///             [true, false, false],
///             [false, false, false],
///             [false, false, false],
///             [false, false, false]
///         ]));
/// let tiles = vec![
///     Tile::new(42, Polyform::polyomino_from_bool_slice(&[[true, true, true], [true, true, true]])),
///     Tile::new(43, Polyform::polyomino_from_bool_slice(&[[true, true, true], [true, true, false]])),
/// ];
/// let cancel_token = CancellationToken::new();
///
/// let result = solve_all_filling(board, &tiles, cancel_token);
/// assert!(result.is_ok());
/// ```
pub fn solve_all_filling(
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

    let result = dlx::solve_all_filling(&board, tiles, cancel_token);
    match &result {
        Ok(solution) => {
            let trim_adjusted_placements: Vec<TilePlacement> = solution
                .placements()
                .iter()
                .map(|placement| {
                    let position = placement.position() + &trim_sides.lower;
                    TilePlacement::new(
                        placement.tile_id(),
                        placement.base().clone(),
                        placement.rotation().clone(),
                        position,
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
    use puzzled_common::polyform::Polyform;
    use puzzled_common::polyform::grid::{Coord, RegularCoord};

    #[test]
    fn test_solve_all_filling_success() {
        let board = Board::new(Polyform::polyomino_from_bool_slice(&[
            [true, false, false],
            [false, false, false],
            [false, false, false],
            [false, false, false]
        ]));
        let tiles = vec![
            Tile::new(42, Polyform::polyomino_from_bool_slice(&[[true, true, true], [true, true, false]])),
            Tile::new(43, Polyform::polyomino_from_bool_slice(&[[true, true, true], [true, true, true]])),
        ];

        let result = solve_all_filling(board, &tiles, CancellationToken::new());
        assert!(result.is_ok());
        let solution = result.unwrap();
        let placements = solution.placements();
        assert_eq!(placements.len(), 2);
        let expected_placement_1 = TilePlacement::new(
            42,
            Polyform::polyomino_from_bool_slice(&[[true, true, true], [true, true, false]]),
            Polyform::polyomino_from_bool_slice(&[[false, true], [true, true], [true, true]]),
            Coord::Regular(RegularCoord::new(0, 0)),
        );
        assert!(placements.contains(&expected_placement_1));
        let expected_placement_2 = TilePlacement::new(
            43,
            Polyform::polyomino_from_bool_slice(&[[true, true, true], [true, true, true]]),
            Polyform::polyomino_from_bool_slice(&[[true, true], [true, true], [true, true]]),
            Coord::Regular(RegularCoord::new(0, 2)),
        );
        assert!(placements.contains(&expected_placement_2));
    }

    #[test]
    fn test_solve_all_filling_success_board_padding() {
        let board = Board::new(Polyform::polyomino_from_bool_slice(&[
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
        ]));
        let tiles = vec![
            Tile::new(42, Polyform::polyomino_from_bool_slice(&[[true, true, true], [true, true, false]])),
            Tile::new(43, Polyform::polyomino_from_bool_slice(&[[true, true, true], [true, true, true]])),
        ];

        let result = solve_all_filling(board, &tiles, CancellationToken::new());
        assert!(result.is_ok());
        let solution = result.unwrap();
        let placements = solution.placements();
        dbg!(&placements);
        assert_eq!(placements.len(), 2);
        let expected_placement_1 = TilePlacement::new(
            42,
            Polyform::polyomino_from_bool_slice(&[[true, true, true], [true, true, false]]),
            Polyform::polyomino_from_bool_slice(&[[false, true], [true, true], [true, true]]),
            Coord::Regular(RegularCoord::new(3, 1)),
        );
        assert!(placements.contains(&expected_placement_1));
        let expected_placement_2 = TilePlacement::new(
            43,
            Polyform::polyomino_from_bool_slice(&[[true, true, true], [true, true, true]]),
            Polyform::polyomino_from_bool_slice(&[[true, true], [true, true], [true, true]]),
            Coord::Regular(RegularCoord::new(3, 3)),
        );
        assert!(placements.contains(&expected_placement_2));
    }

    #[test]
    fn test_solve_all_filling_success_one_tile() {
        let board = Board::new(Polyform::polyomino_from_bool_slice(&[
            [true, false, false],
            [false, false, false],
        ]));
        let tiles = vec![Tile::new(
            42,
            Polyform::polyomino_from_bool_slice(&[[true, true, true], [true, false, true]]),
        )];

        let result = solve_all_filling(board, &tiles, CancellationToken::new());
        assert!(result.is_ok());
        let solution = result.unwrap();
        let placements = solution.placements();
        assert_eq!(placements.len(), 1);
        let expected_placement_1 = TilePlacement::new(
            42,
            Polyform::polyomino_from_bool_slice(&[[true, true, true], [true, false, true]]),
            Polyform::polyomino_from_bool_slice(&[[true, true], [false, true], [true, true]]),
            Coord::Regular(RegularCoord::new(0, 0)),
        );
        assert!(placements.contains(&expected_placement_1));
    }

    #[test]
    fn test_solve_all_filling_failure() {
        let board = Board::new(Polyform::polyomino_from_bool_slice(&[
            [false, false, false],
            [false, false, false],
            [false, false, false],
            [false, false, false],
        ]));
        let tiles = vec![
            Tile::new(42, Polyform::polyomino_from_bool_slice(&[[true, true, true], [false, true, true]])),
            Tile::new(43, Polyform::polyomino_from_bool_slice(&[[true, true, true], [true, true, false]])),
        ];

        let result = solve_all_filling(board, &tiles, CancellationToken::new());
        assert!(result.is_err());
    }

    #[test]
    fn test_solve_all_filling_no_tiles() {
        let board = Board::new(Polyform::polyomino_from_bool_slice(&[
            [false, false, false],
            [false, false, false],
            [false, false, false],
            [false, false, false],
        ]));
        let tiles = vec![];

        let result = solve_all_filling(board, &tiles, CancellationToken::new());
        assert!(result.is_err());
    }

    #[test]
    fn test_solve_all_filling_too_few_tiles() {
        let board = Board::new(Polyform::polyomino_from_bool_slice(&[
            [false, false, false],
            [false, false, false],
            [false, false, false],
            [false, false, false],
        ]));
        let tiles = vec![Tile::new(
            42,
            Polyform::polyomino_from_bool_slice(&[[true, true, true], [true, true, true]]),
        )];

        let result = solve_all_filling(board, &tiles, CancellationToken::new());
        assert!(result.is_err());
    }

    #[test]
    fn test_solve_all_filling_too_many_tiles() {
        let board = Board::new(Polyform::polyomino_from_bool_slice(&[
            [false, false, false],
            [false, false, false],
            [false, false, false],
            [false, false, false],
        ]));
        let tiles = vec![
            Tile::new(42, Polyform::polyomino_from_bool_slice(&[[true, true, true], [true, true, true]])),
            Tile::new(
                43,
                Polyform::polyomino_from_bool_slice(&[[true, true, false], [true, false, true]]),
            ),
            Tile::new(44, Polyform::polyomino_from_bool_slice(&[[true, false, true], [true, true, true]])),
        ];

        let result = solve_all_filling(board, &tiles, CancellationToken::new());
        assert!(result.is_err());
    }

    #[test]
    fn test_solve_all_filling_failure_with_enough_places_filled() {
        let board = Board::new(Polyform::polyomino_from_bool_slice(&[
            [true, false, false],
            [false, false, false],
            [false, false, false],
            [false, false, false],
        ]));
        let tiles = vec![
            Tile::new(42, Polyform::polyomino_from_bool_slice(&[[true, false, true], [true, true, true]])),
            Tile::new(43, Polyform::polyomino_from_bool_slice(&[[true, true, true], [true, true, true]])),
        ];

        let result = solve_all_filling(board, &tiles, CancellationToken::new());
        assert!(result.is_err());
    }

    #[test]
    fn test_solve_all_filling_solved_without_tiles() {
        let board = Board::new(Polyform::polyomino_from_bool_slice(&[
            [true, true, true],
            [true, true, true],
            [true, true, true],
        ]));
        let tiles = vec![];

        let result = solve_all_filling(board, &tiles, CancellationToken::new());
        assert!(result.is_ok());
        let solution = result.unwrap();
        assert!(solution.placements().is_empty());
    }

    #[test]
    fn test_solve_1() {
        let board = Board::new(Polyform::polyomino_from_bool_slice(&[
            [true, true, false, false, true],
            [true, true, false, false, true],
            [true, true, true, false, false],
            [true, false, false, true, true],
            [false, false, false, true, true],
        ]));
        let tiles = vec![
            Tile::new(42, Polyform::polyomino_from_bool_slice(&[[false, true, true], [true, true, true]])),
            Tile::new(
                43,
                Polyform::polyomino_from_bool_slice(&[
                    [true, true, false],
                    [true, true, false],
                    [false, true, true],
                ]),
            ),
        ];

        let result = solve_all_filling(board, &tiles, CancellationToken::new());
        assert!(result.is_ok());
        let solution = result.unwrap();
        let placements = solution.placements();
        assert_eq!(placements.len(), 2);
        let expected_placement_1 = TilePlacement::new(
            42,
            Polyform::polyomino_from_bool_slice(&[[false, true, true], [true, true, true]]),
            Polyform::polyomino_from_bool_slice(&[[false, true, true], [true, true, true]]),
            Coord::Regular(RegularCoord::new(3, 0)),
        );
        assert!(placements.contains(&expected_placement_1));
        let expected_placement_2 = TilePlacement::new(
            43,
            Polyform::polyomino_from_bool_slice(&[
                [true, true, false],
                [true, true, false],
                [false, true, true],
            ]),
            Polyform::polyomino_from_bool_slice(&[
                [true, true, false],
                [true, true, false],
                [false, true, true],
            ]),
            Coord::Regular(RegularCoord::new(0, 2)),
        );
        assert!(placements.contains(&expected_placement_2));
    }

    #[test]
    fn test_solve_tile_can_not_be_placed() {
        let board = Polyform::polyomino_from_bool_slice(&[[false, false], [false, false]]).into();
        let tiles = vec![Tile::new(42, Polyform::polyomino_from_bool_slice(&[[true, true, true, true]]))];

        let result = solve_all_filling(board, &tiles, CancellationToken::new());
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error, UnsolvableReason::NoFit);
    }

    #[test]
    fn test_solve_all_filling_same_tiles() {
        let board = Polyform::polyomino_from_bool_slice(&[
            [true, false, false],
            [false, true, false],
            [false, false, true],
        ])
            .into();
        let tiles = vec![
            Tile::new(42, Polyform::polyomino_from_bool_slice(&[[false, true], [true, true]])),
            Tile::new(43, Polyform::polyomino_from_bool_slice(&[[false, true], [true, true]])),
        ];

        let result = solve_all_filling(board, &tiles, CancellationToken::new());
        assert!(result.is_ok());
        let solution = result.unwrap();
        let placements = solution.placements();
        dbg!(&placements);
        assert_eq!(placements.len(), 2);
        let expected_placement_1_1 = TilePlacement::new(
            42,
            Polyform::polyomino_from_bool_slice(&[[false, true], [true, true]]),
            Polyform::polyomino_from_bool_slice(&[[true, false], [true, true]]),
            Coord::Regular(RegularCoord::new(1, 0)),
        );
        let expected_placement_1_2 = TilePlacement::new(
            42,
            Polyform::polyomino_from_bool_slice(&[[false, true], [true, true]]),
            Polyform::polyomino_from_bool_slice(&[[true, true], [false, true]]),
            Coord::Regular(RegularCoord::new(0, 1)),
        );
        assert!(
            placements.contains(&expected_placement_1_1)
                || placements.contains(&expected_placement_1_2)
        );
        let expected_placement_2_1 = TilePlacement::new(
            43,
            Polyform::polyomino_from_bool_slice(&[[false, true], [true, true]]),
            Polyform::polyomino_from_bool_slice(&[[true, true], [false, true]]),
            Coord::Regular(RegularCoord::new(0, 1)),
        );
        let expected_placement_2_2 = TilePlacement::new(
            43,
            Polyform::polyomino_from_bool_slice(&[[false, true], [true, true]]),
            Polyform::polyomino_from_bool_slice(&[[true, false], [true, true]]),
            Coord::Regular(RegularCoord::new(1, 0)),
        );
        assert!(
            placements.contains(&expected_placement_2_1)
                || placements.contains(&expected_placement_2_2)
        );
    }
}
