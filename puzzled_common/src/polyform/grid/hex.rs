use std::fmt::{Display, Formatter};

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
    pub fn rotate_counterclockwise(&mut self, viewport: Self) {
        todo!()
    }

    pub fn flip_default(&mut self, viewport: Self) {
        todo!()
    }

    pub fn transpose(&mut self) {
        todo!()
    }
}

impl Display for HexCoord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "H({}, {}, {})", self.x, self.y, self.z)
    }
}
