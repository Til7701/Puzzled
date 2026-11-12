use crate::coord::Coord;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

/// A regular coordinate with square elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RegularCoord {
    x: i32,
    y: i32,
}

impl RegularCoord {
    /// Creates a new `RegularCoord` with the given values.
    ///
    /// # Arguments
    ///
    /// * `x`: the first dimension
    /// * `y`: the second dimension
    ///
    /// returns: RegularCoord
    pub fn new(x: i32, y: i32) -> Self {
        RegularCoord { x, y }
    }

    /// Returns the first dimension
    pub fn x(&self) -> i32 {
        self.x
    }

    /// Creates a new `RegularCoord` with the given x as the first dimension
    /// and keeping the other values.
    #[must_use]
    pub fn with_x(&self, x: i32) -> Self {
        RegularCoord { x, ..*self }
    }

    /// Returns the second dimension
    pub fn y(&self) -> i32 {
        self.y
    }

    /// Creates a new `RegularCoord` with the given y as the second dimension
    /// and keeping the other values.
    #[must_use]
    pub fn with_y(&self, y: i32) -> Self {
        RegularCoord { y, ..*self }
    }

    /// Returns the elementwise minimum of this and the other.
    pub fn min(&self, other: Self) -> Self {
        RegularCoord::new(self.x.min(other.x), self.y.min(other.y))
    }

    /// Returns the elementwise maximum of this and the other.
    pub fn max(&self, other: Self) -> Self {
        RegularCoord::new(self.x.max(other.x), self.y.max(other.y))
    }
}

impl Coord for RegularCoord {
    fn rotate_around(&self, prime: Self) -> Self {
        let dx = self.x - prime.x();
        let dy = self.y - prime.y();
        RegularCoord::new(prime.x() - dy, prime.y() + dx)
    }

    fn flip_default(&self, prime: Self) -> Self {
        let x = prime.x() - (self.x - prime.x());
        RegularCoord::new(x, self.y)
    }

    fn transpose(&self) -> Self {
        RegularCoord::new(self.y, self.x)
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
