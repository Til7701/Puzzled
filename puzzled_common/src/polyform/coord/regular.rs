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

    #[must_use]
    pub fn with_x(&self, x: u32) -> Self {
        RegularCoord { x, ..*self }
    }

    pub fn y(&self) -> u32 {
        self.y
    }

    #[must_use]
    pub fn with_y(&self, y: u32) -> Self {
        RegularCoord { y, ..*self }
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

    fn rotate_around(&self, prime: Self, viewport: Self) -> Self {
        let x_wo_viewport = prime.x() as i32 + (self.y as i32 - prime.y() as i32);
        let y_wo_viewport = prime.y() as i32 + (self.x as i32 - prime.x() as i32);

        RegularCoord::new(
            x_wo_viewport.rem_euclid(viewport.x() as i32) as u32,
            y_wo_viewport.rem_euclid(viewport.y() as i32) as u32,
        )
    }

    fn flip_default(&self, prime: Self, viewport: Self) -> Self {
        let x_wo_viewport = prime.x() as i32 - (self.x as i32 + 1);
        let x = if self.x < prime.x {
            x_wo_viewport.rem_euclid(viewport.x() as i32)
        } else if self.x > prime.x {
            x_wo_viewport.rem_euclid(viewport.x() as i32)
        } else {
            self.x as i32
        } as u32;

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
