use crate::{
    BoardConfig, PreviewConfig, ProgressionConfig, PuzzleConfig, PuzzleConfigCollection, TileConfig,
};
use log::debug;
use puzzled_common::array_util;
use rand::rngs::Xoshiro256PlusPlus;
use rand::SeedableRng;

mod growing;
mod random_placement;

pub struct RandomPuzzleSettings<'a> {
    pub seed: u64,
    pub tile_count: usize,
    pub tiles: &'a [TileConfig],
    pub algorithm: Algorithm,
}

pub enum Algorithm {
    RandomPlacement,
    Growing,
}

/// Returns a collection containing exactly one puzzle which was generated.
pub fn random_puzzle(settings: &RandomPuzzleSettings) -> PuzzleConfigCollection {
    debug!(
        "Generating random puzzle with seed {} and {} tiles",
        settings.seed, settings.tile_count
    );
    let mut rng = Xoshiro256PlusPlus::seed_from_u64(settings.seed);

    let (board_layout, tiles) = match settings.algorithm {
        Algorithm::RandomPlacement => random_placement::create_puzzle(settings, &mut rng),
        Algorithm::Growing => growing::create_puzzle(settings, &mut rng),
    };

    debug!("Board:");
    array_util::debug_print(&board_layout);

    debug!("Tiles:");
    for tile in &tiles {
        array_util::debug_print(tile.base());
    }

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
