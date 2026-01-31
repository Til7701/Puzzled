use crate::backtracking::positioned::PositionedTile;
use crate::backtracking::pruner::Pruner;
use crate::bitmask::Bitmask;
use crate::board::Board;
use crate::result::{Solution, UnsolvableReason};
use crate::tile::Tile;
use tokio_util::sync::CancellationToken;

pub mod core;
mod positioned;
mod pruner;

pub async fn solve_all_filling(
    board: Board,
    tiles: &[Tile],
    cancel_token: CancellationToken,
) -> Result<Solution, UnsolvableReason> {
    let mut tiles = tiles.to_vec();
    tiles.sort_by(|a, b| a.base.len().cmp(&b.base.len()).reverse());
    let tiles = tiles;

    let pruner = Pruner::new_for_filling(&board, &tiles);

    let board_bitmask = Bitmask::from(board.get_array());
    let positioned_tiles: Vec<PositionedTile> = tiles
        .iter()
        .map(|tile| PositionedTile::new(tile, &board, &pruner))
        .collect();

    let result = core::solve_filling(
        board.get_array().dim().0 as i32,
        &board_bitmask,
        &positioned_tiles,
        pruner,
        cancel_token,
    )
    .await;

    match result {
        Some(_) => Ok(Solution { placements: vec![] }),
        None => Err(UnsolvableReason::NoFit),
    }
}
