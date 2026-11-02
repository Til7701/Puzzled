use std::ops::{Add, Mul, Sub};

pub mod coord_box;
pub mod hex;
pub mod regular;

pub trait Coord {
    fn zero() -> Self;

    fn one() -> Self;

    fn rotate_counterclockwise(&mut self, viewport: &Self);

    fn flip_default(&mut self, viewport: &Self);

    fn transpose(&mut self);

    fn area(&self) -> usize;
}
