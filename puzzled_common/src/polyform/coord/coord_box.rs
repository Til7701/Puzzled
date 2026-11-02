use crate::polyform::coord::Coord;
use crate::polyform::coord::hex::HexCoord;
use crate::polyform::coord::regular::RegularCoord;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CoordBox {
    Regular(RegularCoord),
    Hex(HexCoord),
}

impl Coord for CoordBox {
    fn zero() -> Self {
        panic!("Not allowed");
    }

    fn one() -> Self {
        panic!("Not allowed");
    }

    fn rotate_counterclockwise(&mut self, viewport: &Self) {
        match (self, viewport) {
            (CoordBox::Regular(regular), CoordBox::Regular(v)) => {
                regular.rotate_counterclockwise(v)
            }
            (CoordBox::Hex(hex), CoordBox::Hex(v)) => hex.rotate_counterclockwise(v),
            _ => unreachable!(),
        }
    }

    fn flip_default(&mut self, viewport: &Self) {
        match (self, viewport) {
            (CoordBox::Regular(regular), CoordBox::Regular(v)) => regular.flip_default(v),
            (CoordBox::Hex(hex), CoordBox::Hex(v)) => hex.flip_default(v),
            _ => unreachable!(),
        }
    }

    fn transpose(&mut self) {
        match self {
            CoordBox::Regular(regular) => regular.transpose(),
            CoordBox::Hex(hex) => hex.transpose(),
        }
    }

    fn area(&self) -> usize {
        match self {
            CoordBox::Regular(regular) => regular.area(),
            CoordBox::Hex(hex) => hex.area(),
        }
    }
}

impl<'a> From<RegularCoord> for CoordBox {
    fn from(value: RegularCoord) -> Self {
        CoordBox::Regular(value)
    }
}

impl<'a> From<HexCoord> for CoordBox {
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

impl Add for &CoordBox {
    type Output = CoordBox;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (CoordBox::Regular(s), CoordBox::Regular(r)) => (s + r).into(),
            (CoordBox::Hex(s), CoordBox::Hex(r)) => (s + r).into(),
            _ => unreachable!(),
        }
    }
}

impl Sub for &CoordBox {
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
            (CoordBox::Hex(s), CoordBox::Hex(r)) => todo!(),
            _ => unreachable!(),
        }
    }
}
