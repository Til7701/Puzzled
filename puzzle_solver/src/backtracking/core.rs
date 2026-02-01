use crate::backtracking::positioned::PositionedTile;
use crate::backtracking::pruner::Pruner;
use crate::bitmask::Bitmask;
use log::debug;
use std::sync::Arc;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

pub async fn solve_filling(
    board_width: i32,
    board_bitmask: &Bitmask,
    positioned_tiles: &[PositionedTile],
    pruner: Pruner,
    cancel_token: CancellationToken,
) -> Option<Vec<usize>> {
    if board_bitmask.all_relevant_bits_set() {
        return Some(Vec::new());
    }

    let solvers: Vec<AllFillingSolver> = prepare_solvers(board_bitmask, positioned_tiles, &pruner);
    let shared = Arc::new(AllFillingShared {
        board_width,
        positioned_tiles: positioned_tiles.to_vec(),
        pruner,
        cancel_token: cancel_token.clone(),
    });
    let mut set: JoinSet<Option<Vec<usize>>> = JoinSet::new();

    let result: Option<Vec<usize>> = {
        for mut solver in solvers.into_iter() {
            set.spawn({
                let shared = shared.clone();
                async move { solver.solve(&shared).await }
            });
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

async fn await_completion(set: &mut JoinSet<Option<Vec<usize>>>) -> Option<Vec<usize>> {
    let mut result: Option<Vec<usize>> = None;
    while let Some(res) = set.join_next().await {
        match res {
            Ok(r) => {
                if r.is_some() {
                    result = r;
                    break;
                }
            }
            Err(_) => {}
        }
    }
    result
}

fn prepare_solvers(
    board_bitmask: &Bitmask,
    positioned_tiles: &[PositionedTile],
    pruner: &Pruner,
) -> Vec<AllFillingSolver> {
    if positioned_tiles.is_empty() {
        return Vec::new();
    }
    let first_tile = positioned_tiles.first().unwrap();
    let mut solvers = Vec::with_capacity(first_tile.bitmasks().len());

    for i in 0..first_tile.bitmasks().len() {
        let placement = &first_tile.bitmasks()[i];
        if board_bitmask.and_is_zero(&placement) {
            let mut board_with_placements = board_bitmask.clone();
            board_with_placements.xor(board_bitmask, placement);

            if pruner.prune(&board_with_placements) {
                continue;
            }

            let mut used_tile_indices: Vec<usize> = vec![0; 1];
            used_tile_indices[0] = i;

            let solver = AllFillingSolver::new(
                &board_with_placements,
                &used_tile_indices,
                positioned_tiles.len(),
            );

            solvers.push(solver);
        }
    }

    solvers
}

/// Shared data for the AllFillingSolver.
struct AllFillingShared {
    board_width: i32,
    positioned_tiles: Vec<PositionedTile>,
    pruner: Pruner,
    cancel_token: CancellationToken,
}

/// Solver for filling the board with all tiles using recursive backtracking.
struct AllFillingSolver {
    start_tile_index: usize,
    board_bitmasks: Vec<Bitmask>,
    used_tile_indices: Vec<usize>,
    tmp_bitmask: Bitmask,
    yield_counter: u32,
}

impl AllFillingSolver {
    fn new(board_bitmasks: &Bitmask, used_tile_indices: &[usize], num_tiles: usize) -> Self {
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
        }
    }

    /// The entry point for the AllFillingSolver to start solving the puzzle.
    ///
    /// This function will only return, if a solution is found, or it is proven that no solution
    /// exists.
    ///
    /// returns: bool: true if a solution is found, false otherwise.
    async fn solve(&mut self, shared: &AllFillingShared) -> Option<Vec<usize>> {
        let solved = self.solve_recursive(self.start_tile_index, shared).await;
        if solved {
            Some(self.used_tile_indices.clone())
        } else {
            None
        }
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
    async fn solve_recursive(&mut self, tile_index: usize, shared: &AllFillingShared) -> bool {
        self.yield_counter += 1;
        if self.yield_counter & 0xff == 0 {
            tokio::task::yield_now().await;
            if shared.cancel_token.is_cancelled() {
                return false;
            }
        }

        // All tiles placed
        if tile_index >= shared.positioned_tiles.len() {
            return self.submit_solution();
        }

        let num_placements = shared.positioned_tiles[tile_index].bitmasks().len();
        for i in 0..num_placements {
            let placement = &shared.positioned_tiles[tile_index].bitmasks()[i];
            if self.board_bitmasks[tile_index - 1].and_is_zero(&placement) {
                self.tmp_bitmask
                    .xor(&self.board_bitmasks[tile_index - 1], &placement);
                if shared.pruner.prune(&self.tmp_bitmask) {
                    continue;
                }
                self.used_tile_indices[tile_index] = i;
                self.board_bitmasks[tile_index] = self.tmp_bitmask.clone();
                if Box::pin(async { self.solve_recursive(tile_index + 1, &shared).await }).await {
                    return true;
                }
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
    fn print_debug(&self, shared: &AllFillingShared) {
        debug!("RecursiveSolver Debug Info:");
        debug!("Board Width: {}", shared.board_width);
        debug!("Start Tile Index: {}", self.start_tile_index);
        debug!("Used Tile Indices: {:?}", self.used_tile_indices);
        for (i, bitmask) in self.board_bitmasks.iter().enumerate() {
            debug!(
                "Board Bitmask after tile {}: {}",
                i,
                bitmask.to_string(shared.board_width)
            );
        }
    }
}
