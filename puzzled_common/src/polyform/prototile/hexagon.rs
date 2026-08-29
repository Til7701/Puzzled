use crate::polyform::grid::Coord;
use crate::polyform::prototile::{Orientation, Prototile};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hexagon<T, C: Coord>
where
    T: Default + Clone,
{
    coord: C,
    orientation: HexagonOrientation,
    data: T,
}

impl<T, C: Coord> Hexagon<T, C>
where
    T: Default + Clone,
{
    pub fn new(coord: C, orientation: HexagonOrientation, data: T) -> Self {
        Hexagon { coord, orientation, data }
    }
}

impl<T, C: Coord> Prototile<HexagonOrientation, C> for Hexagon<T, C>
where
    T: Default + Clone,
{
    fn orientation(&self) -> &HexagonOrientation {
        &self.orientation
    }

    fn coord(&self) -> &C {
        &self.coord
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HexagonOrientation {
    OnSide,
    OnCorner,
}

impl Orientation for HexagonOrientation {}
