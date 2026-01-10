use crate::bitmask::Bitmask;
use crate::board::Board;
use crate::core::PositionedTile;
use crate::result::{Solution, UnsolvableReason};
use crate::tile::Tile;

mod array_util;
mod bitmask;
pub mod board;
mod core;
pub mod result;
pub mod tile;

pub fn solve_all_filling(board: Board, tiles: &[Tile]) -> Result<Solution, UnsolvableReason> {
    board.debug_print();
    for tile in tiles {
        tile.debug_print();
    }

    let mut board = board;
    board.trim();
    board.debug_print();

    let board_bitmask = Bitmask::from(board.get_array());
    let positioned_tiles: Vec<PositionedTile> = tiles
        .iter()
        .enumerate()
        .map(|(index, tile)| PositionedTile::new(index, tile, &board))
        .collect();

    let result = core::solve_filling(
        board.get_array().dim().0 as i32,
        &board_bitmask,
        &positioned_tiles,
    );

    match result {
        Some(_) => Ok(Solution { placements: vec![] }),
        None => Err(UnsolvableReason::NoFit),
    }
}
