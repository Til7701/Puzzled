use crate::polyform::grid::Coord;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Square<T>
where
    T: Clone,
{
    coord: Coord,
    orientation: SquareOrientation,
    data: T,
}

impl<T> Square<T>
where
    T: Clone,
{
    pub fn new(coord: Coord, orientation: SquareOrientation, data: T) -> Self {
        Square { coord, orientation, data }
    }

    pub fn coord(&self) -> &Coord {
        &self.coord
    }

    pub fn orientation(&self) -> SquareOrientation {
        self.orientation
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn rotate_counterclockwise(&mut self, viewport: Coord) {
        self.coord.rotate_counterclockwise(viewport);
    }

    pub fn flip_default(&mut self, viewport: Coord) {
        self.coord.flip_default(viewport);
    }

    pub fn transpose(&mut self) {
        self.coord.transpose();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SquareOrientation {
    OnSide,
    OnCorner,
}
