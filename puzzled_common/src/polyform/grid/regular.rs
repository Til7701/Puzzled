use std::fmt::{Display, Formatter};
use std::ops::Add;

/// A regular grid with square elements.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegularCoord {
    x: u32,
    y: u32,
}

impl RegularCoord {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
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

    pub fn rotate_counterclockwise(&mut self, viewport: &Self) {
        let old_x = self.x;
        let old_y = self.y;
        self.x = old_y;
        self.y = viewport.y - old_x;
    }

    pub fn flip_default(&mut self, viewport: &Self) {
        let old_x = self.x;
        let old_y = self.y;
        self.x = viewport.x - old_x;
        self.y = old_y;
    }

    pub fn transpose(&mut self) {
        let old_x = self.x;
        let old_y = self.y;
        self.x = old_x;
        self.y = old_y;
    }

    pub fn area(&self) -> usize {
        self.x as usize * self.y as usize
    }
}

impl Display for RegularCoord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "R({}, {})", self.x, self.y)
    }
}

impl Add for &RegularCoord {
    type Output = RegularCoord;

    fn add(self, rhs: Self) -> Self::Output {
        RegularCoord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
