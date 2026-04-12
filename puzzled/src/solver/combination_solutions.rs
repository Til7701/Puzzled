use crate::app::puzzle::puzzle_area::puzzle_state::{PuzzleState, UnusedTile};
use crate::global::runtime::get_runtime;
use crate::solver::Solver;
use log::{debug, info};
use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use tokio_util::sync::CancellationToken;

/// The CombinationsSolver can be used to find solutions finding any combination of tiles on the
/// board. You can start it and cancel it any time. If it is running when it is started,
/// the previous call is canceled. Any found solutions are logged.
#[derive(Debug, Clone)]
pub struct CombinationsSolver {
    solver: Solver,
    cancellation_token: Arc<RwLock<Option<CancellationToken>>>,
}

impl CombinationsSolver {
    /// Start solving with the given state and extension.
    pub fn calculate_tile_combinations_to_solve(&self, puzzle_state: PuzzleState) {
        self.stop_calculate_tile_combinations_to_solve();

        let cancellation_token = CancellationToken::new();
        self.cancellation_token
            .write()
            .unwrap()
            .replace(cancellation_token.clone());

        get_runtime().spawn({
            let self_clone = self.clone();
            async move {
                self_clone
                    .find_solutions(puzzle_state, cancellation_token)
                    .await;
            }
        });
    }

    async fn find_solutions(
        &self,
        puzzle_state: PuzzleState,
        cancellation_token: CancellationToken,
    ) {
        let tiles = puzzle_state.unused_tiles;
        let mut grid = puzzle_state.grid;
        let mut iter = TileCombinationsIter::new(&tiles);
        while let Some(tiles) = iter.next()
            && !cancellation_token.is_cancelled()
        {
            let new_puzzle_state = PuzzleState {
                grid,
                unused_tiles: tiles.clone(),
            };
            self.solver.solver_for_target_maybe_callback(
                &new_puzzle_state,
                Box::new({
                    move |result| {
                        debug!("Solver call completed");
                        if let Ok(solution) = result {
                            let mut message: String = "".to_string();
                            for (i, placement) in solution.placements().iter().enumerate() {
                                let tile =
                                    tiles.iter().find(|t| t.base == *placement.base()).unwrap();
                                message = format!(
                                    "{} {}",
                                    message,
                                    Self::create_list_entry_for_tile(tile)
                                );
                                if i < solution.placements().len() - 1 {
                                    message = format!("{},", message);
                                }
                            }
                            info!("Found solution: {}", message);
                        }
                    }
                }),
                true,
                cancellation_token.clone(),
            );
            grid = new_puzzle_state.grid;
        }
        info!(
            "Finished finding combinations of tiles to solve. Cancellation status: {}.",
            cancellation_token.is_cancelled()
        );
    }

    fn create_list_entry_for_tile(unused_tile: &UnusedTile) -> String {
        let name = &unused_tile.name;
        if let Some(name) = name {
            format!("\"{}\"", name)
        } else {
            unused_tile.base.to_string()
        }
    }

    /// Stop solving.
    pub fn stop_calculate_tile_combinations_to_solve(&self) {
        let token = self.cancellation_token.write().unwrap().take();
        if let Some(token) = token {
            token.cancel();
        }
    }
}

impl Default for CombinationsSolver {
    fn default() -> Self {
        CombinationsSolver {
            solver: Solver::default(),
            cancellation_token: Arc::new(RwLock::new(None)),
        }
    }
}

struct TileCombinationsIter<'a> {
    tiles: &'a HashSet<UnusedTile>,
    iteration: usize,
}

impl<'a> TileCombinationsIter<'a> {
    pub fn new(tiles: &'a HashSet<UnusedTile>) -> Self {
        Self {
            tiles,
            iteration: 1,
        }
    }
}

impl<'a> Iterator for TileCombinationsIter<'a> {
    type Item = HashSet<UnusedTile>;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.tiles.len();

        let max = 1usize.checked_shl(n as u32)?;
        if self.iteration >= max {
            debug!(
                "TileCombinationsIter: Reached end of combinations (iteration={}, max={}).",
                self.iteration, max
            );
            return None;
        }

        let mask = self.iteration;
        self.iteration += 1;

        let mut subset = HashSet::with_capacity(mask.count_ones() as usize);
        for (bit, tile) in self.tiles.iter().enumerate() {
            if (mask & (1usize << bit)) != 0 {
                subset.insert(tile.clone());
            }
        }

        Some(subset)
    }
}
