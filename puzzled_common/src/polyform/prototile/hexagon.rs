use crate::polyform::coord::Coord;
use crate::polyform::prototile::{MutPrototile, Prototile};
use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Hexagon<T, C>
where
    T: Clone + Hash,
    C: Coord,
{
    coord: C,
    orientation: HexagonOrientation,
    data: T,
}

impl<T, C> Hexagon<T, C>
where
    T: Clone + Hash,
    C: Coord,
{
    pub fn new(coord: C, orientation: HexagonOrientation, data: T) -> Self {
        Hexagon {
            coord,
            orientation,
            data,
        }
    }
}

impl<T, C> Prototile<T, C, HexagonOrientation> for Hexagon<T, C>
where
    T: Clone + Hash,
    C: Coord,
{
    fn coord(&self) -> C {
        self.coord
    }

    fn orientation(&self) -> HexagonOrientation {
        self.orientation
    }

    fn data(&self) -> &T {
        &self.data
    }
}

impl<T, C> MutPrototile<T, C, HexagonOrientation> for Hexagon<T, C>
where
    T: Clone + Hash,
    C: Coord,
{
    fn set_coord(&mut self, coord: C) {
        self.coord = coord;
    }

    fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    fn set_data(&mut self, data: T) {
        self.data = data;
    }

    fn rotate_counterclockwise(&mut self, prime: C, viewport: C) {
        self.coord.rotate_around(prime, viewport);
    }

    fn flip_default(&mut self, prime: C) {
        self.coord.flip_default(prime);
    }

    fn transpose(&mut self) {
        self.coord.transpose();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HexagonOrientation {
    OnSide,
    OnCorner,
}
