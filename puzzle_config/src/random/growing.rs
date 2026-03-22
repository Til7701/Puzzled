use crate::random::RandomPuzzleSettings;
use crate::{ColorConfig, TileConfig};
use log::debug;
use ndarray::Array2;
use puzzled_common::array_util;
use rand::{Rng, RngExt};
use semver::Op;
use std::thread::sleep;

pub fn create_puzzle(
    settings: &RandomPuzzleSettings,
    rng: &mut dyn Rng,
) -> (Array2<bool>, Vec<TileConfig>) {
    let base_board = generate_base_board(settings, rng);
    let complete_board = grow_until_complete(settings, rng, base_board);

    let board = complete_board.map(|x| x.is_some());
    let tiles = (0..settings.tile_count)
        .map(|i| extract_tile(i as u32, &complete_board))
        .collect();

    (board, tiles)
}

fn generate_base_board(settings: &RandomPuzzleSettings, rng: &mut dyn Rng) -> Array2<Option<u32>> {
    let tile_count = settings.tile_count;
    let size = (tile_count as f64 * 5.0).sqrt() as usize;
    debug!(
        "Generating base board of size {}x{} with {} tiles",
        size, size, tile_count
    );
    let mut base = Array2::from_elem((size, size), None::<u32>);

    // Place tile_count many entries randomly
    for i in 0..tile_count {
        loop {
            let x = rng.random_range(0..size);
            let y = rng.random_range(0..size);
            if base.get((x, y)).unwrap().is_none() {
                base[[x, y]] = Some(i as u32);
                break;
            }
        }
    }

    base
}

fn grow_until_complete(
    settings: &RandomPuzzleSettings,
    rng: &mut dyn Rng,
    mut base_board: Array2<Option<u32>>,
) -> Array2<Option<u32>> {
    while base_board.iter().any(|&x| x.is_none()) {
        base_board = grow(base_board);
    }
    base_board
}

fn grow(base_board: Array2<Option<u32>>) -> Array2<Option<u32>> {
    let mut new_board = base_board.clone();
    let xs = base_board.dim().0;
    let ys = base_board.dim().1;
    let mut just_placed = false;
    for x in 0..xs {
        for y in 0..ys {
            if just_placed || new_board[[x, y]].is_some() {
                just_placed = false;
                continue;
            }
            if x > 0 && base_board[[x - 1, y]].is_some() {
                new_board[[x, y]] = base_board[[x - 1, y]];
                just_placed = true;
            } else if x + 1 < xs && base_board[[x + 1, y]].is_some() {
                new_board[[x, y]] = base_board[[x + 1, y]];
                just_placed = true;
            } else if y > 0 && base_board[[x, y - 1]].is_some() {
                new_board[[x, y]] = base_board[[x, y - 1]];
                just_placed = true;
            } else if y + 1 < ys && base_board[[x, y + 1]].is_some() {
                new_board[[x, y]] = base_board[[x, y + 1]];
                just_placed = true;
            }
        }
    }
    new_board
}

fn extract_tile(tile_index: u32, complete_board: &Array2<Option<u32>>) -> TileConfig {
    let mut base = complete_board.map(|&x| x.filter(|&i| i == tile_index).is_none());
    array_util::remove_true_rows_cols_from_sides(&mut base);
    base = base.map(|x| !x);
    TileConfig::new(
        base,
        ColorConfig::default_with_index(tile_index as usize),
        None,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    #[test]
    fn test_grow() {
        let mut base_board = Array2::from_elem((3, 3), None::<u32>);
        base_board[[0, 0]] = Some(0);
        base_board = grow(base_board);
        assert_eq!(
            base_board,
            arr2(&[
                [Some(0), Some(0), None,],
                [Some(0), None, None,],
                [None, None, None,]
            ])
        );
    }
}
