use crate::puzzle::config::Target;
use crate::puzzle::PuzzleConfig;
use once_cell::sync::Lazy;
use std::backtrace::Backtrace;
use std::sync::{Mutex, MutexGuard, TryLockError};

static APP_STATE: Lazy<Mutex<State>> = Lazy::new(|| Mutex::new(State::default()));

#[derive(Debug, Clone)]
pub struct State {
    pub puzzle_config: PuzzleConfig,
    pub target_selection: Option<Target>,
}

pub fn get_state() -> MutexGuard<'static, State> {
    match APP_STATE.try_lock() {
        Ok(guard) => guard,
        Err(TryLockError::WouldBlock) => {
            eprintln!(
                "get_state: mutex busy (possible deadlock). PID={} Backtrace:\n{:?}",
                std::process::id(),
                Backtrace::capture()
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
        State {
            puzzle_config,
            target_selection: None,
        }
    }
}
