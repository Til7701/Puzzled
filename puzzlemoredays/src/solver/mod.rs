use crate::global::runtime::get_runtime;
use crate::global::state::SolverState::Done;
use crate::global::state::{get_state, get_state_mut, SolverState, State};
use crate::presenter::puzzle_area::puzzle_state::{Cell, PuzzleState};
use log::debug;
use puzzle_config::Target;
use puzzle_solver::board::Board;
use puzzle_solver::tile::Tile;
use std::cmp::PartialEq;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use tokio_util::sync::CancellationToken;

/// Unique identifier for a solver call.
/// It can be used to track and manage individual solver tasks.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd)]
pub struct SolverCallId(u64);

/// Callback type to be invoked upon solver completion.
/// It receives the final `SolverState` as an argument.
pub type OnCompleteCallback = Box<dyn FnOnce(SolverState) + Send>;

static SOLVER_CALL_ID_ATOMIC_COUNTER: AtomicU64 = AtomicU64::new(0);

/// Creates a new unique `SolverCallId` to be used to identify a solver call.
pub fn create_solver_call_id() -> SolverCallId {
    SolverCallId(SOLVER_CALL_ID_ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst))
}

pub fn solve_for_target(
    solver_call_id: &SolverCallId,
    puzzle_state: &PuzzleState,
    on_complete: OnCompleteCallback,
    cancel_token: CancellationToken,
) {
    let board = create_board(puzzle_state);
    let tiles: Vec<Tile> = puzzle_state
        .unused_tiles
        .iter()
        .map(|tile_state| Tile::new(tile_state.base.clone()))
        .collect();

    let runtime = get_runtime();
    let now = Instant::now();
    runtime.spawn({
        let solver_call_id = solver_call_id.clone();
        let cancel_token = cancel_token.clone();
        async move {
            debug!("Starting Solver task.");
            let result = puzzle_solver::solve_all_filling(board, &tiles, cancel_token).await;
            let end = Instant::now();
            let duration = end.duration_since(now);
            handle_on_complete(solver_call_id, result.is_ok(), duration, on_complete);
        }
    });
}

fn handle_on_complete(
    solver_call_id: SolverCallId,
    solvable: bool,
    run_duration: Duration,
    on_complete: OnCompleteCallback,
) {
    let mut state = get_state_mut();
    if let SolverState::Running { call_id, .. } = &state.solver_state
        && *call_id == solver_call_id
    {
        state.solver_state = Done {
            solvable,
            duration: run_duration,
        };
        drop(state);
        on_complete(Done {
            solvable,
            duration: run_duration,
        });
    }
}

/// Interrupts an ongoing solver stored in the given `state`.
///
/// # Arguments
///
/// * `state`: A reference to the current application state containing the solver state.
///
/// returns: ()
pub fn interrupt_solver_call(state: &State) {
    match &state.solver_state {
        SolverState::Running {
            call_id,
            cancel_token,
        } => {
            debug!("Interrupting solver call: {:?}", call_id);
            cancel_token.cancel();
            debug!("Solver call {:?} aborted.", call_id);
        }
        _ => return,
    };
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
pub fn is_solved(puzzle_state: &PuzzleState) -> bool {
    let board = create_board(puzzle_state);
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
fn create_board(puzzle_state: &PuzzleState) -> Board {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_solver_call_id() {
        let id1 = create_solver_call_id();
        let id2 = create_solver_call_id();
        assert_ne!(id1, id2);
        assert!(id1 < id2);
    }
}
