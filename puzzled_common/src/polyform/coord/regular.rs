use crate::polyform::coord::Coord;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

/// A regular grid with square elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RegularCoord {
    x: u32,
    y: u32,
}

impl RegularCoord {
    pub fn new(x: u32, y: u32) -> Self {
        RegularCoord { x, y }
    }

    pub fn x(&self) -> u32 {
        self.x
    }

    pub fn set_x(&mut self, x: u32) {
        self.x = x;
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    pub fn set_y(&mut self, y: u32) {
        self.y = y;
    }

    pub fn min(&self, other: Self) -> Self {
        RegularCoord::new(self.x.min(other.x), self.y.min(other.y))
    }

    pub fn max(&self, other: Self) -> Self {
        RegularCoord::new(self.x.max(other.x), self.y.max(other.y))
    }

    pub fn add_tuple(&self, tuple: (u32, u32)) -> RegularCoord {
        RegularCoord::new(self.x + tuple.0, self.y + tuple.1)
    }
}

impl Coord for RegularCoord {
    fn zero() -> Self {
        Self::new(0, 0)
    }

    fn one() -> Self {
        Self::new(1, 1)
    }

    fn rotate_counterclockwise(&mut self, viewport: Self) {
        let old_x = self.x;
        let old_y = self.y;
        self.x = old_y;
        self.y = viewport.y - old_x;
    }

    fn flip_default(&mut self, viewport: Self) {
        let old_x = self.x;
        let old_y = self.y;
        self.x = viewport.x - old_x;
        self.y = old_y;
    }

    fn transpose(&mut self) {
        let old_x = self.x;
        let old_y = self.y;
        self.x = old_y;
        self.y = old_x;
    }

    fn area(&self) -> usize {
        self.x as usize * self.y as usize
    }
}

impl Display for RegularCoord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "R({}, {})", self.x, self.y)
    }
}

impl Add for RegularCoord {
    type Output = RegularCoord;

    fn add(self, rhs: Self) -> Self::Output {
        RegularCoord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for RegularCoord {
    type Output = RegularCoord;

    fn sub(self, rhs: Self) -> Self::Output {
        RegularCoord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl PartialOrd for &RegularCoord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.x.cmp(&other.x).then(self.y.cmp(&other.y)).into()
    }
}
