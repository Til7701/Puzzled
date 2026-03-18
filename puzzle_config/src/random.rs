use crate::{
    BoardConfig, ColorConfig, PreviewConfig, ProgressionConfig, PuzzleConfig,
    PuzzleConfigCollection, TileConfig,
};
use log::debug;
use ndarray::Array2;
use puzzled_common::array_util;
use rand::prelude::{IndexedRandom, SliceRandom};
use rand::rngs::Xoshiro256PlusPlus;
use rand::{Rng, RngExt, SeedableRng};

pub struct RandomPuzzleSettings<'a> {
    pub seed: u64,
    pub tile_count: usize,
    pub tiles: &'a [TileConfig],
}

/// Returns a collection containing exactly one puzzle which was generated.
pub fn random_puzzle(settings: &RandomPuzzleSettings) -> PuzzleConfigCollection {
    debug!(
        "Generating random puzzle with seed {} and {} tiles",
        settings.seed, settings.tile_count
    );
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
    let mut current_tiles = Vec::new();
    let mut tiles = Vec::with_capacity(settings.tile_count);
    let mut blocked_positions = Vec::new();

    for i in 0..settings.tile_count {
        let tile_placement = place_any_tile(&blocked_positions, settings.tiles, rng);

        tile_placement
            .base
            .indexed_iter()
            .for_each(|((x, y), value)| {
                if *value {
                    blocked_positions.push((tile_placement.x + x, tile_placement.y + y));
                }
            });

        tiles.push(TileConfig::new(
            tile_placement.base.clone(),
            ColorConfig::default_with_index(i),
            None,
        ));
        current_tiles.push(tile_placement);
    }

    let dim = bounding_box_for_tiles(&blocked_positions);
    let mut board = Array2::from_elem(dim, false);
    for tile in current_tiles {
        for ((x, y), value) in tile.base.indexed_iter() {
            if *value {
                board[[tile.x + x, tile.y + y]] = true;
            }
        }
    }

    (board, tiles)
}

struct TilePlacement {
    base: Array2<bool>,
    x: usize,
    y: usize,
}

fn place_any_tile(
    blocked_positions: &[(usize, usize)],
    tiles: &[TileConfig],
    rng: &mut dyn Rng,
) -> TilePlacement {
    loop {
        let dim = bounding_box_for_tiles(blocked_positions);
        let x = rng.random_range(0..(dim.0 + 1));
        let y = rng.random_range(0..(dim.1 + 1));

        if blocked_positions.contains(&(x, y)) {
            debug!(
                "Cannot place at ({}, {}) because it's already occupied",
                x, y
            );
            continue;
        }

        if let Some(rotated_base) = try_place_any_tile_at(blocked_positions, tiles, x, y, rng) {
            return TilePlacement {
                base: rotated_base,
                x,
                y,
            };
        }
    }
}

fn try_place_any_tile_at(
    blocked_positions: &[(usize, usize)],
    tiles: &[TileConfig],
    x: usize,
    y: usize,
    rng: &mut dyn Rng,
) -> Option<Array2<bool>> {
    for _ in 0..tiles.len() {
        let tile = tiles.choose(rng).unwrap();
        let mut rotations: Vec<Array2<bool>> =
            TileRotationIterator::new(tile.base().clone()).collect();
        rotations.shuffle(rng);
        for rotated in rotations {
            if can_tile_be_placed(blocked_positions, &rotated, x, y) {
                return Some(rotated);
            }
        }
    }
    None
}

fn can_tile_be_placed(
    blocked_positions: &[(usize, usize)],
    new_tile: &Array2<bool>,
    x_offset: usize,
    y_offset: usize,
) -> bool {
    !new_tile.indexed_iter().any(|((new_x, new_y), value)| {
        if !value {
            return false;
        }
        let new_tile_board_x = x_offset + new_x;
        let new_tile_board_y = y_offset + new_y;
        blocked_positions.contains(&(new_tile_board_x, new_tile_board_y))
    })
}

fn bounding_box_for_tiles(blocked_positions: &[(usize, usize)]) -> (usize, usize) {
    let max_x = blocked_positions.iter().map(|(x, _)| *x).max().unwrap_or(0);
    let max_y = blocked_positions.iter().map(|(_, y)| *y).max().unwrap_or(0);

    (max_x + 1, max_y + 1)
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
