use crate::random::{Algorithm, RandomPuzzleSettings};
use puzzled_common::polyform::Polyform;
use puzzled_common::polyform::grid::{Coord, RegularCoord};
use rand::{Rng, RngExt};
use std::collections::BTreeMap;

pub fn create_puzzle(
    settings: &RandomPuzzleSettings,
    rng: &mut dyn Rng,
) -> (Polyform<()>, Vec<Polyform<()>>) {
    let tile_count = match settings.algorithm {
        Algorithm::Growing { tile_count, .. } => tile_count,
    };
    let base_board = generate_base_board(settings, rng);
    let mut complete_board = grow_until_complete(rng, base_board);

    let tiles = (0..tile_count)
        .map(|i| extract_tile(i as u32, &complete_board))
        .collect();
    let board = complete_board.map(|_| ());

    (board, tiles)
}

fn generate_base_board(
    settings: &RandomPuzzleSettings,
    rng: &mut dyn Rng,
) -> Polyform<Option<u32>> {
    let (tile_count, board_width, board_height) = match settings.algorithm {
        Algorithm::Growing {
            tile_count,
            board_width,
            board_height,
        } => (tile_count, board_width, board_height),
    };
    let mut base = Polyform::polyomino_sized(
        RegularCoord::new(board_width as u32, board_height as u32),
        None::<u32>,
    );

    for i in 0..tile_count {
        loop {
            let x = rng.random_range(0..board_width);
            let y = rng.random_range(0..board_height);
            if let Some(mut prototile) = base.get_mut(&RegularCoord::new(x as u32, y as u32).into())
                && prototile.data().is_none()
            {
                prototile.set_data(Some(i as u32));
                break;
            }
        }
        if base.iter().all(|x| x.data().is_some()) {
            break;
        }
    }

    base
}

fn grow_until_complete(
    rng: &mut dyn Rng,
    mut base_board: Polyform<Option<u32>>,
) -> Polyform<Option<u32>> {
    while base_board.iter().any(|x| x.data().is_none()) {
        base_board = grow(rng, base_board);
    }
    base_board
}

fn grow(rng: &mut dyn Rng, base_board: Polyform<Option<u32>>) -> Polyform<Option<u32>> {
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

fn tile_indices_sorted_by_size(board: &Polyform<Option<u32>>) -> Vec<u32> {
    let map = board
        .iter()
        .filter_map(|x| *x.data())
        .fold(BTreeMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });
    let mut indices_with_count: Vec<(u32, u32)> = map.into_iter().collect();
    indices_with_count.sort_by_key(|a| a.1);
    indices_with_count.into_iter().map(|x| x.0).collect()
}

fn grow_tile_index(
    rng: &mut dyn Rng,
    base_board: Polyform<Option<u32>>,
    tile_index: u32,
) -> (bool, Polyform<Option<u32>>) {
    let mut new_board = base_board.clone();
    let dim = match base_board.dim() {
        Coord::Regular(dim) => dim,
        _ => unreachable!(),
    };
    let xs = dim.x();
    let ys = dim.y();
    let mut changed = false;

    let new_cell_data = Some(tile_index);
    for _ in 0..100 {
        let x = rng.random_range(0..xs);
        let y = rng.random_range(0..ys);

        if let Some(prototile) = base_board.get(&RegularCoord::new(x, y).into())
            && let Some(index) = prototile.data()
            && *index == tile_index
        {
            if x > 0
                && base_board
                    .get(&RegularCoord::new(x - 1, y).into())
                    .unwrap()
                    .data()
                    .is_none()
            {
                new_board
                    .get_mut(&RegularCoord::new(x - 1, y).into())
                    .unwrap()
                    .set_data(new_cell_data.clone());
                changed = true;
                break;
            } else if x + 1 < xs
                && base_board
                    .get(&RegularCoord::new(x + 1, y).into())
                    .unwrap()
                    .data()
                    .is_none()
            {
                new_board
                    .get_mut(&RegularCoord::new(x + 1, y).into())
                    .unwrap()
                    .set_data(new_cell_data.clone());
                changed = true;
                break;
            } else if y > 0
                && base_board
                    .get(&RegularCoord::new(x, y - 1).into())
                    .unwrap()
                    .data()
                    .is_none()
            {
                new_board
                    .get_mut(&RegularCoord::new(x, y - 1).into())
                    .unwrap()
                    .set_data(new_cell_data.clone());
                changed = true;
                break;
            } else if y + 1 < ys
                && base_board
                    .get(&RegularCoord::new(x, y + 1).into())
                    .unwrap()
                    .data()
                    .is_none()
            {
                new_board
                    .get_mut(&RegularCoord::new(x, y + 1).into())
                    .unwrap()
                    .set_data(new_cell_data.clone());
                changed = true;
                break;
            }
        }
    }

    (changed, new_board)
}

fn extract_tile(tile_index: u32, complete_board: &Polyform<Option<u32>>) -> Polyform<()> {
    let mut base = complete_board
        .clone()
        .filter_map(&|x| x.filter(|i| *i == tile_index));
    base.trim();
    base.map(|_| ())
}
