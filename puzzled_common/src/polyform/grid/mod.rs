mod hex;
mod regular;

pub use hex::HexCoord;
pub use regular::RegularCoord;
use std::fmt::{Display, Formatter};
use std::ops::Add;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Coord {
    Regular(RegularCoord),
    Hex(HexCoord),
}

impl Coord {
    pub fn rotate_counterclockwise(&mut self, viewport: &Coord) {
        match (self, viewport) {
            (Coord::Regular(s), Coord::Regular(v)) => s.rotate_counterclockwise(v),
            (Coord::Hex(s), Coord::Hex(v)) => s.rotate_counterclockwise(v),
            _ => unreachable!(),
        }
    }

    pub fn flip_default(&mut self, viewport: &Coord) {
        match (self, viewport) {
            (Coord::Regular(s), Coord::Regular(v)) => s.flip_default(v),
            (Coord::Hex(s), Coord::Hex(v)) => s.flip_default(v),
            _ => unreachable!(),
        }
    }

    pub fn transpose(&mut self) {
        match self {
            Coord::Regular(s) => s.transpose(),
            Coord::Hex(s) => s.transpose(),
        }
    }

    pub fn area(&self) -> usize {
        match self {
            Coord::Regular(regular) => regular.area(),
            Coord::Hex(hex) => hex.area(),
        }
    }
}

impl<'a> From<RegularCoord> for Coord {
    fn from(value: RegularCoord) -> Self {
        Coord::Regular(value)
    }
}

impl<'a> From<HexCoord> for Coord {
    fn from(value: HexCoord) -> Self {
        Coord::Hex(value)
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Coord::Regular(regular) => regular.fmt(f),
            Coord::Hex(hex) => hex.fmt(f),
        }
    }
}

impl Add for &Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Coord::Regular(s), Coord::Regular(r)) => (s + r).into(),
            (Coord::Hex(s), Coord::Hex(r)) => (s + r).into(),
            _ => unreachable!(),
        }
    }
}
