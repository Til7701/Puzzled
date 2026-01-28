#[derive(Debug, Clone)]
pub enum ProgressionConfig {
    /// Puzzles can be completed in any order.
    Any,
    /// A puzzle must be completed before the next one is unlocked.
    Sequential,
}
