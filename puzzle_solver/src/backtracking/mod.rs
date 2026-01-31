use crate::array_util;
use crate::backtracking::positioned::PositionedTile;
use crate::backtracking::pruner::Pruner;
use crate::bitmask::Bitmask;
use crate::board::Board;
use crate::result::{Solution, TilePlacement, UnsolvableReason};
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
        Some(placements) => Ok(create_solution(
            placements,
            &positioned_tiles,
            &tiles,
            &board,
        )),
        None => Err(UnsolvableReason::NoFit),
    }
}

fn create_solution(
    placements: Vec<usize>,
    positioned_tiles: &[PositionedTile],
    tiles: &[Tile],
    board: &Board,
) -> Solution {
    let tile_placements: Vec<TilePlacement> = placements
        .iter()
        .enumerate()
        .map(|(tile_index, &placement_index)| {
            create_tile_placement(
                placement_index,
                &positioned_tiles[tile_index],
                &tiles[tile_index],
                board,
            )
        })
        .collect();
    Solution::new(tile_placements)
}

fn create_tile_placement(
    placement_index: usize,
    positioned_tile: &PositionedTile,
    tile: &Tile,
    board: &Board,
) -> TilePlacement {
    let bitmask_placement = &positioned_tile.bitmasks()[placement_index];
    let placement_board =
        bitmask_placement.to_array2(board.get_array().dim().0, board.get_array().dim().1);
    let mut inverted_placement = placement_board.mapv(|v| !v);
    array_util::remove_true_rows_cols_from_sides(&mut inverted_placement);
    let rotation = inverted_placement.mapv(|v| !v);

    let x: usize = {
        let mut x_set = false;
        let mut x_start = 0usize;
        for x in 0..placement_board.dim().0 {
            for y in 0..placement_board.dim().1 {
                if placement_board[[x, y]] {
                    x_start = x;
                    x_set = true;
                    break;
                }
            }
            if x_set {
                break;
            }
        }
        x_start
    };

    let y: usize = {
        let mut y_set = false;
        let mut y_start = 0usize;
        for y in 0..placement_board.dim().1 {
            for x in 0..placement_board.dim().0 {
                if placement_board[[x, y]] {
                    y_start = y;
                    y_set = true;
                    break;
                }
            }
            if y_set {
                break;
            }
        }
        y_start
    };

    TilePlacement::new(tile.base().clone(), rotation.clone(), (x, y))
}
