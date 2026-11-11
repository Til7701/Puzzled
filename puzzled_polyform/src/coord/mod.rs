use std::hash::Hash;

pub mod coord_box;
pub mod hex;
pub mod regular;

/// A coordinate in any coordinate system with copy semantics.
pub trait Coord: Clone + Copy + Hash + PartialEq + Eq {
    /// Returns a new coordinate which represents the position this coordinate would be in
    /// when rotated around the given prime.
    ///
    /// # Arguments
    ///
    /// * `prime`: the coordinate to rotate around
    ///
    /// returns: Self
    fn rotate_around(&self, prime: Self) -> Self;

    /// Returns a new coordinate which represents the position this coordinate would be in
    /// when flipped around the given prime.
    ///
    /// # Arguments
    ///
    /// * `prime`: the coordinate to flip with respect to
    ///
    /// returns: Self
    fn flip_default(&self, prime: Self) -> Self;

    /// Returns a new coordinate that represents the transposed coordinate of this one.
    fn transpose(&self) -> Self;

    /// Returns the amount of cells spanned by this one when looking from the origin.
    fn area(&self) -> usize;
}
