use crate::puzzle::get_default_config;
use ndarray::Array2;
use std::fmt::Debug;

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
    target_template: TargetTemplate,
}

impl PuzzleConfig {
    pub fn new(
        name: String,
        board_layout: Array2<bool>,
        area_indices: Array2<i32>,
        display_values: Array2<String>,
        value_order: Array2<i32>,
        area_configs: Vec<AreaConfig>,
        tiles: Vec<TileConfig>,
        solution_statistics: Option<SolutionStatistics>,
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
            target_template,
        }
    }

    pub fn area_count(&self) -> usize {
        self.board_config.area_configs.len()
    }

    pub fn get_display_values_for_area(&self, area_index: i32) -> Vec<(String, TargetIndex)> {
        let mut values = Vec::new();
        for ((x, y), &index) in self.board_config.area_indices.indexed_iter() {
            if index == area_index {
                if let Some(value) = self.board_config.display_values.get((x, y)) {
                    values.push((value.clone(), TargetIndex(x, y)));
                }
            }
        }
        values
    }

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

#[derive(Debug, Clone)]
pub struct BoardConfig {
    pub layout: Array2<bool>,
    pub area_indices: Array2<i32>,
    pub display_values: Array2<String>,
    pub area_configs: Vec<AreaConfig>,
    pub value_order: Array2<i32>,
}

impl BoardConfig {
    pub fn new(
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

#[derive(Debug, Clone)]
pub struct TileConfig {
    pub id: i32,
    pub base: Array2<bool>,
}

impl TileConfig {
    pub fn new(id: i32, base: Array2<bool>) -> TileConfig {
        TileConfig { id, base }
    }
}

#[derive(Debug, Clone)]
pub struct SolutionStatistics {
    pub min_per_target: i32,
    pub max_per_target: i32,
    pub average_per_target: f64,
    pub mean_per_target: i32,
    pub total_solutions: i32,
}

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

#[derive(Debug, Clone)]
pub enum AreaValueFormatter {
    Plain,
    Nth,
}

#[derive(Debug, Clone)]
pub struct TargetTemplate(String);

impl TargetTemplate {
    pub fn new(template: &str) -> Self {
        TargetTemplate(template.to_string())
    }

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

#[derive(Debug, Clone)]
pub struct Target {
    pub indices: Vec<TargetIndex>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TargetIndex(pub usize, pub usize);

impl PartialEq<(i32, i32)> for TargetIndex {
    fn eq(&self, other: &(i32, i32)) -> bool {
        self.0 as i32 == other.0 && self.1 as i32 == other.1
    }
}
