use crate::config::board::BoardConfig;
use crate::config::difficulty::PuzzleDifficultyConfig;
use crate::TileConfig;
use std::collections::HashMap;

pub type PuzzleId = String;

/// Configuration for a puzzle. It describes the board layout and tiles.
/// It additionally contains configuration for the areas to show in the target selection.
/// The solution statistics are optional since they might not be known yet for all puzzles.
#[derive(Debug, Clone)]
pub struct PuzzleConfig {
    /// The index of the puzzle in the collection.
    index: usize,
    /// Unique identifier for the puzzle, used for saving progress and other internal purposes.
    /// By default, this is the index as a hex string.
    id: PuzzleId,
    /// Name of the puzzle to show in the UI.
    name: String,
    description: Option<String>,
    difficulty: Option<PuzzleDifficultyConfig>,
    /// The tiles that can be placed on the board.
    tiles: Vec<TileConfig>,
    /// Configuration of the board layout and areas.
    board_config: BoardConfig,
    additional_info: Option<HashMap<String, String>>,
}

impl PuzzleConfig {
    /// Creates a new PuzzleConfig.
    ///
    /// # Arguments
    ///
    /// * `name`: The name to how in the UI.
    /// * `board_layout`: An array where true indicates a cell where a tile can be placed.
    /// * `area_indices`: An array where each cell contains the index of the area it belongs to.
    /// * `display_values`: An array where each cell contains the display value for that cell.
    /// * `value_order`: An array where each cell contains the order value for that cell in the area it belongs to.
    /// * `area_configs`: Configuration for each area on the board.
    /// * `tiles`: The tiles that can be placed on the board.
    /// * `solution_statistics`: Optional statistics about the solutions for this puzzle.
    /// * `default_target`: Optional default target for the puzzle.
    /// * `target_template`: Template for formatting targets to show in the UI.
    ///
    /// returns: PuzzleConfig
    pub fn new(
        index: usize,
        id: PuzzleId,
        name: String,
        description: Option<String>,
        difficulty: Option<PuzzleDifficultyConfig>,
        tiles: Vec<TileConfig>,
        board_config: BoardConfig,
        additional_info: Option<HashMap<String, String>>,
    ) -> PuzzleConfig {
        PuzzleConfig {
            index,
            id,
            name,
            description,
            difficulty,
            board_config,
            tiles,
            additional_info,
        }
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn id(&self) -> &PuzzleId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn difficulty(&self) -> &Option<PuzzleDifficultyConfig> {
        &self.difficulty
    }

    pub fn tiles(&self) -> &Vec<TileConfig> {
        &self.tiles
    }

    pub fn board_config(&self) -> &BoardConfig {
        &self.board_config
    }

    pub fn additional_info(&self) -> &Option<HashMap<String, String>> {
        &self.additional_info
    }
}
