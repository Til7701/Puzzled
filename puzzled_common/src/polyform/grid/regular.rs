use crate::polyform::grid::Coord;

/// A regular grid with square elements.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RegularCoord {
    x: u32,
    y: u32,
}

impl Coord for RegularCoord {}
