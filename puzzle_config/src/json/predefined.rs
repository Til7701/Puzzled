use crate::json::model::{Board, Tile};
use serde::Deserialize;
use std::collections::HashMap;

pub type Predefined = ConfigStore;
pub type Custom = ConfigStore;

/// Store for predefined or custom tiles and boards.
#[derive(Default, Deserialize)]
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
}
