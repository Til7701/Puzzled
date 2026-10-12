use crate::polyform::coord::Coord;
use crate::polyform::coord::coord_box::CoordBox;
use crate::polyform::prototile::{MutPrototile, Prototile};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Square<T>
where
    T: Clone,
{
    coord: CoordBox,
    orientation: SquareOrientation,
    data: T,
}

impl<T> Square<T>
where
    T: Clone,
{
    pub fn new(coord: CoordBox, orientation: SquareOrientation, data: T) -> Self {
        Square {
            coord,
            orientation,
            data,
        }
    }
}

impl<T> Prototile<T, CoordBox, SquareOrientation> for Square<T>
where
    T: Clone,
{
    fn coord(&self) -> &CoordBox {
        &self.coord
    }

    fn orientation(&self) -> SquareOrientation {
        self.orientation
    }

    fn data(&self) -> &T {
        &self.data
    }
}

impl<T> MutPrototile<T, CoordBox, SquareOrientation> for Square<T>
where
    T: Clone,
{
    fn set_coord(&mut self, coord: CoordBox) {
        self.coord = coord;
    }

    fn set_data(&mut self, data: T) {
        self.data = data;
    }

    fn rotate_counterclockwise(&mut self, viewport: &C) {
        self.coord.rotate_counterclockwise(viewport);
    }

    fn flip_default(&mut self, viewport: &C) {
        self.coord.flip_default(viewport);
    }

    fn transpose(&mut self) {
        self.coord.transpose();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SquareOrientation {
    OnSide,
    OnCorner,
}
