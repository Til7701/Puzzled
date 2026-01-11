use crate::puzzle::config::Target;
use crate::puzzle_state::{Cell, PuzzleState};
use crate::state::SolverState::Done;
use crate::state::{get_runtime, get_state, SolverState, State};
use log::debug;
use puzzle_solver::board::Board;
use puzzle_solver::tile::Tile;
use std::cmp::PartialEq;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SolverCallId(u64);

pub type OnCompleteCallback = Box<dyn FnOnce(SolverState) + Send>;

static SOLVER_CALL_ID_ATOMIC_COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn create_solver_call_id() -> SolverCallId {
    SolverCallId(SOLVER_CALL_ID_ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst))
}

pub fn solve_for_target(
    solver_call_id: &SolverCallId,
    puzzle_state: &PuzzleState,
    target: &Target,
    on_complete: OnCompleteCallback,
    cancel_token: CancellationToken,
) {
    let board = create_board(puzzle_state, target);
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
    let mut state = get_state();
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

pub fn is_solved(puzzle_state: &PuzzleState, target: &Target) -> bool {
    let board = create_board(puzzle_state, target);
    board.get_array().iter().all(|cell| *cell)
}

fn create_board(puzzle_state: &PuzzleState, target: &Target) -> Board {
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

    for index in target.indices.iter() {
        let x = index.0 + 1;
        let y = index.1 + 1;
        board[[x, y]] = true;
    }

    board
}
