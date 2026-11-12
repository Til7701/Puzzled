use crate::coord::Coord;
use crate::prototile::{PrototileMutRef, PrototileRef};
use std::hash::Hash;

pub mod polyhex;
pub mod polyomino;

#[cfg(test)]
mod polyomino_tests;

/// All the operations a polyform with any data value should be able to do.
///
/// T: the datatype stored in the polyform
/// C: the coordinate system used by this polyform
pub trait Poly<T, C: Coord>: Clone
where
    T: Clone + Hash,
{
    /// Returns an immutable reference to the prototile at the specified coordinates if there is one.
    fn get(&self, coord: C) -> Option<PrototileRef<'_, T>>;

    /// Returns a mutable reference to the prototile at the specified coordinates if there is one.
    fn get_mut(&mut self, coord: C) -> Option<PrototileMutRef<'_, T>>;

    /// Returns the dimensions of the polyform.
    ///
    /// This is not representing the highest coordinate of cells. The dimensions may be larger
    /// than that.
    fn dim(&self) -> C;

    /// Returns the dimensions of the polyform in two dimensions.
    ///
    /// Prototiles should approximately be 1x1 and relative to other polyform of the same type
    /// the sizes should match.
    fn relative_cartesian_dim(&self) -> (f64, f64);

    /// Returns the amount of prototiles covered by the dimensions.
    fn area(&self) -> usize;

    /// Rotates the polyform to be higher than wide.
    fn rotate_to_landscape(&mut self);

    /// Rotates the prototile around the prime.
    ///
    /// The prime is the coordinate of a prototile. It may not exist.
    ///
    /// Returns a movement request. If the tile is moved according to the movement request, the
    /// prime prototile will be in the same place as before the rotation.
    fn rotate_around(&mut self, prime: C) -> C;

    /// Flips the prototile around the prime along any valid axis.
    ///
    /// The prime is the coordinate of a prototile. It may not exist.
    ///
    /// Returns a movement request. If the tile is moved according to the movement request, the
    /// prime prototile will be in the same place as before the flipping.
    fn flip(&mut self, prime: C) -> C;

    /// Transposes the polyform.
    fn transpose(&mut self);

    /// Returns a new polyform of the same type, dimension and with the same prototiles,
    /// but with the data values mapped using the provided function.
    fn mapped<R: Clone + Hash, M>(&self, mapper: &M) -> impl Poly<R, C>
    where
        M: Fn(&T) -> R;

    /// Returns a new polyform of the same type, dimension and with the same prototiles,
    /// but with the data values mapped using the provided function.
    fn mapped_indexed<R: Clone + Hash, M>(&self, mapper: &M) -> impl Poly<R, C>
    where
        M: Fn(&T, C) -> R;

    /// Returns a new polyform of the same type and dimension, but prototiles are only
    /// kept, if Some value is returned for them. Otherwise, the prototile is dropped.
    fn filter_mapped<R: Clone + Hash, M>(&self, mapper: &M) -> impl Poly<R, C>
    where
        M: Fn(&T) -> Option<R>;

    /// Adds cells that are adjacent to existing cells. The new cells have the provided data
    /// value.
    fn extend_adjacent(&mut self, value: T);

    /// Trims the dimensions to match the contained prototiles.
    /// Prototiles are moved to the origin as far as possible.
    ///
    /// Returns the amound of cells removed at each side.
    fn trim(&mut self) -> TrimSides<C>;
}

pub trait UnitPoly: Sized + Clone {
    /// Places the given child polyform at every position is fits in this one without rotation
    /// or flipping.
    ///
    /// Both polyforms must be of the same type.
    fn place_on_all_positions(&self, child: &Self) -> Vec<Self>;

    /// Removes all cells that are present in the parent.
    fn remove_parent(&mut self, parent: &Self);

    /// Takes all cells that are withing the dimensions and not currently present in the polyform
    /// and creates a new polyform with them.
    fn inverted(&self) -> Self;

    /// Returns the polyform as a 2D array of 0 and 1 to be used in the puzzled_config.
    fn as_2d_slice(&self) -> String;

    /// Looks for the biggest area of connected cells and counts them.
    fn count_biggest_connected_area_of_cells(&self) -> usize;

    /// Looks for the smallest area of connected cells and counts them.
    fn count_smallest_connected_area_of_cells(&self) -> usize;

    /// Creates a polyform with cells where the slice is true.
    fn from_bool_slice<const N: usize>(slice: &[[bool; N]]) -> Self;
}

/// Represents the number of rows and columns removed from the sides of a polyform.
#[derive(Debug, PartialEq, Eq)]
pub struct TrimSides<C: Coord> {
    /// The number of rows removed from the lower side.
    pub negative: C,
    /// The number of columns removed from the higher side.
    pub positive: C,
}
