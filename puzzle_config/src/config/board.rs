use crate::config::area::AreaConfig;
use crate::{Target, TargetIndex, TargetTemplate};
use ndarray::Array2;

/// Configuration for the board layout and areas.
#[derive(Debug, Clone)]
pub enum BoardConfig {
    Simple {
        layout: Array2<bool>,
    },
    Area {
        layout: Array2<bool>,
        area_indices: Array2<i32>,
        display_values: Array2<String>,
        value_order: Array2<i32>,
        area_configs: Vec<AreaConfig>,
        target_template: TargetTemplate,
    },
}

impl BoardConfig {
    pub fn default_target(&self) -> Option<Target> {
        match self {
            BoardConfig::Simple { .. } => None,
            BoardConfig::Area {
                display_values,
                area_indices,
                area_configs,
                ..
            } => {
                let mut indices = Vec::new();
                for (i, area_config) in area_configs.iter().enumerate() {
                    if let Some(target_index) = Self::find_index_for_value_in_area(
                        area_config.default_value(),
                        i as i32,
                        display_values,
                        area_indices,
                    ) {
                        indices.push(target_index);
                    }
                }
                Some(Target { indices })
            }
        }
    }

    fn find_index_for_value_in_area(
        board_value: &str,
        area_index: i32,
        board_values: &Array2<String>,
        area_indices: &Array2<i32>,
    ) -> Option<TargetIndex> {
        for ((x, y), value) in board_values.indexed_iter() {
            if value == board_value && area_indices[[x, y]] == area_index {
                return Some(TargetIndex(x, y));
            }
        }
        None
    }

    pub fn layout(&self) -> &Array2<bool> {
        match self {
            BoardConfig::Simple { layout } => layout,
            BoardConfig::Area { layout, .. } => layout,
        }
    }

    /// Returns the number of areas defined in the puzzle.
    pub fn area_count(&self) -> usize {
        match self {
            BoardConfig::Simple { .. } => 0,
            BoardConfig::Area { area_configs, .. } => area_configs.len(),
        }
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
        let (area_indices, display_values, value_order) = match self {
            BoardConfig::Simple { .. } => {
                panic!("Simple board config does not have areas");
            }
            BoardConfig::Area {
                area_indices,
                display_values,
                value_order,
                ..
            } => (area_indices, display_values, value_order),
        };
        let mut unordered_values = Vec::new();
        for ((x, y), &index) in area_indices.indexed_iter() {
            if index == area_index {
                if let Some(value) = display_values.get((x, y))
                    && let Some(order) = value_order.get((x, y))
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
        match self {
            BoardConfig::Simple { .. } => {
                panic!("Simple board config does not have target formatting");
            }
            BoardConfig::Area {
                display_values,
                area_configs,
                target_template,
                ..
            } => target_template.format(target, &display_values, &area_configs),
        }
    }
}

pub fn from_predefined_board(name: &str) -> Option<BoardConfig> {
    let dim: Option<(i32, i32)> = name
        .split("x")
        .filter_map(|part| part.parse::<i32>().ok())
        .collect::<Vec<i32>>()
        .get(0..2)
        .and_then(|dims| Some((dims[0], dims[1])));
    dim.map(|(rows, cols)| BoardConfig::Simple {
        layout: Array2::from_shape_fn((rows as usize, cols as usize), |_| true),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::area::{AreaConfig, AreaValueFormatter};
    use crate::config::target::{TargetIndex, TargetTemplate};
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
            AreaConfig::new(
                "Area 0".to_string(),
                AreaValueFormatter::Plain,
                "".to_string(),
            ),
            AreaConfig::new(
                "Area 1".to_string(),
                AreaValueFormatter::Plain,
                "".to_string(),
            ),
        ];

        let board_config = BoardConfig::Area {
            layout: board_layout,
            area_indices,
            display_values,
            value_order,
            area_configs,
            target_template: TargetTemplate::new("{0}, {1}, {2}"),
        };

        let area_0_values = board_config.get_display_values_for_area(0);
        assert_eq!(
            area_0_values,
            vec![
                ("A".to_string(), TargetIndex(0, 0)),
                ("B".to_string(), TargetIndex(0, 1)),
                ("C".to_string(), TargetIndex(1, 0)),
            ]
        );

        let area_1_values = board_config.get_display_values_for_area(1);
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
