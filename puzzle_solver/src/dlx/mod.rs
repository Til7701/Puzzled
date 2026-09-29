mod positioned;
mod pruner;

use crate::board::Board;
use crate::cancellation_token::CancellationToken;
use crate::dlx::positioned::PositionedTile;
use crate::dlx::pruner::Pruner;
use crate::result::{Solution, TilePlacement, UnsolvableReason};
use crate::tile::Tile;
use dlx_rs::Solver;
use log::debug;
use puzzled_common::polyform::Polyform;
use std::num::NonZero;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn solve_all_filling(
    board: &Board,
    tiles: &[Tile],
    cancel_token: CancellationToken,
) -> Result<Solution, UnsolvableReason> {
    if tiles.is_empty() {
        return Ok(Solution::new(vec![]));
    }
    let pruner = Pruner::new(board, tiles);
    let positioned_tiles: Vec<PositionedTile> = tiles
        .iter()
        .map(|tile| PositionedTile::new(tile, board, &pruner))
        .collect();
    debug!("Created positioned tiles");
    let index_with_most_options = positioned_tiles
        .iter()
        .enumerate()
        .max_by(|(_, t1), (_, t2)| t1.all_placements().len().cmp(&t2.all_placements().len()))
        .map(|(i, _)| i)
        .unwrap_or(0);

    let solver = prepare_solver(board, &positioned_tiles, index_with_most_options)?;
    debug!("Solver prepared");
    let tile_with_most_options = &positioned_tiles[index_with_most_options];
    let solution = run_multithreaded(
        solver,
        &cancel_token,
        positioned_tiles.len(),
        index_with_most_options,
        tile_with_most_options,
    );

    if solution.is_err() && cancel_token.is_cancelled() {
        return Err(UnsolvableReason::Cancelled);
    }
    Ok(create_solution(&tiles, &positioned_tiles, solution?))
}

/// Prepares the solver with all placements of all tiles except the tile with the most options
/// indicated by the `index_with_most_options` argument.
/// This also adds and selects the board.
fn prepare_solver(
    board: &Board,
    positioned_tiles: &[PositionedTile],
    index_with_most_options: usize,
) -> Result<Solver<Opt>, UnsolvableReason> {
    let option_count = board.get_polyform().area() + positioned_tiles.len();
    let mut solver = Solver::new(option_count);
    solver.add_option(
        Opt::Board,
        &shape_to_filled_indices(board.get_polyform(), positioned_tiles.len()),
    );

    for (tile_index, positioned_tile) in positioned_tiles.iter().enumerate() {
        if tile_index == index_with_most_options {
            continue;
        }
        add_placements(
            &mut solver,
            positioned_tile.all_placements(),
            tile_index,
            positioned_tiles.len(),
            0,
        );
    }

    solver
        .select(Opt::Board)
        .map_err(|_| UnsolvableReason::NoFit)?;
    Ok(solver)
}

fn run_multithreaded(
    solver: Solver<Opt>,
    cancel_token: &CancellationToken,
    max_tile_index: usize,
    index_with_most_options: usize,
    tile_with_most_options: &PositionedTile,
) -> Result<Vec<Opt>, UnsolvableReason> {
    let desired_parallelism = (thread::available_parallelism()
        .unwrap_or(NonZero::new(4).unwrap())
        .get() as f64
        * 0.8) as usize;
    debug!("Desired parallelism: {}", desired_parallelism);
    let chunk_size = (tile_with_most_options.all_placements().len() / desired_parallelism).max(5);
    debug!("Chunk size: {}", chunk_size);
    let chunks = tile_with_most_options.all_placements().chunks(chunk_size);
    debug!("Chunks: {:?}", chunks.len());
    let mut join_handles: Vec<JoinHandle<Result<Vec<Opt>, UnsolvableReason>>> =
        Vec::with_capacity(chunks.len());
    for (i, chunk) in chunks.into_iter().enumerate() {
        let join_handle = thread::spawn({
            let mut solver = solver.clone();
            let cancel_token = cancel_token.clone();
            let chunk = chunk.to_vec();
            let main_thread = thread::current();
            move || {
                add_placements(
                    &mut solver,
                    &chunk,
                    index_with_most_options,
                    max_tile_index,
                    chunk_size * i,
                );
                let result = solver
                    .solve_cancelable(&|| cancel_token.is_cancelled())
                    .ok_or(UnsolvableReason::NoFit);
                main_thread.unpark();
                result
            }
        });
        join_handles.push(join_handle);
    }

    while !join_handles.is_empty() {
        for finished_handle in join_handles.extract_if(.., |join_handle| join_handle.is_finished())
        {
            let result = finished_handle
                .join()
                .map_err(|_| UnsolvableReason::NoFit)
                .flatten();
            if let Ok(s) = result {
                return Ok(s);
            }
        }
        // park_timeout to avoid any shenanigans with the unpark being timed wrongly.
        thread::park_timeout(Duration::from_millis(200));
    }
    Err(UnsolvableReason::NoFit)
}

/// Adds the given placements to the solver.
/// The given placement shapes have to be a consecutive slice of the original placements
/// list for the tile with the given index. The position index offset is original index of the
/// first placement in the slice.
fn add_placements(
    solver: &mut Solver<Opt>,
    placements: &[Polyform<()>],
    tile_index: usize,
    max_tile_index: usize,
    position_index_offset: usize,
) {
    for (position_index, placement) in placements.iter().enumerate() {
        solver.add_option(
            Opt::Tile {
                tile_index,
                position_index: position_index_offset + position_index,
            },
            &tile_to_filled_indices(placement, tile_index, max_tile_index),
        );
    }
}

/// Creates a list of indices where the tile has cells and prepends the list with the index of
/// the tile. This list can be given to the DLX solver.
fn tile_to_filled_indices(
    tile: &Polyform<()>,
    tile_index: usize,
    max_tile_index: usize,
) -> Vec<usize> {
    let mut tile_indices = shape_to_filled_indices(tile, max_tile_index);
    tile_indices.insert(0, tile_index + 1);
    tile_indices
}

/// Creates a list of indices where the shape is true.
fn shape_to_filled_indices(shape: &Polyform<()>, index_offset: usize) -> Vec<usize> {
    shape
        .iter()
        .enumerate()
        .map(|(i, _)| index_offset + i + 1)
        .collect()
}

fn create_solution(
    tiles: &&[Tile],
    positioned_tiles: &[PositionedTile],
    solution: Vec<Opt>,
) -> Solution {
    Solution::new(
        solution
            .iter()
            .filter_map(|opt| {
                if let Opt::Tile {
                    tile_index,
                    position_index,
                } = opt
                {
                    let tile = &tiles[*tile_index];
                    let mut placed_tile =
                        positioned_tiles[*tile_index].all_placements()[*position_index].clone();
                    let trim = placed_tile.trim();
                    Some(TilePlacement::new(
                        tile.id(),
                        tile.base.clone(),
                        placed_tile,
                        trim.lower,
                    ))
                } else {
                    None
                }
            })
            .collect(),
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
