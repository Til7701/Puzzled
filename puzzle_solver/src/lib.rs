use crate::bitmask::Bitmask;
use crate::board::Board;
use crate::core::PositionedTile;
use crate::result::{Solution, UnsolvableReason};
use crate::tile::Tile;
use log::debug;
use tokio_util::sync::CancellationToken;

mod array_util;
mod banned;
mod bitmask;
pub mod board;
mod core;
pub mod result;
pub mod tile;

pub async fn solve_all_filling(
    board: Board,
    tiles: &[Tile],
    cancel_token: CancellationToken,
) -> Result<Solution, UnsolvableReason> {
    if !plausibility_check(&board, &tiles) {
        debug!("Plausibility check failed.");
        return Err(UnsolvableReason::NoFit);
    }

    let mut board = board;
    board.trim();

    let board_bitmask = Bitmask::from(board.get_array());
    let positioned_tiles: Vec<PositionedTile> = tiles
        .iter()
        .map(|tile| PositionedTile::new(tile, &board))
        .collect();

    let banned_bitmasks = banned::create_banned_bitmasks_for_filling(&board, &tiles);

    let result = core::solve_filling(
        board.get_array().dim().0 as i32,
        &board_bitmask,
        &positioned_tiles,
        banned_bitmasks,
        cancel_token,
    )
    .await;

    match result {
        Some(_) => Ok(Solution { placements: vec![] }),
        None => Err(UnsolvableReason::NoFit),
    }
}

fn plausibility_check(board: &Board, tiles: &[Tile]) -> bool {
    let board_area = board.get_array().iter().filter(|&&cell| !cell).count();
    let tiles_area: usize = tiles
        .iter()
        .map(|tile| tile.base.iter().filter(|&&cell| cell).count())
        .sum();
    debug!(
        "Plausibility check: board area = {}, tiles area = {}",
        board_area, tiles_area
    );
    tiles_area == board_area
}
