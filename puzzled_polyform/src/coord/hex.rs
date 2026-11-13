use crate::coord::Coord;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

/// A coordinate for hexagons.
///
/// See https://www.redblobgames.com/grids/hexagons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct HexCoord {
    q: i32,
    r: i32,
}

impl HexCoord {
    pub const fn new(q: i32, r: i32) -> Self {
        HexCoord { q, r }
    }

    pub const fn from_offset(x: i32, y: i32) -> Self {
        let parity = x & 1;
        let q = y - (x - parity) / 2;
        let r = x;
        HexCoord::new(q, r)
    }

    pub const fn q(&self) -> i32 {
        self.q
    }

    pub const fn r(&self) -> i32 {
        self.r
    }

    pub const fn s(&self) -> i32 {
        0 - self.q - self.r
    }
}

impl Coord for HexCoord {
    fn rotate_around(&self, prime: Self) -> Self {
        let vec = *self - prime;
        let rotated_vec = HexCoord::new(-vec.s(), -vec.q());
        rotated_vec + prime
    }

    fn flip_default(&self, prime: Self) -> Self {
        let vec = *self - prime;
        let flipped_vec = HexCoord::new(vec.s(), vec.r());
        flipped_vec + prime
    }

    fn transpose(&self) -> Self {
        HexCoord::new(self.r(), self.q())
    }

    fn area(&self) -> usize {
        ((self.q + 1) * (self.r + 1)) as usize
    }
}

impl Display for HexCoord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "H({}, {}, {})", self.q, self.r, self.s())
    }
}

impl Add for HexCoord {
    type Output = HexCoord;

    fn add(self, rhs: Self) -> Self::Output {
        HexCoord {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
        }
    }
}

impl Sub for HexCoord {
    type Output = HexCoord;

    fn sub(self, rhs: Self) -> Self::Output {
        HexCoord {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
        }
    }
}
