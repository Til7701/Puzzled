use crate::BoardConfig;
use crate::json::model::{Board, Tile};
use puzzled_polyform::{Polyomino, RegularCoord};
use serde::Deserialize;
use std::collections::HashMap;

pub type Predefined = ConfigStore;
pub type Custom = ConfigStore;

/// Store for predefined or custom tiles and boards.
#[derive(Debug, Default, Deserialize)]
pub struct ConfigStore {
    tiles: HashMap<String, Tile>,
    boards: HashMap<String, Board>,
}

impl ConfigStore {
    pub fn add_tile(&mut self, name: String, tile: Tile) {
        self.tiles.insert(name, tile);
    }

    pub fn get_tile(&self, name: &str) -> Option<Tile> {
        self.tiles.get(name).cloned()
    }

    pub fn add_board(&mut self, name: String, board: Board) {
        self.boards.insert(name, board);
    }

    pub fn get_board(&self, name: &str) -> Option<Board> {
        self.boards.get(name).cloned()
    }

    pub fn predefined_board_from_str(&self, name: &str) -> Option<BoardConfig> {
        name.split("x")
            .filter_map(|part| part.parse::<i32>().ok())
            .collect::<Vec<i32>>()
            .get(0..2)
            .map(|dims| (dims[0], dims[1]))
            .map(|(rows, cols)| BoardConfig::Simple {
                layout: Polyomino::sized(RegularCoord::new(rows, cols), ()).into(),
            })
    }
}
