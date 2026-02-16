use ndarray::Array2;

/// Represents a successful solution to the puzzle.
#[derive(Debug)]
pub struct Solution {
    placements: Vec<TilePlacement>,
}

impl Solution {
    /// Creates a new `Solution` with the given tile placements.
    pub(crate) fn new(placements: Vec<TilePlacement>) -> Self {
        Self { placements }
    }

    /// Returns a reference to the tile placements in the solution.
    pub fn placements(&self) -> &[TilePlacement] {
        &self.placements
    }
}

/// Represents the placement of a tile at a specific position in the puzzle.
#[derive(Debug)]
pub struct TilePlacement {
    /// The base of the tile being placed.
    base: Array2<bool>,
    /// The rotation in which the tile is placed.
    rotation: Array2<bool>,
    /// The (x, y) position where the tile is placed.
    position: (usize, usize),
}

impl TilePlacement {
    /// Creates a new `TilePlacement` with the given base, rotation, and position.
    pub(crate) fn new(
        base: Array2<bool>,
        rotation: Array2<bool>,
        position: (usize, usize),
    ) -> Self {
        Self {
            base,
            rotation,
            position,
        }
    }

    /// Returns a reference to the base layout of the tile.
    pub fn base(&self) -> &Array2<bool> {
        &self.base
    }

    /// Returns a reference to the rotation of the tile as placed.
    pub fn rotation(&self) -> &Array2<bool> {
        &self.rotation
    }

    /// Returns the (x, y) position of the tile.
    pub fn position(&self) -> (usize, usize) {
        self.position
    }
}

/// Represents the reason why a puzzle is unsolvable.
///
/// Currently, the only reason is `NoFit`, indicating that no tiles can fit in the remaining spaces.
///
/// In the future, more reasons can be added as needed.
#[derive(Debug)]
pub enum UnsolvableReason {
    NoFit,
    BoardTooLarge,
}
