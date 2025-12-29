mod bitmask;
mod config;
mod tile;
mod util;

pub(crate) use crate::puzzle::config::PuzzleConfig;
use ndarray::{arr2, Array2};
use tile::Tile;

const DEFAULT_TILES: &'static [&'static [&'static [bool]]] = &[
    &[
        &[true, false, false],
        &[true, true, true],
        &[false, false, true],
    ],
    &[&[true, false, false, false], &[true, true, true, true]],
    &[
        &[true, false],
        &[true, true],
        &[true, false],
        &[true, false],
    ],
    &[&[true, true], &[true, true], &[true, true]],
    &[
        &[true, false],
        &[true, true],
        &[false, true],
        &[false, true],
    ],
    &[&[true, true], &[true, true], &[true, false]],
    &[&[true, true], &[true, false], &[true, true]],
    &[
        &[true, false, false],
        &[true, false, false],
        &[true, true, true],
    ],
];

const DEFAULT_BOARD_LAYOUT: [[bool; 7]; 7] = [
    [false, false, false, false, false, false, true],
    [false, false, false, false, false, false, true],
    [false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false],
    [false, false, false, true, true, true, true],
];

const DEFAULT_BOARD_MEANING_AREAS: [[i32; 7]; 7] = [
    [0, 0, 0, 0, 0, 0, -1],
    [0, 0, 0, 0, 0, 0, -1],
    [1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1, 1, 1],
    [1, 1, 1, -1, -1, -1, -1],
];

const DEFAULT_BOARD_MEANING_VALUES: [[i32; 7]; 7] = [
    [1, 2, 3, 4, 5, 6, -1],
    [7, 8, 9, 10, 11, 12, -1],
    [1, 2, 3, 4, 5, 6, 7],
    [8, 9, 10, 11, 12, 13, 14],
    [15, 16, 17, 18, 19, 20, 21],
    [22, 23, 24, 25, 26, 27, 28],
    [29, 30, 31, -1, -1, -1, -1],
];

const YEAR_TILES: &'static [&'static [&'static [bool]]] = &[
    &[
        &[true, false, false],
        &[true, true, true],
        &[false, false, true],
    ],
    &[&[true, false, false, false], &[true, true, true, true]],
    &[
        &[true, false],
        &[true, true],
        &[true, false],
        &[true, false],
    ],
    &[&[true, true], &[true, true], &[true, true]],
    &[
        &[true, false],
        &[true, true],
        &[false, true],
        &[false, true],
    ],
    &[&[true, true], &[true, true], &[true, false]],
    &[&[true, true], &[true, false], &[true, true]],
    &[
        &[true, false, false],
        &[true, false, false],
        &[true, true, true],
    ],
    &[
        &[true, true, false],
        &[false, true, true],
        &[false, true, true],
    ],
    &[&[true, true, true], &[false, true, false]],
    &[&[true, true, true], &[true, false, false]],
    &[&[true, true, false], &[false, true, true]],
];

const YEAR_BOARD_LAYOUT: [[bool; 11]; 7] = [
    [
        true, true, false, false, false, false, false, false, true, true, true,
    ],
    [
        false, false, false, false, false, false, false, false, true, false, false,
    ],
    [
        false, false, false, false, false, false, false, false, false, false, false,
    ],
    [
        false, false, false, false, false, false, false, false, false, false, false,
    ],
    [
        false, false, false, false, false, false, false, false, false, false, false,
    ],
    [
        false, false, false, false, false, false, false, false, false, false, false,
    ],
    [
        true, true, false, false, false, true, true, true, true, true, true,
    ],
];

const YEAR_BOARD_MEANING_AREAS: [[i32; 11]; 7] = [
    [-1, -1, 0, 0, 0, 0, 0, 0, -1, -1, -1],
    [2, 2, 0, 0, 0, 0, 0, 0, -1, 3, 3],
    [2, 2, 1, 1, 1, 1, 1, 1, 1, 3, 3],
    [2, 2, 1, 1, 1, 1, 1, 1, 1, 3, 3],
    [2, 2, 1, 1, 1, 1, 1, 1, 1, 3, 3],
    [2, 2, 1, 1, 1, 1, 1, 1, 1, 3, 3],
    [-1, -1, 1, 1, 1, -1, -1, -1, -1, -1, -1],
];

const YEAR_BOARD_MEANING_VALUES: [[i32; 11]; 7] = [
    [-1, -1, 1, 2, 3, 4, 5, 6, -1, -1, -1],
    [1, 2, 7, 8, 9, 10, 11, 12, -1, 1, 2],
    [3, 4, 1, 2, 3, 4, 5, 6, 7, 3, 4],
    [5, 6, 8, 9, 10, 11, 12, 13, 14, 5, 6],
    [7, 8, 15, 16, 17, 18, 19, 20, 21, 7, 8],
    [9, 10, 22, 23, 24, 25, 26, 27, 28, 9, 10],
    [-1, -1, 29, 30, 31, -1, -1, -1, -1, -1, -1],
];

pub fn get_default_config() -> PuzzleConfig {
    let tiles = create_tiles(DEFAULT_TILES);
    let board_layout = arr2(&DEFAULT_BOARD_LAYOUT);
    let meaning_areas = arr2(&DEFAULT_BOARD_MEANING_AREAS);
    let meaning_values = arr2(&DEFAULT_BOARD_MEANING_VALUES);
    PuzzleConfig::new(board_layout, meaning_areas, meaning_values, tiles)
}

pub fn get_year_config() -> PuzzleConfig {
    let tiles = create_tiles(YEAR_TILES);
    let board_layout = arr2(&YEAR_BOARD_LAYOUT);
    let meaning_areas = arr2(&YEAR_BOARD_MEANING_AREAS);
    let meaning_values = arr2(&YEAR_BOARD_MEANING_VALUES);
    PuzzleConfig::new(board_layout, meaning_areas, meaning_values, tiles)
}

fn create_tiles(tile_data_list: &'static [&'static [&'static [bool]]]) -> Vec<Tile> {
    let mut tiles: Vec<Tile> = Vec::new();
    for (i, tile_data) in tile_data_list.iter().enumerate() {
        let rows = tile_data.len();
        let cols = tile_data.get(0).map(|r| r.len()).unwrap_or(0);

        let mut flat: Vec<bool> = Vec::with_capacity(rows * cols);
        for row in tile_data.iter() {
            assert_eq!(row.len(), cols, "inconsistent row lengths in tile data");
            flat.extend(row.iter().copied());
        }

        let array: Array2<bool> =
            Array2::from_shape_vec((rows, cols), flat).expect("invalid shape for tile array");
        tiles.push(Tile::new(i as i32, array));
    }
    tiles
}
