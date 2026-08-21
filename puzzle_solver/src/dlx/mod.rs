mod positioned;

use crate::board::Board;
use crate::dlx::positioned::PositionedTile;
use crate::result::{Solution, TilePlacement, UnsolvableReason};
use crate::tile::Tile;
use dlx_rs::Solver;
use puzzled_common::Shape;
use tokio_util::sync::CancellationToken;

pub async fn solve_all_filling(
    board: &Board,
    tiles: &[Tile],
    cancel_token: CancellationToken,
) -> Result<Solution, UnsolvableReason> {
    let positioned_tiles: Vec<PositionedTile> = tiles
        .iter()
        .map(|tile| PositionedTile::new(tile, &board))
        .collect();
    let option_count = board.get_shape().len() + tiles.len();

    let mut solver = Solver::new(option_count);
    solver.add_option(Opt::Board, &board.get_shape().iter().enumerate()
        .filter(|(_, v)| **v)
        .map(|(i, _)| positioned_tiles.len() + i + 1)
        .collect::<Vec<usize>>(),
    );

    for (tile_index, positioned_tile) in positioned_tiles.iter().enumerate() {
        for (position_index, placement) in positioned_tile.all_placements().iter().enumerate() {
            let opt = Opt::Tile {
                tile_index,
                position_index,
            };
            let indices = shape_to_filled_indices(placement, tile_index, positioned_tiles.len());
            solver.add_option(opt, &indices);
        }
    }

    solver.select(Opt::Board).map_err(|_| UnsolvableReason::NoFit)?;
    let solution = solver.solve();
    match solution {
        None => { Err(UnsolvableReason::NoFit) }
        Some(s) => {
            Ok(Solution::new(s.iter()
                .map(|opt| {
                    if let Opt::Tile { tile_index, position_index } = opt {
                        let tile = &tiles[*tile_index];
                        let mut placed_tile = positioned_tiles[*tile_index].all_placements()[*position_index].clone();
                        let trim = placed_tile.trim_matching(false);
                        Some(TilePlacement::new(
                            tile.base.clone(),
                            placed_tile,
                            (trim.lower_x, trim.lower_y),
                        ))
                    } else { None }
                })
                .flatten()
                .collect())
            )
        }
    }
}

fn shape_to_filled_indices(shape: &Shape, tile_index: usize, max_tile_index: usize) -> Vec<usize> {
    let mut vec = Vec::new();
    vec.push(tile_index + 1);
    vec.extend(
        shape.iter().enumerate()
            .filter(|(_, v)| **v)
            .map(|(i, _)| max_tile_index + i + 1)
    );
    vec
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Opt {
    Board,
    Tile {
        tile_index: usize,
        position_index: usize,
    },
}
