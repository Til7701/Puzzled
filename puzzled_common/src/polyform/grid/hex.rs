use crate::polyform::grid::Coord;

/// A grid for hexagons.
///
/// Could maybe be used for triangles as well.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct HexCoord {
    x: u32,
    y: u32,
    z: u32,
}

impl Coord for HexCoord {}
