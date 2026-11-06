use crate::polyform::coord::coord_box::CoordBox;
use crate::polyform::prototile::{Hexagon, MutPrototile, Orientation, Prototile, Square};
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum PrototileMutRef<'a, T>
where
    T: Clone + Hash,
{
    Square(&'a mut Square<T>),
    Hexagon(&'a mut Hexagon<T>),
}
impl<'a, T> Prototile<T, CoordBox, Orientation> for PrototileMutRef<'a, T>
where
    T: Clone + Hash,
{
    fn coord(&self) -> CoordBox {
        match self {
            PrototileMutRef::Square(square) => square.coord(),
            PrototileMutRef::Hexagon(hexagon) => hexagon.coord(),
        }
    }

    fn orientation(&self) -> Orientation {
        match self {
            PrototileMutRef::Square(square) => square.orientation().into(),
            PrototileMutRef::Hexagon(hexagon) => hexagon.orientation().into(),
        }
    }

    fn data(&self) -> &T {
        match self {
            PrototileMutRef::Square(square) => square.data(),
            PrototileMutRef::Hexagon(hexagon) => hexagon.data(),
        }
    }
}
impl<'a, T> MutPrototile<T, CoordBox, Orientation> for PrototileMutRef<'a, T>
where
    T: Clone + Hash,
{
    fn set_coord(&mut self, coord: CoordBox) {
        match self {
            PrototileMutRef::Square(square) => square.set_coord(coord),
            PrototileMutRef::Hexagon(hexagon) => hexagon.set_coord(coord),
        }
    }

    fn data_mut(&mut self) -> &mut T {
        match self {
            PrototileMutRef::Square(square) => square.data_mut(),
            PrototileMutRef::Hexagon(hexagon) => hexagon.data_mut(),
        }
    }

    fn set_data(&mut self, data: T) {
        match self {
            PrototileMutRef::Square(square) => square.set_data(data),
            PrototileMutRef::Hexagon(hexagon) => hexagon.set_data(data),
        }
    }

    fn rotate_counterclockwise(&mut self, prime: CoordBox, viewport: CoordBox) {
        match self {
            PrototileMutRef::Square(square) => square.rotate_counterclockwise(prime, viewport),
            PrototileMutRef::Hexagon(hexagon) => hexagon.rotate_counterclockwise(prime, viewport),
        }
    }

    fn flip_default(&mut self, prime: CoordBox, viewport: CoordBox) {
        match self {
            PrototileMutRef::Square(square) => square.flip_default(prime, viewport),
            PrototileMutRef::Hexagon(hexagon) => hexagon.flip_default(prime, viewport),
        }
    }

    fn transpose(&mut self) {
        match self {
            PrototileMutRef::Square(square) => square.transpose(),
            PrototileMutRef::Hexagon(hexagon) => hexagon.transpose(),
        }
    }
}

impl<'a, T> From<&'a mut Square<T>> for PrototileMutRef<'a, T>
where
    T: Clone + Hash,
{
    fn from(value: &'a mut Square<T>) -> Self {
        PrototileMutRef::Square(value)
    }
}

impl<'a, T> From<&'a mut Hexagon<T>> for PrototileMutRef<'a, T>
where
    T: Clone + Hash,
{
    fn from(value: &'a mut Hexagon<T>) -> Self {
        PrototileMutRef::Hexagon(value)
    }
}
