use crate::config::area::AreaConfig;
use crate::{Target, TargetIndex, TargetTemplate};
use puzzled_common::polyform::{Poly, Polyform, Prototile};
use std::hash::{Hash, Hasher};

pub type SimpleBoardData = ();
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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
                return Some(TargetIndex::new(coord));
            }
        }
        None
    }

    pub fn layout(&self) -> Polyform<()> {
        match self {
            BoardConfig::Simple { layout } => layout.clone(),
            BoardConfig::Area { layout, .. } => layout.clone().mapped(&|_| ()),
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
            BoardConfig::Area { layout, .. } => layout,
        };
        let mut unordered_values = layout
            .iter()
            .filter_map(|prototile| {
                let data = prototile.data();
                let index = data.area_index;
                if index == area_index {
                    let value = &data.display_value;
                    let order = data.value_order;
                    Some((order, value.clone(), TargetIndex::new(prototile.coord())))
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
            BoardConfig::Area { layout, .. } => {
                layout.hash(state);
            }
        }
    }
}
