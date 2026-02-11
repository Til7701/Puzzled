use ndarray::Array2;

/// Configuration for a tile that can be placed on the board.
#[derive(Debug, Clone)]
pub struct TileConfig {
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

    /// Base shape of the tile as a 2D boolean array.
    /// True indicates a filled cell, false indicates an empty cell.
    pub fn base(&self) -> &Array2<bool> {
        &self.base
    }
}
