use crate::puzzle::config::Target;
use crate::puzzle_state::{Cell, PuzzleState};
use crate::state::SolverState::Done;
use crate::state::{get_runtime, get_state, SolverState, State};
use log::debug;
use puzzle_solver::board::Board;
use puzzle_solver::tile::Tile;
use std::cmp::PartialEq;
use std::sync::atomic::{AtomicU64, Ordering};
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

    if !plausibility_check(&board, &tiles) {
        debug!("Plausibility check failed.");
        handle_on_complete(solver_call_id.clone(), false, on_complete);
        return;
    }

    let runtime = get_runtime();
    runtime.spawn({
        let solver_call_id = solver_call_id.clone();
        let cancel_token = cancel_token.clone();
        async move {
            debug!("Starting Solver task.");
            let result = puzzle_solver::solve_all_filling(board, &tiles, cancel_token).await;
            handle_on_complete(solver_call_id, result.is_ok(), on_complete);
        }
    });
}

fn plausibility_check(board: &Board, tiles: &[Tile]) -> bool {
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

fn handle_on_complete(
    solver_call_id: SolverCallId,
    solvable: bool,
    on_complete: OnCompleteCallback,
) {
    let mut state = get_state();
    if let SolverState::Running { call_id, .. } = &state.solver_state
        && *call_id == solver_call_id
    {
        state.solver_state = Done { solvable };
        drop(state);
        on_complete(Done { solvable });
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
