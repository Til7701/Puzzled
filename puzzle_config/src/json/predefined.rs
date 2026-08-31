use crate::json::model::{Board, Tile};
use crate::{BoardConfig, TileConfig};
use puzzled_common::polyform::Polyform;
use puzzled_common::polyform::grid::RegularCoord;
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

    #[deprecated]
    pub(crate) fn take_tiles(&mut self) -> Vec<TileConfig> {
        // let tiles: HashMap<String, Tile> = take(&mut self.tiles);
        // tiles
        //     .into_iter()
        //     .flat_map(|(name, tile)| {
        //         (0, tile, Some(name))
        //             .convert(&Predefined::default(), &mut Custom::default())
        //             .unwrap()
        //     })
        //     .collect()
        unreachable!()
    }

    #[deprecated]
    pub(crate) fn take_boards(&mut self) -> Vec<BoardConfig> {
        // let boards: HashMap<String, Board> = take(&mut self.boards);
        // boards
        //     .into_values()
        //     .map(|board| {
        //         board
        //             .convert(&Predefined::default(), &mut Custom::default())
        //             .unwrap()
        //     })
        //     .collect()
        unreachable!()
    }

    pub fn predefined_board_from_str(&self, name: &str) -> Option<BoardConfig> {
        name.split("x")
            .filter_map(|part| part.parse::<i32>().ok())
            .collect::<Vec<i32>>()
            .get(0..2)
            .map(|dims| (dims[0], dims[1]))
            .map(|(rows, cols)| BoardConfig::Simple {
                layout: Polyform::polyomino_sized(RegularCoord::new(rows as u32, cols as u32), ()),
            })
    }
}
