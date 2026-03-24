use crate::config::area::{AreaConfig, AreaValueFormatter};
use ndarray::Array2;
use std::fmt::{Display, Formatter};

/// Template for formatting targets to show in the UI.
///
/// The placeholders {0}, {1}, {2}, etc. will be replaced with the display values
/// of the corresponding target indices.
/// The area formatter will be applied to each value before inserting it into the template.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TargetTemplate(String);

impl TargetTemplate {
    pub fn new(template: &str) -> Self {
        TargetTemplate(template.to_string())
    }

    /// Formats the given target using this template.
    /// For format the value of each area, the corresponding area puzzle_config is used.
    ///
    /// # Arguments
    ///
    /// * `target`: The target to format.
    /// * `board_values`: The display values of the board.
    /// * `area_configs`: The area configurations to use for formatting the values.
    ///
    /// returns: String
    pub(crate) fn format(
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
        match area_config.formatter() {
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
            AreaValueFormatter::PrefixSuffix { prefix, suffix } => {
                format!("{}{}{}", prefix, value, suffix)
            }
        }
    }
}

/// A target for the puzzle.
///
/// It consists of a list of target indices, each representing a cell on the board.
/// It should have one index per area defined in the puzzle.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Target {
    pub indices: Vec<TargetIndex>,
}

/// Represents the index of a target cell on the board.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
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
