use log::debug;
use puzzled_common::Shape;
use std::collections::HashSet;

/// Represents a tile to place on a board.
/// It is based on a 2D array of booleans, where `true` indicates the presence of a feature
/// (e.g., part of a puzzle piece) and `false` indicates its absence.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tile {
    /// The base 2D boolean array representing the tile.
    /// This is kept for convenience to give back to users who want the original base.
    pub(crate) base: Shape,
    /// All unique rotations and flips of the tile, containing the base orientation as well.
    pub(crate) all_rotations: Vec<Shape>,
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
    /// use puzzled_common::shape::shape_square;
    ///
    /// let base = shape_square(&[[true, false], [true, true]]);
    /// let tile = Tile::new(base);
    /// ```
    pub fn new(base: Shape) -> Tile {
        let mut all_rotations_set: HashSet<Shape> = HashSet::new();

        base.rotations_flips_iter().for_each(|rotation| {
            all_rotations_set.insert(rotation);
        });

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
    /// use puzzled_common::shape::shape_square;
    ///
    /// let base = shape_square(&[[true, false], [true, true]]);
    /// let tile = Tile::new(base.clone());
    /// assert_eq!(tile.base(), &base);
    /// ```
    pub fn base(&self) -> &Shape {
        &self.base
    }

    /// Debug prints the tile's base and all its rotations.
    #[allow(dead_code)]
    pub(crate) fn debug_print(&self) {
        debug!("Tile Base: ");
        self.base.debug_print();
        debug!("All Rotations: ");
        for rotation in &self.all_rotations {
            debug!("Rotation:");
            rotation.debug_print();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use puzzled_common::shape::shape_square;

    #[test]
    fn test_new() {
        let base = shape_square(&[[true, false], [true, true]]);
        let tile = Tile::new(base.clone());

        assert_eq!(tile.base(), &base);
        assert_eq!(tile.all_rotations.len(), 4);
        assert!(
            tile.all_rotations
                .contains(&shape_square(&[[true, false], [true, true]]))
        );
        assert!(
            tile.all_rotations
                .contains(&shape_square(&[[true, true], [true, false]]))
        );
        assert!(
            tile.all_rotations
                .contains(&shape_square(&[[true, true], [false, true]]))
        );
        assert!(
            tile.all_rotations
                .contains(&shape_square(&[[false, true], [true, true]]))
        );
    }

    #[test]
    fn test_new_1x1() {
        let base = shape_square(&[[true]]);
        let tile = Tile::new(base.clone());

        assert_eq!(tile.base(), &base);
        assert_eq!(tile.all_rotations.len(), 1);
        assert!(tile.all_rotations.contains(&shape_square(&[[true]])));
    }

    #[test]
    fn test_new_1x2() {
        let base = shape_square(&[[true, false]]);
        let tile = Tile::new(base.clone());

        assert_eq!(tile.base(), &base);
        assert_eq!(tile.all_rotations.len(), 4);
        assert!(tile.all_rotations.contains(&shape_square(&[[true, false]])));
        assert!(
            tile.all_rotations
                .contains(&shape_square(&[[false], [true]]))
        );
        assert!(tile.all_rotations.contains(&shape_square(&[[false, true]])));
        assert!(
            tile.all_rotations
                .contains(&shape_square(&[[true], [false]]))
        );
    }

    #[test]
    fn test_new_2x3() {
        let base = shape_square(&[[true, false], [true, true], [true, true]]);
        let tile = Tile::new(base.clone());

        assert_eq!(tile.base(), &base);
        assert_eq!(tile.all_rotations.len(), 8);
        assert!(tile.all_rotations.contains(&shape_square(&[
            [true, false],
            [true, true],
            [true, true]
        ])));
        assert!(tile.all_rotations.contains(&shape_square(&[
            [true, true],
            [true, true],
            [false, true]
        ])));
        assert!(tile.all_rotations.contains(&shape_square(&[
            [true, true],
            [true, true],
            [true, false]
        ])));
        assert!(tile.all_rotations.contains(&shape_square(&[
            [false, true],
            [true, true],
            [true, true]
        ])));
        assert!(
            tile.all_rotations
                .contains(&shape_square(&[[false, true, true], [true, true, true]]))
        );
        assert!(
            tile.all_rotations
                .contains(&shape_square(&[[true, true, true], [false, true, true]]))
        );
        assert!(
            tile.all_rotations
                .contains(&shape_square(&[[true, true, false], [true, true, true]]))
        );
        assert!(
            tile.all_rotations
                .contains(&shape_square(&[[true, true, true], [true, true, false]]))
        );
    }
}
