use crate::puzzle::config::Target;
use crate::puzzle::PuzzleConfig;
use crate::solver::SolverCallId;
use once_cell::sync::Lazy;
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

#[derive(Debug)]
pub struct State {
    pub puzzle_config: PuzzleConfig,
    pub target_selection: Option<Target>,
    pub solver_state: SolverState,
    pub preferences_state: PreferencesState,
}

pub fn get_state() -> MutexGuard<'static, State> {
    match APP_STATE.try_lock() {
        Ok(guard) => guard,
        Err(TryLockError::WouldBlock) => {
            eprintln!(
                "get_state: mutex busy (possible deadlock). PID={} Backtrace:\n{:?}",
                std::process::id(),
                Backtrace::force_capture()
            );
            std::thread::sleep(std::time::Duration::from_secs(2));
            APP_STATE.lock().unwrap()
        }
        Err(TryLockError::Poisoned(_)) => APP_STATE.lock().unwrap(),
    }
}

impl Default for State {
    fn default() -> Self {
        let puzzle_config = PuzzleConfig::default();
        let default_target = puzzle_config.default_target.clone();
        State {
            puzzle_config,
            target_selection: default_target,
            solver_state: SolverState::Disabled,
            preferences_state: PreferencesState::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SolverState {
    Disabled,
    Running {
        call_id: SolverCallId,
        cancel_token: CancellationToken,
    },
    Done {
        solvable: bool,
        duration: Duration,
    },
}

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

pub fn take_runtime() -> Runtime {
    let runtime = mem::replace(get_runtime().deref_mut(), create_runtime());
    runtime
}

fn create_runtime() -> Runtime {
    runtime::Builder::new_multi_thread().build().unwrap()
}

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
