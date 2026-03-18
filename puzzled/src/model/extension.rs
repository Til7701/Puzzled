use puzzle_config::{BoardConfig, PuzzleConfig, Target};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum PuzzleTypeExtension {
    Simple,
    Area { target: Option<Target> },
}

impl PuzzleTypeExtension {
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
