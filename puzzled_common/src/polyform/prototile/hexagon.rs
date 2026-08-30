use crate::polyform::grid::Coord;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hexagon<T>
where
    T: Default + Clone,
{
    coord: Coord,
    orientation: HexagonOrientation,
    data: T,
}

impl<T> Hexagon<T>
where
    T: Default + Clone,
{
    pub fn new(coord: Coord, orientation: HexagonOrientation, data: T) -> Self {
        Hexagon { coord, orientation, data }
    }

    pub fn coord(&self) -> &Coord {
        &self.coord
    }

    pub fn orientation(&self) -> HexagonOrientation {
        self.orientation
    }

    pub fn data(&self) -> &T {
        &self.data
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HexagonOrientation {
    OnSide,
    OnCorner,
}
