use crate::coord::Coord;
use crate::coord::hex::HexCoord;
use crate::coord::regular::RegularCoord;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

/// A box to wrap any coordinate type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CoordBox {
    Regular(RegularCoord),
    Hex(HexCoord),
}

impl Coord for CoordBox {
    fn rotate_around(&self, prime: Self) -> Self {
        match (self, prime) {
            (CoordBox::Regular(regular), CoordBox::Regular(p)) => regular.rotate_around(p).into(),
            (CoordBox::Hex(hex), CoordBox::Hex(p)) => hex.rotate_around(p).into(),
            _ => unreachable!(),
        }
    }

    fn flip_default(&self, prime: Self) -> Self {
        match (self, prime) {
            (CoordBox::Regular(regular), CoordBox::Regular(p)) => regular.flip_default(p).into(),
            (CoordBox::Hex(hex), CoordBox::Hex(p)) => hex.flip_default(p).into(),
            _ => unreachable!(),
        }
    }

    fn transpose(&self) -> Self {
        match self {
            CoordBox::Regular(regular) => regular.transpose().into(),
            CoordBox::Hex(hex) => hex.transpose().into(),
        }
    }

    fn area(&self) -> usize {
        match self {
            CoordBox::Regular(regular) => regular.area(),
            CoordBox::Hex(hex) => hex.area(),
        }
    }
}

impl From<RegularCoord> for CoordBox {
    fn from(value: RegularCoord) -> Self {
        CoordBox::Regular(value)
    }
}

impl From<HexCoord> for CoordBox {
    fn from(value: HexCoord) -> Self {
        CoordBox::Hex(value)
    }
}

impl Display for CoordBox {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CoordBox::Regular(regular) => regular.fmt(f),
            CoordBox::Hex(hex) => hex.fmt(f),
        }
    }
}

impl Add for CoordBox {
    type Output = CoordBox;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (CoordBox::Regular(s), CoordBox::Regular(r)) => (s + r).into(),
            (CoordBox::Hex(s), CoordBox::Hex(r)) => (s + r).into(),
            _ => unreachable!(),
        }
    }
}

impl Sub for CoordBox {
    type Output = CoordBox;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (CoordBox::Regular(s), CoordBox::Regular(r)) => (s - r).into(),
            (CoordBox::Hex(s), CoordBox::Hex(r)) => (s - r).into(),
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for &CoordBox {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (CoordBox::Regular(s), CoordBox::Regular(r)) => s.partial_cmp(&r),
            (CoordBox::Hex(_s), CoordBox::Hex(_r)) => todo!(),
            _ => unreachable!(),
        }
    }
}

impl Default for CoordBox {
    fn default() -> Self {
        CoordBox::Regular(RegularCoord::new(0, 0))
    }
}
