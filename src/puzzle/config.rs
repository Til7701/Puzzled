use crate::puzzle::tile::Tile;
use ndarray::Array2;

pub struct PuzzleConfig {
    pub board_layout: Array2<bool>,
    pub meaning_areas: Array2<i32>,
    pub meaning_values: Array2<i32>,
    pub tiles: Vec<Tile>,
}

impl PuzzleConfig {
    pub fn new(
        board_layout: Array2<bool>,
        meaning_areas: Array2<i32>,
        meaning_values: Array2<i32>,
        tiles: Vec<Tile>,
    ) -> PuzzleConfig {
        PuzzleConfig {
            board_layout,
            meaning_areas,
            meaning_values,
            tiles,
        }
    }
}
