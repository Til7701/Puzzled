use crate::polyform::grid::Coord;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hexagon<T>
where
    T: Clone,
{
    coord: Coord,
    orientation: HexagonOrientation,
    data: T,
}

impl<T> Hexagon<T>
where
    T: Clone,
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

    pub fn set_data(&mut self, data: T) {
        self.data = data;
    }

    pub fn rotate_counterclockwise(&mut self, viewport: &Coord) {
        self.coord.rotate_counterclockwise(viewport);
    }

    pub fn flip_default(&mut self, viewport: &Coord) {
        self.coord.flip_default(viewport);
    }

    pub fn transpose(&mut self) {
        self.coord.transpose();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HexagonOrientation {
    OnSide,
    OnCorner,
}
