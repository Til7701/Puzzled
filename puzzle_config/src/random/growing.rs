use crate::random::{Algorithm, RandomPuzzleSettings};
use ndarray::Array2;
use puzzled_common::array_util;
use rand::{Rng, RngExt};
use std::collections::HashMap;

pub fn create_puzzle(
    settings: &RandomPuzzleSettings,
    rng: &mut dyn Rng,
) -> (Array2<bool>, Vec<Array2<bool>>) {
    let tile_count = match settings.algorithm {
        Algorithm::Growing { tile_count, .. } => tile_count,
    };
    let base_board = generate_base_board(settings, rng);
    let complete_board = grow_until_complete(rng, base_board);

    let board = complete_board.map(|x| x.is_some());
    let tiles = (0..tile_count)
        .map(|i| extract_tile(i as u32, &complete_board))
        .collect();

    (board, tiles)
}

fn generate_base_board(settings: &RandomPuzzleSettings, rng: &mut dyn Rng) -> Array2<Option<u32>> {
    let (tile_count, board_width, board_height) = match settings.algorithm {
        Algorithm::Growing {
            tile_count,
            board_width,
            board_height,
        } => (tile_count, board_width, board_height),
    };
    let mut base = Array2::from_elem((board_width, board_height), None::<u32>);

    for i in 0..tile_count {
        loop {
            let x = rng.random_range(0..board_width);
            let y = rng.random_range(0..board_height);
            if base.get((x, y)).unwrap().is_none() {
                base[[x, y]] = Some(i as u32);
                break;
            }
        }
        if base.iter().all(|&x| x.is_some()) {
            break;
        }
    }

    base
}

fn grow_until_complete(
    rng: &mut dyn Rng,
    mut base_board: Array2<Option<u32>>,
) -> Array2<Option<u32>> {
    while base_board.iter().any(|&x| x.is_none()) {
        base_board = grow(rng, base_board);
    }
    base_board
}

fn grow(rng: &mut dyn Rng, base_board: Array2<Option<u32>>) -> Array2<Option<u32>> {
    let mut new_board = base_board.clone();

    let tile_indices = tile_indices_sorted_by_size(&base_board);
    for index in tile_indices {
        let (changed, b) = grow_tile_index(rng, base_board.clone(), index);
        new_board = b;
        if changed {
            break;
        }
    }

    new_board
}

fn tile_indices_sorted_by_size(board: &Array2<Option<u32>>) -> Vec<u32> {
    let map = board
        .iter()
        .filter_map(|&x| x)
        .fold(HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });
    let mut indices_with_count: Vec<(u32, u32)> = map.into_iter().map(|(k, v)| (k, v)).collect();
    indices_with_count.sort_by(|a, b| a.1.cmp(&b.1));
    indices_with_count.into_iter().map(|x| x.0).collect()
}

fn grow_tile_index(
    rng: &mut dyn Rng,
    base_board: Array2<Option<u32>>,
    tile_index: u32,
) -> (bool, Array2<Option<u32>>) {
    let mut new_board = base_board.clone();
    let xs = base_board.dim().0;
    let ys = base_board.dim().1;
    let mut changed = false;

    for _ in 0..100 {
        let x = rng.random_range(0..xs);
        let y = rng.random_range(0..ys);

        if let Some(Some(index)) = base_board.get((x, y))
            && *index == tile_index
        {
            if x > 0 && base_board[[x - 1, y]].is_none() {
                new_board[[x - 1, y]] = base_board[[x, y]];
                changed = true;
                break;
            } else if x + 1 < xs && base_board[[x + 1, y]].is_none() {
                new_board[[x + 1, y]] = base_board[[x, y]];
                changed = true;
                break;
            } else if y > 0 && base_board[[x, y - 1]].is_none() {
                new_board[[x, y - 1]] = base_board[[x, y]];
                changed = true;
                break;
            } else if y + 1 < ys && base_board[[x, y + 1]].is_none() {
                new_board[[x, y + 1]] = base_board[[x, y]];
                changed = true;
                break;
            }
        }
    }

    (changed, new_board)
}

fn extract_tile(tile_index: u32, complete_board: &Array2<Option<u32>>) -> Array2<bool> {
    let mut base = complete_board.map(|&x| x.filter(|&i| i == tile_index).is_none());
    array_util::remove_true_rows_cols_from_sides(&mut base);
    base.map(|x| !x)
}
