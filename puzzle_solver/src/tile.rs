use crate::array_util::{debug_print, rotate_90};
use log::debug;
use ndarray::Array2;
use std::collections::HashSet;

/// Represents a tile to place on a board.
/// It is based on a 2D array of booleans, where `true` indicates the presence of a feature
/// (e.g., part of a puzzle piece) and `false` indicates its absence.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tile {
    /// The base 2D boolean array representing the tile.
    /// This is kept for convenience to give back to users who want the original base.
    pub(crate) base: Array2<bool>,
    /// All unique rotations and flips of the tile, containing the base orientation as well.
    pub(crate) all_rotations: Vec<Array2<bool>>,
}

impl Tile {
    /// Creates a new Tile from a base 2D boolean array.
    ///
    /// A cell containing `true` indicates the presence of a feature (e.g., part of a puzzle piece),
    /// while `false` indicates its absence.
    ///
    /// # Arguments
    ///
    /// * `base`: Array2<bool> - The base 2D boolean array representing the tile.
    ///
    /// returns: Tile
    ///
    /// # Examples
    ///
    /// ```rust
    /// use puzzle_solver::tile::Tile;
    /// use ndarray::arr2;
    ///
    /// let base = arr2(&[[true, false], [true, true]]);
    /// let tile = Tile::new(base);
    /// ```
    pub fn new(base: Array2<bool>) -> Tile {
        let mut all_rotations_set: HashSet<Array2<bool>> = HashSet::new();

        all_rotations_set.insert(base.clone());

        let mut tmp = rotate_90(&base);
        all_rotations_set.insert(tmp.clone());
        tmp = rotate_90(&tmp);
        all_rotations_set.insert(tmp.clone());
        tmp = rotate_90(&tmp);
        all_rotations_set.insert(tmp.clone());

        tmp = base.clone().reversed_axes();
        all_rotations_set.insert(tmp.clone());

        tmp = rotate_90(&tmp);
        all_rotations_set.insert(tmp.clone());
        tmp = rotate_90(&tmp);
        all_rotations_set.insert(tmp.clone());
        tmp = rotate_90(&tmp);
        all_rotations_set.insert(tmp.clone());

        let all_rotations = all_rotations_set.into_iter().collect();
        Tile {
            base,
            all_rotations,
        }
    }

    /// Returns a reference to the base 2D boolean array of the tile.
    /// This is the same array that was used to create the Tile.
    ///
    /// # Arguments
    ///
    /// returns: &Array2<bool>
    ///
    /// # Examples
    ///
    /// ```rust
    /// use puzzle_solver::tile::Tile;
    /// use ndarray::arr2;
    ///
    /// let base = arr2(&[[true, false], [true, true]]);
    /// let tile = Tile::new(base.clone());
    /// assert_eq!(tile.base(), &base);
    /// ```
    pub fn base(&self) -> &Array2<bool> {
        &self.base
    }

    /// Debug prints the tile's base and all its rotations.
    #[allow(dead_code)]
    pub(crate) fn debug_print(&self) {
        debug!("Tile Base: ");
        debug_print(&self.base);
        debug!("All Rotations: ");
        for rotation in &self.all_rotations {
            debug!("Rotation:");
            debug_print(rotation);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::arr2;

    #[test]
    fn test_new() {
        let base = arr2(&[[true, false], [true, true]]);
        let tile = Tile::new(base.clone());

        assert_eq!(tile.base(), &base);
        assert_eq!(tile.all_rotations.len(), 4);
        assert!(
            tile.all_rotations
                .contains(&arr2(&[[true, false], [true, true]]))
        );
        assert!(
            tile.all_rotations
                .contains(&arr2(&[[true, true], [true, false]]))
        );
        assert!(
            tile.all_rotations
                .contains(&arr2(&[[true, true], [false, true]]))
        );
        assert!(
            tile.all_rotations
                .contains(&arr2(&[[false, true], [true, true]]))
        );
    }

    #[test]
    fn test_new_1x1() {
        let base = arr2(&[[true]]);
        let tile = Tile::new(base.clone());

        assert_eq!(tile.base(), &base);
        assert_eq!(tile.all_rotations.len(), 1);
        assert!(tile.all_rotations.contains(&arr2(&[[true]])));
    }

    #[test]
    fn test_new_1x2() {
        let base = arr2(&[[true, false]]);
        let tile = Tile::new(base.clone());

        assert_eq!(tile.base(), &base);
        assert_eq!(tile.all_rotations.len(), 4);
        assert!(tile.all_rotations.contains(&arr2(&[[true, false]])));
        assert!(tile.all_rotations.contains(&arr2(&[[false], [true]])));
        assert!(tile.all_rotations.contains(&arr2(&[[false, true]])));
        assert!(tile.all_rotations.contains(&arr2(&[[true], [false]])));
    }

    #[test]
    fn test_new_2x3() {
        let base = arr2(&[[true, false], [true, true], [true, true]]);
        let tile = Tile::new(base.clone());

        assert_eq!(tile.base(), &base);
        assert_eq!(tile.all_rotations.len(), 8);
        assert!(
            tile.all_rotations
                .contains(&arr2(&[[true, false], [true, true], [true, true]]))
        );
        assert!(
            tile.all_rotations
                .contains(&arr2(&[[true, true], [true, true], [false, true]]))
        );
        assert!(
            tile.all_rotations
                .contains(&arr2(&[[true, true], [true, true], [true, false]]))
        );
        assert!(
            tile.all_rotations
                .contains(&arr2(&[[false, true], [true, true], [true, true]]))
        );
        assert!(
            tile.all_rotations
                .contains(&arr2(&[[false, true, true], [true, true, true]]))
        );
        assert!(
            tile.all_rotations
                .contains(&arr2(&[[true, true, true], [false, true, true]]))
        );
        assert!(
            tile.all_rotations
                .contains(&arr2(&[[true, true, false], [true, true, true]]))
        );
        assert!(
            tile.all_rotations
                .contains(&arr2(&[[true, true, true], [true, true, false]]))
        );
    }
}
