use puzzle_config::{BoardConfig, PuzzleConfig, Target};

/// Extra data needed while the puzzle is solved.
/// The extension may differ a lot and a puzzle as a solved state for each different state.
/// There may be an unbounded amount of different extensions for each puzzle.
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum PuzzleTypeExtension {
    Simple,
    Area { target: Option<Target> },
}

impl PuzzleTypeExtension {
    /// Creates the default extension for a given puzzle.
    ///
    /// This is derived from the puzzle config.
    /// This is the extension to be used to display the solved state in the puzzle and
    /// collection overview.
    ///
    /// # Arguments
    ///
    /// * `puzzle_config`: the puzzle config to create the extension for
    ///
    /// returns: PuzzleTypeExtension
    pub fn default_for_puzzle(puzzle_config: &PuzzleConfig) -> Self {
        match &puzzle_config.board_config() {
            BoardConfig::Simple { .. } => PuzzleTypeExtension::Simple,
            BoardConfig::Area { .. } => {
                let default_target = puzzle_config.board_config().default_target();
                PuzzleTypeExtension::Area {
                    target: default_target,
                }
            }
        }
    }
}
