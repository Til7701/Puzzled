use crate::puzzles;
use crate::solver::SolverCallId;
use once_cell::sync::Lazy;
use puzzle_config::{PuzzleConfig, PuzzleConfigCollection, Target};
use std::backtrace::Backtrace;
use std::mem;
use std::ops::DerefMut;
use std::sync::{Mutex, MutexGuard, TryLockError};
use std::time::Duration;
use tokio::runtime;
use tokio::runtime::Runtime;
use tokio_util::sync::CancellationToken;

static APP_STATE: Lazy<Mutex<State>> = Lazy::new(|| Mutex::new(State::default()));
static RUNTIME: Lazy<Mutex<Runtime>> = Lazy::new(|| Mutex::new(create_runtime()));

/// Represents the global application state.
#[derive(Debug)]
pub struct State {
    /// The currently selected puzzle collection.
    pub puzzle_collection: Option<PuzzleConfigCollection>,
    /// The puzzle configuration currently shown on the screen.
    pub puzzle_config: PuzzleConfig,
    /// The currently selected target for the puzzle.
    pub target_selection: Option<Target>,
    /// The current state of the puzzle solver.
    pub solver_state: SolverState,
    pub preferences_state: PreferencesState,
}

/// Acquires a lock on the global application state and returns a guard to it.
///
/// If the mutex is already locked, it will log a warning and wait before retrying to acquire the
/// lock.
/// This is to help diagnose potential deadlocks in the application.
pub fn get_state() -> MutexGuard<'static, State> {
    match APP_STATE.try_lock() {
        Ok(guard) => guard,
        Err(TryLockError::WouldBlock) => {
            eprintln!(
                "get_state: mutex busy (possible deadlock). PID={} Backtrace:\n{:?}",
                std::process::id(),
                Backtrace::force_capture()
            );
            std::thread::sleep(Duration::from_secs(2));
            APP_STATE.lock().unwrap()
        }
        Err(TryLockError::Poisoned(_)) => APP_STATE.lock().unwrap(),
    }
}

impl Default for State {
    fn default() -> Self {
        let puzzle_config = puzzles::default_puzzle();
        let default_target = puzzle_config.board_config().default_target();
        State {
            puzzle_collection: None,
            puzzle_config,
            target_selection: default_target,
            solver_state: SolverState::Disabled,
            preferences_state: PreferencesState::default(),
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

/// Acquires a lock on the global Tokio runtime and returns a guard to it.
pub fn get_runtime() -> MutexGuard<'static, Runtime> {
    match RUNTIME.try_lock() {
        Ok(guard) => guard,
        Err(TryLockError::WouldBlock) => {
            eprintln!(
                "get_runtime: mutex busy (possible deadlock). PID={} Backtrace:\n{:?}",
                std::process::id(),
                Backtrace::force_capture()
            );
            std::thread::sleep(std::time::Duration::from_secs(2));
            RUNTIME.lock().unwrap()
        }
        Err(TryLockError::Poisoned(_)) => RUNTIME.lock().unwrap(),
    }
}

/// Takes ownership of the global Tokio runtime, replacing it with a new one.
/// This can be used to shut down the current runtime.
pub fn take_runtime() -> Runtime {
    let runtime = mem::replace(get_runtime().deref_mut(), create_runtime());
    runtime
}

/// Creates a new Tokio runtime instance for the solver tasks.
fn create_runtime() -> Runtime {
    runtime::Builder::new_multi_thread().build().unwrap()
}

/// Represents the user preferences state.
///
/// TODO save/load preferences to/from disk.
#[derive(Debug)]
pub struct PreferencesState {
    pub solver_enabled: bool,
}

impl Default for PreferencesState {
    fn default() -> Self {
        PreferencesState {
            solver_enabled: true,
        }
    }
}
