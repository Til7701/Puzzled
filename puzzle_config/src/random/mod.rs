use crate::{
    BoardConfig, ColorConfig, PreviewConfig, ProgressionConfig, PuzzleConfig,
    PuzzleConfigCollection, TileConfig,
};
use ndarray::Array2;
use puzzled_common::array_util::TileRotationIterator;
use rand::rngs::Xoshiro256PlusPlus;
use rand::{Rng, RngExt, SeedableRng};

mod growing;

pub struct RandomPuzzleSettings {
    pub seed: u64,
    pub algorithm: Algorithm,
}

pub enum Algorithm {
    Growing {
        tile_count: usize,
        board_width: usize,
        board_height: usize,
    },
}

/// Returns a collection containing exactly one puzzle which was generated.
pub fn random_puzzle(settings: &RandomPuzzleSettings) -> PuzzleConfigCollection {
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(settings.seed);

    let (board_layout, tiles) = match settings.algorithm {
        Algorithm::Growing { .. } => growing::create_puzzle(settings, &mut rng),
    };

    let board = BoardConfig::Simple {
        layout: board_layout,
    };
    let tiles = tiles
        .iter()
        .enumerate()
        .map(|(i, tile)| {
            let base = random_orientation(&mut rng, tile.clone());
            TileConfig::new(base, ColorConfig::default_with_index(i), None)
        })
        .collect();
    let puzzle = PuzzleConfig::new(
        0,
        "r".to_string(),
        "Random puzzle".to_string(),
        None,
        None,
        false,
        tiles,
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

fn random_orientation(rng: &mut dyn Rng, array: Array2<bool>) -> Array2<bool> {
    let iterator = TileRotationIterator::new(array);
    iterator.into_iter().nth(rng.random_range(0..8)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    #[test]
    fn test_normal() {
        let settings = RandomPuzzleSettings {
            seed: 42,
            algorithm: Algorithm::Growing {
                tile_count: 5,
                board_width: 6,
                board_height: 5,
            },
        };
        let collection = random_puzzle(&settings);

        assert_eq!(1, collection.puzzles().len());
        let puzzle = &collection.puzzles()[0];
        assert_eq!(
            BoardConfig::Simple {
                layout: arr2(&[
                    [true, true, true, true, true],
                    [true, true, true, true, true],
                    [true, true, true, true, true],
                    [true, true, true, true, true],
                    [true, true, true, true, true],
                    [true, true, true, true, true],
                ])
            },
            *puzzle.board_config()
        );
        assert_eq!(5, puzzle.tiles().len());
        assert_eq!(
            TileConfig::new(
                arr2(&[
                    [true, true, true, true, true],
                    [true, true, true, true, false]
                ]),
                ColorConfig::default_with_index(0),
                None
            ),
            puzzle.tiles()[0]
        );
        assert_eq!(
            TileConfig::new(
                arr2(&[
                    [false, false, true],
                    [true, true, true],
                    [false, false, true],
                    [false, false, true],
                    [false, true, true]
                ]),
                ColorConfig::default_with_index(1),
                None
            ),
            puzzle.tiles()[1]
        );
        assert_eq!(
            TileConfig::new(
                arr2(&[[true, true, true, true]]),
                ColorConfig::default_with_index(2),
                None
            ),
            puzzle.tiles()[2]
        );
        assert_eq!(
            TileConfig::new(
                arr2(&[[false, false, true], [true, true, true]]),
                ColorConfig::default_with_index(3),
                None
            ),
            puzzle.tiles()[3]
        );
        assert_eq!(
            TileConfig::new(
                arr2(&[[true], [true], [true], [true], [true]]),
                ColorConfig::default_with_index(4),
                None
            ),
            puzzle.tiles()[4]
        );
    }

    #[test]
    fn test_small() {
        let settings = RandomPuzzleSettings {
            seed: 349587345923,
            algorithm: Algorithm::Growing {
                tile_count: 5,
                board_width: 2,
                board_height: 2,
            },
        };
        let collection = random_puzzle(&settings);

        assert_eq!(1, collection.puzzles().len());
        let puzzle = &collection.puzzles()[0];
        assert_eq!(
            BoardConfig::Simple {
                layout: arr2(&[[true, true], [true, true],])
            },
            *puzzle.board_config()
        );
        assert_eq!(4, puzzle.tiles().len());
        assert_eq!(
            TileConfig::new(arr2(&[[true]]), ColorConfig::default_with_index(0), None),
            puzzle.tiles()[0]
        );
        assert_eq!(
            TileConfig::new(arr2(&[[true]]), ColorConfig::default_with_index(1), None),
            puzzle.tiles()[1]
        );
        assert_eq!(
            TileConfig::new(arr2(&[[true]]), ColorConfig::default_with_index(2), None),
            puzzle.tiles()[2]
        );
        assert_eq!(
            TileConfig::new(arr2(&[[true]]), ColorConfig::default_with_index(3), None),
            puzzle.tiles()[3]
        );
    }
}
