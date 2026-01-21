/// Statistics about the solutions for a puzzle.
///
/// This may be provided in the puzzle configuration if known.
#[derive(Debug, Clone)]
pub struct SolutionStatistics {
    min_per_target: i32,
    max_per_target: i32,
    average_per_target: f64,
    mean_per_target: i32,
    total_solutions: i32,
}

impl SolutionStatistics {
    pub fn min_per_target(&self) -> i32 {
        self.min_per_target
    }

    pub fn max_per_target(&self) -> i32 {
        self.max_per_target
    }

    pub fn average_per_target(&self) -> f64 {
        self.average_per_target
    }

    pub fn mean_per_target(&self) -> i32 {
        self.mean_per_target
    }

    pub fn total_solutions(&self) -> i32 {
        self.total_solutions
    }
}
