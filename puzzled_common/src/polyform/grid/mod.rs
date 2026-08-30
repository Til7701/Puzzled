mod hex;
mod regular;

use std::fmt::{Display, Formatter};
pub use hex::HexCoord;
pub use regular::RegularCoord;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Coord {
    Regular(RegularCoord),
    Hex(HexCoord),
}

impl Coord {
    pub fn rotate_counterclockwise(&mut self, viewport: Coord) {
        match (self, viewport) {
            (Coord::Regular(s), Coord::Regular(v)) => s.rotate_counterclockwise(v),
            (Coord::Hex(s), Coord::Hex(v)) => s.rotate_counterclockwise(v),
            _ => unreachable!()
        }
    }

    pub fn flip_default(&mut self, viewport: Coord) {
        match (self, viewport) {
            (Coord::Regular(s), Coord::Regular(v)) => s.flip_default(v),
            (Coord::Hex(s), Coord::Hex(v)) => s.flip_default(v),
            _ => unreachable!()
        }
    }

    pub fn transpose(&mut self) {
        match (self) {
            Coord::Regular(s) => s.transpose(),
            Coord::Hex(s) => s.transpose(),
        }
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Coord::Regular(regular) => { regular.fmt(f) }
            Coord::Hex(hex) => { hex.fmt(f) }
        }
    }
}
