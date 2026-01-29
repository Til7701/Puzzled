use crate::array_util;
use crate::banned::BannedBitmask;
use crate::bitmask::Bitmask;
use crate::board::Board;
use crate::tile::Tile;
use log::debug;
use ndarray::Array2;
use std::sync::Arc;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

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
    pub(crate) fn new(tile: &Tile, board: &Board) -> Self {
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
            .collect();

        PositionedTile { bitmasks }
    }

    #[allow(dead_code)]
    fn print_debug(&self, board_width: i32) {
        for bitmask in self.bitmasks.iter() {
            debug!("{}", &bitmask.to_string(board_width));
        }
    }
}

pub async fn solve_filling(
    board_width: i32,
    board_bitmask: &Bitmask,
    positioned_tiles: &[PositionedTile],
    banned_bitmasks: Vec<BannedBitmask>,
    cancel_token: CancellationToken,
) -> Option<Vec<usize>> {
    if board_bitmask.all_relevant_bits_set() {
        return Some(Vec::new());
    }

    let solvers: Vec<AllFillingSolver> = prepare_solvers(
        board_width,
        board_bitmask,
        positioned_tiles,
        banned_bitmasks,
        &cancel_token,
    );
    let mut set: JoinSet<bool> = JoinSet::new();

    let result: Option<Vec<usize>> = {
        for mut solver in solvers.into_iter() {
            set.spawn(async move { solver.solve().await });
        }
        tokio::select! {
            _ = cancel_token.cancelled() => {
                debug!("Cancellation requested, aborting all solver tasks.");
                None
            }
            res = await_completion(&mut set) => {
                debug!("Solver Finished, aborting remaining solver tasks.");
                res
            }
        }
    };
    set.abort_all();
    result
}

async fn await_completion(set: &mut JoinSet<bool>) -> Option<Vec<usize>> {
    let mut result: Option<Vec<usize>> = None;
    while let Some(res) = set.join_next().await {
        match res {
            Ok(solved) => {
                if solved {
                    result = Some(Vec::new());
                    break;
                }
            }
            Err(_) => {}
        }
    }
    result
}

fn prepare_solvers(
    board_width: i32,
    board_bitmask: &Bitmask,
    positioned_tiles: &[PositionedTile],
    banned_bitmasks: Vec<BannedBitmask>,
    cancel_token: &CancellationToken,
) -> Vec<AllFillingSolver> {
    if positioned_tiles.is_empty() {
        return Vec::new();
    }
    let first_tile = positioned_tiles.first().unwrap();
    let mut solvers = Vec::with_capacity(first_tile.bitmasks.len());

    let shared = Arc::new(AllFillingShared {
        board_width,
        positioned_tiles: positioned_tiles.to_vec(),
        banned_bitmasks,
        cancel_token: cancel_token.clone(),
    });

    for i in 0..first_tile.bitmasks.len() {
        let placement = &first_tile.bitmasks[i];
        if board_bitmask.and_is_zero(&placement) {
            let mut board_with_placements = board_bitmask.clone();
            board_with_placements.xor(board_bitmask, placement);
            let mut used_tile_indices: Vec<usize> = vec![0; 1];
            used_tile_indices[0] = i;

            let solver =
                AllFillingSolver::new(&board_with_placements, &used_tile_indices, shared.clone());

            solvers.push(solver);
        }
    }

    solvers
}

/// Shared data for the AllFillingSolver.
struct AllFillingShared {
    board_width: i32,
    positioned_tiles: Vec<PositionedTile>,
    banned_bitmasks: Vec<BannedBitmask>,
    cancel_token: CancellationToken,
}

/// Solver for filling the board with all tiles using recursive backtracking.
struct AllFillingSolver {
    start_tile_index: usize,
    board_bitmasks: Vec<Bitmask>,
    used_tile_indices: Vec<usize>,
    tmp_bitmask: Bitmask,
    yield_counter: u32,
    shared: Arc<AllFillingShared>,
}

impl AllFillingSolver {
    fn new(
        board_bitmasks: &Bitmask,
        used_tile_indices: &[usize],
        shared: Arc<AllFillingShared>,
    ) -> Self {
        let num_tiles = shared.positioned_tiles.len();

        let mut use_tile_indices_vec: Vec<usize> = Vec::with_capacity(num_tiles);
        for used_tile_index in used_tile_indices {
            use_tile_indices_vec.push(*used_tile_index);
        }
        for _ in used_tile_indices.len()..num_tiles {
            use_tile_indices_vec.push(0);
        }
        AllFillingSolver {
            start_tile_index: used_tile_indices.len(),
            board_bitmasks: vec![board_bitmasks.clone(); num_tiles],
            used_tile_indices: use_tile_indices_vec,
            tmp_bitmask: Bitmask::new(board_bitmasks.relevant_bits()),
            yield_counter: 0,
            shared,
        }
    }

    /// The entry point for the AllFillingSolver to start solving the puzzle.
    ///
    /// This function will only return, if a solution is found, or it is proven that no solution
    /// exists.
    ///
    /// returns: bool: true if a solution is found, false otherwise.
    async fn solve(&mut self) -> bool {
        self.solve_recursive(self.start_tile_index).await
    }

    /// The main recursive solver function.
    ///
    /// This function attempts to place tiles on the board recursively.
    /// If a valid placement is found, it proceeds to the next tile with a recursive call.
    /// If a recursive call finds a solution, it returns true and propagates the success back up the
    /// call stack.
    /// If no valid placements are found for a tile, it backtracks and tries the next placement.
    /// If all placements are exhausted without finding a solution, it returns false.
    ///
    /// From time to time it yields to the tokio runtime to allow cancellation.
    ///
    /// # Arguments
    ///
    /// * `tile_index`:
    ///
    /// returns: bool
    async fn solve_recursive(&mut self, tile_index: usize) -> bool {
        self.yield_counter += 1;
        if self.yield_counter & 0xf == 0 {
            tokio::task::yield_now().await;
            if self.shared.cancel_token.is_cancelled() {
                return false;
            }
        }

        // All tiles placed
        if tile_index >= self.shared.positioned_tiles.len() {
            return self.submit_solution();
        }

        let num_placements = self.shared.positioned_tiles[tile_index].bitmasks.len();
        for i in 0..num_placements {
            let placement = &self.shared.positioned_tiles[tile_index].bitmasks[i];
            if self.board_bitmasks[tile_index - 1].and_is_zero(&placement) {
                self.tmp_bitmask
                    .xor(&self.board_bitmasks[tile_index - 1], &placement);
                if self.prune(&self.tmp_bitmask) {
                    continue;
                }
                self.used_tile_indices[tile_index] = i;
                self.board_bitmasks[tile_index] = self.tmp_bitmask.clone();
                if Box::pin(async { self.solve_recursive(tile_index + 1).await }).await {
                    return true;
                }
            }
        }

        false
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
    fn prune(&self, current_board: &Bitmask) -> bool {
        for banned in self.shared.banned_bitmasks.iter() {
            if banned.matches(current_board) {
                return true;
            }
        }
        false
    }

    /// Determines if the current board state represents a complete solution.
    ///
    /// If the current board is a correct solution, it returns true.
    /// Otherwise, it returns false.
    fn submit_solution(&self) -> bool {
        debug!("Submitting solution...");
        let board_filled = self.board_bitmasks.last().unwrap().all_relevant_bits_set();
        if board_filled {
            debug!(
                "Solution found with tile placements: {:?}",
                self.used_tile_indices
            );
        }
        board_filled
    }

    #[allow(dead_code)]
    fn print_debug(&self) {
        debug!("RecursiveSolver Debug Info:");
        debug!("Board Width: {}", self.shared.board_width);
        debug!("Start Tile Index: {}", self.start_tile_index);
        debug!("Used Tile Indices: {:?}", self.used_tile_indices);
        for (i, bitmask) in self.board_bitmasks.iter().enumerate() {
            debug!(
                "Board Bitmask after tile {}: {}",
                i,
                bitmask.to_string(self.shared.board_width)
            );
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

        let positioned_tile = PositionedTile::new(&tile, &board);
        assert_eq!(positioned_tile.bitmasks.len(), 22);

        assert!(positioned_tile.bitmasks.contains(&Bitmask::from(&arr2(&[
            [false, true, true, false],
            [false, true, true, true],
            [false, false, false, false]
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
            [false, false, false, false],
            [false, true, true, true],
            [false, true, true, false]
        ]))));

        assert!(positioned_tile.bitmasks.contains(&Bitmask::from(&arr2(&[
            [false, true, false, false],
            [true, true, false, false],
            [true, true, false, false]
        ]))));
        assert!(positioned_tile.bitmasks.contains(&Bitmask::from(&arr2(&[
            [false, false, true, false],
            [false, true, true, false],
            [false, true, true, false]
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
            [false, false, true, false],
            [false, false, true, true],
            [false, false, true, true]
        ]))));

        assert!(positioned_tile.bitmasks.contains(&Bitmask::from(&arr2(&[
            [false, true, true, false],
            [false, true, true, false],
            [false, false, true, false]
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
            [false, false, true, true],
            [false, false, true, true],
            [false, false, true, false]
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
            [true, true, true, false],
            [false, true, true, false]
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

        let positioned_tile = PositionedTile::new(&tile, &board);
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

        let positioned_tile = PositionedTile::new(&tile, &board);
        assert!(positioned_tile.bitmasks.is_empty());
    }

    #[test]
    fn test_positioned_tile_new_duplicates() {
        let board = Board::new((3, 3));
        let tile = Tile::new(arr2(&[[true, true], [true, true]]));

        let positioned_tile = PositionedTile::new(&tile, &board);
        assert_eq!(positioned_tile.bitmasks.len(), 4);
    }
}
