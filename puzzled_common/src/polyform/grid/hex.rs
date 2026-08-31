use std::fmt::{Display, Formatter};
use std::ops::Add;

/// A grid for hexagons.
///
/// Could maybe be used for triangles as well.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HexCoord {
    x: u32,
    y: u32,
    z: u32,
}

impl HexCoord {
    pub fn rotate_counterclockwise(&mut self, viewport: &Self) {
        todo!()
    }

    pub fn flip_default(&mut self, viewport: &Self) {
        todo!()
    }

    pub fn transpose(&mut self) {
        todo!()
    }

    pub fn area(&self) -> usize {
        todo!()
    }
}

impl Display for HexCoord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "H({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add for &HexCoord {
    type Output = HexCoord;

    fn add(self, rhs: Self) -> Self::Output {
        HexCoord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
