use crate::config::color::ColorConfig;
use ndarray::Array2;
use std::hash::{DefaultHasher, Hash, Hasher};

/// Configuration for a tile that can be placed on the board.
#[derive(Debug, Clone)]
pub struct TileConfig {
    base: Array2<bool>,
    color: ColorConfig,
}

impl TileConfig {
    /// Creates a new TileConfig.
    ///
    /// # Arguments
    ///
    /// * `base`: Base shape of the tile as a 2D boolean array.
    ///
    /// returns: TileConfig
    pub fn new(base: Array2<bool>, color: ColorConfig) -> TileConfig {
        TileConfig { base, color }
    }

    /// Base shape of the tile as a 2D boolean array.
    /// True indicates a filled cell, false indicates an empty cell.
    pub fn base(&self) -> &Array2<bool> {
        &self.base
    }

    /// Color of the tile.
    pub fn color(&self) -> ColorConfig {
        self.color
    }
}

impl Hash for TileConfig {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base.hash(state);
    }

    fn hash_slice<H: Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        let mut hashes: Vec<u64> = data
            .iter()
            .map(|item| {
                let mut hasher = DefaultHasher::new();
                item.hash(&mut hasher);
                hasher.finish()
            })
            .collect();

        // Sort the hashes to ensure order-independence
        hashes.sort_unstable();

        for hash in hashes {
            hash.hash(state);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::array;

    #[test]
    fn test_hash() {
        let tile1 = TileConfig::new(
            array![[true, false], [false, true]],
            ColorConfig::default_with_index(0),
        );
        let tile2 = TileConfig::new(
            array![[true, false], [false, true]],
            ColorConfig::default_with_index(1),
        );
        let tile3 = TileConfig::new(
            array![[false, true], [true, false]],
            ColorConfig::default_with_index(1),
        );

        let mut hasher1 = DefaultHasher::new();
        tile1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        tile2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        let mut hasher3 = DefaultHasher::new();
        tile3.hash(&mut hasher3);
        let hash3 = hasher3.finish();

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    fn test_hash_slice_any_order() {
        let tile1 = TileConfig::new(
            array![[true, false], [false, true]],
            ColorConfig::default_with_index(0),
        );
        let tile2 = TileConfig::new(
            array![[false, true], [true, false]],
            ColorConfig::default_with_index(1),
        );

        let mut hasher1 = DefaultHasher::new();
        TileConfig::hash_slice(&[tile1.clone(), tile2.clone()], &mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        TileConfig::hash_slice(&[tile2, tile1], &mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2);
    }
}
