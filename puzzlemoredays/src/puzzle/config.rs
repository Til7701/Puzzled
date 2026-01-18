use crate::puzzle::get_default_config;
use ndarray::Array2;
use std::fmt::{Debug, Display, Formatter};

/// Configuration for a puzzle. It describes the board layout and tiles.
/// It additionally contains configuration for the areas to show in the target selection.
/// The solution statistics are optional since they might not be known yet for all puzzles.
#[derive(Debug, Clone)]
pub struct PuzzleConfig {
    /// Name of the puzzle to show in the UI.
    pub name: String,
    /// Configuration of the board layout and areas.
    pub board_config: BoardConfig,
    /// The tiles that can be placed on the board.
    pub tiles: Vec<TileConfig>,
    pub solution_statistics: Option<SolutionStatistics>,
    pub default_target: Option<Target>,
    target_template: TargetTemplate,
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
        name: String,
        board_layout: Array2<bool>,
        area_indices: Array2<i32>,
        display_values: Array2<String>,
        value_order: Array2<i32>,
        area_configs: Vec<AreaConfig>,
        tiles: Vec<TileConfig>,
        solution_statistics: Option<SolutionStatistics>,
        default_target: Option<Target>,
        target_template: TargetTemplate,
    ) -> PuzzleConfig {
        PuzzleConfig {
            name,
            board_config: BoardConfig::new(
                board_layout,
                area_indices,
                display_values,
                area_configs,
                value_order,
            ),
            tiles,
            solution_statistics,
            default_target,
            target_template,
        }
    }

    /// Returns the number of areas defined in the puzzle.
    pub fn area_count(&self) -> usize {
        self.board_config.area_configs.len()
    }

    /// Returns the display values and their target indices for the given area index.
    /// The values are returned in the order they are defined in the board configuration.
    ///
    /// # Arguments
    ///
    /// * `area_index`: The index of the area to get the display values for.
    ///
    /// returns: Vec<(String, TargetIndex), Global>
    pub fn get_display_values_for_area(&self, area_index: i32) -> Vec<(String, TargetIndex)> {
        let mut unordered_values = Vec::new();
        for ((x, y), &index) in self.board_config.area_indices.indexed_iter() {
            if index == area_index {
                if let Some(value) = self.board_config.display_values.get((x, y))
                    && let Some(order) = self.board_config.value_order.get((x, y))
                {
                    unordered_values.push((order, value.clone(), TargetIndex(x, y)));
                }
            }
        }
        unordered_values.sort_by_key(|(order, _, _)| *order);
        unordered_values
            .into_iter()
            .map(|(_, value, target_index)| (value, target_index))
            .collect()
    }

    /// Formats the given target using the target template for this puzzle.
    ///
    /// The returned string can be shown in the UI to represent the target.
    ///
    /// # Arguments
    ///
    /// * `target`: The target to format.
    ///
    /// returns: String
    pub fn format_target(&self, target: &Target) -> String {
        self.target_template.format(
            target,
            &self.board_config.display_values,
            &self.board_config.area_configs,
        )
    }
}

impl Default for PuzzleConfig {
    fn default() -> Self {
        get_default_config()
    }
}

/// Configuration for the board layout and areas.
#[derive(Debug, Clone)]
pub struct BoardConfig {
    pub layout: Array2<bool>,
    pub area_indices: Array2<i32>,
    pub display_values: Array2<String>,
    pub area_configs: Vec<AreaConfig>,
    pub value_order: Array2<i32>,
}

impl BoardConfig {
    fn new(
        layout: Array2<bool>,
        indices: Array2<i32>,
        display_values: Array2<String>,
        area_configs: Vec<AreaConfig>,
        value_order: Array2<i32>,
    ) -> BoardConfig {
        BoardConfig {
            layout,
            area_indices: indices,
            display_values,
            area_configs,
            value_order,
        }
    }
}

/// Configuration for a tile that can be placed on the board.
#[derive(Debug, Clone)]
pub struct TileConfig {
    /// Unique identifier for the tile.
    pub id: i32,
    /// Base shape of the tile as a 2D boolean array.
    /// True indicates a filled cell, false indicates an empty cell.
    pub base: Array2<bool>,
}

impl TileConfig {
    /// Creates a new TileConfig.
    ///
    /// # Arguments
    ///
    /// * `id`: Unique identifier for the tile.
    /// * `base`: Base shape of the tile as a 2D boolean array.
    ///
    /// returns: TileConfig
    pub fn new(id: i32, base: Array2<bool>) -> TileConfig {
        TileConfig { id, base }
    }
}

/// Statistics about the solutions for a puzzle.
///
/// This may be provided in the puzzle configuration if known.
#[derive(Debug, Clone)]
pub struct SolutionStatistics {
    pub min_per_target: i32,
    pub max_per_target: i32,
    pub average_per_target: f64,
    pub mean_per_target: i32,
    pub total_solutions: i32,
}

/// Metadata for an area on the board.
/// Includes the name and the formatter for the area values.
/// This is used by the target selection UI.
#[derive(Debug, Clone)]
pub struct AreaConfig {
    pub name: String,
    pub formatter: AreaValueFormatter,
}

impl AreaConfig {
    pub fn new(name: String, area_value_formatter: AreaValueFormatter) -> Self {
        AreaConfig {
            name,
            formatter: area_value_formatter,
        }
    }
}

/// Formatter for a value for an area to display on the target selection button.
#[derive(Debug, Clone)]
pub enum AreaValueFormatter {
    /// Displays the value as is.
    Plain,
    /// Formats the value as an ordinal number (1st, 2nd, 3rd, 4th, etc.).
    Nth,
}

/// Template for formatting targets to show in the UI.
///
/// The placeholders {0}, {1}, {2}, etc. will be replaced with the display values
/// of the corresponding target indices.
/// The area formatter will be applied to each value before inserting it into the template.
#[derive(Debug, Clone)]
pub struct TargetTemplate(String);

impl TargetTemplate {
    pub fn new(template: &str) -> Self {
        TargetTemplate(template.to_string())
    }

    /// Formats the given target using this template.
    /// For format the value of each area, the corresponding area config is used.
    ///
    /// # Arguments
    ///
    /// * `target`: The target to format.
    /// * `board_values`: The display values of the board.
    /// * `area_configs`: The area configurations to use for formatting the values.
    ///
    /// returns: String
    fn format(
        &self,
        target: &Target,
        board_values: &Array2<String>,
        area_configs: &[AreaConfig],
    ) -> String {
        let values: Vec<String> = target
            .indices
            .iter()
            .map(|TargetIndex(x, y)| {
                board_values
                    .get((*x, *y))
                    .cloned()
                    .unwrap_or_else(|| "???".to_string())
            })
            .collect();

        let mut result = self.0.clone();
        for (i, value) in values.iter().enumerate() {
            let placeholder = format!("{{{}}}", i);
            let value: String = {
                if let Some(area_config) = area_configs.get(i) {
                    self.format_value(value, area_config)
                } else {
                    value.clone()
                }
            };
            result = result.replace(&placeholder, value.as_str());
        }
        result
    }

    /// This function implements the formatting logic for a single value based on the area
    /// configuration.
    ///
    /// # Arguments
    ///
    /// * `value`: The value to format.
    /// * `area_config`: The area configuration to use for formatting.
    ///
    /// returns: String
    fn format_value(&self, value: &str, area_config: &AreaConfig) -> String {
        match area_config.formatter {
            AreaValueFormatter::Plain => value.to_string(),
            AreaValueFormatter::Nth => match value {
                "1" => "1st".to_string(),
                "2" => "2nd".to_string(),
                "3" => "3rd".to_string(),
                "21" => "21st".to_string(),
                "22" => "22nd".to_string(),
                "23" => "23rd".to_string(),
                "31" => "31st".to_string(),
                _ => format!("{}th", value),
            },
        }
    }
}

/// A target for the puzzle.
///
/// It consists of a list of target indices, each representing a cell on the board.
/// It should have one index per area defined in the puzzle.
#[derive(Debug, Clone)]
pub struct Target {
    pub indices: Vec<TargetIndex>,
}

/// Represents the index of a target cell on the board.
#[derive(Debug, Clone, PartialEq)]
pub struct TargetIndex(pub usize, pub usize);

impl PartialEq<(i32, i32)> for TargetIndex {
    fn eq(&self, other: &(i32, i32)) -> bool {
        self.0 as i32 == other.0 && self.1 as i32 == other.1
    }
}

impl Display for TargetIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    #[test]
    fn test_puzzle_config_get_display_values_for_area() {
        let board_layout = arr2(&[[true, true, false], [true, true, true], [false, true, true]]);
        let area_indices = arr2(&[[0, 0, -1], [0, 1, 1], [-1, 1, 1]]);
        let display_values = arr2(&[
            ["A".to_string(), "B".to_string(), "".to_string()],
            ["C".to_string(), "D".to_string(), "E".to_string()],
            ["".to_string(), "F".to_string(), "G".to_string()],
        ]);
        let value_order = arr2(&[[0, 1, -1], [2, 0, 3], [-1, 2, 1]]);
        let area_configs = vec![
            AreaConfig::new("Area 0".to_string(), AreaValueFormatter::Plain),
            AreaConfig::new("Area 1".to_string(), AreaValueFormatter::Plain),
        ];

        let puzzle_config = PuzzleConfig::new(
            "Test Puzzle".to_string(),
            board_layout,
            area_indices,
            display_values,
            value_order,
            area_configs,
            vec![],
            None,
            None,
            TargetTemplate::new("{0}, {1}, {2}"),
        );

        let area_0_values = puzzle_config.get_display_values_for_area(0);
        assert_eq!(
            area_0_values,
            vec![
                ("A".to_string(), TargetIndex(0, 0)),
                ("B".to_string(), TargetIndex(0, 1)),
                ("C".to_string(), TargetIndex(1, 0)),
            ]
        );

        let area_1_values = puzzle_config.get_display_values_for_area(1);
        assert_eq!(
            area_1_values,
            vec![
                ("D".to_string(), TargetIndex(1, 1)),
                ("G".to_string(), TargetIndex(2, 2)),
                ("F".to_string(), TargetIndex(2, 1)),
                ("E".to_string(), TargetIndex(1, 2)),
            ]
        );
    }
}
