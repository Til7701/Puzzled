mod positioned;
mod pruner;

use crate::board::Board;
use crate::dlx::positioned::PositionedTile;
use crate::dlx::pruner::Pruner;
use crate::result::{Solution, TilePlacement, UnsolvableReason};
use crate::tile::Tile;
use dlx_rs::Solver;
use puzzled_common::Shape;
use std::num::NonZero;
use std::thread;
use tokio_util::sync::CancellationToken;

pub async fn solve_all_filling(
    board: &Board,
    tiles: &[Tile],
    cancel_token: CancellationToken,
) -> Result<Solution, UnsolvableReason> {
    let pruner = Pruner::new(board, tiles);
    let positioned_tiles: Vec<PositionedTile> = tiles
        .iter()
        .map(|tile| PositionedTile::new(tile, &board, &pruner))
        .collect();
    let option_count = board.get_shape().len() + tiles.len();
    let index_with_most_options = positioned_tiles.iter().enumerate()
        .max_by(|(_, t1), (_, t2)| t1.all_placements().len().cmp(&t2.all_placements().len()))
        .map(|(i, _)| i)
        .unwrap_or(0);

    let mut solver = Solver::new(option_count);
    solver.add_option(
        Opt::Board,
        &shape_to_filled_indices(board.get_shape(), positioned_tiles.len()),
    );

    for (tile_index, positioned_tile) in positioned_tiles.iter().enumerate() {
        if tile_index == index_with_most_options {
            continue;
        }
        for (position_index, placement) in positioned_tile.all_placements().iter().enumerate() {
            solver.add_option(Opt::Tile {
                tile_index,
                position_index,
            }, &tile_to_filled_indices(placement, tile_index, positioned_tiles.len()));
        }
    }

    solver
        .select(Opt::Board)
        .map_err(|_| UnsolvableReason::NoFit)?;

    let tile_with_most_options = &positioned_tiles[index_with_most_options];
    let parallelism = thread::available_parallelism().unwrap_or(NonZero::new(4).unwrap());
    let mut join_handles = Vec::with_capacity(parallelism.get());
    for i in parallelism {
        let opt = Opt::Tile {
            tile_index: index_with_most_options,
            position_index,
        };
        let indices = tile_to_filled_indices(placement, index_with_most_options, positioned_tiles.len());
        let join_handle = thread::spawn({
            let mut solver = solver.clone();
            let cancel_token = cancel_token.clone();
            move || {
                solver.add_option(opt, &indices);
                solver.select(opt).map_err(|_| UnsolvableReason::NoFit)?;
                solver.solve_cancelable(&|| cancel_token.is_cancelled()).ok_or(UnsolvableReason::NoFit)
            }
        });
        join_handles.push(join_handle);
    }

    let mut solution: Result<Vec<Opt>, UnsolvableReason> = Err(UnsolvableReason::NoFit);
    for join_handle in join_handles {
        let result = join_handle.join().map_err(|_| UnsolvableReason::NoFit);
        if result.is_ok() {
            solution = result?;
            break;
        }
    }

    if cancel_token.is_cancelled() {
        return Err(UnsolvableReason::Cancelled);
    }
    Ok(create_solution(&tiles, &positioned_tiles, solution?))
}

fn shape_to_filled_indices(shape: &Shape, index_offset: usize) -> Vec<usize> {
    shape
        .iter()
        .enumerate()
        .filter(|(_, v)| **v)
        .map(|(i, _)| index_offset + i + 1)
        .collect()
}

fn tile_to_filled_indices(tile: &Shape, tile_index: usize, max_tile_index: usize) -> Vec<usize> {
    let mut tile_indices = shape_to_filled_indices(tile, max_tile_index);
    tile_indices.push(tile_index + 1);
    tile_indices
}

fn create_solution(
    tiles: &&[Tile],
    positioned_tiles: &[PositionedTile],
    solution: Vec<Opt>,
) -> Solution {
    Solution::new(
        solution.iter()
            .map(|opt| {
                if let Opt::Tile {
                    tile_index,
                    position_index,
                } = opt
                {
                    let tile = &tiles[*tile_index];
                    let mut placed_tile =
                        positioned_tiles[*tile_index].all_placements()[*position_index].clone();
                    let trim = placed_tile.trim_matching(false);
                    Some(TilePlacement::new(
                        tile.base.clone(),
                        placed_tile,
                        (trim.lower_x, trim.lower_y),
                    ))
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    )
}

/// Identifies an option given to the solver to choose.
///
/// The `Board` variant must only be used once for the board.
/// The `Tile` variant identifies, which tile this option corresponds to and its positioned tile
/// index to identify where it is placed and how it is rotated.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Opt {
    Board,
    Tile {
        tile_index: usize,
        position_index: usize,
    },
}
