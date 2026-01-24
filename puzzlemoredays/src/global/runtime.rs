use once_cell::sync::Lazy;
use std::backtrace::Backtrace;
use std::mem;
use std::ops::DerefMut;
use std::sync::{Mutex, MutexGuard, TryLockError};
use tokio::runtime;
use tokio::runtime::Runtime;

static RUNTIME: Lazy<Mutex<Runtime>> = Lazy::new(|| Mutex::new(create_runtime()));

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
