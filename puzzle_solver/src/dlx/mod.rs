mod positioned;

use crate::board::Board;
use crate::cancellation_token::CancellationToken;
use crate::dlx::positioned::PositionedTile;
use crate::result::{Solution, TilePlacement, UnsolvableReason};
use crate::tile::Tile;
use dlx_rs::Solver;
use puzzled_common::Shape;

pub fn solve_all_filling(
    board: &Board,
    tiles: &[Tile],
    cancel_token: CancellationToken,
) -> Result<Solution, UnsolvableReason> {
    let positioned_tiles: Vec<PositionedTile> = tiles
        .iter()
        .map(|tile| PositionedTile::new(tile, board))
        .collect();
    let option_count = board.get_shape().len() + tiles.len();

    let mut solver = Solver::new(option_count);
    solver.add_option(
        Opt::Board,
        &shape_to_filled_indices(board.get_shape(), positioned_tiles.len()),
    );

    for (tile_index, positioned_tile) in positioned_tiles.iter().enumerate() {
        for (position_index, placement) in positioned_tile.all_placements().iter().enumerate() {
            let opt = Opt::Tile {
                tile_index,
                position_index,
            };
            let indices = tile_to_filled_indices(placement, tile_index, positioned_tiles.len());
            solver.add_option(opt, &indices);
        }
    }

    solver
        .select(Opt::Board)
        .map_err(|_| UnsolvableReason::NoFit)?;
    let solution = solver.solve_cancelable(&|| cancel_token.is_cancelled());
    if cancel_token.is_cancelled() {
        return Err(UnsolvableReason::Cancelled);
    }
    create_solution(&tiles, &positioned_tiles, solution)
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
    solution: Option<Vec<Opt>>,
) -> Result<Solution, UnsolvableReason> {
    match solution {
        None => Err(UnsolvableReason::NoFit),
        Some(s) => Ok(Solution::new(
            s.iter()
                .filter_map(|opt| {
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
                .collect(),
        )),
    }
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
