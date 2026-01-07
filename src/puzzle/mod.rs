pub mod config;

use crate::puzzle::config::AreaValueFormatter::{Nth, Plain};
pub(crate) use crate::puzzle::config::PuzzleConfig;
use crate::puzzle::config::{AreaConfig, SolutionStatistics, TargetTemplate, TileConfig};
use crate::util::transform;
use ndarray::{arr2, Array2};

fn default_tiles() -> Vec<Array2<bool>> {
    vec![
        arr2(&[
            [true, false, false],
            [true, true, true],
            [false, false, true],
        ]),
        arr2(&[[true, false, false, false], [true, true, true, true]]),
        arr2(&[[true, false], [true, true], [true, false], [true, false]]),
        arr2(&[[true, true], [true, true], [true, true]]),
        arr2(&[[true, false], [true, true], [false, true], [false, true]]),
        arr2(&[[true, true], [true, true], [true, false]]),
        arr2(&[[true, true], [true, false], [true, true]]),
        arr2(&[
            [true, false, false],
            [true, false, false],
            [true, true, true],
        ]),
    ]
}

fn default_board_layout() -> Array2<bool> {
    arr2(&[
        [true, true, true, true, true, true, false],
        [true, true, true, true, true, true, false],
        [true, true, true, true, true, true, true],
        [true, true, true, true, true, true, true],
        [true, true, true, true, true, true, true],
        [true, true, true, true, true, true, true],
        [true, true, true, false, false, false, false],
    ])
}

fn default_board_meaning_areas() -> Array2<i32> {
    arr2(&[
        [1, 1, 1, 1, 1, 1, -1],
        [1, 1, 1, 1, 1, 1, -1],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, -1, -1, -1, -1],
    ])
}

fn default_board_value_order() -> Array2<i32> {
    arr2(&[
        [1, 2, 3, 4, 5, 6, -1],
        [7, 8, 9, 10, 11, 12, -1],
        [1, 2, 3, 4, 5, 6, 7],
        [8, 9, 10, 11, 12, 13, 14],
        [15, 16, 17, 18, 19, 20, 21],
        [22, 23, 24, 25, 26, 27, 28],
        [29, 30, 31, -1, -1, -1, -1],
    ])
}

fn default_board_display_values() -> Array2<String> {
    arr2(&[
        ["Jan", "Feb", "Mar", "Apr", "May", "Jun", ""],
        ["Jul", "Aug", "Sep", "Oct", "Nov", "Dec", ""],
        ["1", "2", "3", "4", "5", "6", "7"],
        ["8", "9", "10", "11", "12", "13", "14"],
        ["15", "16", "17", "18", "19", "20", "21"],
        ["22", "23", "24", "25", "26", "27", "28"],
        ["29", "30", "31", "", "", "", ""],
    ])
    .mapv(str::to_string)
}

fn year_tiles() -> Vec<Array2<bool>> {
    vec![
        arr2(&[
            [true, false, false],
            [true, true, true],
            [false, false, true],
        ]),
        arr2(&[[true, false, false, false], [true, true, true, true]]),
        arr2(&[[true, false], [true, true], [true, false], [true, false]]),
        arr2(&[[true, true], [true, true], [true, true]]),
        arr2(&[[true, false], [true, true], [false, true], [false, true]]),
        arr2(&[[true, true], [true, true], [true, false]]),
        arr2(&[[true, true], [true, false], [true, true]]),
        arr2(&[
            [true, false, false],
            [true, false, false],
            [true, true, true],
        ]),
        arr2(&[
            [true, true, false],
            [false, true, true],
            [false, true, true],
        ]),
        arr2(&[[true, true, true], [false, true, false]]),
        arr2(&[[true, true, true], [true, false, false]]),
        arr2(&[[true, true, false], [false, true, true]]),
    ]
}

fn year_board_layout() -> Array2<bool> {
    arr2(&[
        [
            true, true, true, true, true, true, false, false, false, false, false,
        ],
        [
            true, true, true, true, true, true, false, true, true, true, true,
        ],
        [
            true, true, true, true, true, true, true, true, true, true, true,
        ],
        [
            true, true, true, true, true, true, true, true, true, true, true,
        ],
        [
            true, true, true, true, true, true, true, true, true, true, true,
        ],
        [
            true, true, true, true, true, true, true, true, true, true, true,
        ],
        [
            true, true, true, false, false, false, false, false, false, false, false,
        ],
    ])
}

fn year_board_meaning_areas() -> Array2<i32> {
    arr2(&[
        [1, 1, 1, 1, 1, 1, -1, -1, -1, -1, -1],
        [1, 1, 1, 1, 1, 1, -1, 2, 2, 3, 3],
        [0, 0, 0, 0, 0, 0, 0, 2, 2, 3, 3],
        [0, 0, 0, 0, 0, 0, 0, 2, 2, 3, 3],
        [0, 0, 0, 0, 0, 0, 0, 2, 2, 3, 3],
        [0, 0, 0, 0, 0, 0, 0, 2, 2, 3, 3],
        [0, 0, 0, -1, -1, -1, -1, -1, -1, -1, -1],
    ])
}

fn year_board_value_order() -> Array2<i32> {
    arr2(&[
        [1, 2, 3, 4, 5, 6, -1, -1, -1, -1, -1],
        [7, 8, 9, 10, 11, 12, -1, 1, 2, 1, 2],
        [1, 2, 3, 4, 5, 6, 7, 3, 4, 3, 4],
        [8, 9, 10, 11, 12, 13, 14, 5, 6, 5, 6],
        [15, 16, 17, 18, 19, 20, 21, 7, 8, 7, 8],
        [22, 23, 24, 25, 26, 27, 28, 9, 0, 9, 0],
        [29, 30, 31, -1, -1, -1, -1, -1, -1, -1, -1],
    ])
}

fn year_board_meaning_display_values() -> Array2<String> {
    arr2(&[
        ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "", "", "", "", ""],
        [
            "Jul", "Aug", "Sep", "Oct", "Nov", "Dec", "", "1", "2", "1", "2",
        ],
        ["1", "2", "3", "4", "5", "6", "7", "3", "4", "3", "4"],
        ["8", "9", "10", "11", "12", "13", "14", "5", "6", "5", "6"],
        ["15", "16", "17", "18", "19", "20", "21", "7", "8", "7", "8"],
        ["22", "23", "24", "25", "26", "27", "28", "9", "0", "9", "0"],
        ["29", "30", "31", "", "", "", "", "", "", "", ""],
    ])
    .mapv(str::to_string)
}

pub fn get_default_config() -> PuzzleConfig {
    let tiles = create_tiles(&mut default_tiles());
    let board_layout = transform(&mut default_board_layout());
    let area_indices = transform(&mut default_board_meaning_areas());
    let display_values = transform(&mut default_board_display_values());
    let value_order = transform(&mut default_board_value_order());
    PuzzleConfig::new(
        "Default Puzzle".to_string(),
        board_layout,
        area_indices,
        display_values,
        value_order,
        vec![
            AreaConfig::new("Day".to_string(), Nth),
            AreaConfig::new("Month".to_string(), Plain),
        ],
        tiles,
        Some(SolutionStatistics {
            min_per_target: 7,
            max_per_target: 216,
            average_per_target: 67.3682795698925,
            mean_per_target: 40,
            total_solutions: 25061,
        }),
        TargetTemplate::new("{0} of {1}"),
    )
}

pub fn get_year_config() -> PuzzleConfig {
    let tiles = create_tiles(&mut year_tiles());
    let board_layout = transform(&mut year_board_layout());
    let area_indices = transform(&mut year_board_meaning_areas());
    let display_values = transform(&mut year_board_meaning_display_values());
    let value_order = transform(&mut year_board_value_order());
    PuzzleConfig::new(
        "Year Puzzle".to_string(),
        board_layout,
        area_indices,
        display_values,
        value_order,
        vec![
            AreaConfig::new("Day".to_string(), Nth),
            AreaConfig::new("Month".to_string(), Plain),
            AreaConfig::new("First Digit of Year".to_string(), Plain),
            AreaConfig::new("Second Digit of Year".to_string(), Plain),
        ],
        tiles,
        Some(SolutionStatistics {
            min_per_target: 1292,
            max_per_target: 469467,
            average_per_target: 37393.1052150538,
            mean_per_target: 103348,
            total_solutions: 1391023514,
        }),
        TargetTemplate::new("{0} of {1} {2}{3}"),
    )
}

fn create_tiles(tile_data_list: &mut Vec<Array2<bool>>) -> Vec<TileConfig> {
    let mut tiles: Vec<TileConfig> = Vec::new();
    for (i, tile_data) in tile_data_list.iter_mut().enumerate() {
        let transformed_data = transform(tile_data);
        tiles.push(TileConfig::new(i as i32, transformed_data));
    }
    tiles
}
