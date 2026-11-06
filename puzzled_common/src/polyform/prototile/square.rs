use crate::polyform::coord::Coord;
use crate::polyform::coord::coord_box::CoordBox;
use crate::polyform::prototile::{MutPrototile, Prototile};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Square<T>
where
    T: Clone + Hash,
{
    coord: CoordBox,
    orientation: SquareOrientation,
    data: T,
}

impl<T> Square<T>
where
    T: Clone + Hash,
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
    T: Clone + Hash,
{
    fn coord(&self) -> CoordBox {
        self.coord
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
    T: Clone + Hash,
{
    fn set_coord(&mut self, coord: CoordBox) {
        self.coord = coord;
    }

    fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    fn set_data(&mut self, data: T) {
        self.data = data;
    }

    fn rotate_counterclockwise(&mut self, prime: CoordBox, viewport: CoordBox) {
        self.coord = self.coord.rotate_around(prime, viewport);
    }

    fn flip_default(&mut self, prime: CoordBox, viewport: CoordBox) {
        self.coord = self.coord.flip_default(prime, viewport);
    }

    fn transpose(&mut self) {
        self.coord = self.coord.transpose();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SquareOrientation {
    OnSide,
    OnCorner,
}
