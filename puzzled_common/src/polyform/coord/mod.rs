pub mod coord_box;
pub mod hex;
pub mod regular;

pub trait Coord: Clone + Copy {
    fn zero() -> Self;

    fn one() -> Self;

    fn rotate_around(&self, prime: Self, viewport: Self) -> Self;

    fn flip_default(&self, prime: Self, viewport: Self) -> Self;

    fn transpose(&self) -> Self;

    fn area(&self) -> usize;
}
