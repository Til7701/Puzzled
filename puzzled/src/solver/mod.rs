pub mod combination_solutions;

use crate::app::puzzle::puzzle_area::puzzle_state::{Cell, PuzzleState};
use crate::global::runtime::get_runtime;
use log::debug;
use puzzle_solver::board::Board;
use puzzle_solver::result::{Solution, UnsolvableReason};
use puzzle_solver::tile::Tile;
use std::cmp::PartialEq;
use std::ops::Deref;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, LazyLock, RwLock};
use std::time::Instant;
use tokio_util::sync::CancellationToken;

/// Represents the current state of the puzzle solver.
#[derive(Debug, Default, Clone)]
enum SolverState {
    /// Solver did not run yet. This is the state at application start.
    #[default]
    Idle,
    /// Solver is currently running.
    /// It can be canceled using the provided cancellation token.
    Running {
        call_id: SolverCallId,
        cancel_token: CancellationToken,
    },
}

/// Unique identifier for a solver call.
/// It can be used to track and manage individual solver tasks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd)]
struct SolverCallId(u64);

/// Callback type to be invoked upon solver completion.
/// It receives the final `SolverState` as an argument.
pub type OnCompleteCallback = Box<dyn Fn(Result<Solution, UnsolvableReason>) + Send>;

static SOLVER_CALL_ID_ATOMIC_COUNTER: AtomicU64 = AtomicU64::new(0);

static SOLVER: LazyLock<Solver> = LazyLock::new(|| Solver {
    state: Arc::new(RwLock::new(SolverState::default())),
});

#[derive(Debug, Clone)]
pub struct Solver {
    state: Arc<RwLock<SolverState>>,
}

impl Default for Solver {
    fn default() -> Self {
        SOLVER.clone()
    }
}

impl Solver {
    /// Creates a new unique `SolverCallId` to be used to identify a solver call.
    fn create_solver_call_id(&self) -> SolverCallId {
        SolverCallId(SOLVER_CALL_ID_ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst))
    }

    pub fn solve_for_target(
        &self,
        puzzle_state: &PuzzleState,
        on_complete: OnCompleteCallback,
        cancel_token: CancellationToken,
    ) {
        self.solver_for_target_maybe_callback(puzzle_state, on_complete, false, cancel_token);
    }

    pub fn solver_for_target_maybe_callback(
        &self,
        puzzle_state: &PuzzleState,
        on_complete: OnCompleteCallback,
        always_run_callback: bool,
        cancel_token: CancellationToken,
    ) {
        self.interrupt_solver_call();
        let solver_call_id = self.create_solver_call_id();
        let mut state = self.state.write().unwrap();
        *state = SolverState::Running {
            call_id: solver_call_id.clone(),
            cancel_token: cancel_token.clone(),
        };

        let board = self.create_board(puzzle_state);
        let tiles: Vec<Tile> = puzzle_state
            .unused_tiles
            .iter()
            .map(|tile_state| Tile::new(tile_state.base.clone()))
            .collect();

        let runtime = get_runtime();
        let now = Instant::now();
        runtime.spawn({
            let self_clone = self.clone();
            let solver_call_id = solver_call_id.clone();
            let cancel_token = cancel_token.clone();
            async move {
                debug!("Starting Solver task.");
                let result = puzzle_solver::solve_all_filling(board, &tiles, cancel_token).await;
                let end = Instant::now();
                let duration = end.duration_since(now);
                debug!(
                    "Solver task completed in {}.",
                    humantime::format_duration(duration)
                );
                if always_run_callback {
                    on_complete(result);
                } else {
                    self_clone.handle_on_complete(solver_call_id, result, on_complete);
                }
            }
        });
    }

    fn handle_on_complete(
        &self,
        solver_call_id: SolverCallId,
        result: Result<Solution, UnsolvableReason>,
        on_complete: OnCompleteCallback,
    ) {
        let state = self.state.read().unwrap();
        if let SolverState::Running { call_id, .. } = state.deref()
            && *call_id == solver_call_id
        {
            on_complete(result);
        }
    }

    /// Interrupts an ongoing solver stored in the given `state`.
    ///
    /// # Arguments
    ///
    /// * `state`: A reference to the current application state containing the solver state.
    ///
    /// returns: ()
    pub fn interrupt_solver_call(&self) {
        if let SolverState::Running {
            call_id,
            cancel_token,
        } = self.state.read().unwrap().deref()
        {
            debug!("Interrupting solver call: {:?}", call_id);
            cancel_token.cancel();
            debug!("Solver call {:?} aborted.", call_id);
        }
    }

    /// Checks if the given puzzle state is already solved for the specified target.
    /// This can be used to skip unnecessary solver calls.
    ///
    /// # Arguments
    ///
    /// * `puzzle_state`: A reference to the current puzzle state.
    /// * `target`: A reference to the target configuration to check against.
    ///
    /// returns: bool
    pub fn is_solved(&self, puzzle_state: &PuzzleState) -> bool {
        let board = self.create_board(puzzle_state);
        board.get_array().iter().all(|cell| *cell)
    }

    /// Creates a board representation from the given puzzle state and target to give to the solver.
    ///
    /// # Arguments
    ///
    /// * `puzzle_state`: A reference to the current puzzle state.
    /// * `target`: A reference to the target configuration.
    ///
    /// returns: Board
    fn create_board(&self, puzzle_state: &PuzzleState) -> Board {
        let dims = puzzle_state.grid.dim();
        let mut board = Board::new(dims);

        puzzle_state.grid.indexed_iter().for_each(|((x, y), cell)| {
            let is_filled = match cell {
                Cell::Empty(cell_data) => !cell_data.is_on_board,
                Cell::One(_, _) => true,
                Cell::Many(_, _) => true,
            };

            board[[x, y]] = is_filled;
        });

        board
    }
}

#[cfg(test)]
mod tests {
    use crate::solver::Solver;

    #[test]
    fn test_create_solver_call_id() {
        let solver = Solver::default();
        let id1 = solver.create_solver_call_id();
        let id2 = solver.create_solver_call_id();
        assert_ne!(id1, id2);
        assert!(id1 < id2);
    }
}
