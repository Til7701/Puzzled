use std::hash::Hash;

pub mod coord_box;
pub mod hex;
pub mod regular;

pub trait Coord: Clone + Copy + Hash + PartialEq + Eq {
    fn zero() -> Self;

    fn one() -> Self;

    fn rotate_around(&self, prime: Self) -> Self;

    fn flip_default(&self, prime: Self) -> Self;

    fn transpose(&self) -> Self;

    fn area(&self) -> usize;
}
