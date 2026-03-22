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
