use crate::array_util;
use crate::bitmask::Bitmask;
use crate::board::Board;
use crate::tile::Tile;

pub struct PositionedTile {
    pub tile_index: usize,
    pub bitmasks: Vec<Bitmask>,
}

impl PositionedTile {
    pub fn new(tile_index: usize, tile: &Tile, board: &Board) -> Self {
        let all_placements = array_util::place_on_all_positions(&tile.base, board.get_array());
        let bitmasks: Vec<Bitmask> = all_placements
            .iter()
            .map(|array| Bitmask::from(array))
            .collect();

        PositionedTile {
            tile_index,
            bitmasks,
        }
    }
}
