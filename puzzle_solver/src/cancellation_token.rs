use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

/// A cancellation token can be used to signal that a solver task should be canceled.
///
/// Instances can be cloned and moved between threads safely to distribute the same token.
///
/// ```rust
/// use std::thread;
/// use puzzle_solver::cancellation_token::CancellationToken;
///
/// let t1 = CancellationToken::new();
/// let t2 = t1.clone();
///
/// thread::spawn(move || {
///     t1.cancel();
/// }).join();
///
/// assert!(t2.is_cancelled());
/// ```
#[derive(Clone, Debug)]
pub struct CancellationToken {
    /// Is initialized as false and set to true, if cancel is called on the token.
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    /// Creates a new cancellation token.
    ///
    /// ```rust
    /// use puzzle_solver::cancellation_token::CancellationToken;
    ///
    /// let token = CancellationToken::new();
    /// assert!(!token.is_cancelled());
    /// ```
    pub fn new() -> CancellationToken {
        CancellationToken {
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Signals that tasks associated with this token should be canceled.
    /// ```rust
    /// use puzzle_solver::cancellation_token::CancellationToken;
    ///
    /// let token = CancellationToken::new();
    /// token.cancel();
    /// assert!(token.is_cancelled());
    /// ```
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }

    /// Returns true, if [CancellationToken::cancel] has been called on this token after
    /// its creation. False otherwise.
    /// ```rust
    /// use puzzle_solver::cancellation_token::CancellationToken;
    ///
    /// let token = CancellationToken::new();
    /// assert!(!token.is_cancelled());
    /// token.cancel();
    /// assert!(token.is_cancelled());
    /// ```
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }
}
