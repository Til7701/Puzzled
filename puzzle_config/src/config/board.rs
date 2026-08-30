use crate::config::area::AreaConfig;
use crate::{Target, TargetIndex, TargetTemplate};
use puzzled_common::polyform::Polyform;
use std::hash::{Hash, Hasher};

pub type SimpleBoardData = ();
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct AreaBoardData {
    pub area_index: i32,
    pub display_value: String,
    pub value_order: i32,
}

/// Configuration for the board layout and areas.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BoardConfig {
    Simple {
        layout: Polyform<()>,
    },
    Area {
        layout: Polyform<AreaBoardData>,
        area_configs: Vec<AreaConfig>,
        target_template: TargetTemplate,
    },
}

impl BoardConfig {
    pub fn default_target(&self) -> Option<Target> {
        match self {
            BoardConfig::Simple { .. } => None,
            BoardConfig::Area {
                layout,
                area_configs,
                ..
            } => {
                let mut indices = Vec::new();
                for (i, area_config) in area_configs.iter().enumerate() {
                    if let Some(target_index) = Self::find_index_for_value_in_area(
                        area_config.default_value(),
                        i as i32,
                        layout,
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
        layout: &Polyform<AreaBoardData>,
    ) -> Option<TargetIndex> {
        for prototile in layout.iter() {
            let data = prototile.data();
            if data.display_value == board_value && data.area_index == area_index {
                let coord = prototile.coord();
                return Some(TargetIndex(*coord));
            }
        }
        None
    }

    pub fn layout<T>(&self) -> &Polyform<T> {
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
        let layout = match self {
            BoardConfig::Simple { .. } => {
                panic!("Simple board config does not have areas");
            }
            BoardConfig::Area {
                layout,
                ..
            } => layout,
        };
        let mut unordered_values = layout
            .iter()
            .filter_map(|prototile| {
                let data = prototile.data();
                let index = data.area_index();
                if index == area_index {
                    let value = data.display_value();
                    let order = data.value_order();
                    Some((*order, value.clone(), TargetIndex(prototile.coord())))
                } else {
                    None
                }
            })
            .collect::<Vec<(i32, String, TargetIndex)>>();
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
                layout,
                area_configs,
                target_template,
                ..
            } => target_template.format(target, layout, area_configs),
        }
    }
}

impl Hash for BoardConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            BoardConfig::Simple { layout } => {
                layout.hash(state);
            }
            BoardConfig::Area {
                layout,
                ..
            } => {
                layout.hash(state);
            }
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::config::area::{AreaConfig, AreaValueFormatter};
//     use crate::config::target::{TargetIndex, TargetTemplate};
//     use ndarray::arr2;
//     use puzzled_common::shape::shape_square;
//
//     #[test]
//     fn test_puzzle_config_get_display_values_for_area() {
//         let board_layout =
//             shape_square(&[[true, true, false], [true, true, true], [false, true, true]]);
//         let area_indices = arr2(&[[0, 0, -1], [0, 1, 1], [-1, 1, 1]]);
//         let display_values = arr2(&[
//             ["A".to_string(), "B".to_string(), "".to_string()],
//             ["C".to_string(), "D".to_string(), "E".to_string()],
//             ["".to_string(), "F".to_string(), "G".to_string()],
//         ]);
//         let value_order = arr2(&[[0, 1, -1], [2, 0, 3], [-1, 2, 1]]);
//         let area_configs = vec![
//             AreaConfig::new(
//                 "Area 0".to_string(),
//                 AreaValueFormatter::Plain,
//                 "".to_string(),
//             ),
//             AreaConfig::new(
//                 "Area 1".to_string(),
//                 AreaValueFormatter::Plain,
//                 "".to_string(),
//             ),
//         ];
//
//         let board_config = BoardConfig::Area {
//             layout: board_layout,
//             area_configs,
//             target_template: TargetTemplate::new("{0}, {1}, {2}"),
//         };
//
//         let area_0_values = board_config.get_display_values_for_area(0);
//         assert_eq!(
//             area_0_values,
//             vec![
//                 ("A".to_string(), TargetIndex(0, 0)),
//                 ("B".to_string(), TargetIndex(0, 1)),
//                 ("C".to_string(), TargetIndex(1, 0)),
//             ]
//         );
//
//         let area_1_values = board_config.get_display_values_for_area(1);
//         assert_eq!(
//             area_1_values,
//             vec![
//                 ("D".to_string(), TargetIndex(1, 1)),
//                 ("G".to_string(), TargetIndex(2, 2)),
//                 ("F".to_string(), TargetIndex(2, 1)),
//                 ("E".to_string(), TargetIndex(1, 2)),
//             ]
//         );
//     }
// }
