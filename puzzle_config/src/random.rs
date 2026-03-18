use crate::{
    BoardConfig, PreviewConfig, ProgressionConfig, PuzzleConfig, PuzzleConfigCollection, TileConfig,
};
use ndarray::Array2;

pub struct RandomPuzzleSettings<'a> {
    pub seed: u64,
    pub tiles: &'a [TileConfig],
}

/// Returns a collection containing exactly one puzzle which was generated.
pub fn random_puzzle(settings: &RandomPuzzleSettings) -> PuzzleConfigCollection {
    let mut array = Array2::default((5, 5));
    array[[0, 0]] = true;
    let board = BoardConfig::Simple { layout: array };
    let tiles = settings.tiles;
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
