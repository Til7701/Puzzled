use crate::solver::SolverCallId;
use once_cell::sync::Lazy;
use puzzle_config::{PuzzleConfig, PuzzleConfigCollection, Target};
use std::backtrace::Backtrace;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError};
use std::time::Duration;
use tokio_util::sync::CancellationToken;

static APP_STATE: Lazy<RwLock<State>> = Lazy::new(|| RwLock::new(State::default()));

/// Represents the global application state.
#[derive(Debug)]
pub struct State {
    /// The currently selected puzzle collection.
    pub puzzle_collection: Option<PuzzleConfigCollection>,
    /// The puzzle configuration currently shown on the screen.
    pub puzzle_config: Option<PuzzleConfig>,
    pub puzzle_type_extension: Option<PuzzleTypeExtension>,
    /// The current state of the puzzle solver.
    pub solver_state: SolverState,
}

pub fn get_state() -> RwLockReadGuard<'static, State> {
    match APP_STATE.try_read() {
        Ok(guard) => guard,
        Err(TryLockError::WouldBlock) => {
            eprintln!(
                "get_state: rwlock busy (possible deadlock). PID={} Backtrace:\n{:?}",
                std::process::id(),
                Backtrace::force_capture()
            );
            std::thread::sleep(Duration::from_secs(2));
            APP_STATE.read().unwrap()
        }
        Err(TryLockError::Poisoned(_)) => APP_STATE.read().unwrap(),
    }
}

pub fn get_state_mut() -> RwLockWriteGuard<'static, State> {
    match APP_STATE.try_write() {
        Ok(guard) => guard,
        Err(TryLockError::WouldBlock) => {
            eprintln!(
                "get_state_mut: rwlock busy (possible deadlock). PID={} Backtrace:\n{:?}",
                std::process::id(),
                Backtrace::force_capture()
            );
            std::thread::sleep(Duration::from_secs(2));
            APP_STATE.write().unwrap()
        }
        Err(TryLockError::Poisoned(_)) => APP_STATE.write().unwrap(),
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            puzzle_collection: None,
            puzzle_config: None,
            puzzle_type_extension: None,
            solver_state: SolverState::Disabled,
        }
    }
}

/// Represents the current state of the puzzle solver.
#[derive(Debug, Clone)]
pub enum SolverState {
    /// Solver did not run yet. This is the state at application start.
    Initial,
    /// When no target day is selected, the solver is not available.
    NotAvailable,
    /// Solver is disabled in preferences.
    Disabled,
    /// Solver is currently running.
    /// It can be canceled using the provided cancellation token.
    Running {
        call_id: SolverCallId,
        cancel_token: CancellationToken,
    },
    /// Solver has finished.
    Done {
        /// Whether the puzzle is solvable.
        solvable: bool,
        /// Duration the solver took to complete.
        duration: Duration,
    },
}

#[derive(Debug)]
pub enum PuzzleTypeExtension {
    Simple,
    Area { target: Option<Target> },
}
