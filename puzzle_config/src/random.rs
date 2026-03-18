use crate::{
    BoardConfig, PreviewConfig, ProgressionConfig, PuzzleConfig, PuzzleConfigCollection, TileConfig,
};
use ndarray::Array2;
use puzzled_common::array_util;
use rand::prelude::IndexedRandom;
use rand::rngs::Xoshiro256PlusPlus;
use rand::{Rng, RngExt, SeedableRng};

pub struct RandomPuzzleSettings<'a> {
    pub seed: u64,
    pub tile_count: usize,
    pub tiles: &'a [TileConfig],
}

/// Returns a collection containing exactly one puzzle which was generated.
pub fn random_puzzle(settings: &RandomPuzzleSettings) -> PuzzleConfigCollection {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(settings.seed);
    let (board_layout, tiles) = create_puzzle(settings, &mut rng);
    let board = BoardConfig::Simple {
        layout: board_layout,
    };
    let puzzle = PuzzleConfig::new(
        0,
        "r".to_string(),
        "Random puzzle".to_string(),
        None,
        None,
        false,
        tiles.to_vec(),
        board,
        None,
    );
    PuzzleConfigCollection::new(
        "Random".to_string(),
        None,
        "Puzzled".to_string(),
        "de.til7701.Puzzled.Random".to_string(),
        None,
        ProgressionConfig::Any,
        PreviewConfig::default(),
        vec![puzzle],
    )
}

fn create_puzzle(
    settings: &RandomPuzzleSettings,
    rng: &mut dyn Rng,
) -> (Array2<bool>, Vec<TileConfig>) {
    let mut board = Array2::default((5, 5));
    let mut tiles = Vec::with_capacity(settings.tile_count);

    for _ in 0..settings.tile_count {
        let (tile, new_board) = place_any_tile(&mut board, settings.tiles, rng);
        board = new_board;
        tiles.push(tile);
    }

    (board, tiles)
}

fn place_any_tile(
    board: &Array2<bool>,
    tiles: &[TileConfig],
    rng: &mut dyn Rng,
) -> (TileConfig, Array2<bool>) {
    loop {
        let dim = board.dim();
        let x = rng.random_range(0..dim.0);
        let y = rng.random_range(0..dim.1);

        if board[[x, y]] == true {
            continue;
        }

        if let Some((tile, rotated_base)) = try_place_any_tile_at(board, tiles, x, y, rng) {
            let board = array_util::or_arrays_at(board, &rotated_base, x as isize, y as isize);
            return (tile, board);
        }
    }
}

fn try_place_any_tile_at(
    board: &Array2<bool>,
    tiles: &[TileConfig],
    x: usize,
    y: usize,
    rng: &mut dyn Rng,
) -> Option<(TileConfig, Array2<bool>)> {
    for _ in 0..tiles.len() {
        let tile = tiles.choose(rng).unwrap();
        let rotation_iter = TileRotationIterator::new(tile.base().clone());
        for rotated in rotation_iter {
            if can_tile_be_placed(board, &rotated, x as isize, y as isize) {
                return Some((tile.clone(), rotated));
            }
        }
    }
    None
}

fn can_tile_be_placed(
    board: &Array2<bool>,
    tile: &Array2<bool>,
    x_offset: isize,
    y_offset: isize,
) -> bool {
    let child_xs = tile.nrows();
    let child_ys = tile.ncols();

    for x in 0..child_xs {
        for y in 0..child_ys {
            let parent_x = x as isize + x_offset;
            let parent_y = y as isize + y_offset;
            if parent_x >= 0
                && parent_x < board.nrows() as isize
                && parent_y >= 0
                && parent_y < board.ncols() as isize
                && board[[parent_x as usize, parent_y as usize]] == true
                && tile[[x, y]] == true
            {
                return false;
            }
        }
    }
    true
}

struct TileRotationIterator {
    current: Array2<bool>,
    iteration: u8,
}

impl<'a> TileRotationIterator {
    pub fn new(tile: Array2<bool>) -> Self {
        Self {
            current: tile,
            iteration: 0,
        }
    }
}

impl Iterator for TileRotationIterator {
    type Item = Array2<bool>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.iteration >= 8 {
            return None;
        }
        if self.iteration == 4 {
            self.current = self.current.clone().reversed_axes();
        }
        let current = self.current.clone();
        let rotated = array_util::rotate_90(&self.current);
        self.current = rotated;
        self.iteration += 1;
        Some(current)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    #[test]
    fn test_tile_rotation_iterator() {
        let base = arr2(&[[true, false], [false, false]]);
        let mut iter = TileRotationIterator::new(base);

        assert_eq!(iter.next(), Some(arr2(&[[true, false], [false, false]])));
        assert_eq!(iter.next(), Some(arr2(&[[false, true], [false, false]])));
        assert_eq!(iter.next(), Some(arr2(&[[false, false], [false, true]])));
        assert_eq!(iter.next(), Some(arr2(&[[false, false], [true, false]])));

        assert_eq!(iter.next(), Some(arr2(&[[true, false], [false, false]])));
        assert_eq!(iter.next(), Some(arr2(&[[false, true], [false, false]])));
        assert_eq!(iter.next(), Some(arr2(&[[false, false], [false, true]])));
        assert_eq!(iter.next(), Some(arr2(&[[false, false], [true, false]])));
        assert_eq!(iter.next(), None);
    }
}
