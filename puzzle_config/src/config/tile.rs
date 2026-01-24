use ndarray::{arr2, Array2};

/// Configuration for a tile that can be placed on the board.
#[derive(Debug, Clone)]
pub struct TileConfig {
    /// Base shape of the tile as a 2D boolean array.
    /// True indicates a filled cell, false indicates an empty cell.
    base: Array2<bool>,
}

impl TileConfig {
    /// Creates a new TileConfig.
    ///
    /// # Arguments
    ///
    /// * `base`: Base shape of the tile as a 2D boolean array.
    ///
    /// returns: TileConfig
    pub fn new(base: Array2<bool>) -> TileConfig {
        TileConfig { base }
    }

    pub fn base(&self) -> &Array2<bool> {
        &self.base
    }
}

pub fn from_predefined_tile(name: &str) -> Option<TileConfig> {
    match name {
        // https://en.wikipedia.org/wiki/Domino_(mathematics)
        "D2" => Some(TileConfig::new(arr2(&[[true, true]]).reversed_axes())),

        // https://en.wikipedia.org/wiki/Tromino
        "I3" => Some(TileConfig::new(arr2(&[[true, true, true]]).reversed_axes())),
        "L3" => Some(TileConfig::new(
            arr2(&[[true, false], [true, true]]).reversed_axes(),
        )),

        // https://en.wikipedia.org/wiki/Tetromino
        "I4" => Some(TileConfig::new(
            arr2(&[[true, true, true, true]]).reversed_axes(),
        )),
        "O4" => Some(TileConfig::new(
            arr2(&[[true, true], [true, true]]).reversed_axes(),
        )),
        "T4" => Some(TileConfig::new(arr2(&[
            [false, true, false],
            [true, true, true],
        ]))),
        "J4" => Some(TileConfig::new(
            arr2(&[[true, false, false], [true, true, true]]).reversed_axes(),
        )),
        "L4" => Some(TileConfig::new(
            arr2(&[[false, false, true], [true, true, true]]).reversed_axes(),
        )),
        "S4" => Some(TileConfig::new(
            arr2(&[[false, true, true], [true, true, false]]).reversed_axes(),
        )),
        "Z4" => Some(TileConfig::new(
            arr2(&[[true, true, false], [false, true, true]]).reversed_axes(),
        )),

        // https://en.wikipedia.org/wiki/Pentomino
        "F5" => Some(TileConfig::new(
            arr2(&[
                [false, true, true],
                [true, true, false],
                [false, true, false],
            ])
            .reversed_axes(),
        )),
        "I5" => Some(TileConfig::new(
            arr2(&[[true, true, true, true, true]]).reversed_axes(),
        )),
        "L5" => Some(TileConfig::new(
            arr2(&[[true, false, false, false], [true, true, true, true]]).reversed_axes(),
        )),
        "N5" => Some(TileConfig::new(
            arr2(&[[true, true, false, false], [false, true, true, true]]).reversed_axes(),
        )),
        "P5" => Some(TileConfig::new(
            arr2(&[[true, true, false], [true, true, true]]).reversed_axes(),
        )),
        "T5" => Some(TileConfig::new(
            arr2(&[
                [true, true, true],
                [false, true, false],
                [false, true, false],
            ])
            .reversed_axes(),
        )),
        "U5" => Some(TileConfig::new(
            arr2(&[[true, false, true], [true, true, true]]).reversed_axes(),
        )),
        "V5" => Some(TileConfig::new(
            arr2(&[
                [true, false, false],
                [true, false, false],
                [true, true, true],
            ])
            .reversed_axes(),
        )),
        "W5" => Some(TileConfig::new(
            arr2(&[
                [true, false, false],
                [true, true, false],
                [false, true, true],
            ])
            .reversed_axes(),
        )),
        "X5" => Some(TileConfig::new(
            arr2(&[
                [false, true, false],
                [true, true, true],
                [false, true, false],
            ])
            .reversed_axes(),
        )),
        "Y5" => Some(TileConfig::new(
            arr2(&[[false, true, false, false], [true, true, true, true]]).reversed_axes(),
        )),
        "Z5" => Some(TileConfig::new(
            arr2(&[
                [true, true, false],
                [false, true, false],
                [false, true, true],
            ])
            .reversed_axes(),
        )),

        _ => None,
    }
}
